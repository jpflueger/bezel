# Contributing to Bezel

Thanks for being here. Three things to know before your first PR.

**1. The contract is the product.** Anything under `wit/` is a public interface with a stability promise. Changes there need an issue using the "Contract change" template and, from 0.3, an RFC (`docs/rfcs/`). Every element needs `docs/elements/<name>.md` with a Native host mapping; `scripts/contract-lint.py` enforces it and CI runs it.

**2. Hosts must stay equivalent.** If you change `runtime/dom`, `bezel-host-webview` or a future host, the conformance suite (`crates/bezel-conformance`) must still pass on all hosts. Never add a convenience only one host can honour. No HTML, CSS text or JavaScript ever crosses the app boundary.

**3. Decisions are written down.** Load-bearing choices live in `docs/adr/`. If your PR contradicts one, propose a new ADR that supersedes it rather than working around it.

## Setup

```
rustup default stable
pip install componentize-py
npm i -g @bytecodealliance/jco
cargo install wit-deps-cli wasm-tools
(cd runtime/dom && npm install && npm run build)
cargo test --workspace
python scripts/contract-lint.py .
```

Linux needs `libwebkit2gtk-4.1-dev libgtk-3-dev`. macOS and Windows need nothing extra for the Webview host.

## Good first issues

Filtered on the tracker: `label:"good first issue"`. Each is scoped to one evening, has a named file, and a named test.

## Style

`cargo fmt`, `cargo clippy -D warnings`, `ruff`. Prose in docs follows `brand/` voice rules: sentence case, no exclamation marks, name the host in every screenshot caption.

## Licence

MIT OR Apache-2.0, at your option. By contributing you agree your contribution is licensed the same way.
