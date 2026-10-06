// world bezel:ui/app@0.1.0
export type * as BezelUiCapabilities010 from './interfaces/bezel-ui-capabilities.js'; // import bezel:ui/capabilities@0.1.0
export type * as BezelUiEvents010 from './interfaces/bezel-ui-events.js'; // import bezel:ui/events@0.1.0
export type * as BezelUiStyle010 from './interfaces/bezel-ui-style.js'; // import bezel:ui/style@0.1.0
export type * as BezelUiTree010 from './interfaces/bezel-ui-tree.js'; // import bezel:ui/tree@0.1.0
export type * as BezelUiWindow010 from './interfaces/bezel-ui-window.js'; // import bezel:ui/window@0.1.0
/**
* Called once after instantiation. Returns when the app decides to exit.
*/
export function run(): Promise<void>;
/**
* Optional: hot-reload state carry-over (see architecture §12).
*/
export function snapshot(): Uint8Array | undefined;
export function restore(state: Uint8Array): void;
export type Option<T> = { tag: 'none' } | { tag: 'some', val: T };
