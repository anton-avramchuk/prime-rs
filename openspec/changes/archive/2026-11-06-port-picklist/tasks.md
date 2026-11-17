# Tasks

## 1. Foundation

- [x] 1.1 `picklist` in `theme.components`, `cargo xtask gen-theme`; `Listbox::header_content`
- [x] 1.2 Check against the source's `moveRight/AllRight/Left/AllLeft`, `onSourceItemDblClick`, `*Disabled`

## 2. Crate `prime-picklist`

- [x] 2.1 Module `transfer.rs` with table tests
- [x] 2.2 The `PickList` component, `PickListPt`: button panels and two `Listbox`es, SSR tests, rustdoc
- [x] 2.3 Transfer, reordering, double click, `keep_selection`, search, `disabled`, events
- [x] 2.4 Browser `tests/web.rs`

## 3. Showcase and e2e

- [x] 3.1 Page `/picklist`, menu, route, `stylesheet(&[..])`, the crate in the workspace and showcase
- [x] 3.2 `e2e/tests/pages.json` and `picklist.spec.ts`

## 4. Benchmarks and budgets

- [x] 4.1 Scenarios `-1` and `-1000`, `case-picklist`, budgets

## 5. Integration

- [x] 5.1 `docs/picklist.md`; `prime-picklist` in the browser job of `ci.yml` (three places)
- [x] 5.2 fmt, clippy, tests, deny, machete, `openspec validate port-picklist`

## Workflow follow-up

- Wave 6 — CI once at the end of the wave; commit and archive locally.
