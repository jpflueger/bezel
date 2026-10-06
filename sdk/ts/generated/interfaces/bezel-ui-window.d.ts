/** @module Interface bezel:ui/window@0.1.0 **/
export function setTitle(title: string): void;
export function setSize(s: Size): void;
export function currentSize(): Size;
/**
 * The host asks before closing; the app answers within one turn or the
 * host closes anyway.
 */
export function requestClose(allow: boolean): void;
export interface Size {
  width: number,
  height: number,
}
