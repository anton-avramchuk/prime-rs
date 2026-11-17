# Design

## Context

Requirements are in `specs/dialog`. PrimeNG 21.1.10 sources: `dialog/dialog.ts`, `dialog/style/dialogstyle.ts`; demos `apps/showcase/doc/dialog/*-doc.ts`. Models in the project: `prime-drawer` (`visible`/`rendered`/`leaving` state, `Portal` with markup built inside a closure, `Core` with cleanup closures, `FocusTrap`, `block_body_scroll`, Escape on the topmost window, leave on `animationend` with a fallback timer), `prime-popover` (window listeners), `prime-core::zindex`.

Observations from the source:
- The `div.p-dialog-mask` mask is present **always** (also for a non-modal dialog): `position: fixed`, full screen, `display: flex`; window alignment via `justify-content` / `align-items` by `position`; `pointer-events: none` for a non-modal one, `pointer-events: auto` on the window root. In PrimeNG these rules are inline styles from `dialogstyle.ts`, they are not in the theme CSS: we write them inline ourselves.
- Window animation: the `p-dialog` name of `@primeuix/motion` yields the classes `p-dialog-enter-active` / `p-dialog-leave-active`; the modal dialog's mask — `p-overlay-mask-enter-active` / `-leave-active`. Whether the corresponding `@keyframes` exist in `@primeuix/styles/dialog` is determined by generating the theme (see Open Questions).
- Dragging: `mousedown` on the header (not inside `[data-pc-section=headeractions]`) → `mousemove`/`mouseup` on `window`; `left`/`top` computed from `getBoundingClientRect` adjusted for margins, `position: fixed`, `margin: 0`; with `keepInViewport` updates that go beyond `minX/minY` or the screen are skipped.
- Resizing: the `p-resizable-handle` handle, `mousemove` on `window`; in PrimeNG when "not dragged yet" (`top`/`left` unset) the delta is doubled — the window is centered, so it grows in both directions; we replicate this.
- Maximizing: `maximized` toggles, `p-dialog-maximized`; for a non-modal one without `blockScroll` it locks scrolling (`blockBodyScroll`); reset on close.
- `breakpoints`: a `<style>` with rules `.p-dialog[<uuid>]:not(.p-dialog-maximized){width:… !important}` is created in `head`; the `<uuid>` attribute is set on the window.
- `Escape` closes the window only if the window's `z-index` equals the topmost (`ZIndexUtils.getCurrent`).
- Closing by mask — `mousedown` on the mask itself (the event target is the mask).
- Focus on show: first focusable in the content → footer → header, with a delay (`transitionOptions` or 5 ms).

## Goals / Non-Goals

**Goals:** the same DOM, classes, pointer and keyboard behavior, layer order; `prime-dom` facilities and `Drawer` solutions without copying mistakes.

**Non-Goals:** `appendTo`, `@primeuix/motion`, `OverlayService`, a shared "modal" helper with `Drawer` (extract when a third consumer needs it).

## Decisions

### D1. Crate
`prime-dialog` with `Dialog`, `DialogPosition`, `DialogPt`. Depends on `prime-core`, `prime-dom`, `prime-button`, `prime-icons` (`times`, `windowmaximize`, `windowminimize`), `web-sys`.

### D2. State
As in `Drawer`: `visible` (desired), `rendered` (in DOM), `leaving` (leave playing); additionally `maximized: RwSignal<bool>`. A single `Effect` on `(visible, panel mounted)` triggers `opened()` / `begin_leave()`; `finish_leave()` — on the window's `animationend` or the fallback timer (700 ms; the animation is 150–300 ms, we take a common margin). Re-showing during leave returns the window to the "open" state.

