# ADR-0004: Build on the Component Model and WASI 0.3, not a bespoke ABI

- **Status:** accepted
- **Date:** 2026-09-10
- **Deciders:** @justin
- **PRD / issues:** C-7 · E1, E5, E12

## Context
A bespoke ABI over core Wasm offers full control of stability and the fastest possible bulk-data path (shared linear memory), but requires writing and maintaining a binding generator for every supported language. The Component Model provides typed interfaces (records, variants, resources), versioned worlds, capability-by-import, native async (WASI 0.3, June 2026), and toolchains maintained by the Bytecode Alliance for Rust, JS/TS, Go, Python and C#. It is pre-1.0; 0.2→0.3 changed the async model, and 0.3.x will add cooperative threads and lazy handles.

## Decision
We will express the entire control plane — tree ops, events, resources, capabilities, window — in WIT on the Component Model, targeting WASI 0.3 and Wasmtime ≥ 46. Nodes are WIT `resource`s so guest drops trigger host cleanup. We reserve one escape hatch for bulk data (a shared-buffer resource with an agreed binary layout), to be designed only after profiling in Phase 1 shows a need.

## Alternatives considered
- **Own ABI + FlatBuffers/Cap'n Proto over linear memory** — fastest bulk path, total stability control; but the polyglot promise becomes a treadmill of binding generators, historically fatal for polyglot toolkits.
- **WASI 0.2 for broader current toolchain support** — loses native async, which the event stream and futures depend on; 0.3 guest support is landing across toolchains through 2026.

## Consequences
Easier: language reach without owning toolchains; composition (wac) gives a plugin system almost free; shared vocabulary with Spin, wasmCloud, Zed. Harder: upstream churn until Component Model 1.0; occasional toolchain gaps (componentize-py size, TinyGo subset). Mitigations: version the world, support N−1, keep SDKs thin, upgrade Wasmtime quarterly rather than monthly. Reversal cost: high after 0.1.

## Review trigger
A third forced rewrite of the SDK layer due to upstream changes. Then: freeze on a Wasmtime LTS and stop tracking upstream minors.
