/** @module Interface bezel:ui/tree@0.1.0 **/
/**
 * One crossing for N mutations. The whole batch is validated first, then
 * applied atomically before the next frame. If any op is invalid, none is
 * applied and the first offending op is reported.
 */
export function apply(ops: Array<Op>): void;
/**
 * Mounts `root` as the window's content. The previous root, if any, is
 * unmounted and destroyed if the app no longer holds it. Setting the
 * current root again is a no-op. Fails with `invalid-child` if `root` has
 * a parent; the mounted root cannot be attached anywhere until replaced.
 */
export function setRoot(root: Node): void;
export type Style = import('./bezel-ui-style.js').Style;
/**
 * The complete v0.1 element set (ADR-0006). Every host must implement all
 * of them. Adding one is a minor version bump and requires
 * docs/elements/<name>.md with a "Native host mapping" section.
 * # Variants
 * 
 * ## `"box"`
 * 
 * Generic layout container. The only element whose `display`,
 * `direction`, `gap` and grid fields lay out its children.
 * ## `"text"`
 * 
 * Static text. Wraps within its box; inherits font and colour.
 * ## `"input"`
 * 
 * Single-line text input. Its text is host-owned.
 * ## `"textarea"`
 * 
 * Multi-line text input. Its text is host-owned.
 * ## `"button"`
 * 
 * Push button. Emits `click`.
 * ## `"checkbox"`
 * 
 * Boolean toggle. Its checked state is host-owned; emits `change`.
 * ## `"select"`
 * 
 * Single-choice dropdown over `options`. Its selection is host-owned;
 * emits `change`.
 * ## `"image"`
 * 
 * Raster image from a bezel-asset path.
 * ## `"scroll"`
 * 
 * Scrollable viewport around at most one child. Its offset is host-owned
 * and never reported per frame.
 * ## `"list"`
 * 
 * Virtualised list. The host asks for row windows with `request-rows`;
 * the app materialises them as children.
 * ## `"divider"`
 * 
 * Visual separator, horizontal in column parents and vertical in row
 * parents.
 */
export type Element = 'box' | 'text' | 'input' | 'textarea' | 'button' | 'checkbox' | 'select' | 'image' | 'scroll' | 'list' | 'divider';
/**
 * The semantic events a node can subscribe to with `prop.listen`. Payloads
 * are described by `events.event`. Nothing finer-grained (pointer motion,
 * scroll deltas, IME composition) is ever delivered.
 * # Variants
 * 
 * ## `"click"`
 * 
 * Activation by pointer, keyboard or assistive technology.
 * ## `"change"`
 * 
 * The host-owned value of input, textarea, checkbox or select changed.
 * ## `"submit"`
 * 
 * Enter in an input.
 * ## `"focus"`
 * 
 * ## `"blur"`
 * 
 * ## `"key"`
 * 
 * A key press while the node has focus.
 * ## `"resize"`
 * 
 * The node's laid-out size changed.
 * ## `"request-rows"`
 * 
 * A list wants rows materialised for a window.
 */
export type EventKind = 'click' | 'change' | 'submit' | 'focus' | 'blur' | 'key' | 'resize' | 'request-rows';
/**
 * One entry of a `select`.
 */
export interface OptionItem {
  /**
   * Returned by `node.value` and carried by `change` when chosen.
   */
  value: string,
  /**
   * Shown to the user and used as the accessible name.
   */
  label: string,
}
/**
 * A property set on a node, either through `node.set` or `op.set`. Each
 * case lists the elements it applies to; setting it on any other element
 * is an `inapplicable-prop` error.
 */
export type Prop = PropText | PropPlaceholder | PropEnabled | PropChecked | PropSrc | PropAlt | PropLabel | PropOptions | PropSelected | PropRowCount | PropStyle | PropListen | PropUnlisten;
/**
 * text, button, input, textarea. On input and textarea this replaces the
 * host-owned value and does not emit `change`.
 */
export interface PropText {
  tag: 'text',
  val: string,
}
/**
 * input, textarea: hint shown while the value is empty.
 */
export interface PropPlaceholder {
  tag: 'placeholder',
  val: string,
}
/**
 * Any element. A disabled element and its descendants are not focusable
 * and emit no `click`, `change`, `submit`, `key`, `focus` or `blur`;
 * `resize` and `request-rows` are unaffected. Default true.
 */
export interface PropEnabled {
  tag: 'enabled',
  val: boolean,
}
/**
 * checkbox: replaces the host-owned checked state without emitting
 * `change`. Default false.
 */
export interface PropChecked {
  tag: 'checked',
  val: boolean,
}
/**
 * image: a bezel-asset path, never a URL to remote content.
 */
export interface PropSrc {
  tag: 'src',
  val: string,
}
/**
 * image: the accessible name.
 */
