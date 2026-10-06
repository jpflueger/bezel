# `divider`

Visual separator (horizontal in column parents, vertical in row parents).

## Props
See `wit/tree.wit`. Only the props listed there apply; setting an inapplicable prop is a batch error.

## Events
Listed per element in `docs/elements.md` (generated). `divider` follows the semantic-events-only rule (ADR-0003).

## Webview host mapping
One DOM node created by `runtime/dom`; resolved style applied as inline CSS custom properties. No app-supplied HTML ever.

## Native host mapping
Vello line/rect using `border-color`. AccessKit `Splitter`/none (decorative).

## Accessibility
Role, name and state as described above must appear in the platform tree on every host; verified by the conformance a11y diff.
