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
import sys
from collections.abc import Awaitable, Callable, Iterable
from typing import Any, ClassVar

__all__ = [
    "App",
    "BatchError",
    "Box",
    "Button",
    "Checkbox",
    "Divider",
    "Image",
    "Input",
    "List",
    "Scroll",
    "Select",
    "Text",
    "Textarea",
    "run",
]


# ---- batching -------------------------------------------------------------
class BatchError(Exception):
    """The host rejected a batch; nothing in it was applied."""


class _Batch:
    ops: ClassVar[list] = []
    labels: ClassVar[list[str]] = []  # what queued each op, for error messages

    @classmethod
    def push(cls, op, label: str) -> None:
        cls.ops.append(op)
        cls.labels.append(label)

    @classmethod
    def flush(cls) -> None:
        if not cls.ops:
            return
        from . import _bindings  # generated

        ops, labels = cls.ops, cls.labels
        cls.ops, cls.labels = [], []
        try:
            _bindings.tree.apply(ops)
        except _bindings.types.Err as e:  # result<_, tree-error>
            err = e.value
            raise BatchError(f"{labels[err.index]}: {err.code.name.lower()}") from e


# ---- elements --------------------------------------------------------------
class Element:
    _kind: str = "box"

    def __init__(
        self,
        *,
        style: dict | None = None,
        children: Iterable[Element] = (),
        **props: Any,
    ):
        from . import _bindings

        self._node = _bindings.tree.Node(
            getattr(_bindings.tree.Element, self._kind.upper())
        )
        self._handlers: dict[str, Callable[..., Awaitable[None] | None]] = {}
        for k, v in props.items():
            setattr(self, k, v)
        if style:
            self.style = style
        for c in children:
            self.append(c)

    # every public attribute maps to a prop; assignment queues, never crosses immediately
    def __setattr__(self, name: str, value: Any) -> None:
        if name.startswith("_"):
            return object.__setattr__(self, name, value)
        if name.startswith("on_"):
            kind = name[3:].replace("_", "-")
            self._handlers[kind] = value
            from . import _bindings

            _Batch.push(
                _bindings.tree.Op_Set(
                    (self._node, _bindings.tree.Prop_Listen(_kind(kind)))
                ),
                f"{self._kind}.{name}",
            )
            _registry[self._node.id()] = self
            return
        from . import _bindings

        _Batch.push(
            _bindings.tree.Op_Set((self._node, _prop(name, value))),
            f"{self._kind}.{name}",
        )
        object.__setattr__(self, name, value)

    def append(self, child: Element) -> None:
        from . import _bindings

        _Batch.push(
            _bindings.tree.Op_Append((self._node, child._node)),
            f"{self._kind}.append({child._kind})",
        )

    def remove(self, child: Element) -> None:
        from . import _bindings

        _Batch.push(
            _bindings.tree.Op_Remove((self._node, child._node)),
            f"{self._kind}.remove({child._kind})",
        )

    @property
    def value(self) -> str | bool | tuple[float, float] | None:
        """Host-owned state, read synchronously (wit `node-value`)."""
        return getattr(self._node.value(), "value", None)


class Box(Element):
    _kind = "box"


class Text(Element):
    _kind = "text"

    def __init__(self, text: str = "", **kw):
        super().__init__(text=text, **kw)


class Input(Element):
    _kind = "input"


class Textarea(Element):
    _kind = "textarea"


class Button(Element):
    _kind = "button"

    def __init__(self, text: str = "", **kw):
        super().__init__(text=text, **kw)


class Checkbox(Element):
    _kind = "checkbox"


class Select(Element):
    _kind = "select"


class Image(Element):
    _kind = "image"


class Scroll(Element):
    _kind = "scroll"


class List(Element):
    _kind = "list"


class Divider(Element):
    _kind = "divider"


_registry: dict[int, Element] = {}


def _kind(name: str):
    from . import _bindings

    return getattr(_bindings.tree.EventKind, name.upper().replace("-", "_"))


def _prop(name: str, value: Any):
    from . import _bindings

    if name == "style":
        return _bindings.tree.Prop_Style(_style(value))
    cls = getattr(_bindings.tree, f"Prop_{name.title().replace('_', '')}")
    return cls(value)


def _style(d: dict):
    from . import _bindings

    s = _bindings.style.Style()
    for k, v in d.items():
        if k in ("padding", "margin") and isinstance(v, (int, float)):
            v = _bindings.style.Edges(v, v, v, v)
        setattr(s, k.replace("-", "_"), v)
    return s


# ---- app -------------------------------------------------------------------
class App:
    def build(self) -> Element:
        raise NotImplementedError

    async def main(self) -> None:
        from . import _bindings

        root = self.build()
        _bindings.tree.set_root(root._node)
        _Batch.flush()
        async for (
            ev
        ) in _bindings.events.subscribe():  # WASI 0.3 stream → async iterator
            el = _registry.get(ev.node)
            h = el._handlers.get(_kind_name(ev.kind)) if el else None
            try:
                if h:
                    r = h(ev)
                    if asyncio.iscoroutine(r):
                        await r
                _Batch.flush()  # one apply per handled event
            except BatchError as e:  # a bad batch is dropped; the app keeps running
                print(f"bezel: batch rejected: {e}", file=sys.stderr)


def _kind_name(k) -> str:
    return str(k).split(".")[-1].lower().replace("_", "-")


def run(app_cls: type[App]) -> None:
    """Entry point exported as `run` in the component world."""
    asyncio.run(app_cls().main())
