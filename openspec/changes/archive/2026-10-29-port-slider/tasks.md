# Tasks

## 1. Crate and component

- [x] 1.1 Create the `crates/prime-slider` crate (`publish = false`, features `ssr`/`hydrate`, dependency `prime-core`, `web-sys` with the needed events and geometry, wasm dev-dependencies as in `prime-scrollpanel`), add to the workspace; check `cargo check -p prime-slider --all-features`
- [x] 1.2 `state.rs`: `percent`, `value_from_percent`, `normalize`, `step_change`, `key_target`, `bar_percent`, `touch_percent`, root classes and `data-p` per `sliderstyle.ts` and `Slider.dataP`; table-driven tests
- [x] 1.3 Implement `Slider`, `SliderPt`, `SliderChange`, `SliderSlideEnd`, `SliderOrientation` (value and range, markup and handle position, `aria-*`, keyboard, value update, events); SSR tests (structure, position, `aria-*`, orientations, classes, `data-p`, `disabled`, `pt`, branch parity)
- [x] 1.4 Pointer input: click on the track, mouse dragging with `window` listeners only for the duration of dragging, touch, `animate`, `data-p-sliding`, RTL
- [x] 1.5 Browser tests `tests/web.rs`: keys, click on the track, dragging and listener removal, `disabled`, range, external signal and reactive bounds, handle and focus preservation
- [x] 1.6 rustdoc with an example; `cargo test -p prime-slider --all-features`

## 2. Theme, showcase, e2e

- [x] 2.1 Add `slider` to `theme.components` (`upstream.toml`), `cargo xtask gen-theme`, make sure the CSS of other components did not change, wire up the CSS
- [x] 2.2 The `/slider` page (`basic`, `range`, `step`, `vertical`, `filter`, `input`), menu, route, `stylesheet(&[..])`
- [x] 2.3 `e2e/tests/pages.json` and `e2e/tests/slider.spec.ts` (dragging and keys; wait for `networkidle`); restart the showcase server before the run; `npx playwright test slider`

## 3. Benchmarks and budgets

- [x] 3.1 Scenarios `slider-1`, `slider-1000`, `case-slider`, budgets by measurement (first `cargo bench -p render-bench --bench alloc`)

## 4. Integration

- [x] 4.1 Add `prime-slider` to the `ci.yml` browser job (3 places); `docs/slider.md`; clippy, tests, deny, machete, `openspec validate port-slider`; CI `dev` green

## Workflow follow-up

- Archive after CI.
