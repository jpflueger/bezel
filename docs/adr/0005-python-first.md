# ADR-0005: Python is the beachhead language; Rust is the dogfood SDK; TypeScript third

- **Status:** accepted (conditional on the Phase 0 spike)
- **Date:** 2026-09-10
- **Deciders:** @justin
- **PRD / issues:** S-1, S-2, S-6, H-4 · E4, E5, E12

## Context
Rust developers already have Tauri, egui, Iced and Slint; C# has Avalonia and MAUI; Go has Wails and Fyne. Python has the largest developer population with no good cross-platform desktop story: internal tools ship as Streamlit apps, Jupyter widgets, Tkinter, or a local Flask server and a browser tab. componentize-py produces components from Python today, at the cost of bundling CPython (tens of MB) and a slower cold start.

## Decision
We will build and market the Python SDK first, with the Rust SDK developed in parallel as the fastest way to iterate the contract and as the host author's own tool. TypeScript follows in 0.3 to make clear the project is not anti-web and to attract DOM-runtime contributors. No fourth SDK is maintained by the core project before 1.0.

## Alternatives considered
- **Rust first** — natural for the author, but enters a saturated market and would attract the same Rust-GUI audience as five other projects.
- **TypeScript first** — cheapest via jco and web-familiar, but competes head-on with Tauri and Electron on their home turf.
- **Go first** — real demand, TinyGo emits components directly, but Wails/Fyne already serve it adequately.

## Consequences
Easier: a clear audience with real pain; a demo ("pip install, one file, three installers plus a web bundle") that spreads on its own. Harder: Python's packaged size and startup are release gates (≤ 40 MB, ≤ 900 ms to first paint); the SDK must hide async batching behind ordinary attribute assignment.

## Review trigger
The Phase 0 componentize-py spike (E5) misses the size or startup budget by more than 50% with no credible path. Then TypeScript becomes the beachhead and Python second.
