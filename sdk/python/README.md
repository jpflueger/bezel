# bezel (Python SDK)

```
pip install bezel
bezel new hello --sdk python && cd hello && bezel dev
```

The SDK is a thin idiomatic layer over bindings generated from `bezel:ui` by componentize-py. Users never import anything named wit, wasm or component (PRD S-2).
