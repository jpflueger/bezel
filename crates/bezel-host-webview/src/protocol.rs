//! Binary frame format shared with runtime/dom/src/protocol.ts. Keep both in sync; docs/protocol.md is the spec.
//!
//! frame  := u8 version(=1) · varint op_count · op*
//! op     := u8 opcode · payload
//! ids    := varint ; strings := varint len · utf8 (interned: varint 0 · varint index after first use)

use bezel_core::{NodeId, Prop, ResolvedStyle};
use std::collections::HashMap;

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum OpCode {
    Mount = 1,
    Create = 2,
    Prop = 3,
    Style = 4,
    Append = 5,
    Insert = 6,
    Remove = 7,
    Destroy = 8,
}

#[derive(Default)]
pub struct PatchWriter {
    buf: Vec<u8>,
    count: u32,
    interned: HashMap<String, u32>,
}

impl PatchWriter {
    pub fn op(&mut self, code: OpCode) -> &mut Self {
        if self.buf.is_empty() {
            self.buf.push(1);
            self.buf.extend([0, 0, 0, 0]);
        }
        self.count += 1;
        self.buf.push(code as u8);
        self
    }
    pub fn id(&mut self, id: NodeId) -> &mut Self {
        self.varint(id.0)
    }
    pub fn u8(&mut self, v: u8) -> &mut Self {
        self.buf.push(v);
        self
    }
    pub fn varint(&mut self, mut v: u32) -> &mut Self {
        loop {
            let b = (v & 0x7f) as u8;
            v >>= 7;
            if v == 0 {
                self.buf.push(b);
                break;
            }
            self.buf.push(b | 0x80);
        }
        self
    }
    pub fn str(&mut self, s: &str) -> &mut Self {
        if let Some(&i) = self.interned.get(s) {
            self.varint(0).varint(i)
        } else {
            let i = self.interned.len() as u32 + 1;
            self.interned.insert(s.to_owned(), i);
            self.varint(s.len() as u32 + 1);
            self.buf.extend_from_slice(s.as_bytes());
            self
        }
    }
    pub fn prop(&mut self, p: &Prop) -> &mut Self {
        match p {
            Prop::Text(s) => self.u8(1).str(s),
            Prop::Placeholder(s) => self.u8(2).str(s),
            Prop::Enabled(b) => self.u8(3).u8(*b as u8),
            Prop::Checked(b) => self.u8(4).u8(*b as u8),
            Prop::Src(s) => self.u8(5).str(s),
            Prop::Alt(s) => self.u8(6).str(s),
            Prop::Label(s) => self.u8(7).str(s),
            Prop::Selected(s) => self.u8(9).str(s),
            Prop::RowCount(n) => self.u8(10).varint(*n),
            Prop::Options(items) => {
                self.u8(8).varint(items.len() as u32);
                for (v, l) in items {
                    self.str(v).str(l);
                }
                self
            }
            Prop::Listen(k) => self.u8(11).u8(*k as u8),
            Prop::Unlisten(k) => self.u8(12).u8(*k as u8),
            Prop::Style(_) => self,
        }
    }
    pub fn style(&mut self, s: &ResolvedStyle) -> &mut Self {
        // Phase 0: encode as a fixed-order record; the DOM runtime maps fields to CSS custom properties.
        let f = |x: f32| x.to_le_bytes();
        self.u8(s.display as u8).u8(s.direction as u8);
        self.buf.extend(f(s.gap));
        self.u8(s.align_items as u8).u8(s.justify_content as u8);
        self.buf.extend(f(s.flex_grow));
        self.buf.extend(f(s.flex_shrink));
        for e in [s.padding, s.margin] {
            for v in [e.top, e.right, e.bottom, e.left] {
                self.buf.extend(f(v));
            }
        }
        self.buf
            .extend([s.color.0, s.color.1, s.color.2, s.color.3]);
        match s.background {
            Some(c) => {
                self.u8(1);
                self.buf.extend([c.0, c.1, c.2, c.3]);
            }
            None => {
                self.u8(0);
            }
        }
        self.buf.extend(f(s.border_width));
        self.buf.extend([
            s.border_color.0,
            s.border_color.1,
            s.border_color.2,
            s.border_color.3,
        ]);
        self.buf.extend(f(s.radius));
        self.buf.extend(f(s.opacity));
        self.buf.extend(f(s.font_size));
        self.u8(s.font_weight as u8);
        let fam = s.font_family.clone();
        self.str(&fam);
        self.buf.extend(f(s.line_height));
        self.u8(s.visible as u8);
        // dimensions & overflow: TODO in E3 (encode Dimension as tag+f32)
        self
    }
    pub fn finish(&mut self) -> Vec<u8> {
        if self.buf.is_empty() {
            return vec![];
        }
        let c = self.count.to_le_bytes();
        self.buf[1..5].copy_from_slice(&c);
        self.count = 0;
        std::mem::take(&mut self.buf)
    }
}
