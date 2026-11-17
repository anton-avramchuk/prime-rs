# Tasks

## 1. Crate and component

- [x] 1.1 Create the crate `crates/prime-knob` (`publish = false`, features `ssr`/`hydrate`, dependency `prime-core`, `web-sys` with mouse and touch events, `DomRect`, wasm dev-dependencies as in `prime-slider`), add it to the workspace; check `cargo check -p prime-knob --all-features`
- [x] 1.2 `state.rs`: `map_range`, `value_from_angle`, `zero_radians`, `value_radians`, `range_path`, `value_path`, `key_target`, `display_text`, root classes after `knobstyle.ts`; table-driven tests
- [x] 1.3 Implement `Knob`, `KnobPt` (value, SVG and arcs, text and template, colors and size, `aria-*`, `tabindex`, keyboard, `on_change`); SSR tests (structure, `aria-*`, size, paths at key points, template and colors, `show_value`, `disabled`/`readonly`, `pt`, branch parity)
- [x] 1.4 Pointer input: click, dragging with mouse and touch with `window` listeners only while pressed, focus on press, skipping `click` after dragging
- [x] 1.5 Browser tests `tests/web.rs`: keys, click at characteristic points and the gap, step, dragging with leaving the SVG, listener removal, skipping `click`, `readonly`/`disabled`, external signal, reactive bounds
- [x] 1.6 rustdoc with an example; `cargo test -p prime-knob --all-features`

## 2. Theme, showcase, e2e

- [x] 2.1 Add `knob` to `theme.components` (`upstream.toml`), `cargo xtask gen-theme`, make sure the CSS of other components did not change, wire up the CSS
- [x] 2.2 Page `/knob` (`basic`, `minmax`, `reactive`, `size`, `step`, `stroke`, `template`, `color`, `readonly`, `disabled`), menu, route, `stylesheet(&[..])`
- [x] 2.3 `e2e/tests/pages.json` and `e2e/tests/knob.spec.ts` (dragging and keys; wait for `networkidle`); restart the showcase server before the run; `npx playwright test knob`

## 3. Benchmarks and budgets

- [x] 3.1 Scenarios `knob-1`, `knob-1000`, `case-knob`, budgets from measurement (first `cargo bench -p render-bench --bench alloc`)

## 4. Integration

- [x] 4.1 Add `prime-knob` to the browser job of `ci.yml` (3 places); `docs/knob.md`; clippy, tests, deny, machete, `openspec validate port-knob`; CI `dev` green

## Workflow follow-up

- Archive after CI.
