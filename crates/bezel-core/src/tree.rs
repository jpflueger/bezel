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
}

#[derive(Debug)]
pub struct Node {
    pub el: Element,
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,
    pub style: Style,
    pub listens: Vec<EventKind>,
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
            },
        );
        be.create(id, el);
        id
    }

    pub fn destroy(&mut self, id: NodeId, be: &mut dyn Backend) {
        if let Some(n) = self.nodes.remove(&id) {
            for c in n.children {
                self.destroy(c, be);
            }
            if let Some(p) = n.parent.and_then(|p| self.nodes.get_mut(&p)) {
                p.children.retain(|c| *c != id);
            }
            be.destroy(id);
        }
    }

    pub fn set_root(&mut self, id: NodeId, be: &mut dyn Backend) {
        self.root = Some(id);
        be.mount(id);
    }

    /// Apply one batch atomically: validate everything first, then mutate.
    pub fn apply(&mut self, ops: Vec<Op>, be: &mut dyn Backend) -> Result<(), TreeError> {
        for op in &ops {
            self.validate(op)?;
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
                    if let Some(pn) = self.nodes.get_mut(&p) {
                        pn.children.retain(|x| *x != c);
                    }
                    if let Some(cn) = self.nodes.get_mut(&c) {
                        cn.parent = None;
                    }
                    be.remove(p, c);
                }
            }
        }
        be.commit();
        Ok(())
    }

    fn validate(&self, op: &Op) -> Result<(), TreeError> {
        let known = |id: NodeId| {
            self.nodes
                .contains_key(&id)
                .then_some(())
                .ok_or(TreeError::UnknownNode(id))
        };
        match op {
            Op::Set(id, prop) => {
                known(*id)?;
                applicable(self.nodes[id].el, prop)
            }
            Op::Append(p, c) => {
                known(*p)?;
                known(*c)?;
                self.no_cycle(*p, *c)
            }
            Op::Insert(p, c, b) => {
                known(*p)?;
                known(*c)?;
                known(*b)?;
                self.no_cycle(*p, *c)
            }
            Op::Remove(p, c) => {
                known(*p)?;
                known(*c)
            }
        }
    }

    fn no_cycle(&self, parent: NodeId, child: NodeId) -> Result<(), TreeError> {
        let mut cur = Some(parent);
        while let Some(id) = cur {
            if id == child {
                return Err(TreeError::Cycle(child));
            }
            cur = self.nodes[&id].parent;
        }
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
        t.destroy(a, &mut be);
        let b = t.create(Element::Box, &mut be);
        assert_ne!(a, b);
    }
}
