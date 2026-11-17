# Tasks

## 1. Foundation

- [x] 1.1 `orderlist` in `theme.components` and `primeng_extra`, `cargo xtask gen-theme`; `p-orderlist-controls` is present
- [x] 1.2 Cross-check against the source's `moveUp/Top/Down/Bottom`, `moveDisabled`

## 2. Crate `prime-orderlist`

- [x] 2.1 The module `reorder.rs` with table tests
- [x] 2.2 The `OrderList`, `OrderListPt` component: the button panel and `Listbox`, SSR tests, rustdoc
- [x] 2.3 The button actions, `on_reorder`, `on_selection_change`, `disabled`
- [x] 2.4 Browser `tests/web.rs`

## 3. Showcase and e2e

- [x] 3.1 The `/orderlist` page, the menu, the route, `stylesheet(&[..])`, the crate in the workspace and the showcase
- [x] 3.2 `e2e/tests/pages.json` and `orderlist.spec.ts`

## 4. Benchmarks and budgets

- [x] 4.1 Scenarios `-1` and `-1000`, `case-orderlist`, budgets

## 5. Integration

- [x] 5.1 `docs/orderlist.md`; `prime-orderlist` in the browser job of `ci.yml` (three places)
- [x] 5.2 fmt, clippy, tests, deny, machete, `openspec validate port-orderlist`

## Workflow follow-up

- Wave 6 — CI once at the end of the wave; commit and archive locally.
