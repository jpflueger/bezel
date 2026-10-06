//! Webview host (ADR-0002). See architecture §07.
//!
//! SKELETON: wires the process model; the DOM backend and channel encoding are Phase 0 work (E3).
//!
//! UI thread: tao event loop + wry webview loading the embedded DOM runtime (runtime/dom/dist/runtime.js).
//! App thread: Tokio runtime + Wasmtime store + bezel_core::Tree with `DomBackend`.
//! Channels: patches (app→UI, binary frames via `bezel://` custom scheme or `evaluate_script` with base64),
//!           events  (UI→app, wry IPC handler, binary frames).

#[allow(dead_code)] // not wired into the app thread until E3
mod backend;
#[allow(dead_code)]
mod protocol;

use anyhow::Result;

const RUNTIME_JS: &str = include_str!("../../../runtime/dom/dist/runtime.js"); // built by `npm run build` in runtime/dom
const SHELL_HTML: &str = r#"<!doctype html><html><head><meta charset="utf-8">
<meta http-equiv="Content-Security-Policy" content="default-src 'none'; script-src 'self' bezel:; style-src 'unsafe-inline'; img-src bezel: data:; font-src bezel:; connect-src 'none'">
</head><body><div id="root"></div><script src="bezel://runtime/runtime.js"></script></body></html>"#;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    // 1. Parse args: path to app.wasm/.cwasm, manifest, dev flags.
    // 2. Spawn the app thread: tokio runtime → bezel_core::runtime instantiate → run().await
    // 3. Build tao EventLoop + Window; wry WebViewBuilder:
    //      .with_custom_protocol("bezel", serve_runtime_and_assets)
    //      .with_ipc_handler(forward_binary_event_to_app_thread)
    //      .with_html(SHELL_HTML)
    // 4. Event loop: on patch from app thread → webview.evaluate_script(protocol::to_js_apply(patch))
    //                on window events → translate to bezel window events
    // 5. Crash isolation: app thread panics/traps → render error state in the webview; keep the window alive.
    let _ = (RUNTIME_JS, SHELL_HTML);
    eprintln!("bezel-host-webview: skeleton — implement per architecture §07 (issues E3)");
    Ok(())
}
