//! Conformance harness (E7). The mechanism that keeps hosts equivalent and the contract honest.
//!
//! * `golden`:  run an example against `RecordingBackend`, serialise the resolved tree after each scripted step,
//!              compare with `golden/<example>/<step>.json`.
//! * `budget`:  count Wasm crossings during scripted scroll/hover/type; any per-frame crossing fails.
//! * `shots`:   (CI only) screenshot real hosts on 3 OSes; per-host tolerance; known deltas recorded, not hidden.
//! * `a11y`:    dump UIA / AX / AT-SPI trees; diff roles, names, states across hosts.

use bezel_core::{NodeId, Tree};
use serde::Serialize;

#[derive(Serialize)]
pub struct Snapshot { pub id: u32, pub el: String, pub children: Vec<Snapshot> }

pub fn snapshot(tree: &Tree, id: NodeId) -> Snapshot {
    let n = tree.get(id).expect("node");
    Snapshot { id: id.0, el: format!("{:?}", n.el), children: n.children.iter().map(|c| snapshot(tree, *c)).collect() }
}

/// Counts boundary crossings; the host increments it in the wasmtime call hooks.
#[derive(Default)]
pub struct CrossingCounter { pub into_guest: u64, pub out_of_guest: u64, pub frames: u64 }
impl CrossingCounter {
    pub fn per_frame(&self) -> f64 { if self.frames == 0 { 0.0 } else { (self.into_guest + self.out_of_guest) as f64 / self.frames as f64 } }
}
