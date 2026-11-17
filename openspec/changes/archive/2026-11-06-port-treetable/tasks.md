# Tasks

## 1. Foundation

- [x] 1.1 `treetable` in `theme.components` and `primeng_extra`, `cargo xtask gen-theme`
- [x] 1.2 Check against the source `serializeNodes`, `serializePageNodes`, `sortNodes`, `propagateSelectionUp/Down`, `TreeTableToggler`

## 2. Crate `prime-treetable`

- [x] 2.1 `tree.rs` model with table-driven tests
- [x] 2.2 `TreeTable<T>` component, `TreeNode<T>`, `TreeTablePt`: markup, expand button, headers, paginator, SSR tests, rustdoc
- [x] 2.3 Expansion, sorting, selection by click and checkboxes, node events
- [x] 2.4 Browser `tests/web.rs`

## 3. Showcase and e2e

- [x] 3.1 Page `/treetable`, menu, route, crate in the workspace and showcase
- [x] 3.2 `e2e/tests/pages.json` and `treetable.spec.ts`

## 4. Benchmarks and budgets

- [x] 4.1 Scenarios `-1` and `-1000`, `case-treetable`, budgets

## 5. Integration

- [x] 5.1 `docs/treetable.md`; `prime-treetable` in the browser job of `ci.yml` (three places)
- [x] 5.2 fmt, clippy, tests, deny, machete, `openspec validate port-treetable`

## Workflow follow-up

- Wave 7 is complete: push and a single CI run for the whole wave.
