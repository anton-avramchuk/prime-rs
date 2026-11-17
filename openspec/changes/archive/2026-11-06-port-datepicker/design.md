# Design

## Context

Requirements are in `specs/datepicker`. PrimeNG 21.1.10 sources: `datepicker/datepicker.ts` (`createMonth`, `isSelectable`, `selectDate`, `onDateSelect`, `formatDate`, `parseDate`, navigation), `style/datepickerstyle.ts`, `@primeuix/styles/datepicker`. Reference implementations: `prime-colorpicker` (overlay inside the root, outside close), `prime-inputmask` (field with InputText classes), `prime-select`.

## Goals / Non-Goals

**Goals:** the same DOM, the same classes, the same calendar formulas and date format.

**Non-Goals (stage 2+):** time, keyboard navigation across cells, `touch_ui`, `data_type`, `keep_invalid`, templates, locales.

## Decisions

### D1. Calendar logic without libraries
`CalDate` (month 0-11, like JS) and Howard Hinnant's algorithms for converting to a day number; the grid, constraints, selection, formatting and parsing are pure functions in `calendar.rs` with table-driven tests. "Today" is passed as a parameter: `js_sys::Date` in the browser, `SystemTime` in native builds, the `today` property for tests.

### D2. State
`value: RwSignal<Vec<CalDate>>`, `view: RwSignal<View>` (current view), `current_month`, `current_year`, `open: RwSignal<bool>`, and the field text is a local signal that is overwritten when the value changes and on blur. Months are recomputed by a `Memo` from month/year/constraints.

### D3. Overlay
The panel is inside the root (`align_overlay(Relative)`, `ZGroup::Overlay`), outside click and `resize` on `window` (subscriptions via `set_timeout(0)`), `Escape` and `Tab` in the field. Closing after selection uses `set_timeout` of 150 ms.

### D4. Input
`input` parses the text according to the mode (separators), unavailable dates are not accepted; `blur` restores the formatted text; pressing `Enter` closes the panel.

### D5. Tests
Calendar: tables. SSR: field, icon, panel (inline), header, grid, selected days, constraints, `show_week`, modes, month/year. Browser `tests/web.rs`: open/close, selection, range, multiple, navigation, views, text input, panel buttons, `disabled`. e2e: styles and real clicks.

## Risks / Trade-offs

- [Scope] Keyboard navigation across cells is deferred - recorded in the docs.
- [Time zone] Dates without time or zone - no midnight discrepancies with `Date`.

## Open Questions

- None.

## Migration Plan

No migration. Rollback is `git revert`.
