# Patch/event wire protocol (host ↔ DOM runtime)

Shared by `crates/bezel-host-webview/src/protocol.rs` (writer) and `runtime/dom/src/protocol.ts` (reader). Version 1.

```
frame   := u8 version(1) · u32le op_count · op*
op      := u8 opcode · payload
opcode  := 1 mount(id) | 2 create(id, u8 element) | 3 prop(id, u8 tag, value) | 4 style(id, style-record)
         | 5 append(parent, child) | 6 insert(parent, child, before) | 7 remove(parent, child) | 8 destroy(id)
id      := varint (LEB128, u32)
string  := varint(len+1) utf8            -- first occurrence; interned per frame writer session
         | varint(0) varint(index)       -- subsequent occurrences
```

Events go the other way as JSON in Phase 0 (`{node, kind, text?, key?, rows?}`) and switch to binary once measured.
Rule: nothing in this protocol may carry HTML, CSS text, or JavaScript.
