// Bezel DOM runtime. Owns the DOM. Applies patches from the host, sends semantic events back.
// Invariants (ADR-0003/0006): no per-frame events leave this file; no HTML strings from the app are ever inserted.
import { Reader, OpCode, ELEMENTS } from "./protocol";

const nodes = new Map<number, HTMLElement>();
const root = document.getElementById("root")!;

declare const window: Window & { ipc?: { postMessage(s: string): void }; __bezel_apply?: (b64: string) => void };

function create(id: number, el: number): HTMLElement {
  const kind = ELEMENTS[el];
  const e = document.createElement(({ box: "div", text: "span", input: "input", textarea: "textarea", button: "button",
    checkbox: "input", select: "select", image: "img", scroll: "div", list: "div", divider: "hr" } as Record<string,string>)[kind]);
  e.dataset.bz = kind; e.dataset.id = String(id);
  if (kind === "checkbox") (e as HTMLInputElement).type = "checkbox";
  if (kind === "scroll") e.style.overflow = "auto";
  if (kind === "list") setupVirtualList(id, e);
  return e;
}

function applyStyle(e: HTMLElement, r: Reader) {
  const display = ["flex", "grid", "none"][r.u8()], dir = ["row", "column"][r.u8()], gap = r.f32();
  const ai = ["stretch","flex-start","center","flex-end","baseline"][r.u8()], jc = ["flex-start","center","flex-end","space-between","space-around","space-evenly"][r.u8()];
  const grow = r.f32(), shrink = r.f32();
  const pad = [r.f32(), r.f32(), r.f32(), r.f32()], mar = [r.f32(), r.f32(), r.f32(), r.f32()];
  const color = r.rgba(); const hasBg = r.u8(); const bg = hasBg ? r.rgba() : "transparent";
  const bw = r.f32(), bc = r.rgba(), radius = r.f32(), opacity = r.f32(), fs = r.f32(), fw = [400,500,600,700][r.u8()];
  const fam = r.str(), lh = r.f32(), visible = r.u8();
  Object.assign(e.style, { display, flexDirection: dir, gap: `${gap}px`, alignItems: ai, justifyContent: jc, flexGrow: String(grow), flexShrink: String(shrink),
    padding: pad.map(v => `${v}px`).join(" "), margin: mar.map(v => `${v}px`).join(" "), color, background: bg,
    border: bw ? `${bw}px solid ${bc}` : "none", borderRadius: `${radius}px`, opacity: String(opacity), fontSize: `${fs}px`, fontWeight: String(fw),
    fontFamily: `var(--bz-font-${fam}, system-ui)`, lineHeight: String(lh), visibility: visible ? "visible" : "hidden" });
}

function applyProp(e: HTMLElement, r: Reader) {
  const tag = r.u8();
  switch (tag) {
    case 1: { const t = r.str(); if (e instanceof HTMLInputElement || e instanceof HTMLTextAreaElement) e.value = t; else e.textContent = t; break; }
    case 2: (e as HTMLInputElement).placeholder = r.str(); break;
    case 3: (e as HTMLButtonElement).disabled = !r.u8(); break;
    case 4: (e as HTMLInputElement).checked = !!r.u8(); break;
    case 5: (e as HTMLImageElement).src = `bezel://assets/${r.str()}`; break;
    case 6: (e as HTMLImageElement).alt = r.str(); break;
    case 7: e.setAttribute("aria-label", r.str()); break;
    case 8: { const n = r.varint(); e.replaceChildren(); for (let i = 0; i < n; i++) { const o = document.createElement("option"); o.value = r.str(); o.textContent = r.str(); e.appendChild(o); } break; }
    case 9: (e as HTMLSelectElement).value = r.str(); break;
    case 10: e.dataset.rows = String(r.varint()); (e as any).__bzRefresh?.(); break;
    case 11: listen(e, r.u8()); break;
    case 12: unlisten(e, r.u8()); break;
  }
}

const KINDS = ["click","change","submit","focus","blur","key","resize","request-rows"] as const;
function send(ev: object) { window.ipc?.postMessage(JSON.stringify(ev)); /* Phase 0: JSON; binary once measured */ }
function listen(e: HTMLElement, k: number) {
  const kind = KINDS[k]; const id = Number(e.dataset.id);
  const h = (dom: Event) => {
    if (kind === "change") send({ node: id, kind, text: (dom.target as HTMLInputElement).value ?? String((dom.target as HTMLInputElement).checked) });
    else if (kind === "key") { const ke = dom as KeyboardEvent; send({ node: id, kind, key: { key: ke.key, ctrl: ke.ctrlKey, alt: ke.altKey, shift: ke.shiftKey, meta: ke.metaKey } }); }
    else send({ node: id, kind });
  };
  const domName = kind === "change" ? "input" : kind === "key" ? "keydown" : kind === "submit" ? "keydown" : kind;
  if (kind === "submit") { const sh = (ke: KeyboardEvent) => { if (ke.key === "Enter") send({ node: id, kind, text: (e as HTMLInputElement).value }); }; e.addEventListener("keydown", sh); (e as any)[`__bz_${kind}`] = sh; return; }
  e.addEventListener(domName, h); (e as any)[`__bz_${kind}`] = h;
}
function unlisten(e: HTMLElement, k: number) { const kind = KINDS[k]; const h = (e as any)[`__bz_${kind}`]; if (h) e.removeEventListener(kind === "change" ? "input" : kind, h); }

function setupVirtualList(id: number, e: HTMLElement) {
  // Windowed rendering: children materialised by the app in response to request-rows(range).
  // Scroll handling stays here — it never crosses the boundary except to ask for new rows.
  e.style.overflow = "auto"; let lastReq = "";
  const refresh = () => { const rowH = 28, first = Math.floor(e.scrollTop / rowH), count = Math.ceil(e.clientHeight / rowH) + 10;
    const key = `${first}:${count}`; if (key !== lastReq) { lastReq = key; send({ node: id, kind: "request-rows", rows: { start: first, end: Math.min(first + count, Number(e.dataset.rows ?? 0)) } }); } };
  e.addEventListener("scroll", () => requestAnimationFrame(refresh), { passive: true }); (e as any).__bzRefresh = refresh;
}

export function apply(frame: Uint8Array) {
  const r = new Reader(frame); if (r.u8() !== 1) throw new Error("bad frame version");
  const count = r.varint();
  for (let n = 0; n < count && !r.done(); n++) {
    const op = r.u8() as OpCode;
    switch (op) {
      case OpCode.Mount: root.replaceChildren(nodes.get(r.varint())!); break;
      case OpCode.Create: { const id = r.varint(); nodes.set(id, create(id, r.u8())); break; }
      case OpCode.Prop: applyProp(nodes.get(r.varint())!, r); break;
      case OpCode.Style: applyStyle(nodes.get(r.varint())!, r); break;
      case OpCode.Append: nodes.get(r.varint())!.appendChild(nodes.get(r.varint())!); break;
      case OpCode.Insert: { const p = nodes.get(r.varint())!, c = nodes.get(r.varint())!, b = nodes.get(r.varint())!; p.insertBefore(c, b); break; }
      case OpCode.Remove: { const p = nodes.get(r.varint())!, c = nodes.get(r.varint())!; if (c.parentElement === p) p.removeChild(c); break; }
      case OpCode.Destroy: { const id = r.varint(); nodes.get(id)?.remove(); nodes.delete(id); break; }
    }
  }
}

// Entry point used by the Webview host (base64 over evaluate_script) and by the Web host (direct call).
window.__bezel_apply = (b64: string) => apply(Uint8Array.from(atob(b64), c => c.charCodeAt(0)));
