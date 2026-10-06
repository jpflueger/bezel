//! Semantic events (ADR-0003). Mirrors wit/events.wit.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventKind {
    Click,
    Change,
    Submit,
    Focus,
    Blur,
    Key,
    Resize,
    RequestRows,
}

#[derive(Clone, Debug, PartialEq)]
pub struct KeyInfo {
    pub key: String,
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub meta: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Event {
    pub node: u32,
    pub kind: EventKind,
    pub text: Option<String>,
    pub key: Option<KeyInfo>,
    pub rows: Option<(u32, u32)>,
    pub size: Option<(f32, f32)>,
    pub at: u64,
}

/// Coalesces per-frame duplicates (e.g. several `change` events on one node) so the
/// guest sees at most one per node per kind per frame.
#[derive(Default)]
pub struct Coalescer {
    pending: Vec<Event>,
}

impl Coalescer {
    pub fn push(&mut self, e: Event) {
        if matches!(e.kind, EventKind::Change | EventKind::Resize) {
            if let Some(slot) = self
                .pending
                .iter_mut()
                .rev()
                .find(|p| p.node == e.node && p.kind == e.kind)
            {
                *slot = e;
                return;
            }
        }
        self.pending.push(e);
    }
    pub fn drain(&mut self) -> Vec<Event> {
        std::mem::take(&mut self.pending)
    }
}
