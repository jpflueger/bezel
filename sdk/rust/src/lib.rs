//! Bezel Rust SDK (E4). SKELETON.
//!
//! ```ignore
//! wit_bindgen::generate!({ world: "app", path: "../../wit", async: true });
//! ```
//! Design: `Element` builders queue ops into a thread-local batch; `flush()` sends one `apply`.
//! A small view-function layer (`view(&state) -> El`) diffs against retained handles. It is sugar; the protocol is the product.

pub mod prelude {
    pub use crate::{el::*, App};
}

pub trait App {
    type State: Default;
    fn view(state: &Self::State) -> el::El;
    fn update(state: &mut Self::State, ev: Event) {}
}

#[derive(Debug, Clone)]
pub struct Event { pub node: u32, pub kind: EventKind, pub text: Option<String> }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventKind { Click, Change, Submit, Focus, Blur, Key, Resize, RequestRows }

pub mod el {
    /// Declarative element description; diffed against host handles by the runtime layer.
    #[derive(Debug, Clone)]
    pub struct El { pub kind: &'static str, pub text: Option<String>, pub children: Vec<El>, pub on: Vec<(super::EventKind, u32)>, pub style: Option<Style> }
    #[derive(Debug, Clone, Default)]
    pub struct Style { pub direction: Option<&'static str>, pub gap: Option<f32>, pub padding: Option<f32> }
    pub fn column(children: impl IntoIterator<Item = El>) -> El { El { kind: "box", text: None, children: children.into_iter().collect(), on: vec![], style: Some(Style { direction: Some("column"), ..Default::default() }) } }
    pub fn text(s: impl Into<String>) -> El { El { kind: "text", text: Some(s.into()), children: vec![], on: vec![], style: None } }
    pub fn button(s: impl Into<String>, on_click: u32) -> El { El { kind: "button", text: Some(s.into()), children: vec![], on: vec![(super::EventKind::Click, on_click)], style: None } }
    pub fn input(placeholder: impl Into<String>) -> El { El { kind: "input", text: Some(placeholder.into()), children: vec![], on: vec![], style: None } }
}
