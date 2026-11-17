# Design

## Context

Requirements are in `specs/popover`. PrimeNG 21.1.10 sources: `popover/popover.ts`, `popover/style/*`; demos `apps/showcase/doc/popover/*-doc.ts`. The overlay model in the project is `prime-select` (panel inside the root, `prime_dom::align_overlay(Relative)`, `ZGroup::Overlay`, outside click and `resize` on `window`, `is_disposed()` checks in timers, `on_cleanup`). The model for shared state with a `Copy` handle is `prime-inplace`.

Observations about the source:
- Popover is imperative: `toggle(event, target?)`, `show`, `hide`. The target is `target ?? event.currentTarget ?? event.target`; `show` with `event` and `target` set calls `stopPropagation` so that the same click does not close the panel through the document.
- Positioning is `absolutePosition(container, target)`; arrow: if the panel's left edge is to the left of the target's left edge, `--p-popover-arrow-left = targetLeft − containerLeft − 2·borderRadius`, otherwise 0; if the panel's top is above the target's top — `data-p-popover-flipped` and `p-popover-flipped`.
- Closing: a document click (on iOS — `touchstart`), `document:keydown.escape`, window `resize` (not on touch devices), `ConnectedOverlayScrollHandler` on the target's ancestors. The `selfClick` flag and `OverlayService.clickObservable` are needed so that a click on a nested overlay does not close the outer panel.
- The `p-anchored-overlay` animation via `@primeuix/motion`; the panel is created on show (`render`) and removed after it leaves.

## Goals / Non-Goals

**Goals:** the same DOM, classes and closing behaviour; imperative control without external state; reuse of the `Select` overlay solutions.

**Non-Goals:** `appendTo`, `@primeuix/motion` transitions, the `OverlayService` bus, the `datatable` demo (see proposal).

## Decisions

### D1. Crate
`prime-popover` with `Popover`, `PopoverRef`, `PopoverPt`. Depends on `prime-rs-core`, `prime-dom`, `web-sys`.

### D2. Control object `PopoverRef`
`#[derive(Clone, Copy)] struct PopoverRef { visible: RwSignal<bool>, target: StoredValue<Option<SendWrapper<HtmlElement>>, LocalStorage> }`. `new()` creates the signals; methods `show(&MouseEvent)`, `show_at(&MouseEvent, &HtmlElement)`, `toggle`, `toggle_at`, `hide()`, `visible() -> bool`. For calling from a Leptos handler (`on:click=move |e| op.toggle(&e)`) the target is taken from `e.current_target()`. `Popover` reads `PopoverRef` from the `popover_ref` property; without it, its own one is created and is inaccessible (the component is useless — `popover_ref` is required, the `PopoverRef` type without `Option`).

Alternative — a trigger `NodeRef` and an `open` signal: simpler, but does not repeat the PrimeNG API (`op.toggle($event)`, `op.show($event, target)`), so it was not chosen.

### D3. Pure logic (`state.rs`)
- `root_classes(flipped)` → `p-popover p-component` (+ `p-popover-flipped`).
- `arrow_left(container_left, target_left, border_radius) -> f64` (`0` if the panel's left edge is not to the left of the target).
- `is_flipped(container_top, target_top) -> bool`.
- `outside_click(inside_panel, inside_target) -> bool`.
- `retarget(current: Option<&Id>, next: &Id) -> Retarget { Show, Toggle, Switch }` — the `toggle` decision with a visible/hidden panel and a target change.
Table tests for each.

### D4. Show
`Popover` renders the panel under `Show when=visible`. After insertion into the DOM (`request_animation_frame`, `wasm32` only): `align_overlay(panel, target, Relative, false)` → `arrow_left`/`flipped` from the `getBoundingClientRect` of the panel and the target, `z-index` from `prime_rs_core::zindex::generate(ZGroup::Overlay, base)` (if `auto_z_index`), `autofocus`, then `on_show`. `window` listeners: `click` (with `outside_click` checks via `contains`), `resize`, `keydown` (Escape), `scroll` in the capture phase on the target's ancestors — removed in `on_cleanup` and on hide (as in `Select`).

### D5. The same click
`show`/`toggle` from the target's click handler call `event.stop_propagation()` only when the window/document events are subscribed after the click: the outside listener is attached after `request_animation_frame`, so the click that opened the panel does not reach it. A click on the target with a visible panel is handled by `outside_click` (the target counts as "inside") — closing happens through the target handler's `toggle`, not the window.

### D6. Target change
`toggle` with a different target while the panel is visible: `hide()`, then on the next tick `show` at the new target (the counterpart of `destroyCallback`). The "waiting for show" state is in a `StoredValue`; an `is_disposed()` check before showing.

### D7. CSS
`@primeuix/styles` has a `popover` module; it is added to `theme.components`. `cargo xtask gen-theme`, check that the CSS of the others did not change. If the module has a `/* For PrimeNG */` block, add `popover` to `primeng_extra`.

### D8. Tests
SSR: hidden by default, `id`/`style`/`aria-*`/`pt`. Browser `tests/web.rs`: `show`/`hide`/`toggle`, explicit target, target change, click outside/inside/on the target, `dismissable=false`, Escape, `resize`, `autofocus`, the order of `on_show`/`on_hide`, listener removal. e2e: real mouse and keyboard, placement above the target at the bottom edge, ancestor scroll. The crate is added in three places in `ci.yml`.

### D9. Benchmarks
Scenarios `popover-1`/`-1000` (hidden panel) and `case-popover`; first `cargo bench -p render-bench --bench alloc`, for wasm — a temporary large budget (replace by line number, not by a global `sed`).

## Risks / Trade-offs

- [Panel inside the root, not in `body`] → a parent with `overflow: hidden` will clip the panel; recorded in docs, the `Select` solution is the same. `Relative` requires an ancestor with `position: relative`: in the demos the trigger and `Popover` live in it.
- [No `OverlayService`] → a click in a nested overlay (e.g. `Select` inside Popover) may close the outer panel if the nested overlay's dropdown panel ends up outside the DOM subtree. Inside the `Select` root the panel stays inside the Popover DOM, so `contains` works; recorded in docs.
- [No animation] → the panel appears instantly; the difference is in docs.

## Open Questions

None: `appendTo` and animations are out of scope, the rest repeats the `Select` solutions.

## Migration Plan

No migration. Rollback — `git revert`.
