# ADR-0002: Ship a webview host first; the GPU host is gated on demand

- **Status:** accepted
- **Date:** 2026-09-10
- **Deciders:** @justin
- **PRD / issues:** H-1, H-4, H-7 · E3, E16

## Context
A system webview (WebView2, WKWebView, WebKitGTK via tao + wry) provides layout, text shaping, IME, accessibility and painting for free, at the cost of rendering differences between operating systems and dependence on each OS's webview cadence. A framework-owned renderer (Masonry + Vello + AccessKit) removes those differences but requires owning text editing, IME and accessibility parity, which is a multi-year effort even when building on Linebender's crates.

## Decision
We will ship the Webview host as the first and default host. The Native host (`bezel-host-native`) will not start until a documented gate is met: Webview-host users are reporting cross-OS rendering differences, webview bugs, or startup cost as blocking problems. When built, it must pass the full conformance suite before it can be selected without a flag.

## Alternatives considered
- **GPU host first** — the original plan; see ADR-0001 for why it delays adoption.
- **Native OS widgets** — best platform fidelity, but three widget models to map and custom widgets become escape-hatch hell; not compatible with a small CSS-shaped element set.
- **Both hosts in parallel from day one** — doubles Phase 0–1 effort for one person; the webview host alone proves the thesis.

## Consequences
Easier: public release in months; text, IME and accessibility deferred, not solved. Harder: "same pixels everywhere" cannot be claimed until the Native host exists; the docs must say so plainly. The webview host must never expose HTML or the hosts stop being equivalent (see ADR-0003). Reversal cost: none — the Native host is additive.

## Review trigger
Gate review at the start of each minor release from 0.3: collect evidence from issues and the opt-in survey; write an ADR to open or keep closed.
