# `list`

Virtualised list: `row-count` plus `request-rows(range)` events; rows are children materialised by the app.

## Props
See `wit/tree.wit`. Only the props listed there apply; setting an inapplicable prop is a batch error.

## Events
Listed per element in `docs/elements.md` (generated). `list` follows the semantic-events-only rule (ADR-0003).

## Webview host mapping
One DOM node created by `runtime/dom`; resolved style applied as inline CSS custom properties. No app-supplied HTML ever.

## Native host mapping
Masonry virtual list built on `Portal` with a lazy child provider; same request-rows protocol. AccessKit `List` with `ListItem` children and `size-of-set`/`position-in-set`.

## Accessibility
Role, name and state as described above must appear in the platform tree on every host; verified by the conformance a11y diff.
