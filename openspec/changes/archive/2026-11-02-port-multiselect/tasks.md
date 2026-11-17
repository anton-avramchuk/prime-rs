# Tasks

## 1. Foundation

- [x] 1.1 Add `multiselect` to `theme.components` (`upstream.toml`), `cargo xtask gen-theme`; check that the CSS of the other components did not change, that the `p-multiselect-*` classes exist and that `Chip` and `iconfield` cover the chips and the search field

## 2. Crate

- [x] 2.1 `crates/prime-multiselect`: `logic.rs` (model, flat list, filter, indices, selection, ranges, `toggle_all`, `all_selected`, selection limit, `label_text`, `chip_labels`, `summary`, classes, `data_p`) with table tests
- [x] 2.2 The `MultiSelect` component and `MultiSelectPt`: root, hidden field, label (`comma`/`chip`), clear and arrow/loading icons, panel with header, search, list, groups and empty message, views, SSR tests, rustdoc
- [x] 2.3 Panel: open/close, alignment, z-index, click outside, `resize`, search field focus, `reset_filter_on_hide`, `on_show`/`on_hide`, external `open`
- [x] 2.4 Selection: click on an option, selection limit, "select all" checkbox (with search), chips and their removal, clear, `on_change`/`on_remove`/`on_clear`/`on_select_all_change`, `readonly`/`disabled`/`loading`
- [x] 2.5 Keys: navigation and opening the panel, ranges, `Ctrl+A`, `Alt+↑`, `Tab`, `Escape`, type-ahead search, `focus_on_hover`, `select_on_focus`, scrolling to the focused one, `on_focus`/`on_blur`, search field and `on_filter`; browser `tests/web.rs`

## 3. Showcase and e2e

- [x] 3.1 Page `/multiselect` (`basic`, `chips`, `clearicon`, `disabled`, `filled`, `filter`, `fluid`, `group`, `invalid`, `loadingstate`, `sizes`, `template`, `floatlabel`, `iftalabel`), menu, route, `stylesheet(&[..])`, the crate in the workspace and showcase
- [x] 3.2 `e2e/tests/pages.json` and `multiselect.spec.ts` (markup, opening and position of the panel, mouse selection, chips, "select all", search, keys); wait for `networkidle`, restart the showcase server; `PW_CHANNEL=chrome npx playwright test multiselect`

## 4. Benchmarks and budgets

- [x] 4.1 Scenarios `-1` and `-1000`, `case-multiselect` and budgets (first `cargo bench -p render-bench --bench alloc`; wasm measurement — in the background, with a separate budgets file)

## 5. Integration

- [x] 5.1 `docs/multiselect.md` (differences from PrimeNG, what was not verified); `prime-multiselect` in the browser job of `ci.yml` (three places)
- [x] 5.2 `cargo fmt --all -- --check`, clippy with the CI exclusions and `--all-features` (and for the wasm target), tests, deny, machete, `openspec validate port-multiselect`; CI `dev` green

## Workflow follow-up

- Archive after CI.
- Next: Paginator, then Stepper.
