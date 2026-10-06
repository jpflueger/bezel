//! `DomBackend`: implements bezel_core::Backend by emitting binary patch frames for the DOM runtime.
//! One patch buffer per `commit()`. No app logic. No HTML strings ever (ADR-0006).

use crate::protocol::{OpCode, PatchWriter};
use bezel_core::{Backend, Element, Event, NodeId, Prop, ResolvedStyle};

pub struct DomBackend {
    w: PatchWriter,
    pub outbox: std::sync::mpsc::Sender<Vec<u8>>,
    pub inbox: std::sync::mpsc::Receiver<Event>,
}

impl DomBackend {
    pub fn new(
        outbox: std::sync::mpsc::Sender<Vec<u8>>,
        inbox: std::sync::mpsc::Receiver<Event>,
    ) -> Self {
        Self {
            w: PatchWriter::default(),
            outbox,
            inbox,
        }
    }
}

impl Backend for DomBackend {
    fn mount(&mut self, root: NodeId) {
        self.w.op(OpCode::Mount).id(root);
    }
    fn create(&mut self, id: NodeId, el: Element) {
        self.w.op(OpCode::Create).id(id).u8(el as u8);
    }
    fn set_prop(&mut self, id: NodeId, prop: &Prop, style: Option<&ResolvedStyle>) {
        match prop {
            Prop::Style(_) => {
                if let Some(s) = style {
                    self.w.op(OpCode::Style).id(id).style(s);
                }
            }
            other => {
                self.w.op(OpCode::Prop).id(id).prop(other);
            }
        }
    }
    fn append(&mut self, p: NodeId, c: NodeId) {
        self.w.op(OpCode::Append).id(p).id(c);
    }
    fn insert(&mut self, p: NodeId, c: NodeId, b: NodeId) {
        self.w.op(OpCode::Insert).id(p).id(c).id(b);
    }
    fn remove(&mut self, p: NodeId, c: NodeId) {
        self.w.op(OpCode::Remove).id(p).id(c);
    }
    fn destroy(&mut self, id: NodeId) {
        self.w.op(OpCode::Destroy).id(id);
    }
    fn value(&self, _id: NodeId) -> String {
        // Phase 0 decision: synchronous IPC round-trip vs mirrored cache updated by change events.
        // Start with the mirrored cache (cheaper, no UI-thread block); measure staleness in tests.
        String::new()
    }
    fn commit(&mut self) {
        let frame = self.w.finish();
        if !frame.is_empty() {
            let _ = self.outbox.send(frame);
        }
    }
    fn poll_events(&mut self, out: &mut Vec<Event>) {
        while let Ok(e) = self.inbox.try_recv() {
            out.push(e);
        }
    }
}
