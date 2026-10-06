# ADR-0001: The WIT contract is the product; hosts are replaceable

- **Status:** accepted
- **Date:** 2026-09-10
- **Deciders:** @justin
- **PRD / issues:** C-1, H-5, H-7 · E1, E7

## Context
The original plan led with a framework-owned GPU renderer. A renderer is the most expensive component to build and the last to become usable, which would leave nothing shippable for two to three years. Meanwhile the thing that actually differentiates the project — an app written in any language as a Wasm component that runs on desktop and web from one artifact — depends only on a stable interface between app and host, not on which host draws it.

Zed's extension system demonstrates that a Rust host plus a versioned WIT world supporting N−1 versions is a workable production pattern. WASI 0.3 (June 2026) provides native async streams, which fit a UI event loop directly.

## Decision
We will define `bezel:ui` as a versioned WIT world and treat it as the primary artifact of the project. Hosts implement a single `Backend` trait behind it; SDKs are thin idiomatic wrappers over generated bindings. No feature exists unless it is expressed in the contract, and no contract feature may name a DOM, webview or GPU concept. A conformance suite proves every host equivalent.

## Alternatives considered
- **Renderer-first (original plan)** — maximises long-term control but delays first users by years and risks designing the contract around one renderer's limits.
- **Framework-specific API per language, no shared contract** — faster for one language, but every subsequent language and host becomes a rewrite; this is the historical failure mode of polyglot toolkits.

## Consequences
Easier: adding hosts and languages; parallel contribution; a credible story that apps outlive any one host implementation. Harder: every new element or prop requires a written note on how each host (including the future Native host) implements it; the contract's evolution is slower than a single-host framework's would be. Reversal cost: low in Phase 0, high after 0.2 when third-party apps exist.

## Review trigger
If, by the end of 0.1, the conformance suite cannot keep two hosts equivalent for the three example apps without escape hatches, the contract is too thin or the hosts too different; re-open.
