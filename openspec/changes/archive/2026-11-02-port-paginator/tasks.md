# Tasks

## 1. Foundation

- [x] 1.1 Add `paginator` to `theme.components` (`upstream.toml`), `cargo xtask gen-theme`; check that the CSS of other components did not change, that the `p-paginator-*` classes are present and that `select`/`inputnumber` cover the nested lists and field

## 2. Crate

- [x] 2.1 `crates/prime-paginator`: `state.rs` (`page_count`, `page`, `link_bounds`, `links`, `report`, `rows_items`, `page_items`, `clamp_first_after_total`, `next_first_for_rows`, classes) with table-driven tests
- [x] 2.2 `Paginator`, `PaginatorState`, `RowsPerPage`, `PaginatorPt` component: root, buttons, links, report, `template_left`/`template_right`, icons, hiding, SSR tests, rustdoc
- [x] 2.3 Behaviour: `change_page`, buttons and links, `on_page_change`, two-way `first`/`rows`, stepping back when `total_records` shrinks
- [x] 2.4 Lists and field: jumping to a page via list and field, rows count with "Show all", sync with `first`/`rows`; browser `tests/web.rs`

## 3. Showcase and e2e

- [x] 3.1 `/paginator` page (`basic`, `currentpagereport`, `template`, `images`), menu, route, `stylesheet(&[..])`, the crate in the workspace and showcase
- [x] 3.2 `e2e/tests/pages.json` and `paginator.spec.ts` (markup, buttons and links, link window, lists, report); wait for `networkidle`, restart the showcase server; `PW_CHANNEL=chrome npx playwright test paginator`

## 4. Benchmarks and budgets

- [x] 4.1 Scenarios `-1` and `-1000`, `case-paginator` and budgets (first `cargo bench -p render-bench --bench alloc`; wasm measurement — in the background, with a separate budgets file)

## 5. Integration

- [x] 5.1 `docs/paginator.md` (differences from PrimeNG, what was not verified); `prime-paginator` in the browser job of `ci.yml` (three places)
- [x] 5.2 `cargo fmt --all -- --check`, clippy with CI exclusions and `--all-features` (and for the wasm target), tests, deny, machete, `openspec validate port-paginator`; CI `dev` is green

## Workflow follow-up

- Archive after CI.
- Next: Stepper.
