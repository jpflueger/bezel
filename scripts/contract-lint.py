#!/usr/bin/env python3
"""Contract lint: fail if wit/ leaks a host concept or an element lacks docs.

Rules (ADR-0001, ADR-0006):
  1. No identifier in wit/*.wit may match the DOM/CSS/JS denylist.
  2. Every variant of `enum element` in tree.wit needs docs/elements/<name>.md
     containing a "## Native host mapping" heading.
Usage: scripts/contract-lint.py [repo-root]
"""
import re, sys, pathlib

ROOT = pathlib.Path(sys.argv[1] if len(sys.argv) > 1 else ".").resolve()
DENY = {
    # DOM / JS
    "innerhtml", "outerhtml", "classname", "classlist", "queryselector", "getelementbyid",
    "addeventlistener", "onclick", "onchange", "dataset", "shadowroot", "document", "window-object",
    "div", "span", "iframe", "script", "eval", "html", "dom", "css", "js", "javascript",
    # CSS-specific units/keywords we don't expose
    "em", "rem", "vh", "vw", "important", "selector", "pseudo", "zindex", "z-index",
    # host/renderer concepts
    "webview", "wry", "tao", "winit", "gpu", "webgpu", "vello", "masonry", "skia", "canvas2d",
}
IDENT = re.compile(r"[A-Za-z%][A-Za-z0-9-]*")

def strip_comments(src: str) -> str:
    src = re.sub(r"/\*.*?\*/", "", src, flags=re.S)
    return re.sub(r"//[^\n]*", "", src)

errors = []
for wit in sorted((ROOT / "wit").glob("*.wit")):
    src = strip_comments(wit.read_text())
    for m in IDENT.finditer(src):
        ident = m.group(0).lstrip("%").lower()
        if ident in DENY:
            line = src[: m.start()].count("\n") + 1
            errors.append(f"{wit.relative_to(ROOT)}:{line}: forbidden identifier '{m.group(0)}' (host/DOM concept)")

tree = (ROOT / "wit" / "tree.wit").read_text()
enum_src = re.search(r"enum\s+element\s*\{([^}]*)\}", strip_comments(tree), re.S)
if not enum_src:
    errors.append("wit/tree.wit: could not find `enum element`")
else:
    for name in re.findall(r"%?([a-z][a-z0-9-]*)", enum_src.group(1)):
        doc = ROOT / "docs" / "elements" / f"{name}.md"
        if not doc.exists():
            errors.append(f"docs/elements/{name}.md missing for element '{name}'")
        elif "## Native host mapping" not in doc.read_text():
            errors.append(f"docs/elements/{name}.md lacks a '## Native host mapping' section")

if errors:
    print("\n".join(errors)); print(f"\ncontract-lint: {len(errors)} problem(s)"); sys.exit(1)
print("contract-lint: ok")
