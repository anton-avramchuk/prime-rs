# Proposal

## Why

DatePicker — выбор даты: поле ввода с календарём во всплывающей панели либо встроенный календарь. Последний компонент волны 6. По решению пользователя делается **этап 1**: выбор одной даты, нескольких и диапазона, месяцы и годы, ограничения и недоступные даты, оверлей и встроенный вид. Время и остальное — следующими этапами.

## What Changes

- Новый крейт `prime-datepicker` (порт `p-datepicker` из PrimeNG 21.1.10, этап 1).
- Чистый модуль `calendar.rs` без внешних библиотек дат: `CalDate` (год, месяц 0–11, день), день недели, сетка месяца (`create_month`, `first_day_index`, номера недель), ограничения (`Limits`: `min`, `max`, `disabled_dates`, `disabled_days`, `select_other_months`), выбор (`select_date`, `is_selected`, диапазон), десятилетие, `format_date` / `parse_date` (формат jQuery UI: `d dd o oo D DD m mm M MM y yy`, `'литерал'`); табличные тесты.
- `DatePicker`: корень `div.p-datepicker.p-component.p-inputwrapper` (`p-datepicker-fluid`, `p-invalid`, `p-variant-filled`, `p-focus`, `p-inputwrapper-filled`), поле `input.p-datepicker-input` (классы InputText), значок очистки, кнопка или значок календаря, панель `div.p-datepicker-panel` (`-inline`, `p-disabled`) > `.p-datepicker-calendar-container` > `.p-datepicker-calendar` (шапка с кнопками и заголовком, таблица `.p-datepicker-day-view`), `.p-datepicker-month-view`, `.p-datepicker-year-view`, `.p-datepicker-buttonbar`.
- Значение — `RwSignal<Vec<CalDate>>` (одна дата — 0 или 1 элемент; несколько — n; диапазон — `[начало]` или `[начало, конец]`) и `selection_mode`.
- Свойства: `selection_mode`, `date_format`, `min_date`, `max_date`, `disabled_dates`, `disabled_days`, `first_day_of_week`, `show_other_months`, `select_other_months`, `show_week`, `number_of_months`, `view` (`date` | `month` | `year`), `inline`, `show_icon`, `icon_display`, `show_button_bar`, `show_clear`, `show_on_focus`, `hide_on_date_time_select`, `max_date_count`, `default_date`, `placeholder`, `readonly_input`, `disabled`, `invalid`, `variant`, `size`, `fluid`, `input_id`, `tabindex`, `aria_label`, `style`, `style_class`, `multiple_separator`, `range_separator`, `today` (для тестов); события `on_select`, `on_clear`, `on_show`, `on_hide`, `on_month_change`, `on_year_change`, `on_today_click`, `on_clear_click`, `on_focus`, `on_blur`.
- Поведение: ввод текста разбирается по формату; клик по дню выбирает, повторный клик в `multiple` снимает; панель закрывается после завершения выбора (через 150 мс, как в PrimeNG), по клику снаружи, `Escape`, `Tab`; кнопки «Сегодня» и «Очистить»; переключение дней/месяцев/лет по заголовку, стрелки листают месяц/год/десятилетие.
- Страница `/datepicker` (`basic`, `format`, `inline`, `multiple`, `range`, `minmax`, `buttonbar`, `view`, `disabled`, `invalid`, `weeks`), e2e, бенчмарки, `docs/datepicker.md`, `datepicker` в `theme.components`.
- **Не входит (этап 2+):** время (`show_time`, `time_only`, `hour_format`, `step_*`), клавиатурная навигация по ячейкам (стрелки/`Home`/`End`/`PageUp`/`PageDown` внутри календаря), `touch_ui`, `data_type`, `keep_invalid`, `selectionMode` в формах Angular, шаблоны Angular, `motionOptions`, `responsiveOptions`, `appendTo`, `maskable`, названия месяцев и дней на других языках (английские + замена через `Names`).

## Capabilities

### New Capabilities
- `datepicker`: выбор даты, нескольких дат и диапазона в календаре.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-datepicker` (`publish = false`); зависимости: `prime-core`, `prime-dom`, `prime-button`, `prime-inputtext`, `prime-icons`, `js-sys`.
- Изменяются `Cargo.toml` workspace, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml`.
