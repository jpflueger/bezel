# bezel-host-native

Intentionally empty. Per ADR-0002 this host (winit + Masonry + Vello + AccessKit behind the same `Backend` trait) starts only when the gate is met:
Webview-host users reporting cross-OS rendering differences, webview bugs or startup cost as blocking.

Until then, the contract is kept honest for it by `scripts/contract-lint.py`: every element needs a written Native-host mapping in `docs/elements/`.
