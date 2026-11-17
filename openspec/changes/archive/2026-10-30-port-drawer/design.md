# Design

## Context

Requirements are in `specs/drawer`. PrimeNG 21.1.10 sources: `drawer/drawer.ts`, `drawer/style/drawerstyle.ts`; demos `apps/showcase/doc/drawer/*-doc.ts`. Reference implementations in the project: `prime-popover` (`Core` with cleanup closures, window listeners, `PopoverRef`), `prime-select` (`ZGroup`, `on_cleanup`), `prime-dom` (`Portal` and `FocusTrap` are ready, no consumers yet).

Observations about the source:
- The state has two levels: `visible` (desired) and `modalVisible` (in the DOM). The panel is created when `visible=true` and removed after `onAfterLeave` of the leave animation (`@primeuix/motion`).
- The mask is created imperatively in `body` in `enableModality()` with `z-index = panel z - 1` (or `z of the topmost open panel - 1`); on close it gets `p-overlay-mask-leave-active` and is removed on `animationend`.
- The `For PrimeNG` block in `drawerstyle.ts` is marked `/** For PrimeNG **/` (two asterisks), so `xtask gen-theme` does not find it: support for this spelling is needed. It contains positions (`p-drawer-left` ...), `position: fixed` and the animation classes `p-drawer-enter-*` / `p-drawer-leave-*`, which refer to `@keyframes p-animate-drawer-*`; these keyframes are absent from the project's `base.css` - check after generation (see Open Questions).
- `Escape` closes the panel through a document listener, only if its `z-index` equals the topmost one (`ZIndexUtils.get`); the `keydown` handler on the root itself calls `hide(false)`, which closes nothing - this leftover is not ported.
- `onHide` in PrimeNG is called only from `close()` (user actions), not on programmatic close.

## Goals / Non-Goals

**Goals:** the same DOM, classes, closing and layer order; the panel, mask, scroll lock and focus trap built on ready-made `prime-dom` facilities.

**Non-Goals:** `appendTo`, `@primeuix/motion`, `closeButtonProps` (see proposal).

## Decisions

### D1. Crate
`prime-drawer` with `Drawer`, `DrawerPosition`, `DrawerPt`. Depends on `prime-rs-core`, `prime-dom`, `prime-button`, `prime-icons` (`times`), `web-sys`.

### D2. State: `visible` and `rendered`
`visible: RwSignal<bool>` is the desired state (a two-way signal or its own). `rendered: RwSignal<bool>` - the panel is in the DOM. `Effect`: `visible` became true -> `rendered=true`; became false and `rendered` -> the leave starts: the root class changes to `p-drawer-leave-*`, the mask's to `p-overlay-mask-leave-active`, then `rendered=false` after the root's `animationend` or via a fallback timer (`set_timeout` 700 ms - the animation lasts 500 ms; needed if styles are not loaded or `prefers-reduced-motion` is on), with an `is_disposed()` check.

### D3. Pure logic (`state.rs`)
- `DrawerPosition` and `position_class(position, full_screen)` -> `left|right|top|bottom|full`.
- `root_classes(position, full_screen, open) -> String`, `enter_class(..)`, `leave_class(..)`, `mask_classes(modal, leaving, full)`.
- `is_topmost(own_z, top_z) -> bool`.
- `mask_z(own_z) -> i32` (`own_z - 1`).
Table-driven tests for each.

### D4. Output into `body`
`Portal` from `prime-dom` moves the panel and mask into `body` as one block: first the mask, then the panel (DOM order for overlap; `z-index` is set explicitly). The mask is a separate `div`, not a descendant of the panel, so a click on the panel does not reach it. Outside the browser (SSR) `Portal` renders nothing.

### D5. Layers and the effect side (`wasm32` only)
After the panel is inserted (an `Effect` tracking the panel's `NodeRef`, as in Popover): `z-index` from `prime_rs_core::zindex::generate(ZGroup::Modal, base)` (with `auto_z_index`), the mask gets `z-index - 1`; `FocusTrap::attach(&panel)`; `block_body_scroll()` with `block_scroll`; a `keydown` subscription on `window` with `close_on_escape` (on the next tick, as in Popover); `on_show`. On close: the listener, the trap, `unblock_body_scroll()`, `revert(z)`, `on_hide`.

### D6. Closing
A single `close()`: `visible.set(false)`. Sources: a click on the mask with `dismissible` (the click is on the mask, not the panel - the mask is a separate element), the close button, `Escape` on the topmost panel (`is_topmost` by `zindex::current()`). `on_hide` is called on any transition of `visible` to false (in PrimeNG only on user actions; the difference is recorded in the docs).

### D7. Scroll lock
`prime_dom::dom::block_body_scroll()` / `unblock_body_scroll()` with a shared per-thread counter: the `p-overflow-hidden` class is set at 0->1 and removed at 1->0. The pure part - the counter - is tested separately; DOM access only in the browser. The scrollbar width (`--p-scrollbar-width`) is set on lock, as in PrimeNG `blockBodyScroll`.

### D8. Close button
`Button` with `text`, `rounded`, `severity=Secondary` and `TimesIcon` (or `close_icon`); `aria_close_label` defaults from the locale (`aria.close`); `Enter` on the button closes (native `button` behaviour).

### D9. CSS
`drawer` is added to `theme.components` and `primeng_extra`; `xtask gen-theme` is extended with the `/** For PrimeNG **/` marker (a test next to the existing ones). Check: the CSS of other components did not change.

### D10. Tests
SSR: hidden by default, properties do not leak. Browser `tests/web.rs` (one sequential test run, as for Popover - window listeners are shared): showing, closing by button/mask/Escape, `dismissible=false`, `modal=false`, positions, animation classes, removal from the DOM after leave, `block_scroll`, Tab cycle, two panels (z-index, Escape closes the topmost), `on_show`/`on_hide`, listener removal. e2e: a real mouse and keyboard, scrolling is blocked. The crate is added in three places of `ci.yml`.

### D11. Benchmarks
Scenarios `drawer-1`/`-1000` (hidden panel) and `case-drawer`; first `cargo bench -p render-bench --bench alloc`, for wasm a temporary large budget (replace by a unique line, not a global `sed`).

## Risks / Trade-offs

- [Fallback leave timer] -> if `animationend` never arrives and the timer is shorter than the animation, the panel disappears early; the timer is 700 ms for a 500 ms animation.
- [Portal and hydration] -> a panel in `body` does not take part in hydration; a hidden-by-default panel renders nothing on SSR, and one opened before hydration is not supported and is recorded in the docs.
- [No `@primeuix/motion`] -> animations only via CSS classes; `motionOptions` are not ported.
- [`on_hide` on programmatic close] -> a difference from PrimeNG, recorded in the docs.

## Open Questions

- Whether `@keyframes p-animate-drawer-*` exist in `@primeuix/styles/drawer` after `gen-theme`. If not, they must be taken from the PrimeNG sources (search in `packages/primeng` and `@primeuix/*`); if they are nowhere, write custom keyframes in `xtask` (a shift by the panel width/height) and note it in the docs. Resolved in the first theme task, before the other style work.

## Migration Plan

No migration. Rollback is `git revert`.
