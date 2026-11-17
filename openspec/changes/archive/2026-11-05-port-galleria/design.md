# Design

## Context

The requirements are in `specs/galleria`. PrimeNG 21.1.10 sources: `galleria/galleria.ts` (`Galleria`, `GalleriaContent`, `GalleriaItem`, `GalleriaThumbnails`), `style/galleriastyle.ts`, `@primeuix/styles/galleria`. Templates in the project: `prime-drawer` (`Portal`, mask, `rendered`/`leaving`, `FocusTrap`, `block_body_scroll`), `prime-carousel` (the shift of the strip, arrows), `prime-slider` (listeners).

## Goals / Non-Goals

**Goals:** the same DOM and classes, the same formulas of the thumbnail window.

**Non-Goals:** the system full screen, `Bind`, Angular forms, configuring the animation through options.

## Decisions

### D1. Pure model (`model.rs`)
Functions without the DOM: `median_index(n)`, `total_pages(len, n)`, `shifted_items(len, n, active) -> i32` (the formulas of `onAfterContentChecked`), `nav_backward_disabled`/`nav_forward_disabled`, `first_active`/`last_active`/`is_item_active`, `next_index`/`prev_index` (with `circular`), `responsive_num_visible(options, width, base)`, `position_class(prefix, pos)`, `thumbnail_item_classes`. In PrimeNG `step()` always overrides the offset by recomputing it from the index before changing the active index, so for us the offset is a function `(active, num_visible)`; there is no separate state.

### D2. Data and templates
`GalleriaImage { item_image_src, thumbnail_image_src, alt, title }`; custom templates — `item: Callback<GalleriaImage, AnyView>`, `thumbnail`, `caption` (`Callback`), `indicator: Callback<usize, AnyView>`, `header`/`footer` — `ViewFn`. All are stored in `Arc<Parts>` (the children of `Portal` are `Fn + Send + Sync`).

### D3. State
`active: RwSignal<usize>` (external or internal), `slide_show_active: RwSignal<bool>`, `num_visible_now: RwSignal<usize>` (adaptive), `items_hidden: RwSignal<bool>` (the class `p-items-hidden` on the strip: removed when the index changes, set on `transitionend`). The offset is a reactive `translate3d(…)` and `transition: transform 500ms ease 0s` on the strip.

### D4. Auto-change
`set_interval` under `cfg(wasm32)` inside an `Effect` tied to `auto_play` and `slide_show_active`; removal in `on_cleanup`; stopping — `stop_slide_show`.

### D5. Full screen
As in Drawer: `visible` + `rendered` + `leaving`; the mask and the content — in `Portal`; the root with `p-galleria-fullscreen`; `animationend` of the root (`p-galleria-leave-active`) or a fallback timer of 700 ms completes the exit. A click on the mask closes only when `target == currentTarget`. The layer — `ZGroup::Modal`.

### D6. Adaptivity
`responsive_num_visible` is called on creation and on `resize` of the window (`window_event_listener`) only when `responsive_options` are set; `flex: 1 0 (100/n)%` for the thumbnails — an inline style (instead of PrimeNG's dynamic `<style>`).

### D7. Tests
The model — tables. SSR: the markup of the basics, indicators, captions, positions, `full_screen` closed. Browser `tests/web.rs` as one sequential test: navigation, `circular`, indicators, the keys of the thumbnails, auto-change, full screen (opening/closing/a click on the mask). e2e: real clicks and styles.

## Risks / Trade-offs

- [Touch] The thumbnails are scrolled by touch — emulation only.
- [Exit animation] `animationend` + a fallback timer, as in Drawer.

## Open Questions

- None.

## Migration Plan

No migration. Rollback — `git revert`.
