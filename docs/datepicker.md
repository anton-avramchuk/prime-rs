# DatePicker (этап 1)

Порт `p-datepicker` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`). Это **этап 1**: время, клавиатурная
навигация по ячейкам и ряд редких свойств перенесены не будут в этом изменении (список — ниже).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-datepicker::DatePicker`, `DatePickerPt`, `DatePickerView`, `IconDisplay`, `MonthYear` | `packages/primeng/src/datepicker/datepicker.ts`, `datepicker/style/datepickerstyle.ts` |
| Календарь, ограничения, выбор, формат и разбор | `prime-datepicker::calendar` — чистый порт `createMonth`, `isSelectable`, `selectDate`, `getWeekNumber`, `formatDate`, `parseDate`, табличные тесты; свои даты `CalDate` без внешних библиотек |
| CSS | `@primeuix/styles/datepicker` (`datepicker` в `theme.components`) |
| Демонстрации | `apps/showcase/doc/datepicker/*-doc.ts` (`basic`, `format`, `inline`, `multiple`, `range`, `minmax`, `buttonbar`, `month`, `year`, `icon`, `weeks`, `multiplemonths`, `disabled`, `invalid`) |

Спека: `openspec/specs/datepicker`. Значение — `RwSignal<Vec<CalDate>>`: пусто, одна дата, несколько или `[начало, конец]` (диапазон без конца — `[начало]`); режим — `selection_mode`.

```rust
let value = RwSignal::new(Vec::new());
view! { <DatePicker value=value selection_mode=SelectionMode::Range date_format="dd.mm.yy" show_button_bar=true /> }
```

## Что совпадает с PrimeNG

- Разметка и классы: поле `p-datepicker-input` (классы InputText), `p-datepicker-dropdown`, `p-datepicker-input-icon-container`, `p-datepicker-clear-icon`, панель `p-datepicker-panel` (`-inline`), `calendar-container`, `calendar`, `header`, `title`, `select-month`, `select-year`, `decade`, `day-view`, `weekday-cell`, `weeknumber`, `day-cell` (`other-month`, `today`), `day` (`selected`, `selected-range`), `month-view`, `year-view`, `buttonbar`.
- Формулы: сетка месяца и первый день недели, номера недель (ISO или от первого дня года), десятилетие, доступность месяцев и годов, выбор одной даты, нескольких (с `max_date_count`) и диапазона.
- Формат `d dd o oo D DD m mm M MM y yy 'литерал'` для вывода и разбора ввода; разбор двузначного года с порогом «текущий год % 100 + 10».
- Панель открывается по фокусу/клику/кнопке, закрывается после завершённого выбора (через 150 мс), по клику снаружи, `Escape`, `Tab`, `resize`; кнопки «Сегодня» и «Очистить»; виды «дни/месяцы/годы» и выбор месяца или года целиком (`view`).

## Отличия

| Отличие | Причина |
|---|---|
| Значение — `Vec<CalDate>` (дата без времени и часового пояса), а не `Date`/`Date[]`/`null` | Типизированная модель; время — этап 2. |
| Панель выводится внутри корня (`align_overlay(Relative)`), нет `appendTo`, `motionOptions`, `touchUI` | Тот же подход, что у Select и ColorPicker. |
| Названия месяцев и дней — английские, замена через `Locale` (`monthNames`, `monthNamesShort`, `dayNames`, `dayNamesShort`, `dayNamesMin` — списки через запятую) | В `Locale` значения — строки. |
| Нет формата `@` и `!` (метки времени) | Нужны время и часовой пояс. |
| «Сегодня» можно задать свойством `today` | Детерминированные тесты и демо. |
| Недоступность дня по `min`/`max` считается по дате, а не по времени внутри дня | Времени нет (этап 1). |

## Не вошло в этап 1

- Время: `show_time`, `time_only`, `hour_format`, `show_seconds`, шаги, выбор AM/PM.
- Клавиатурная навигация по ячейкам (`ArrowKeys`, `Home`/`End`, `PageUp`/`PageDown`) и фокус-ловушка панели; из клавиатуры поле открывает панель (`ArrowDown`), закрывает `Escape`/`Tab`/`Enter`.
- `keep_invalid`, `data_type`, `maskable`, `responsive_options`, шаблоны (`date`, `disabled-date`, `header`, `footer`, `decade`…), `float_label` внутри компонента (работает обёртка `FloatLabel`), `selection_mode` в формах Angular.

## Что не проверялось

- Касание на устройствах; чтение с экрана (`aria-*` сверены с PrimeNG, поведение скринридеров — нет); раскладка справа налево.

## Производительность

См. `benches/` (`datepicker-1`, `datepicker-1000`, `case-datepicker`) и `benches/*budgets.toml`.
