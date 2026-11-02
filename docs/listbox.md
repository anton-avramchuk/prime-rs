# Listbox

Порт `p-listbox` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-listbox::Listbox` | `packages/primeng/src/listbox/listbox.ts`, `listbox/style/listboxstyle.ts` |
| CSS | `@primeuix/styles/listbox`, пресет Aura и блок `/* For PrimeNG */` (шапка, поле поиска, высота контейнера) — `cargo xtask gen-theme`, `primeng_extra` |
| Демонстрации | `apps/showcase/doc/listbox/*-doc.ts` (`basic`, `multiple`, `group`, `filter`, `checkbox`, `checkmark`, `disabled`, `invalid`, `template`) |

Спека: `openspec/specs/listbox`.

## Что совпадает с PrimeNG

- Структура: корень `div.p-listbox`, шапка `p-listbox-header` (флажок «выбрать все», поле поиска в `p-iconfield` со значком `p-inputicon`), контейнер `p-listbox-list-container` с `max-height` (по умолчанию `14rem`), список `ul.p-listbox-list[role=listbox]`, варианты `li.p-listbox-option[role=option]`, заголовки групп `li.p-listbox-option-group`, сообщение пустого списка `div.p-listbox-empty-message`.
- Классы корня и их порядок (`striped`, `p-disabled`, `p-invalid`, `p-listbox-fluid`), `data-p` корня, `p-listbox-option-selected` (только при `highlight_on_select`), `p-focus`, `data-p-selected`/`-focused`/`-disabled` у вариантов.
- `aria-*`: `aria-multiselectable`, `aria-disabled`, `aria-activedescendant` (у списка — пока он в фокусе, у поля поиска — всегда, когда есть подсвеченный вариант), `aria-selected`, `aria-setsize`/`aria-posinset` без учёта заголовков групп, идентификаторы вариантов `{id}_{индекс}` по списку вместе с заголовками.
- Выбор: `onOptionSelectSingle`/`Multiple` с `meta_key_selection` (в одиночном режиме без `Ctrl` повторный клик оставляет выбор; в множественном без `Ctrl` клик заменяет выбор), мета-режим отключается после касания (`touchend`), `Shift`+стрелки и `Ctrl+Shift+Home/End` выбирают диапазон, `Ctrl+A` — все допустимые видимые варианты.
- Клавиши списка: ↓ ↑ Home End Enter NumpadEnter Space Shift, поиск набором (накопление 500 мс), `Tab` не перехватывается. Клавиши поля поиска: ↓ ↑ ← → Home End Enter Shift.
- Фокус: `auto_option_focus`, `focus_on_hover` (только пока список в фокусе), `select_on_focus`; блюр сбрасывает фокусный вариант, опору диапазона и накопленную строку поиска.
- Флажок «выбрать все» работает с видимыми (отфильтрованными) допустимыми вариантами; `Enter` на нём переключает, `Tab` уводит фокус в список.
- Флажки вариантов и «выбрать все» — та же разметка и классы, что у `Checkbox` (`p-checkbox`, `p-checkbox-input`, `p-checkbox-box`, `p-checkbox-icon`) с добавочным классом `p-listbox-option-check-icon`; сам флажок состояния не хранит — клик по нему отменяется и обрабатывается списком.

## Отличия

| Отличие | Причина |
|---|---|
| Корень — `<div>`, а не хост `<p-listbox>` | Хост в Angular — деталь фреймворка. |
| Значение — `value: RwSignal<Option<String>>` (одиночный режим) и `values: RwSignal<Vec<String>>` (`multiple`) вместо `ngModel`; без них внутреннее состояние (`default_value`, `default_values`) | Двусторонняя привязка без форм-инфраструктуры; два сигнала вместо одного значения произвольного типа. |
| Варианты — `ListboxOption { label, value, disabled }` и `ListboxGroup { label, items }`, значения — строки; нет `optionLabel`, `optionValue`, `optionDisabled`, `optionGroupLabel`, `optionGroupChildren`, `dataKey` | Поля произвольного объекта заменены типизированными вариантами, как у Select. |
| Поиск — подстрока метки без учёта регистра (пробелы по краям отбрасываются); нет `filterBy`, `filterFields`, `filterMatchMode`, `filterLocale`, `searchLocale` | PrimeNG по умолчанию делает то же самое. |
| Нет виртуальной прокрутки и ленивой загрузки, `dragdrop` и `dropListData`, скрытых границ фокуса `p-hidden-focusable`, `aria-live`-сообщений (`searchMessage`, `selectionMessage`, `emptyMessage` для чтения с экрана), шаблонов `header`, `footer`, `filter`, `empty`, `loader`, внешнего `selectAll` | Вне объёма change. |
| `PageUp`/`PageDown` прокручивают контейнер на высоту видимой области | В PrimeNG они вызывают `scrollInView(0)` — прокрутку к первому варианту, что на практике ведёт себя как `Home` без смены фокуса; у нас — обычная постраничная прокрутка. |
| Фокусный вариант при `mousedown` и наведении переносится без прокрутки и без `select_on_focus` | Выбор в этот момент всё равно произойдёт кликом; иначе при `select_on_focus` клик выбрал бы вариант дважды. |
| Подпись флажка «выбрать все» для чтения с экрана — `aria.selectAll`/`aria.unselectAll` перевода по состоянию | В PrimeNG `toggleAllAriaLabel` берётся так же, но из `config.translation`. |
| Свойство `fluid` — `OptProp<bool>`: без значения берётся из контекста `Fluid` | Как `hasFluid = fluid() ?? !!pcFluid` в PrimeNG; см. `docs/field-wrappers.md`. |
| Идентификатор `pn_id_<n>` берётся из общего хелпера `prime_core::ids::next_unique`, либо свойство `id` | Одинаков на сервере и в браузере. |
| Нет демонстраций `reactiveforms`, `templatedrivenforms`, `virtualscroll`, `dragdrop`; в `template` вместо флагов с CDN коды стран | Объём change и независимость от CDN. |

Список (`ul` и варианты) перестраивается только при смене вариантов, групп или строки поиска; выбор, фокус и подсветка — отдельные реактивные атрибуты вариантов, поэтому элементы не пересоздаются при навигации и клике.

## Что не проверялось

- Касание: мета-режим после `touchend` проверен только логикой, на настоящем сенсорном вводе — нет.
- Чтение с экрана: атрибуты `aria-*` сверены с PrimeNG, поведение скринридеров не проверялось.

## Производительность

См. `benches/` (`listbox-1`, `listbox-1000`, `case-listbox`) и `benches/*budgets.toml`.
