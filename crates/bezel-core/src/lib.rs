//! bezel-core — the host-agnostic heart of Bezel (ADR-0001, ADR-0003).
//!
//! Layering (top to bottom):
//!   guest (`bezel:ui/app`)  →  [`runtime`] (wasmtime bindings)  →  [`tree`] + [`style`] + [`events`]
//!   →  [`Backend`] (implemented by each host crate).
//!
//! STATUS: skeleton generated at project bootstrap. Compiles conceptually; not yet built in CI.

pub mod events;
pub mod policy;
#[cfg(feature = "wasmtime")]
pub mod runtime;
pub mod style;
pub mod tree;

pub use events::{Event, EventKind};
pub use style::{ResolvedStyle, Style, Theme};
pub use tree::{Element, NodeId, Op, Prop, Tree};

/// The only thing that differs between the Webview, Web and Native hosts.
///
/// Rules for implementors:
/// * Never resolve styles: [`ResolvedStyle`] is absolute. Map it 1:1.
/// * Never generate per-frame events. Hover, scroll, caret, IME stay inside the host.
/// * `commit` is called once per `apply` batch; batch DOM/widget work there.
pub trait Backend {
    /// Show `root` as the window's content, replacing any previous root.
    fn mount(&mut self, root: NodeId);
    fn create(&mut self, id: NodeId, el: Element);
    fn set_prop(&mut self, id: NodeId, prop: &Prop, style: Option<&ResolvedStyle>);
    fn append(&mut self, parent: NodeId, child: NodeId);
    fn insert(&mut self, parent: NodeId, child: NodeId, before: NodeId);
    fn remove(&mut self, parent: NodeId, child: NodeId);
    fn destroy(&mut self, id: NodeId);
    /// Host-owned state: input text, select value, checkbox, scroll offset.
    fn value(&self, id: NodeId) -> String;
    fn commit(&mut self);
    fn poll_events(&mut self, out: &mut Vec<Event>);
}

/// A backend that records calls; used by golden-tree tests and as a reference.
#[derive(Default)]
pub struct RecordingBackend {
    pub log: Vec<String>,
    pub values: std::collections::HashMap<NodeId, String>,
}

impl Backend for RecordingBackend {
    fn mount(&mut self, root: NodeId) {
        self.log.push(format!("mount {root:?}"));
    }
    fn create(&mut self, id: NodeId, el: Element) {
        self.log.push(format!("create {id:?} {el:?}"));
    }
    fn set_prop(&mut self, id: NodeId, prop: &Prop, _s: Option<&ResolvedStyle>) {
        self.log.push(format!("set {id:?} {prop:?}"));
    }
    fn append(&mut self, p: NodeId, c: NodeId) {
        self.log.push(format!("append {p:?} {c:?}"));
    }
    fn insert(&mut self, p: NodeId, c: NodeId, b: NodeId) {
        self.log.push(format!("insert {p:?} {c:?} before {b:?}"));
    }
    fn remove(&mut self, p: NodeId, c: NodeId) {
        self.log.push(format!("remove {p:?} {c:?}"));
    }
    fn destroy(&mut self, id: NodeId) {
        self.log.push(format!("destroy {id:?}"));
    }
    fn value(&self, id: NodeId) -> String {
        self.values.get(&id).cloned().unwrap_or_default()
    }
    fn commit(&mut self) {
        self.log.push("commit".into());
    }
    fn poll_events(&mut self, _out: &mut Vec<Event>) {}
}
