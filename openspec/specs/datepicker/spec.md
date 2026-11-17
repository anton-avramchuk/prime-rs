# datepicker Specification

## Purpose
DatePicker - selecting a date, several dates and a range in a calendar.

## Requirements

### Requirement: Calendar grid
The month grid SHALL contain weeks of seven cells, starting with `first_day_of_week` (0 is Sunday); the first week is padded with days of the previous month, the last with days of the next; a cell is marked `other_month`, `today`, `selectable`. Days of adjacent months SHALL be unavailable unless `select_other_months` is enabled.

#### Scenario: November 2026
- **WHEN** `first_day_of_week = 0`
- **THEN** the first cell is November 1 (Sunday), there are five weeks in total, the last cell is December 5 with `other_month`

#### Scenario: Week starting on Monday
- **WHEN** `first_day_of_week = 1`
- **THEN** the first cell is October 26 with `other_month`

### Requirement: Constraints
A date SHALL be unavailable if it is earlier than `min_date`, later than `max_date`, is in `disabled_dates`, or its weekday is in `disabled_days`. A month and a year SHALL be unavailable if they contain no available dates.

#### Scenario: Bounds
- **WHEN** `min_date = 10`, `max_date = 20` of the month
- **THEN** 9 and 21 are unavailable, 10 and 20 are available

### Requirement: Selection
`single` mode SHALL replace the value with the selected date; `multiple` - add the date or remove it if already selected (at most `max_date_count`); `range` - the first selection sets the start, the second (not earlier than the start) sets the end, otherwise the selection starts over. Selected days SHALL get `p-datepicker-day-selected` (in a range - the edges `-selected`, the middle `-selected-range`). An unavailable day is not selected.

#### Scenario: Range
- **WHEN** the 5th and then the 9th are selected
- **THEN** the value is `[5, 9]`, days 5 and 9 are `p-datepicker-day-selected`, 6-8 are `p-datepicker-day-selected-range`

#### Scenario: Deselection
- **WHEN** mode `multiple`, repeated click on a selected day
- **THEN** the date is removed from the value

### Requirement: Format and input
The field value SHALL be formatted by `date_format` (default `mm/dd/yy`; `multiple` joined with ", ", `range` as "start - end"). Typed text SHALL be parsed by the same format; valid available dates update the value and the calendar month; on blur the text reverts to the formatted value.

#### Scenario: Format
- **WHEN** the date is November 5, 2026, format `dd.mm.yy`
- **THEN** the field shows `05.11.2026`

#### Scenario: Input
- **WHEN** the user types `11/05/2026`
- **THEN** the value is November 5, 2026

### Requirement: Panel and navigation
The panel SHALL open on focus/click (`show_on_focus`) and by the button, close on outside click, `Escape`, `Tab` and after the selection is complete (`hide_on_date_time_select`); with `inline` it is always present. The "previous" and "next" buttons SHALL page the month ("days" view), year ("months" view), decade ("years" view); the header SHALL switch views; choosing a month and a year returns to the finer view or selects the date if `view` is set to `month` or `year`. The "Today" and "Clear" buttons SHALL select today's date and clear the value.

#### Scenario: Paging
- **WHEN** November 2026 is open, click "next"
- **THEN** December 2026 is shown, `on_month_change` is called

#### Scenario: Year
- **WHEN** click on the year title, then on 2030
- **THEN** the months of 2030 are shown
