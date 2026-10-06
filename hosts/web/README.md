# Web host

`bezel pack --web` → `jco transpile app.wasm` → ES module + core wasm, bundled with `runtime/dom`. The `bezel:ui` imports are implemented in TypeScript against the DOM (E9). Capabilities: OPFS for fs, `fetch` allow-list for network, everything else `granted() = false`.
