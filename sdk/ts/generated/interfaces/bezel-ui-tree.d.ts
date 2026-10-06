/** @module Interface bezel:ui/tree@0.1.0 **/
/**
 * One crossing for N mutations. Applied atomically before the next frame.
 * An invalid op fails the whole batch; the failure is delivered as an event.
 */
export function apply(ops: Array<Op>): void;
export function setRoot(root: Node): void;
export type Style = import('./bezel-ui-style.js').Style;
/**
 * The complete v0.1 element set. Every host must implement all of them.
 * Adding one is a minor version bump and requires docs/elements/<name>.md
 * with a "Native host mapping" section.
 * # Variants
 * 
 * ## `"box"`
 * 
 * ## `"text"`
 * 
 * ## `"input"`
 * 
 * ## `"textarea"`
 * 
 * ## `"button"`
 * 
 * ## `"checkbox"`
 * 
 * ## `"select"`
 * 
 * ## `"image"`
 * 
 * ## `"scroll"`
 * 
 * ## `"list"`
 * 
 * ## `"divider"`
 */
export type Element = 'box' | 'text' | 'input' | 'textarea' | 'button' | 'checkbox' | 'select' | 'image' | 'scroll' | 'list' | 'divider';
/**
 * # Variants
 * 
 * ## `"click"`
 * 
 * ## `"change"`
 * 
 * ## `"submit"`
 * 
 * ## `"focus"`
 * 
 * ## `"blur"`
 * 
 * ## `"key"`
 * 
 * ## `"resize"`
 * 
 * ## `"request-rows"`
 */
export type EventKind = 'click' | 'change' | 'submit' | 'focus' | 'blur' | 'key' | 'resize' | 'request-rows';
export interface OptionItem {
  value: string,
  label: string,
}
export type Prop = PropText | PropPlaceholder | PropEnabled | PropChecked | PropSrc | PropAlt | PropLabel | PropOptions | PropSelected | PropRowCount | PropStyle | PropListen | PropUnlisten;
export interface PropText {
  tag: 'text',
  val: string,
}
export interface PropPlaceholder {
  tag: 'placeholder',
  val: string,
}
export interface PropEnabled {
  tag: 'enabled',
  val: boolean,
}
export interface PropChecked {
  tag: 'checked',
  val: boolean,
}
export interface PropSrc {
  tag: 'src',
  val: string,
}
/**
 * image: bezel-asset path, never a URL to remote content
 */
export interface PropAlt {
  tag: 'alt',
  val: string,
}
/**
 * image accessibility name
 */
export interface PropLabel {
  tag: 'label',
  val: string,
}
/**
 * accessibility name override
 */
export interface PropOptions {
  tag: 'options',
  val: Array<OptionItem>,
}
/**
 * select
 */
export interface PropSelected {
  tag: 'selected',
  val: string,
}
/**
 * select
 */
export interface PropRowCount {
  tag: 'row-count',
  val: number,
}
/**
 * list: total rows; host requests windows via request-rows
 */
export interface PropStyle {
  tag: 'style',
  val: Style,
}
export interface PropListen {
  tag: 'listen',
  val: EventKind,
}
/**
 * subscribe this node to an event kind
 */
export interface PropUnlisten {
  tag: 'unlisten',
  val: EventKind,
}
export type Op = OpSet | OpAppend | OpInsert | OpRemove;
export interface OpSet {
  tag: 'set',
  val: [Node, Prop],
}
export interface OpAppend {
  tag: 'append',
  val: [Node, Node],
}
export interface OpInsert {
  tag: 'insert',
  val: [Node, Node, Node],
}
export interface OpRemove {
  tag: 'remove',
  val: [Node, Node],
}

export class Node {
  constructor(el: Element)
  /**
  * Stable for the life of the resource; ids are never reused in a session.
  */
  id(): number;
  set(p: Prop): void;
  append(child: Node): void;
  insert(child: Node, before: Node): void;
  remove(child: Node): void;
  /**
  * Host-owned state, readable synchronously: input/textarea text,
  * select value, checkbox ("true"/"false"), scroll offset ("x,y").
  */
  value(): string;
}
