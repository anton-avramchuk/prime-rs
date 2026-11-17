# Proposal

## Why

DatePicker - date selection: an input with a calendar in a popup panel, or an inline calendar. The last component of wave 6. By the user's decision **stage 1** is implemented: selecting one date, several dates and a range, months and years, constraints and disabled dates, overlay and inline view. Time and the rest come in later stages.

## What Changes

- New crate `prime-datepicker` (port of `p-datepicker` from PrimeNG 21.1.10, stage 1).
- Pure module `calendar.rs` without external date libraries: `CalDate` (year, month 0-11, day), weekday, month grid (`create_month`, `first_day_index`, week numbers), constraints (`Limits`: `min`, `max`, `disabled_dates`, `disabled_days`, `select_other_months`), selection (`select_date`, `is_selected`, range), decade, `format_date` / `parse_date` (jQuery UI format: `d dd o oo D DD m mm M MM y yy`, `'literal'`); table-driven tests.
- `DatePicker`: root `div.p-datepicker.p-component.p-inputwrapper` (`p-datepicker-fluid`, `p-invalid`, `p-variant-filled`, `p-focus`, `p-inputwrapper-filled`), field `input.p-datepicker-input` (InputText classes), clear icon, calendar button or icon, panel `div.p-datepicker-panel` (`-inline`, `p-disabled`) > `.p-datepicker-calendar-container` > `.p-datepicker-calendar` (header with buttons and title, `.p-datepicker-day-view` table), `.p-datepicker-month-view`, `.p-datepicker-year-view`, `.p-datepicker-buttonbar`.
- The value is `RwSignal<Vec<CalDate>>` (one date is 0 or 1 elements; several is n; a range is `[start]` or `[start, end]`) and `selection_mode`.
- Properties: `selection_mode`, `date_format`, `min_date`, `max_date`, `disabled_dates`, `disabled_days`, `first_day_of_week`, `show_other_months`, `select_other_months`, `show_week`, `number_of_months`, `view` (`date` | `month` | `year`), `inline`, `show_icon`, `icon_display`, `show_button_bar`, `show_clear`, `show_on_focus`, `hide_on_date_time_select`, `max_date_count`, `default_date`, `placeholder`, `readonly_input`, `disabled`, `invalid`, `variant`, `size`, `fluid`, `input_id`, `tabindex`, `aria_label`, `style`, `style_class`, `multiple_separator`, `range_separator`, `today` (for tests); events `on_select`, `on_clear`, `on_show`, `on_hide`, `on_month_change`, `on_year_change`, `on_today_click`, `on_clear_click`, `on_focus`, `on_blur`.
- Behaviour: typed text is parsed by the format; a click on a day selects it, a repeated click in `multiple` deselects; the panel closes after the selection is complete (after 150 ms, as in PrimeNG), on outside click, `Escape`, `Tab`; "Today" and "Clear" buttons; switching days/months/years via the header, arrows page month/year/decade.
- Page `/datepicker` (`basic`, `format`, `inline`, `multiple`, `range`, `minmax`, `buttonbar`, `view`, `disabled`, `invalid`, `weeks`), e2e, benchmarks, `docs/datepicker.md`, `datepicker` in `theme.components`.
- **Out of scope (stage 2+):** time (`show_time`, `time_only`, `hour_format`, `step_*`), keyboard navigation across cells (arrows/`Home`/`End`/`PageUp`/`PageDown` inside the calendar), `touch_ui`, `data_type`, `keep_invalid`, `selectionMode` in Angular forms, Angular templates, `motionOptions`, `responsiveOptions`, `appendTo`, `maskable`, month and day names in other languages (English + override via `Names`).

## Capabilities

### New Capabilities
- `datepicker`: selecting a date, several dates and a range in a calendar.

### Modified Capabilities
<!-- None. -->

## Impact

- New crate `crates/prime-datepicker` (`publish = false`); dependencies: `prime-rs-core`, `prime-dom`, `prime-button`, `prime-inputtext`, `prime-icons`, `js-sys`.
- Changed: workspace `Cargo.toml`, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml`.
