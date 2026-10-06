# `select`

Single-choice dropdown; `options`, `selected`; emits `change`.

## Props
See `wit/tree.wit`. Only the props listed there apply; setting an inapplicable prop is a batch error.

## Events
Listed per element in `docs/elements.md` (generated). `select` follows the semantic-events-only rule (ADR-0003).

## Webview host mapping
One DOM node created by `runtime/dom`; resolved style applied as inline CSS custom properties. No app-supplied HTML ever.

## Native host mapping
Composite: Masonry button + popup list (or a native menu on macOS). AccessKit `ComboBox` + `ListBox` items.

## Accessibility
Role, name and state as described above must appear in the platform tree on every host; verified by the conformance a11y diff.
