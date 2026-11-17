# Tasks

## 1. Foundation

- [x] 1.1 Add `listbox` to `theme.components` and `primeng_extra` (`upstream.toml`), `cargo xtask gen-theme`; check that the CSS of other components did not change and that `p-listbox-*` classes are present

## 2. Crate

- [x] 2.1 `crates/prime-listbox`: `logic.rs` (`ListboxOption`, `ListboxGroup`, `Entry`, `visible_entries`, indices, `select_single`, `select_multiple`, `select_range`, `select_all`, `all_selected`, `find_by_prefix`, `root_classes`, `option_classes`, `data_p`) with table-driven tests
- [x] 2.2 `Listbox` component and `ListboxPt`: root, header, search field, list, groups, empty message, `option`/`group`, SSR tests, rustdoc
- [x] 2.3 Selection: single and multiple modes, `meta_key_selection`, option and "select all" checkboxes, `checkmark`, `readonly`/`disabled`, `on_change`/`on_click`/`on_dbl_click`/`on_select_all_change`
- [x] 2.4 Focus and keys: list focus, arrows, Home/End, PageUp/PageDown, Enter/Space, `Shift` ranges, `Ctrl+A`, type-ahead search, `focus_on_hover`, `select_on_focus`, scrolling to the focused one, `on_focus`/`on_blur`, search field and `on_filter`; browser `tests/web.rs`

## 3. Showcase and e2e

- [x] 3.1 Page `/listbox` (`basic`, `multiple`, `group`, `filter`, `checkbox`, `checkmark`, `disabled`, `invalid`, `template`), menu, route, `stylesheet(&[..])`, the crate in the workspace and showcase
- [x] 3.2 `e2e/tests/pages.json` and `listbox.spec.ts` (markup, mouse selection, multiple selection with `Ctrl`/`Shift`, checkboxes, search, keys); wait for `networkidle`, restart the showcase server; `PW_CHANNEL=chrome npx playwright test listbox`

## 4. Benchmarks and budgets

- [x] 4.1 Scenarios `-1` and `-1000`, `case-listbox` and budgets (first `cargo bench -p render-bench --bench alloc`; wasm measurement in the background)

## 5. Integration

- [x] 5.1 `docs/listbox.md` (differences from PrimeNG, what was not verified); `prime-listbox` in the browser job of `ci.yml` (three places)
- [x] 5.2 `cargo fmt --all -- --check`, clippy with CI exclusions and `--all-features` (also for the wasm target), tests, deny, machete, `openspec validate port-listbox`; CI `dev` green

## Workflow follow-up

- Archive after CI.
- Next: MultiSelect.
