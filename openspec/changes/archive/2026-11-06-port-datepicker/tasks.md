# Tasks

## 1. Foundation

- [x] 1.1 `datepicker` in `theme.components`, `cargo xtask gen-theme`; `p-datepicker-day-selected-range` is present
- [x] 1.2 Cross-check against the source's `createMonth`, `isSelectable`, `selectDate`, `formatDate`, `parseDate`, navigation

## 2. Crate `prime-datepicker`

- [x] 2.1 `calendar.rs` module with table-driven tests
- [x] 2.2 `DatePicker` component, `DatePickerPt`: field, icons, panel, header, day grid, months, years, panel buttons, SSR tests, rustdoc
- [x] 2.3 Selection (`single`/`multiple`/`range`), constraints, text input, formatting
- [x] 2.4 Overlay (open, close, `inline`), navigation and views, "Today"/"Clear" buttons
- [x] 2.5 Browser `tests/web.rs`

## 3. Showcase and e2e

- [x] 3.1 Page `/datepicker`, menu, route, `stylesheet(&[..])`, the crate in the workspace and showcase
- [x] 3.2 `e2e/tests/pages.json` and `datepicker.spec.ts`

## 4. Benchmarks and budgets

- [x] 4.1 Scenarios `-1` and `-1000`, `case-datepicker`, budgets

## 5. Integration

- [x] 5.1 `docs/datepicker.md`; `prime-datepicker` in the browser job of `ci.yml` (three places)
- [x] 5.2 fmt, clippy, tests, deny, machete, `openspec validate port-datepicker`

## Workflow follow-up

- Wave 6 - CI once at the end of the wave; after archiving - push and one CI run for the whole wave 6.
