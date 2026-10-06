<p><img src="brand/bezel-mark.svg" width="72" alt=""></p>

# Bezel

**Your language. Any frame.**

Bezel runs desktop and web apps written as WebAssembly components. You write the app in Python, Rust or TypeScript; Bezel runs it on Windows, macOS, Linux and in the browser. There is no JavaScript in your app and no Chromium in the download.

> **Status: pre-alpha scaffold.** This repository was bootstrapped from a feasibility study, an architecture document and a PRD (see `docs/`). `bezel-core` compiles and its tests pass; the hosts, CLI and SDKs are skeletons with the real design encoded in comments and issues. Nothing here runs an app yet. Start with `docs/adr/` and the [issues](https://github.com/jpflueger/bezel/issues) (epics are labelled `type:epic`).

## How it works

An app is a component that targets the `bezel:ui` WIT world (`wit/`). It describes its interface as a tree of about a dozen CSS-shaped elements, mutates that tree in batches, and receives semantic events. A **host** owns the tree, lays it out, paints it and talks to the OS. The first host borrows the system webview; a later host owns its own GPU renderer; a browser host runs the same component from static files. Apps never know which host they are on.

```
pip install bezel
bezel new hello --sdk python
cd hello && bezel dev            # window opens; edits hot-reload
bezel pack                       # signed installers for 3 OSes + a web bundle
```

## Repository map

| Path | What |
|------|------|
| `wit/` | **The contract.** `bezel:ui@0.1` — tree, style, events, capabilities, window. Validated in CI; linted for host leaks. |
| `crates/bezel-core` | Host-agnostic core: retained tree, style resolution, event coalescing, capability policy, `Backend` trait. |
| `crates/bezel-host-webview` | Webview host (tao + wry + Wasmtime). Emits binary patches to `runtime/dom`. |
| `runtime/dom` | TypeScript DOM runtime shared by the Webview and Web hosts. No app logic. |
| `hosts/web` | Web host: jco output + the same DOM runtime. |
| `crates/bezel-cli` | `bezel new · dev · build · pack · doctor`. |
| `crates/bezel-conformance` | Golden trees, crossing budgets, screenshot and a11y diffs. |
| `sdk/{python,rust,ts}` | Idiomatic wrappers over generated bindings. |
| `examples/` | Counter in Python and Rust; todo and log-viewer to follow. |
| `docs/adr` · `docs/rfcs` · `docs/elements` | Decisions, proposals, per-element specs with Native host mappings. |
| `brand/` | Mark, wordmark, design tokens. |

## Why not Tauri?

Tauri is a good choice if your UI team already works in HTML and JavaScript; it is mature and well supported. Bezel is for teams that don't want a second language for the UI: your app is one component in Python, Rust or TypeScript, it runs on the desktop and in a browser from the same file, and it can only touch what its manifest declares.

## Licence

Dual-licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your option.
