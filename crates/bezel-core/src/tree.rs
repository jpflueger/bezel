//! Retained element tree. Ids are never reused within a session (ADR-0003).

use crate::style::{ResolvedStyle, Style, Theme};
use crate::{Backend, EventKind};
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NodeId(pub u32);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Element {
    Box,
    Text,
    Input,
    Textarea,
    Button,
    Checkbox,
    Select,
    Image,
    Scroll,
    List,
    Divider,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Prop {
    Text(String),
    Placeholder(String),
    Enabled(bool),
    Checked(bool),
    Src(String),
    Alt(String),
    Label(String),
    Options(Vec<(String, String)>),
    Selected(String),
    RowCount(u32),
    Style(Style),
    Listen(EventKind),
    Unlisten(EventKind),
}

// Mirrors the WIT `op` variant; revisit boxing `Prop` once batch throughput is measured.
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug)]
pub enum Op {
    Set(NodeId, Prop),
    Append(NodeId, NodeId),
    Insert(NodeId, NodeId, NodeId),
    Remove(NodeId, NodeId),
}

#[derive(Debug, thiserror::Error)]
pub enum TreeError {
    #[error("unknown node {0:?}")]
    UnknownNode(NodeId),
    #[error("prop {prop} not applicable to {el:?}")]
    Inapplicable { el: Element, prop: &'static str },
    #[error("cycle: {0:?} would become its own ancestor")]
    Cycle(NodeId),
    #[error("{child:?} cannot be a child of {parent:?}")]
    InvalidChild { parent: NodeId, child: NodeId },
    #[error("{0:?} is attached to a parent and cannot be the root")]
    RootHasParent(NodeId),
}

/// A rejected batch: the error and the position of the op that caused it.
#[derive(Debug, thiserror::Error)]
#[error("op {index}: {error}")]
pub struct BatchError {
    pub index: usize,
    pub error: TreeError,
}

#[derive(Debug)]
pub struct Node {
    pub el: Element,
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,
    pub style: Style,
    pub listens: Vec<EventKind>,
    /// The app still holds a handle. A node lives while it is held, attached
    /// or mounted as root (wit/tree.wit `resource node`).
    pub held: bool,
}

#[derive(Default)]
pub struct Tree {
    next: u32,
    nodes: HashMap<NodeId, Node>,
    root: Option<NodeId>,
    dirty_style: Vec<NodeId>,
    pub theme: Theme,
}

impl Tree {
    pub fn create(&mut self, el: Element, be: &mut dyn Backend) -> NodeId {
        self.next += 1;
        let id = NodeId(self.next);
        self.nodes.insert(
            id,
            Node {
                el,
                parent: None,
                children: vec![],
                style: Style::default(),
                listens: vec![],
                held: true,
            },
        );
        be.create(id, el);
        id
    }

    /// The app dropped its handle. The node is destroyed now if it is also
    /// detached and not the root; otherwise when it next becomes so.
    pub fn release(&mut self, id: NodeId, be: &mut dyn Backend) {
        if let Some(n) = self.nodes.get_mut(&id) {
            n.held = false;
            self.collect(id, be);
        }
    }

    /// Destroy `id` if nothing keeps it alive. Its children are detached and
    /// destroyed in turn unless the app still holds them.
    fn collect(&mut self, id: NodeId, be: &mut dyn Backend) {
        let dead = self.root != Some(id)
            && self
                .nodes
                .get(&id)
                .is_some_and(|n| !n.held && n.parent.is_none());
        if !dead {
            return;
        }
        let n = self.nodes.remove(&id).unwrap();
        for c in n.children {
            self.nodes.get_mut(&c).unwrap().parent = None;
            be.remove(id, c);
            self.collect(c, be);
        }
        be.destroy(id);
    }

    /// Mount `id` as the window's content, replacing (and possibly
    /// collecting) the previous root. Setting the current root is a no-op.
    pub fn set_root(&mut self, id: NodeId, be: &mut dyn Backend) -> Result<(), TreeError> {
        let node = self.nodes.get(&id).ok_or(TreeError::UnknownNode(id))?;
        if self.root == Some(id) {
            return Ok(());
        }
        if node.parent.is_some() {
            return Err(TreeError::RootHasParent(id));
        }
        let old = self.root.replace(id);
        be.mount(id);
        if let Some(old) = old {
            self.collect(old, be);
        }
        Ok(())
    }

