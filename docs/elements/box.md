# `box`

Generic layout container. The only element that takes grid/flex children semantics.

## Props
See `wit/tree.wit`. Only the props listed there apply; setting an inapplicable prop is a batch error.

## Events
Listed per element in `docs/elements.md` (generated). `box` follows the semantic-events-only rule (ADR-0003).

## Webview host mapping
One DOM node created by `runtime/dom`; resolved style applied as inline CSS custom properties. No app-supplied HTML ever.

## Native host mapping
Masonry `Flex`/`Grid` container widget with Taffy layout; paint background/border/radius via Vello `fill_rect`/`stroke`. No AccessKit role unless `label` is set (then `Group`).

## Accessibility
Role, name and state as described above must appear in the platform tree on every host; verified by the conformance a11y diff.
