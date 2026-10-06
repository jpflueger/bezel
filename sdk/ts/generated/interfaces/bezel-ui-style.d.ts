/** @module Interface bezel:ui/style@0.1.0 **/
/**
 * # Variants
 * 
 * ## `"flex"`
 * 
 * ## `"grid"`
 * 
 * ## `"none"`
 */
export type Display = 'flex' | 'grid' | 'none';
/**
 * # Variants
 * 
 * ## `"row"`
 * 
 * ## `"column"`
 */
export type Direction = 'row' | 'column';
/**
 * # Variants
 * 
 * ## `"no-wrap"`
 * 
 * ## `"wrap"`
 */
export type Wrap = 'no-wrap' | 'wrap';
/**
 * # Variants
 * 
 * ## `"start"`
 * 
 * ## `"center"`
 * 
 * ## `"end"`
 * 
 * ## `"stretch"`
 * 
 * ## `"baseline"`
 */
export type Align = 'start' | 'center' | 'end' | 'stretch' | 'baseline';
/**
 * # Variants
 * 
 * ## `"start"`
 * 
 * ## `"center"`
 * 
 * ## `"end"`
 * 
 * ## `"space-between"`
 * 
 * ## `"space-around"`
 * 
 * ## `"space-evenly"`
 */
export type Justify = 'start' | 'center' | 'end' | 'space-between' | 'space-around' | 'space-evenly';
/**
 * # Variants
 * 
 * ## `"visible"`
 * 
 * ## `"hidden"`
 * 
 * ## `"scroll"`
 * 
 * ## `"auto"`
 */
export type Overflow = 'visible' | 'hidden' | 'scroll' | 'auto';
/**
 * # Variants
 * 
 * ## `"default"`
 * 
 * ## `"pointer"`
 * 
 * ## `"text"`
 * 
 * ## `"move"`
 * 
 * ## `"not-allowed"`
 */
export type Cursor = 'default' | 'pointer' | 'text' | 'move' | 'not-allowed';
/**
 * # Variants
 * 
 * ## `"regular"`
 * 
 * ## `"medium"`
 * 
 * ## `"semibold"`
 * 
 * ## `"bold"`
 */
export type FontWeight = 'regular' | 'medium' | 'semibold' | 'bold';
export type Dimension = DimensionAuto | DimensionPx | DimensionPercent | DimensionFr;
export interface DimensionAuto {
  tag: 'auto',
}
export interface DimensionPx {
  tag: 'px',
  val: number,
}
export interface DimensionPercent {
  tag: 'percent',
  val: number,
}
export interface DimensionFr {
  tag: 'fr',
  val: number,
}
export interface Edges {
  top: number,
  right: number,
  bottom: number,
  left: number,
}
export interface Color {
  r: number,
  g: number,
  b: number,
  a: number,
}
/**
 * A theme token resolves to a color on the host (light/dark aware).
 * # Variants
 * 
 * ## `"fg"`
 * 
 * ## `"fg-muted"`
 * 
 * ## `"bg"`
 * 
 * ## `"surface"`
 * 
 * ## `"accent"`
 * 
 * ## `"accent-fg"`
 * 
 * ## `"border"`
 * 
 * ## `"ok"`
 * 
 * ## `"warn"`
 * 
 * ## `"bad"`
 */
export type ColorToken = 'fg' | 'fg-muted' | 'bg' | 'surface' | 'accent' | 'accent-fg' | 'border' | 'ok' | 'warn' | 'bad';
export type Paint = PaintRgba | PaintToken;
export interface PaintRgba {
  tag: 'rgba',
  val: Color,
}
export interface PaintToken {
  tag: 'token',
  val: ColorToken,
}
export interface Font {
  size?: number,
  weight?: FontWeight,
  family?: string,
  /**
   * "sans" | "serif" | "mono" tokens; hosts map to real fonts
   */
  lineHeight?: number,
}
export interface GridTrack {
  size: Dimension,
}
/**
 * Sparse: every field optional; the host resolves against the theme
 * and inheritance rules once, then hands an absolute style to the backend.
 */
export interface Style {
  display?: Display,
  direction?: Direction,
  wrap?: Wrap,
  gap?: number,
  alignItems?: Align,
  justifyContent?: Justify,
  alignSelf?: Align,
  flexGrow?: number,
  flexShrink?: number,
  flexBasis?: Dimension,
  gridColumns?: Array<GridTrack>,
  gridRows?: Array<GridTrack>,
  gridColumn?: [number, number],
  gridRow?: [number, number],
  width?: Dimension,
  height?: Dimension,
  minWidth?: Dimension,
  minHeight?: Dimension,
  maxWidth?: Dimension,
  maxHeight?: Dimension,
  padding?: Edges,
  margin?: Edges,
  overflow?: Overflow,
  color?: Paint,
  background?: Paint,
  borderWidth?: number,
  borderColor?: Paint,
  radius?: number,
  opacity?: number,
  font?: Font,
  cursor?: Cursor,
  visible?: boolean,
}
