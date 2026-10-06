# ADR-0003: The host owns a retained element tree; apps send batched ops and receive semantic events

- **Status:** accepted
- **Date:** 2026-09-10
- **Deciders:** @justin
- **PRD / issues:** C-4, C-5, C-6 · E2, E3

## Context
Every crossing of the Wasm boundary costs a canonical-ABI lift/lower plus copies for lists and strings; interpreted or GC'd guests add their own overhead per call. A 60 fps budget is 16 ms. Three protocol shapes were considered: immediate-mode draw commands per frame (thousands of crossings per frame), a guest-owned virtual tree with diff/patch (one reconciler per language), and a host-owned retained tree manipulated through handles.

## Decision
We will keep the element tree on the host. Apps hold `resource node` handles, mutate through `apply(list<op>)` (one crossing for N mutations, applied atomically before the next frame), and receive only semantic events (click, change, submit, focus, blur, key, resize, lifecycle) over a WASI 0.3 `stream<event>`. Hover, scroll, caret, IME, focus rings, animation and list virtualisation never cross the boundary. Host-owned state (input value, selection, scroll offset, checked) is readable synchronously via `node.value()`.

## Alternatives considered
- **Immediate mode over the wire** — simplest SDK, but a Python guest cannot hold 60 fps and there is no accessibility tree to expose.
- **Guest-owned tree with diff/patch** — good performance, but requires a reconciler in every SDK and two trees to keep in sync.

## Consequences
Easier: slow guests feel fast; one accessibility tree; SDKs can offer any declarative sugar (React-like, Flutter-like, Elm-like) that compiles to the same ops, exactly as the DOM supports many frameworks. Harder: the element vocabulary is the host's; custom widgets happen by composition and, from 0.3, a retained-display-list `canvas`. Reversal cost: very high after 0.1; this is the load-bearing decision.

## Review trigger
Phase 0 exit test: a 10k-row list scrolling at 60 fps with zero per-frame crossings. Failure re-opens this ADR before anything else is built.
