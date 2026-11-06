# Design

## Context

Требования — в `specs/datepicker`. Исходники PrimeNG 21.1.10: `datepicker/datepicker.ts` (`createMonth`, `isSelectable`, `selectDate`, `onDateSelect`, `formatDate`, `parseDate`, навигация), `style/datepickerstyle.ts`, `@primeuix/styles/datepicker`. Образцы: `prime-colorpicker` (оверлей внутри корня, закрытие снаружи), `prime-inputmask` (поле на классах InputText), `prime-select`.

## Goals / Non-Goals

**Goals:** тот же DOM, те же классы, те же формулы календаря и формат даты.

**Non-Goals (этап 2+):** время, клавиатурная навигация по ячейкам, `touch_ui`, `data_type`, `keep_invalid`, шаблоны, языки.

## Decisions

### D1. Календарная логика без библиотек
`CalDate` (месяц 0–11, как JS) и алгоритмы Говарда Хиннанта для перевода в номер дня; сетка, ограничения, выбор, формат и разбор — чистые функции в `calendar.rs` с табличными тестами. «Сегодня» передаётся параметром: `js_sys::Date` в браузере, `SystemTime` в нативных сборках, свойство `today` для тестов.

### D2. Состояние
`value: RwSignal<Vec<CalDate>>`, `view: RwSignal<View>` (текущий вид), `current_month`, `current_year`, `open: RwSignal<bool>`, текст поля — локальный сигнал, который перезаписывается при изменении значения и потере фокуса. Месяцы пересчитываются `Memo` из месяца/года/ограничений.

### D3. Оверлей
Панель внутри корня (`align_overlay(Relative)`, `ZGroup::Overlay`), клик снаружи и `resize` на `window` (подписки через `set_timeout(0)`), `Escape` и `Tab` в поле. Закрытие после выбора — `set_timeout` 150 мс.

### D4. Ввод
`input` разбирает текст по режиму (разделители), недоступные даты не принимаются; `blur` возвращает форматированный текст; ввод клавиш `Enter` закрывает панель.

### D5. Тесты
Календарь — таблицы. SSR: поле, значок, панель (inline), заголовок, сетка, выбранные дни, ограничения, `show_week`, режимы, месяц/год. Браузерные `tests/web.rs`: открытие/закрытие, выбор, диапазон, множественный, навигация, виды, ввод текста, кнопки панели, `disabled`. e2e: стили и реальные клики.

## Risks / Trade-offs

- [Объём] Клавиатурная навигация по ячейкам отложена — записано в docs.
- [Часовой пояс] Даты без времени и пояса — расхождений с `Date` по полуночи нет.

## Open Questions

- Нет.

## Migration Plan

Миграции нет. Откат — `git revert`.