    /// Apply one batch atomically: validate everything first, then mutate.
    pub fn apply(&mut self, ops: Vec<Op>, be: &mut dyn Backend) -> Result<(), BatchError> {
        let mut scratch = Scratch::new(self);
        for (index, op) in ops.iter().enumerate() {
            scratch
                .validate(op)
                .map_err(|error| BatchError { index, error })?;
        }
        for op in ops {
            match op {
                Op::Set(id, prop) => {
                    let node = self.nodes.get_mut(&id).unwrap();
                    match &prop {
                        Prop::Style(s) => {
                            node.style.merge(s);
                            self.dirty_style.push(id);
                        }
                        Prop::Listen(k) => node.listens.push(*k),
                        Prop::Unlisten(k) => node.listens.retain(|x| x != k),
                        _ => {}
                    }
                    let resolved = matches!(prop, Prop::Style(_)).then(|| self.resolve(id));
                    be.set_prop(id, &prop, resolved.as_ref());
                }
                Op::Append(p, c) => {
                    self.link(p, c, None);
                    be.append(p, c);
                }
                Op::Insert(p, c, b) => {
                    self.link(p, c, Some(b));
                    be.insert(p, c, b);
                }
                Op::Remove(p, c) => {
                    if self.nodes[&c].parent != Some(p) {
                        continue;
                    }
                    self.nodes.get_mut(&p).unwrap().children.retain(|x| *x != c);
                    self.nodes.get_mut(&c).unwrap().parent = None;
                    be.remove(p, c);
                    self.collect(c, be);
                }
            }
        }
        be.commit();
        Ok(())
    }

    fn link(&mut self, p: NodeId, c: NodeId, before: Option<NodeId>) {
        if let Some(old) = self.nodes[&c].parent {
            if let Some(o) = self.nodes.get_mut(&old) {
                o.children.retain(|x| *x != c);
            }
        }
        self.nodes.get_mut(&c).unwrap().parent = Some(p);
        let pn = self.nodes.get_mut(&p).unwrap();
        match before.and_then(|b| pn.children.iter().position(|x| *x == b)) {
            Some(i) => pn.children.insert(i, c),
            None => pn.children.push(c),
        }
    }

    /// Sparse style + theme + inheritance → absolute style. Backends never resolve.
    pub fn resolve(&self, id: NodeId) -> ResolvedStyle {
        let mut chain = vec![];
        let mut cur = Some(id);
        while let Some(n) = cur {
            chain.push(n);
            cur = self.nodes[&n].parent;
        }
        let mut out = ResolvedStyle::from_theme(&self.theme);
        for n in chain.iter().rev() {
            out.apply(&self.nodes[n].style, &self.theme);
        }
        out
    }

    pub fn listens(&self, id: NodeId, kind: EventKind) -> bool {
        self.nodes
            .get(&id)
            .is_some_and(|n| n.listens.contains(&kind))
    }
    pub fn root(&self) -> Option<NodeId> {
        self.root
    }
    pub fn get(&self, id: NodeId) -> Option<&Node> {
        self.nodes.get(&id)
    }
}

/// The batch as validation sees it: the tree's structure plus the effect of
/// every op validated so far, so a batch is checked against the state each op
/// would actually meet.
struct Scratch<'a> {
    tree: &'a Tree,
    parent: HashMap<NodeId, Option<NodeId>>,
    children: HashMap<NodeId, Vec<NodeId>>,
}

impl<'a> Scratch<'a> {
    fn new(tree: &'a Tree) -> Self {
        Self {
            tree,
            parent: HashMap::new(),
            children: HashMap::new(),
        }
    }

    fn known(&self, id: NodeId) -> Result<(), TreeError> {
        self.tree
            .nodes
            .contains_key(&id)
            .then_some(())
            .ok_or(TreeError::UnknownNode(id))
    }

    fn parent(&self, id: NodeId) -> Option<NodeId> {
        self.parent
            .get(&id)
            .copied()
            .unwrap_or(self.tree.nodes[&id].parent)
    }

    fn children_mut(&mut self, id: NodeId) -> &mut Vec<NodeId> {
        let tree = self.tree;
        self.children
            .entry(id)
            .or_insert_with(|| tree.nodes[&id].children.clone())
    }

