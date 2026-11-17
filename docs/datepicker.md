# DatePicker (stage 1)

Port of `p-datepicker` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`). This is **stage 1**: time, keyboard
navigation across cells and a number of rare properties will not be ported in this change (list below).

| Our component | PrimeNG sources |
|---|---|
| `prime-datepicker::DatePicker`, `DatePickerPt`, `DatePickerView`, `IconDisplay`, `MonthYear` | `packages/primeng/src/datepicker/datepicker.ts`, `datepicker/style/datepickerstyle.ts` |
| Calendar, constraints, selection, formatting and parsing | `prime-datepicker::calendar` - a pure port of `createMonth`, `isSelectable`, `selectDate`, `getWeekNumber`, `formatDate`, `parseDate`, table-driven tests; own `CalDate` dates without external libraries |
| CSS | `@primeuix/styles/datepicker` (`datepicker` in `theme.components`) |
| Demos | `apps/showcase/doc/datepicker/*-doc.ts` (`basic`, `format`, `inline`, `multiple`, `range`, `minmax`, `buttonbar`, `month`, `year`, `icon`, `weeks`, `multiplemonths`, `disabled`, `invalid`) |

Spec: `openspec/specs/datepicker`. The value is `RwSignal<Vec<CalDate>>`: empty, one date, several, or `[start, end]` (a range without an end is `[start]`); the mode is `selection_mode`.

```rust
let value = RwSignal::new(Vec::new());
view! { <DatePicker value=value selection_mode=SelectionMode::Range date_format="dd.mm.yy" show_button_bar=true /> }
```

## What matches PrimeNG

- Markup and classes: field `p-datepicker-input` (InputText classes), `p-datepicker-dropdown`, `p-datepicker-input-icon-container`, `p-datepicker-clear-icon`, panel `p-datepicker-panel` (`-inline`), `calendar-container`, `calendar`, `header`, `title`, `select-month`, `select-year`, `decade`, `day-view`, `weekday-cell`, `weeknumber`, `day-cell` (`other-month`, `today`), `day` (`selected`, `selected-range`), `month-view`, `year-view`, `buttonbar`.
- Formulas: month grid and first day of the week, week numbers (ISO or from the first day of the year), decade, month and year availability, selecting one date, several (with `max_date_count`) and a range.
- Format `d dd o oo D DD m mm M MM y yy 'literal'` for output and input parsing; two-digit year parsing with the threshold "current year % 100 + 10".
- The panel opens on focus/click/button, closes after a completed selection (after 150 ms), on outside click, `Escape`, `Tab`, `resize`; "Today" and "Clear" buttons; "days/months/years" views and selecting a whole month or year (`view`).

## Differences

| Difference | Reason |
|---|---|
| The value is `Vec<CalDate>` (a date without time or time zone), not `Date`/`Date[]`/`null` | Typed model; time is stage 2. |
| The panel is rendered inside the root (`align_overlay(Relative)`), no `appendTo`, `motionOptions`, `touchUI` | Same approach as Select and ColorPicker. |
| Month and day names are English, overridable via `Locale` (`monthNames`, `monthNamesShort`, `dayNames`, `dayNamesShort`, `dayNamesMin` - comma-separated lists) | In `Locale` values are strings. |
| No `@` and `!` formats (timestamps) | They need time and time zone. |
| "Today" can be set with the `today` property | Deterministic tests and demos. |
| Day unavailability by `min`/`max` is computed by date, not by time within the day | There is no time (stage 1). |

## Not included in stage 1

- Time: `show_time`, `time_only`, `hour_format`, `show_seconds`, steps, AM/PM selection.
- Keyboard navigation across cells (`ArrowKeys`, `Home`/`End`, `PageUp`/`PageDown`) and the panel focus trap; from the keyboard the field opens the panel (`ArrowDown`) and `Escape`/`Tab`/`Enter` close it.
- `keep_invalid`, `data_type`, `maskable`, `responsive_options`, templates (`date`, `disabled-date`, `header`, `footer`, `decade`...), `float_label` inside the component (the `FloatLabel` wrapper works), `selection_mode` in Angular forms.

## Not verified

- Touch on devices; screen reader support (`aria-*` checked against PrimeNG, screen reader behaviour not); right-to-left layout.

## Performance

See `benches/` (`datepicker-1`, `datepicker-1000`, `case-datepicker`) and `benches/*budgets.toml`.
