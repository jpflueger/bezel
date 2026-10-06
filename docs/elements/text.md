# `text`

Static text. Wraps within its box; inherits font/color.

## Props
See `wit/tree.wit`. Only the props listed there apply; setting an inapplicable prop is a batch error.

## Events
Listed per element in `docs/elements.md` (generated). `text` follows the semantic-events-only rule (ADR-0003).

## Webview host mapping
One DOM node created by `runtime/dom`; resolved style applied as inline CSS custom properties. No app-supplied HTML ever.

## Native host mapping
Masonry `Label` (Parley layout). AccessKit `Label`/`StaticText` with the text as name.

## Accessibility
Role, name and state as described above must appear in the platform tree on every host; verified by the conformance a11y diff.
