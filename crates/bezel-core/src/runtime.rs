//! Wasmtime glue: instantiates `bezel:ui/app`, implements the host side of the
//! contract over [`crate::Tree`] and a [`crate::Backend`].
//!
//! SKELETON: the `bindgen!` invocation and Host impls follow the shape in the
//! feasibility study §10 / architecture §05. Fill in during Phase 0 (E2).
//!
//! ```ignore
//! wasmtime::component::bindgen!({ world: "app", path: "../../wit", async: true });
//!
//! pub struct HostState<B: Backend> { table: ResourceTable, wasi: WasiCtx, tree: Tree, backend: B, events: Coalescer }
//!
//! impl<B: Backend> bezel::ui::tree::HostNode for HostState<B> { /* new/set/append/insert/remove/value/drop */ }
//! impl<B: Backend> bezel::ui::tree::Host for HostState<B>     { /* apply(list<op>) → tree.apply */ }
//! impl<B: Backend> bezel::ui::events::Host for HostState<B>   { /* subscribe → stream fed by Coalescer::drain each frame */ }
//! impl<B: Backend> bezel::ui::capabilities::Host for HostState<B> { /* policy.granted */ }
//! ```
//!
//! Engine config: `Config::new().async_support(true).wasm_component_model_async(true).epoch_interruption(true)`.
//! Load precompiled `.cwasm` with `Component::deserialize_file` when available; fall back to `from_file` in dev.

pub struct RuntimeConfig {
    pub precompiled: bool,
    pub epoch_tick_ms: u64,
    pub memory_limit_bytes: usize,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        RuntimeConfig {
            precompiled: true,
            epoch_tick_ms: 10,
            memory_limit_bytes: 512 << 20,
        }
    }
}
