# MultiSelect

Порт `p-multiselect` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-multiselect::MultiSelect` | `packages/primeng/src/multiselect/multiselect.ts` (`MultiSelect` и `MultiSelectItem`), `multiselect/style/multiselectstyle.ts` |
| CSS | `@primeuix/styles/multiselect`, `iconfield`, `chip`, `checkbox` и пресет Aura — `cargo xtask gen-theme`; блок `/* For PrimeNG */` содержит только селекторы `ng-invalid` Angular и не берётся |
| Демонстрации | `apps/showcase/doc/multiselect/*-doc.ts` (`basic`, `chips`, `clearicon`, `disabled`, `filled`, `filter`, `fluid`, `group`, `invalid`, `loadingstate`, `sizes`, `template`, `floatlabel`, `iftalabel`) |

Спека: `openspec/specs/multiselect`.

## Что совпадает с PrimeNG

- Структура: корень `div.p-multiselect`, скрытое поле `input[role=combobox]` в `p-hidden-accessible`, `p-multiselect-label-container` > `p-multiselect-label`, значок очистки `p-multiselect-clear-icon`, `p-multiselect-dropdown` со стрелкой или значком загрузки, панель `div.p-multiselect-overlay` (id `{id}_list`) со скрытыми границами `p-hidden-focusable`, шапкой `p-multiselect-header` (флажок «выбрать все» и `p-iconfield.p-multiselect-filter-container` с полем `p-multiselect-filter`), контейнером `p-multiselect-list-container`, списком `ul.p-multiselect-list[role=listbox][aria-multiselectable]`, вариантами `li.p-multiselect-option` с флажком, заголовками групп `p-multiselect-option-group` и `li.p-multiselect-empty-message`.
- Классы корня и их порядок, `data-p` корня и метки (`placeholder`, `clearable`, `disabled`, размер, `has-chip`, `empty`), `p-multiselect-label-empty` с текстом `empty` (скрыт стилем), `p-multiselect-option-selected` (при `highlight_on_select`), `p-focus`, `data-p-focused`/`-highlight`/`-selected`/`-disabled`, `aria-selected`/`aria-checked`.
- Метка: значения через «, », при превышении `max_selected_labels` (по умолчанию 3) — сводка из `selected_items_label` или перевода `selectionMessage` с подстановкой `{0}`; режим `chip` — чипы `Chip` в `p-multiselect-chip-item`. Как в PrimeNG, в режиме `chip` сводка показывается уже при `выбрано >= max_selected_labels` (PrimeNG сравнивает длину усечённого списка с пределом).
- Выбор не закрывает панель; предел `selection_limit` делает невыбранные варианты недоступными (выбранные можно снять) и убирает флажок «выбрать все»; «выбрать все» собирает значения по формуле `onToggleAll` (недоступные выбранные остаются, при включённом поле поиска сохраняется выбор вне результата поиска) и сообщает `on_select_all_change` только если итог пуст или содержит все варианты.
- Клавиши: ↓ ↑ Home End PageUp PageDown Enter Space Escape Tab Shift (диапазоны, `Ctrl+Shift+Home/End`), `Ctrl+A`, `Alt+↑` (выбрать и закрыть), поиск набором (500 мс); клавиши поля поиска — ↓ ↑ ← → Home End Enter.
- Закрытие по повторному клику, клику вне, `resize`, `Escape`, `Tab`; фокус остаётся на скрытом поле (нажатие мыши внутри панели не уводит его, кроме поля поиска).

## Отличия

| Отличие | Причина |
|---|---|
| Корень — `<div>`, а не хост `<p-multiselect>` | Хост в Angular — деталь фреймворка. |
| `values: RwSignal<Vec<String>>` вместо `ngModel`; без него внутреннее состояние (`default_values`); открытость панели — двусторонний `open` | Двусторонняя привязка без форм-инфраструктуры. |
| Варианты — `MultiSelectOption { label, value, disabled }` и `MultiSelectGroup { label, items }`, значения — строки; нет `optionLabel`, `optionValue`, `optionDisabled`, `optionGroup*`, `dataKey`, `filterBy`, `filterFields`, `filterMatchMode`, `filterLocale` | Как у Select и Listbox. |
| Панель всегда внутри компонента, без `appendTo` и `overlayOptions`, без анимации появления | Как у Select: нет общего Overlay; предок с `overflow` может обрезать панель. |
| Нет виртуальной прокрутки и ленивой загрузки, `tooltip*`, `autofocus`, внешнего `selectAll`, шаблонов `header`, `footer`, `filter`, `empty`, `loader`, `headercheckboxicon` и значков шаблонов (`filtericon`, `itemcheckboxicon`) | Вне объёма change. |
| `End` переносит фокус на последний допустимый вариант | PrimeNG использует `findLastFocusedOptionIndex`, то есть при наличии выбора переходит к первому выбранному, а не к концу списка — похоже на ошибку. `↑` без фокуса идёт на последний выбранный (или последний допустимый). |
| `PageUp`/`PageDown` прокручивают контейнер на высоту видимой области | В PrimeNG — прокрутка к первому/последнему варианту. |
| `Tab` на открытой панели с полем поиска не перехватывается — фокус уходит в поле поиска естественным порядком, закрытие по `Tab` из поля идёт через последнюю скрытую границу | Упрощение цепочки `hasFocusableElements` PrimeNG; результат тот же. Без поля поиска `Tab` выбирает фокусный вариант (если он не выбран) и закрывает панель. |
| `autofocus_filter` по умолчанию выключен | Как `autofocusFilter = false` в PrimeNG. |
| Удаление чипа: клик по значку вызывает `on_remove` и `on_change`, панель не открывается | `Chip` скрывается сам; список значков перестраивается из `values`. |
| Свойство `fluid` — `OptProp<bool>`: без значения берётся из контекста `Fluid` | См. `docs/field-wrappers.md`. |
| Идентификатор `pn_id_<n>` — из `prime_core::ids::next_unique`, либо свойство `id` | Одинаков на сервере и в браузере. |
| Нет демонстраций `reactiveforms`, `templatedrivenforms`, `virtualscroll`; в `template` вместо флагов с CDN коды стран и без шапки и подвала | Объём change и независимость от CDN. |

## Что не проверялось

- Касание и чтение с экрана: атрибуты `aria-*` сверены с PrimeNG, поведение скринридеров и сенсорного ввода — нет.
- Положение панели при узком окне и при переполнении родителя с `overflow: hidden`.

## Производительность

См. `benches/` (`multiselect-1`, `multiselect-1000`, `case-multiselect`) и `benches/*budgets.toml`. Логика списка пока дублирует `prime-listbox`; общий модуль выделим при третьем потребителе.
