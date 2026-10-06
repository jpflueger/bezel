"""Bezel Python SDK — declarative elements over the bezel:ui contract.

SKELETON. The generated bindings module (``bezel._bindings``) is produced by
``componentize-py bindings`` against wit/ at build time; this file shows the
public surface and the batching model (ADR-0003):

* attribute assignment on an element queues an ``op``;
* the queue flushes with ONE ``apply`` call at the end of every handler turn;
* host-owned state (``Input.value``) is read synchronously.
"""
from __future__ import annotations
import asyncio
from dataclasses import dataclass, field
from typing import Any, Awaitable, Callable, Iterable

__all__ = ["App", "Box", "Text", "Input", "Textarea", "Button", "Checkbox", "Select", "Image", "Scroll", "List", "Divider", "run"]

# ---- batching -------------------------------------------------------------
class _Batch:
    ops: list = []
    @classmethod
    def push(cls, op) -> None: cls.ops.append(op)
    @classmethod
    def flush(cls) -> None:
        if cls.ops:
            from . import _bindings  # generated
            _bindings.tree.apply(cls.ops); cls.ops = []

# ---- elements --------------------------------------------------------------
class Element:
    _kind: str = "box"
    def __init__(self, *, style: dict | None = None, children: Iterable["Element"] = (), **props: Any):
        from . import _bindings
        self._node = _bindings.tree.Node(getattr(_bindings.tree.Element, self._kind.upper()))
        self._handlers: dict[str, Callable[..., Awaitable[None] | None]] = {}
        for k, v in props.items(): setattr(self, k, v)
        if style: self.style = style
        for c in children: self.append(c)

    # every public attribute maps to a prop; assignment queues, never crosses immediately
    def __setattr__(self, name: str, value: Any) -> None:
        if name.startswith("_"): return object.__setattr__(self, name, value)
        if name.startswith("on_"):
            kind = name[3:].replace("_", "-"); self._handlers[kind] = value
            from . import _bindings; _Batch.push(_bindings.tree.Op_Set((self._node, _bindings.tree.Prop_Listen(_kind(kind)))))
            _registry[self._node.id()] = self; return
        from . import _bindings
        _Batch.push(_bindings.tree.Op_Set((self._node, _prop(name, value))))
        object.__setattr__(self, name, value)

    def append(self, child: "Element") -> None:
        from . import _bindings; _Batch.push(_bindings.tree.Op_Append((self._node, child._node)))
    def remove(self, child: "Element") -> None:
        from . import _bindings; _Batch.push(_bindings.tree.Op_Remove((self._node, child._node)))
    @property
    def value(self) -> str:  # host-owned state, synchronous read
        return self._node.value()

class Box(Element): _kind = "box"
class Text(Element):
    _kind = "text"
    def __init__(self, text: str = "", **kw): super().__init__(text=text, **kw)
class Input(Element): _kind = "input"
class Textarea(Element): _kind = "textarea"
class Button(Element):
    _kind = "button"
    def __init__(self, text: str = "", **kw): super().__init__(text=text, **kw)
class Checkbox(Element): _kind = "checkbox"
class Select(Element): _kind = "select"
class Image(Element): _kind = "image"
class Scroll(Element): _kind = "scroll"
class List(Element): _kind = "list"
class Divider(Element): _kind = "divider"

_registry: dict[int, Element] = {}

def _kind(name: str):
    from . import _bindings; return getattr(_bindings.tree.EventKind, name.upper().replace("-", "_"))

def _prop(name: str, value: Any):
    from . import _bindings
    if name == "style": return _bindings.tree.Prop_Style(_style(value))
    cls = getattr(_bindings.tree, f"Prop_{name.title().replace('_', '')}")
    return cls(value)

def _style(d: dict):
    from . import _bindings
    s = _bindings.style.Style()
    for k, v in d.items():
        if k in ("padding", "margin") and isinstance(v, (int, float)): v = _bindings.style.Edges(v, v, v, v)
        setattr(s, k.replace("-", "_"), v)
    return s

# ---- app -------------------------------------------------------------------
class App:
    def build(self) -> Element: raise NotImplementedError
    async def main(self) -> None:
        from . import _bindings
        root = self.build(); _bindings.tree.set_root(root._node); _Batch.flush()
        async for ev in _bindings.events.subscribe():          # WASI 0.3 stream → async iterator
            el = _registry.get(ev.node)
            h = el._handlers.get(_kind_name(ev.kind)) if el else None
            if h:
                r = h(ev)
                if asyncio.iscoroutine(r): await r
            _Batch.flush()                                       # one apply per handled event

def _kind_name(k) -> str: return str(k).split(".")[-1].lower().replace("_", "-")

def run(app_cls: type[App]) -> None:
    """Entry point exported as `run` in the component world."""
    asyncio.run(app_cls().main())