    fn validate(&mut self, op: &Op) -> Result<(), TreeError> {
        match op {
            Op::Set(id, prop) => {
                self.known(*id)?;
                applicable(self.tree.nodes[id].el, prop)
            }
            Op::Append(p, c) | Op::Insert(p, c, _) => {
                self.known(*p)?;
                self.known(*c)?;
                if let Op::Insert(_, _, b) = op {
                    self.known(*b)?;
                }
                self.attach(*p, *c)
            }
            Op::Remove(p, c) => {
                self.known(*p)?;
                self.known(*c)?;
                if self.parent(*c) == Some(*p) {
                    self.children_mut(*p).retain(|x| x != c);
                    self.parent.insert(*c, None);
                }
                Ok(())
            }
        }
    }

    fn attach(&mut self, p: NodeId, c: NodeId) -> Result<(), TreeError> {
        let mut cur = Some(p);
        while let Some(id) = cur {
            if id == c {
                return Err(TreeError::Cycle(c));
            }
            cur = self.parent(id);
        }
        let invalid = TreeError::InvalidChild {
            parent: p,
            child: c,
        };
        if self.tree.root == Some(c) {
            return Err(invalid);
        }
        if let Some(old) = self.parent(c) {
            self.children_mut(old).retain(|x| *x != c);
        }
        self.parent.insert(c, Some(p));
        let kids = self.children_mut(p);
        kids.push(c);
        if kids.len() > max_children(self.tree.nodes[&p].el) {
            return Err(invalid);
        }
        Ok(())
    }
}

/// How many children each element accepts (wit/tree.wit `error-code.invalid-child`).
fn max_children(el: Element) -> usize {
    use Element::*;
    match el {
        Box | List => usize::MAX,
        Scroll => 1,
        Text | Input | Textarea | Button | Checkbox | Select | Image | Divider => 0,
    }
}

fn applicable(el: Element, prop: &Prop) -> Result<(), TreeError> {
    use Element::*;
    let ok = match prop {
        Prop::Text(_) => matches!(el, Text | Button | Input | Textarea),
        Prop::Placeholder(_) => matches!(el, Input | Textarea),
        Prop::Checked(_) => matches!(el, Checkbox),
        Prop::Src(_) | Prop::Alt(_) => matches!(el, Image),
        Prop::Options(_) | Prop::Selected(_) => matches!(el, Select),
        Prop::RowCount(_) => matches!(el, List),
        Prop::Enabled(_)
        | Prop::Label(_)
        | Prop::Style(_)
        | Prop::Listen(_)
        | Prop::Unlisten(_) => true,
    };
    if ok {
        Ok(())
    } else {
        Err(TreeError::Inapplicable {
            el,
            prop: prop_name(prop),
        })
    }
}

