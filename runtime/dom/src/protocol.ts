// Binary frame reader — mirrors crates/bezel-host-webview/src/protocol.rs. Spec: docs/protocol.md
export const enum OpCode { Mount = 1, Create, Prop, Style, Append, Insert, Remove, Destroy }
export const ELEMENTS = ["box","text","input","textarea","button","checkbox","select","image","scroll","list","divider"] as const;

export class Reader {
  private i = 0; private interned: string[] = [""];
  constructor(private b: Uint8Array, private dv = new DataView(b.buffer, b.byteOffset, b.byteLength)) {}
  u8() { return this.b[this.i++]; }
  f32() { const v = this.dv.getFloat32(this.i, true); this.i += 4; return v; }
  varint() { let v = 0, s = 0, byte: number; do { byte = this.b[this.i++]; v |= (byte & 0x7f) << s; s += 7; } while (byte & 0x80); return v >>> 0; }
  str() { const n = this.varint(); if (n === 0) return this.interned[this.varint()]; const s = new TextDecoder().decode(this.b.subarray(this.i, this.i + n - 1)); this.i += n - 1; this.interned.push(s); return s; }
  rgba() { const [r,g,b,a] = [this.u8(), this.u8(), this.u8(), this.u8()]; return `rgba(${r},${g},${b},${a/255})`; }
  done() { return this.i >= this.b.length; }
}