export interface PropAlt {
  tag: 'alt',
  val: string,
}
/**
 * Any element: overrides the accessible name the host would derive.
 */
export interface PropLabel {
  tag: 'label',
  val: string,
}
/**
 * select: the full set of choices, replacing any previous set.
 */
export interface PropOptions {
  tag: 'options',
  val: Array<OptionItem>,
}
/**
 * select: replaces the host-owned selection with the option whose
 * `value` matches, without emitting `change`. No match clears it.
 */
export interface PropSelected {
  tag: 'selected',
  val: string,
}
/**
 * list: the total number of rows. The host requests windows of them
 * through `request-rows`.
 */
export interface PropRowCount {
  tag: 'row-count',
  val: number,
}
/**
 * Any element: merged into the node's style. Fields left as `none` keep
 * their current value.
 */
export interface PropStyle {
  tag: 'style',
  val: Style,
}
/**
 * Any element: deliver events of this kind for this node.
 */
export interface PropListen {
  tag: 'listen',
  val: EventKind,
}
/**
 * Any element: stop delivering events of this kind for this node.
 */
export interface PropUnlisten {
  tag: 'unlisten',
  val: EventKind,
}
/**
 * Why a mutation was rejected. Adding a case is a major version bump.
 * # Variants
 * 
 * ## `"inapplicable-prop"`
 * 
 * The prop does not apply to the node's element (see `prop`).
 * ## `"cycle"`
 * 
 * The op would make a node its own ancestor.
 * ## `"invalid-child"`
 * 
 * The parent does not accept this child: only box and list take any
 * number of children, scroll takes at most one, and every other element
 * takes none. The mounted root cannot be attached anywhere, and a node
 * with a parent cannot become the root.
 */
export type ErrorCode = 'inapplicable-prop' | 'cycle' | 'invalid-child';
/**
 * A rejected mutation. When a batch fails, nothing in it was applied.
 */
export interface TreeError {
  /**
   * Position of the offending op in the batch; 0 for `node` methods and
   * `set-root`.
   */
  index: number,
  /**
   * `node.id` of the node the offending op targeted (the parent, for
   * structural ops).
   */
  node: number,
  code: ErrorCode,
}
/**
 * The host-owned state `node.value` reports.
 */
export type NodeValue = NodeValueNone | NodeValueText | NodeValueChecked | NodeValueSelected | NodeValueScrollOffset;
/**
 * Elements without host-owned state.
 */
export interface NodeValueNone {
  tag: 'none',
}
/**
 * input, textarea: the current text.
 */
export interface NodeValueText {
  tag: 'text',
  val: string,
}
/**
 * checkbox: whether it is checked.
 */
export interface NodeValueChecked {
  tag: 'checked',
  val: boolean,
}
/**
 * select: the `value` of the chosen option, if any.
 */
export interface NodeValueSelected {
  tag: 'selected',
  val: string | undefined,
}
/**
 * scroll: the offset as (x, y) in logical pixels.
 */
export interface NodeValueScrollOffset {
  tag: 'scroll-offset',
  val: [number, number],
}
/**
 * One mutation. Structural ops name the parent first.
 */
export type Op = OpSet | OpAppend | OpInsert | OpRemove;
/**
 * (target, prop): set a prop on target.
 */
export interface OpSet {
  tag: 'set',
  val: [Node, Prop],
}
/**
 * (parent, child): move child to the end of parent's children, first
 * detaching it from any current parent.
 */
export interface OpAppend {
  tag: 'append',
  val: [Node, Node],
}
/**
 * (parent, child, before): as `append`, but place child immediately
 * before `before`. If `before` is not a child of parent, child is
 * appended.
 */
export interface OpInsert {
  tag: 'insert',
  val: [Node, Node, Node],
}
/**
 * (parent, child): detach child from parent; it is destroyed if the app
 * no longer holds it. A no-op if child is not a child of parent.
 */
export interface OpRemove {
  tag: 'remove',
  val: [Node, Node],
}

export class Node {
  /**
  * Creates a detached node with no props set.
  */
  constructor(el: Element)
  /**
  * Stable for the life of the resource and never reused in a session.
  * `events.event.node` carries this id.
  */
  id(): number;
  /**
  * Same as `apply([op.set((this, p))])`.
  */
  set(p: Prop): void;
  /**
  * Same as `apply([op.append((this, child))])`.
  */
  append(child: Node): void;
  /**
  * Same as `apply([op.insert((this, child, before))])`.
  */
  insert(child: Node, before: Node): void;
  /**
  * Same as `apply([op.remove((this, child))])`.
  */
  remove(child: Node): void;
  /**
  * Host-owned state, readable synchronously without a pending event.
  */
  value(): NodeValue;
}
