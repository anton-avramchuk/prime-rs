# Design

## Context

The requirements are in `specs/blockui`. PrimeNG 21.1.10 sources: `blockui/blockui.ts` (about 200 lines), `blockui/style/blockuistyle.ts`; styles `@primeuix/styles/blockui`; demos `apps/showcase/doc/blockui/{basic,document}-doc.ts`. Project templates: `prime-drawer` (the state machine `visible → rendered → leaving`, `Portal`, the layer `ZGroup::Modal`, `block_body_scroll`, the fallback exit timer, `animationend`), `prime-dialog` (the same, with the shared scroll counter).

Observations on the source:
- The host is the mask itself: `block()` sets `display: flex`, moves the host into `target.getBlockableElement()` (and sets `position: relative` on it) or into `body` (with `blockBodyScroll()`), sets the layer `modal + baseZIndex` and the classes `p-overlay-mask p-overlay-mask-enter-active`; `unblock()` switches the enter class to `p-overlay-mask-leave-active` and on `animationend` calls `destroyModal()` (display none, removing the classes and the layer, returning from `body`, `unblockBodyScroll()`).
- Root classes: `p-blockui p-blockui-mask`, for the document also `p-blockui-mask-document`; `aria-busy` = `blocked`.
- Theme: `.p-blockui-mask.p-overlay-mask { position: absolute }`, `.p-blockui-mask-document.p-overlay-mask { position: fixed }`.
- In PrimeNG the host is not hidden when not blocked (its content is visible); here, without blocking, nothing is in the DOM.
- PrimeNG does not restore `position: relative` on the target; here the inline value is restored.

## Goals / Non-Goals

**Goals:** the same classes and behaviour of the layer, scrolling and animation; the goal is any element via `NodeRef`; release of resources on unmount.

**Non-Goals:** the `BlockableUI` interface, the `content` template, `Bind`, `pTemplate`.

## Decisions

### D1. Crate and API
`prime-blockui` with `BlockUI`, `BlockTarget`. `BlockTarget` — a `Clone` wrapper over `Arc<dyn Fn() -> Option<web_sys::Element> + Send + Sync>`; `From<NodeRef<E>>` for Leptos elements. Properties: `blocked: Prop<bool>`, `target: Option<BlockTarget>`, `auto_z_index: OptProp<bool>`, `base_z_index: Prop<i32>`, `style`, `style_class`, `pt: Option<Pt>`, `children: Option<ChildrenFn>`.

### D2. State
`rendered`, `leaving`, `mask: NodeRef<Div>`, `z_index`, `scroll_blocked`, `saved_position: StoredValue<Option<(Element, String)>>`, `timer`, `was_open` — as in Drawer. Effect (only `cfg(wasm32)`): `blocked` on → cancel the timer, `leaving = false`, `rendered = true`, and when the mask has appeared in the DOM — `opened()` (the layer, scrolling, `position: relative` of the target); `blocked` off → `begin_leave()` (the exit class, a fallback timer of 400 ms); `animationend` of the mask or the timer → `finish_leave()` (releasing the layer, the scrolling and the position, `rendered = false`).

### D3. Output
`rendered.get().then(|| <Portal mount=target><div class=… aria-busy=… style="display: flex …">…</div></Portal>)`. The target is computed when the `Portal` is built (`BlockTarget::resolve()`); if a target is set but the element does not exist yet — the mask is not rendered and it retries on the next change of `blocked`. The content is built anew on each show (`ChildrenFn`).

### D4. Pure logic (`state.rs`)
`mask_classes(document, leaving, extra)`, `aria_busy(leaving)`, `mask_style(auto_z, z, extra)` — classes and strings in table tests.

### D5. CSS
`blockui` is added to `theme.components`. The mask rules in `base.css` (`p-overlay-mask` and the animations) are already used by Drawer/Dialog; check `p-blockui-mask*`.

### D6. Tests
SSR: with any `blocked` there is no mask (a `rustdoc` test). Browser `tests/web.rs`: the mask in the target (position, `aria-busy`, content, layer), restoring `position`, the document (the mask in `body`, the class, blocking and releasing the scroll), the exit (classes, removal), re-enabling during the exit, unmounting with the blocking on, `auto_z_index = false`. e2e: a real mouse on the buttons, a panel covered by the mask (a click under the mask does not reach it), the document is blocked for three seconds and then unblocked.

### D7. Demos and benchmarks
`basic` — a `Panel` with a `NodeRef` on the container and the Block/Unblock buttons; `document` — a "Block" button, the blocking is released after three seconds (`set_timeout`). Bench `blockui-1`/`-1000` (SSR, nothing is drawn, the wrapper is measured), `case-blockui`.

## Risks / Trade-offs

- [The target is not ready when `blocked = true` from the very start] The mask is not rendered while the element does not exist; after mounting, the next change of the signal will show it. Recorded in the docs; for an initial blocking of the target `blocked` must be enabled after mounting.
- [A shared scroll counter] The blocking is released exactly once per call (`scroll_blocked`), as in Drawer.

## Open Questions

- Whether `on_block`/`on_unblock` events are needed: PrimeNG has none, so we do not add them.

## Migration Plan

No migration. Rollback — `git revert`.
