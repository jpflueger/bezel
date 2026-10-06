# Decision records

One file per decision, numbered, never edited after acceptance except to change **Status** (superseded/deprecated) and add a pointer to the successor. Propose a new ADR by copying `0000-template.md`.

| # | Title | Status |
|---|-------|--------|
| 0001 | The WIT contract is the product; hosts are replaceable | accepted |
| 0002 | Ship a webview host first; the GPU host is gated on demand | accepted |
| 0003 | The host owns a retained element tree; apps send batched ops and receive semantic events | accepted |
| 0004 | Build on the Component Model and WASI 0.3, not a bespoke ABI | accepted |
| 0005 | Python is the beachhead language; Rust is the dogfood SDK; TypeScript third | accepted (conditional) |
| 0006 | A small, CSS-shaped element set instead of a widget library | accepted |
| 0007 | Capabilities are declared in bezel.toml, compiled into the world, enforced by the host | accepted |

Decisions that change the contract (`wit/`) additionally require an RFC once the RFC process is live (0.3).
