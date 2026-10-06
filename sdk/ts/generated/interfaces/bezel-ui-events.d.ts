/** @module Interface bezel:ui/events@0.1.0 **/
/**
 * WASI 0.3 native async. The host is the single event loop.
 */
export function subscribe(): AsyncIterable<Event>;
export type EventKind = import('./bezel-ui-tree.js').EventKind;
export interface KeyInfo {
  key: string,
  ctrl: boolean,
  alt: boolean,
  shift: boolean,
  meta: boolean,
}
export interface Range {
  start: number,
  end: number,
}
export interface Event {
  node: number,
  kind: EventKind,
  text?: string,
  /**
   * change/submit: current value
   */
  key?: KeyInfo,
  /**
   * key
   */
  rows?: Range,
  /**
   * request-rows: window the host wants materialised
   */
  size?: [number, number],
  /**
   * resize
   * Monotonic milliseconds from the host's clock.
   */
  at: bigint,
}
