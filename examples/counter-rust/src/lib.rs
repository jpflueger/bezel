use bezel::prelude::*;

#[derive(Default)]
struct State { n: u32, name: String }

struct Counter;
impl App for Counter {
    type State = State;
    fn view(s: &State) -> El {
        column([input("your name"), text(format!("{} · {}", s.name, s.n)), button("+1", 1)])
    }
    fn update(s: &mut State, ev: bezel::Event) {
        match (ev.kind, ev.node) { (bezel::EventKind::Click, _) => s.n += 1, (bezel::EventKind::Change, _) => s.name = ev.text.unwrap_or_default(), _ => {} }
    }
}
// bezel::export!(Counter);  // generates the `run` export once the SDK runtime layer exists (E4)
