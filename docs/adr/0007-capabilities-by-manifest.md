# ADR-0007: Capabilities are declared in bezel.toml, compiled into the world, enforced by the host, shown to the user

- **Status:** accepted
- **Date:** 2026-09-10
- **Deciders:** @justin
- **PRD / issues:** P-4, P-5, H-2, H-3 · E10

## Context
The app is untrusted code. Electron's default posture is weak; Tauri allow-lists commands. The Component Model already provides capability-by-import: a component can only call what its world imports. WASI 0.3 filesystem access is via preopens; HTTP via `wasi:http` can be allow-listed host-side.

## Decision
We will make `bezel.toml` the single declaration of capabilities. At build time the CLI prunes the world so undeclared WASI interfaces are not imported at all; at run time the host preopens only declared paths, allow-lists only declared hosts, and returns `false` from `granted()` for anything else. The host shows declared capabilities in plain language on first run and allows revocation. Apps run under epoch interruption and a memory limit; a trap or infinite loop yields an error state, never a frozen or dead host. The webview host's CSP forbids remote content, eval and any script other than the embedded runtime. No usage telemetry ships in hosts or CLI.

## Alternatives considered
- **Ambient WASI (full fs/network) with OS sandboxing only** — simpler, but loses the plain-language story and depends on per-OS sandbox quality.
- **Runtime prompts for every access** — fatiguing; declared-up-front plus revocation is what users understand from mobile platforms.

## Consequences
Easier: a one-screen security review for locked-down environments; a credible differentiator versus Electron. Harder: apps must declare paths and hosts up front; the Web host maps capabilities to OPFS and fetch with a different fidelity, which the docs must state.

## Review trigger
Any conformance security test failing, or any request to add an "allow all" capability. The latter is denied by default and requires an ADR to reconsider.