### D3. Pure logic (`state.rs`)
- `DialogPosition`, `mask_classes(position, modal)`, `root_classes(maximized, leaving)`.
- `mask_inline_style(position, modal) -> String` (`position:fixed; … justify-content / align-items / pointer-events`), `root_inline_style()`.
- `drag_target(offset_left, offset_top, margin_left, margin_top, dx, dy) -> (left, top)` and `clamp_drag(left, top, width, height, viewport_w, viewport_h, min_x, min_y, keep) -> (Option<left>, Option<top>)` — independent acceptance per axis, as in PrimeNG.
- `resize_to(width, height, dx, dy, has_been_dragged, min_w, min_h, offset_left, offset_top, viewport_w, viewport_h) -> (Option<width>, Option<height>)`.
- `breakpoints_css(id, &[(bp, width)]) -> String`.
- `is_topmost(own_z, top_z)`, `mask_z(panel_z)`.
Table-driven tests for each.

### D4. Rendering into `body`
`Portal` moves the mask and the window nested in it as one block (in PrimeNG the window is a descendant of the mask, so `Portal` holds `div.p-dialog-mask > div.p-dialog`). The markup is built inside the `Portal` closure (properties in `Arc<Parts>`), as in `Drawer`. A mask click is detected by the event target (`target == currentTarget`).

### D5. Pointer
Dragging and resizing — `mousemove`/`mouseup` listeners on `window`, attached on `mousedown` and removed on `mouseup` and on close; values are applied directly to element styles (not via signals — frequent events). Text selection is blocked by the `data-p-unselectable-text` attribute and `user-select: none` on `body` for the duration of the gesture. The position is remembered in the window style and reset on close.

### D6. Maximizing and scrolling
`maximize()` toggles the signal and calls `on_maximize`; scroll locking — via `block_body_scroll` bound to `Core.scroll_blocked` (a separate flag for maximizing and for modality, both released in `release_layer`). The shared `prime-dom` counter makes nested dialogs safe.

### D7. Responsiveness
`breakpoints` → a `<style>` in `head` is created on show and removed on `release_layer`/unmount; the window gets a `data-dlg-<id>` attribute (instead of a bare `uuid`), the rule `.p-dialog[data-dlg-<id>]:not(.p-dialog-maximized)`.

### D8. Focus
`focus()` along the chain content → footer → header via `prime_dom::first_focusable`, 5 ms delay (`set_timeout`); `FocusTrap` with `focus_trap`.

### D9. CSS
`dialog` is added to `theme.components`; `dialogstyle.ts` has no `For PrimeNG` block (position and alignment are inline styles, see D3). Check: the CSS of other components did not change.

### D10. Tests
SSR: hidden by default, properties do not leak. Browser `tests/web.rs` (a single sequential run, as for `Drawer`): showing in `body`, buttons, closing via button/Escape/mask (`dismissable_mask`), non-modal (clicks pass through), positions, dragging and boundary, resizing, maximizing, `breakpoints`, focus, two dialogs, re-showing during leave, events, listener removal. e2e: real mouse (`page.mouse`) — dragging, resizing, maximizing; keyboard. The crate is added in three places of `ci.yml`.

### D11. Benchmarks
Scenarios `dialog-1`/`-1000` (hidden dialog) and `case-dialog`; first `cargo bench -p render-bench --bench alloc`, for wasm — a temporary large budget (replace by a unique line).

## Risks / Trade-offs

- [Animations] `p-dialog-enter-active` may have no `@keyframes` in the theme → resolved in the first task; the fallback leave timer works without animation too.
- [Fallback timer] if the animation is longer than the timer, the window disappears earlier; we take 700 ms (as in `Drawer`).
- [Pointer on touch screens] dragging and resizing — mouse only (as in PrimeNG); recorded in docs.
- [Duplication with Drawer] the common part (layers, scrolling, Escape, leave) is repeated; extract it as a separate change when a third consumer appears.

## Open Questions

- Whether `@keyframes` exist for `p-dialog-enter-active` / `p-dialog-leave-active` in `@primeuix/styles/dialog` after `gen-theme`. If not — look in `packages/primeng` and `@primeuix/*`; if they are nowhere, write our own keyframes in `xtask` (scale 0.7 and opacity, `transformOptions: scale(0.7)` in the source) and note it in docs. Resolved in the first task.

## Migration Plan

No migration. Rollback — `git revert`.
