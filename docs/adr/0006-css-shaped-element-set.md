# ADR-0006: A small, CSS-shaped element set instead of a widget library

- **Status:** accepted
- **Date:** 2026-09-10
- **Deciders:** @justin
- **PRD / issues:** C-2, C-3 · E1, E13

## Context
New UI toolkits stall when they ask developers to learn dozens of widgets and a bespoke styling model. Taffy implements flexbox and CSS Grid; every webview implements them natively; Masonry uses Taffy-compatible layout. The webview host maps a CSS-shaped model one-to-one, the future Native host implements the same spec with Taffy, and developers arrive already knowing it.

## Decision
We will ship roughly a dozen elements — `box, text, input, textarea, button, checkbox, select, image, scroll, list, divider` and, from 0.3, `canvas` — plus one `style` record whose layout half is the flexbox/grid subset Taffy supports and whose paint half is box model, colour, typography tokens, visibility and cursor. Rich components (data grids, editors, charts) are built from these by apps or the community, not added to the contract. Custom drawing uses a retained display list on `canvas`, re-sent only on change.

## Alternatives considered
- **Full widget library (tabs, trees, grids, date pickers) in the contract** — every widget must be implemented by every host forever; slows contract evolution and bloats hosts.
- **Bespoke layout model** — nothing to inherit from webviews or Taffy; every developer starts from zero.
- **Expose raw HTML/CSS in the webview host** — fastest for web developers, permanently forks the hosts, and makes the Native host second-class before it exists. Explicitly forbidden.

## Consequences
Easier: 1:1 host mappings; small conformance surface; transferable knowledge. Harder: some UIs need composition the author must write; a `canvas` escape hatch is required by 0.3. The contract lint enforces the denylist of DOM/CSS/JS identifiers so "CSS-shaped" never becomes "CSS-leaked".

## Review trigger
End of 0.1: if any example app or the author's daily-driver tool needed raw HTML to be reasonable, grow the element set via ADR — never open the escape hatch.
