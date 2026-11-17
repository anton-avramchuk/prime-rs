# Tasks

## 1. Foundation

- [x] 1.1 `colorpicker` in `theme.components` (`upstream.toml`), `cargo xtask gen-theme`; the CSS of other components did not change, `p-colorpicker-hue-handle` is present
- [x] 1.2 Check against the source's `pickColor`, `pickHue`, `updateUI`, `writeControlValue`, `onInputKeydown`, `HSBtoRGB`/`RGBtoHSB`

## 2. Crate `prime-colorpicker`

- [x] 2.1 Module `color.rs` with table-driven tests
- [x] 2.2 Component `ColorPicker`, `ColorPickerPt`, `ColorPickerRef`: swatch, panel, `inline`, SSR tests, rustdoc
- [x] 2.3 Opening/closing (click, `Space`, `Escape`, `Tab`, outside click, resize), `on_show`/`on_hide`
- [x] 2.4 Mouse and touch gestures on the field and scale, `p-colorpicker-dragging`, `on_change`, formats `hex`/`rgb`/`hsb`, external value, `disabled`
- [x] 2.5 Browser `tests/web.rs` (one sequential test)

## 3. Showcase and e2e

- [x] 3.1 Page `/colorpicker` (`basic`, `inline`, `format`, `disabled`), menu, route, `stylesheet(&[..])`, crate in the workspace and showcase
- [x] 3.2 `e2e/tests/pages.json` and `colorpicker.spec.ts`; `networkidle`, restart the showcase server; `PW_CHANNEL=chrome npx playwright test colorpicker`

## 4. Benchmarks and budgets

- [x] 4.1 Scenarios `-1` and `-1000`, `case-colorpicker`, budgets (allocations — `cargo bench -p render-bench --bench alloc`, wasm — a temporary large budget)

## 5. Integration

- [x] 5.1 `docs/colorpicker.md`; `prime-colorpicker` in the browser job of `ci.yml` (three places)
- [x] 5.2 `cargo fmt --all -- --check`, clippy with CI exclusions and `--all-features`, tests (check the wasm tests' `test result` by line), deny, machete, `openspec validate port-colorpicker`

## Workflow follow-up

- Wave 6 — CI once at the end of the wave; after apply commit locally and archive.