fn prop_name(p: &Prop) -> &'static str {
    match p {
        Prop::Text(_) => "text",
        Prop::Placeholder(_) => "placeholder",
        Prop::Enabled(_) => "enabled",
        Prop::Checked(_) => "checked",
        Prop::Src(_) => "src",
        Prop::Alt(_) => "alt",
        Prop::Label(_) => "label",
        Prop::Options(_) => "options",
        Prop::Selected(_) => "selected",
        Prop::RowCount(_) => "row-count",
        Prop::Style(_) => "style",
        Prop::Listen(_) => "listen",
        Prop::Unlisten(_) => "unlisten",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RecordingBackend;

    #[test]
    fn batch_is_atomic() {
        let mut t = Tree::default();
        let mut be = RecordingBackend::default();
        let root = t.create(Element::Box, &mut be);
        let txt = t.create(Element::Text, &mut be);
        let bad = Op::Set(txt, Prop::Checked(true)); // inapplicable → whole batch fails
        let res = t.apply(vec![Op::Append(root, txt), bad], &mut be);
        assert!(res.is_err());
        assert!(
            t.get(txt).unwrap().parent.is_none(),
            "nothing from the failed batch was applied"
        );
    }

    #[test]
    fn ids_are_never_reused() {
        let mut t = Tree::default();
        let mut be = RecordingBackend::default();
        let a = t.create(Element::Box, &mut be);
        t.release(a, &mut be);
        assert!(t.get(a).is_none());
        let b = t.create(Element::Box, &mut be);
        assert_ne!(a, b);
    }

    #[test]
    fn attached_nodes_outlive_their_handles() {
        let mut t = Tree::default();
        let mut be = RecordingBackend::default();
        let root = t.create(Element::Box, &mut be);
        let a = t.create(Element::Box, &mut be);
        let b = t.create(Element::Text, &mut be);
        let held = t.create(Element::Text, &mut be);
        t.set_root(root, &mut be).unwrap();
        t.apply(
            vec![Op::Append(root, a), Op::Append(a, b), Op::Append(a, held)],
            &mut be,
        )
        .unwrap();
        t.release(a, &mut be);
        t.release(b, &mut be);
        assert!(
            t.get(a).is_some() && t.get(b).is_some(),
            "attached keeps alive"
        );

        t.apply(vec![Op::Remove(root, a)], &mut be).unwrap();
        assert!(
            t.get(a).is_none() && t.get(b).is_none(),
            "detached and unheld"
        );
        assert!(
            t.get(held).unwrap().parent.is_none(),
            "held child survives, detached"
        );
    }

    #[test]
    fn remove_of_a_non_child_is_a_no_op() {
        let mut t = Tree::default();
        let mut be = RecordingBackend::default();
        let p = t.create(Element::Box, &mut be);
        let q = t.create(Element::Box, &mut be);
        let c = t.create(Element::Text, &mut be);
        t.apply(vec![Op::Append(q, c)], &mut be).unwrap();
        be.log.clear();
        t.apply(vec![Op::Remove(p, c)], &mut be).unwrap();
        assert_eq!(t.get(c).unwrap().parent, Some(q));
        assert_eq!(t.get(q).unwrap().children, vec![c]);
        assert_eq!(be.log, vec!["commit"]);
    }

    #[test]
    fn cycle_across_ops_fails_the_batch() {
        let mut t = Tree::default();
        let mut be = RecordingBackend::default();
        let a = t.create(Element::Box, &mut be);
        let b = t.create(Element::Box, &mut be);
        let err = t
            .apply(vec![Op::Append(a, b), Op::Append(b, a)], &mut be)
            .unwrap_err();
        assert_eq!(err.index, 1);
        assert!(matches!(err.error, TreeError::Cycle(_)));
        assert!(t.get(a).unwrap().parent.is_none() && t.get(b).unwrap().parent.is_none());
    }

    #[test]
    fn elements_limit_their_children() {
        let mut t = Tree::default();
        let mut be = RecordingBackend::default();
        let text = t.create(Element::Text, &mut be);
        let scroll = t.create(Element::Scroll, &mut be);
        let x = t.create(Element::Box, &mut be);
        let y = t.create(Element::Box, &mut be);
        let invalid = |r: Result<(), BatchError>| {
            matches!(
                r,
                Err(BatchError {
                    error: TreeError::InvalidChild { .. },
                    ..
                })
            )
        };
        assert!(invalid(t.apply(vec![Op::Append(text, x)], &mut be)));
        assert!(invalid(t.apply(
            vec![Op::Append(scroll, x), Op::Append(scroll, y)],
            &mut be
        )));
        // moving the only child out first makes room
        t.apply(vec![Op::Append(scroll, x)], &mut be).unwrap();
        t.apply(vec![Op::Remove(scroll, x), Op::Append(scroll, y)], &mut be)
            .unwrap();
    }

    #[test]
    fn set_root_edge_cases() {
        let mut t = Tree::default();
        let mut be = RecordingBackend::default();
        let r1 = t.create(Element::Box, &mut be);
        let r2 = t.create(Element::Box, &mut be);
        let c = t.create(Element::Box, &mut be);
        t.apply(vec![Op::Append(r2, c)], &mut be).unwrap();
        assert!(matches!(
            t.set_root(c, &mut be),
            Err(TreeError::RootHasParent(_))
        ));

        t.set_root(r1, &mut be).unwrap();
        t.release(r1, &mut be);
        assert!(t.get(r1).is_some(), "mounted keeps alive");
        assert!(t.apply(vec![Op::Append(r2, r1)], &mut be).is_err());
        be.log.clear();
        t.set_root(r1, &mut be).unwrap();
        assert!(be.log.is_empty(), "setting the current root is a no-op");

        t.set_root(r2, &mut be).unwrap();
        assert!(t.get(r1).is_none(), "replaced, unheld root is destroyed");
    }
}
