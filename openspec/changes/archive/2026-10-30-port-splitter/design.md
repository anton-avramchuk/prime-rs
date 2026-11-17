# Design

## Context

Requirements are in `specs/splitter`. PrimeNG 21.1.10 sources: `splitter/splitter.ts`, `splitter/style/splitterstyle.ts`; demos `apps/showcase/doc/splitter/{horizontal,vertical,size,nested}-doc.ts`. Reference implementations in the project: `prime-slider` (dragging: `window_event_listener` from a handler, handle in `StoredValue<_, LocalStorage>`, removal in `on_cleanup`), `prime-knob` (pointer coordinates), `prime-fieldset` (two-way signal).

Observations about the source:
- PrimeNG writes panel sizes directly into the DOM elements' `style.flexBasis`; we keep a `RwSignal<Vec<f64>>` and render `flex-basis` reactively.
- `validateResize` compares `minSizes[0]` with the previous panel and `minSizes[1]` with the next one regardless of the gutter index (for three or more panels this is a bug in the source); we use `minSizes[i]` and `minSizes[i+1]`.
- Keyboard: `keydown` sets a 40 ms timer and calls `resizeStart(keydown)` + `onResize(step)`, where neighbor sizes are taken in pixels; `keyup` finishes. We apply the step on every `keydown` without a timer: `Δ% = 100 · step / root_size`.
- Touch: `touchstart`/`touchmove`/`touchend` on the gutter plus document listeners; in the first version we handle touch the same way as mouse, via `window` listeners, without a device check.
- `p-splitterpanel-nested` and `nested` in `data-p` are computed in PrimeNG from `contentChild(Splitter)`; we introduce an explicit `nested` property.
- A gutter is rendered after every panel except the last (`*ngIf="i !== panels.length - 1"`).

## Decisions

### D1. Crate and dependencies
`prime-splitter` with `Splitter`, `SplitterLayout { Horizontal, Vertical }`, `SplitterStorage { Session, Local }`, `SplitterResizeEvent { sizes }`, `SplitterPt { root, panel, gutter, gutter_handle }`. Depends on `prime-core` and `web-sys`.

### D2. Pure logic (`state.rs`)
- `initial_sizes(panel_sizes, n) -> Vec<f64>` - given non-zero ones or `100/n`.
- `flex_basis(size, n, gutter) -> String` — `calc(<size>% - <(n-1)*gutter>px)`.
- `drag_delta(start, current, container, rtl, horizontal) -> f64` - the offset in percent.
- `apply(sizes, index, prev, next, min_sizes) -> Option<(f64, f64)>` - the `min_sizes` check for a pair.
- `key_step(layout, key, step) -> Option<f64>` - the sign of the step by key and layout.
- `root_classes`, `panel_classes`, `data_p`.
Table-driven tests for each function.

### D3. State
`sizes: RwSignal<Vec<f64>>` (percent), `drag: StoredValue<Option<Drag>>` (index, start position, start sizes of the pair, root size), `resizing: RwSignal<Option<usize>>` for classes and `data-p-gutter-resizing`; `prev_sizes: RwSignal<Vec<f64>>` is not needed: `aria-valuenow` is taken from `sizes[i]`.

### D4. Dragging
`on:mousedown` on the gutter: measures the root size (`getBoundingClientRect` along the axis), remembers the start position (`page_x/page_y` - as in the original) and the sizes of the pair, calls `on_resize_start`, and sets a `window_event_listener` for `mousemove` and `mouseup`/`touchmove`/`touchend`. The listener handle lives in `StoredValue<_, LocalStorage>` and is removed on `mouseup`/`touchend` and in `on_cleanup`.

### D5. State in storage
`state_key` + `state_storage`; reading in an `Effect` after mount (`wasm32` only), writing when a resize finishes. Storage errors (forbidden, full) are silently ignored. The format is a JSON array of numbers, as in PrimeNG; parsing is manual without new dependencies (we do not add `serde_json`).

### D6. Keyboard
Handle: `on:keydown` applies the step to the pair `(i, i+1)`, always `preventDefault` for arrows (as in the original, even for the "wrong" axis); `on:keyup` finishes (save, `on_resize_end`). `on_resize_start` is called on the first `keydown` of a series (until there was a `keyup`).

### D7. CSS
`splitter` is added to `theme.components`; `cargo xtask gen-theme`; check that the CSS of others did not change.

### D8. Tests
SSR: markup, `layout`, default and given sizes, `nested`, `pt`. Browser `tests/web.rs`: dragging (synthetic `mousedown`/`mousemove`/`mouseup`), `min_sizes`, keys, `on_resize_*`, storage, listener removal. e2e: real mouse dragging and arrows in Chrome, nested Splitter. The crate is added in three places of `ci.yml`.

### D9. Benchmarks
Scenarios `splitter-1`/`-1000` (two panels), `case-splitter`; first `cargo bench -p render-bench --bench alloc`, for wasm a temporary large budget.

## Risks / Trade-offs

- [`min_sizes` divergence from PrimeNG for three or more panels] -> an intentional fix, recorded in the docs.
- [Touch is not verified on real devices] -> honestly recorded in the docs.
- [Storage is unavailable in SSR] -> reading only on the client; on the server always the initial sizes.

## Migration Plan

No migration. Rollback is `git revert`.

## Open Questions

None.
