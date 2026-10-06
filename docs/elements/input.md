# `input`

Single-line text input. Value is host-owned; read via `value()`.

## Props
See `wit/tree.wit`. Only the props listed there apply; setting an inapplicable prop is a batch error.

## Events
Listed per element in `docs/elements.md` (generated). `input` follows the semantic-events-only rule (ADR-0003).

## Webview host mapping
One DOM node created by `runtime/dom`; resolved style applied as inline CSS custom properties. No app-supplied HTML ever.

## Native host mapping
Masonry `TextInput` (Parley `PlainEditor`, IME via ui-events). AccessKit `TextInput` with value, selection, placeholder.

## Accessibility
Role, name and state as described above must appear in the platform tree on every host; verified by the conformance a11y diff.
