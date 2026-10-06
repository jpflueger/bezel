/** @module Interface bezel:ui/capabilities@0.1.0 **/
export function granted(c: Capability): boolean;
/**
 * Prompts the user if the manifest declares the capability as optional.
 */
export function request(c: Capability): PromiseLike<boolean>;
/**
 * # Variants
 * 
 * ## `"fs-read"`
 * 
 * ## `"fs-write"`
 * 
 * ## `"network"`
 * 
 * ## `"clipboard"`
 * 
 * ## `"notifications"`
 */
export type Capability = 'fs-read' | 'fs-write' | 'network' | 'clipboard' | 'notifications';
