# Select

Порт `p-select` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-select::Select` | `packages/primeng/src/select/select.ts` (`Select` и `SelectItem`), `select/style/selectstyle.ts` |
| CSS | `@primeuix/styles/select` и `iconfield` (поле поиска), пресет Aura и блок `/* For PrimeNG */` (цвет подсказки) — `cargo xtask gen-theme`, `primeng_extra` |
| Демонстрации | `apps/showcase/doc/select/*-doc.ts` (`basic`, `checkmark`, `clearicon`, `disabled`, `filled`, `filter`, `fluid`, `group`, `invalid`, `loadingstate`, `sizes`, `template`, `floatlabel`, `iftalabel`) |

Спека: `openspec/specs/select`. Объём первого change: ядро, фильтр и группы.

## Что совпадает с PrimeNG

- Структура: корень `div.p-select`, метка `span.p-select-label[role=combobox]`, значок очистки `p-select-clear-icon`, кнопка `div.p-select-dropdown[role=button]` со значком, панель `div.p-select-overlay` со скрытыми границами `p-hidden-focusable`, шапкой `p-select-header` (поле поиска `p-select-filter` в `p-iconfield` со значком `p-inputicon`), списком `ul.p-select-list[role=listbox]` с `li.p-select-option`, `li.p-select-option-group` (и `span.p-select-option-group-label`), `li.p-select-empty-message`.
- Классы корня и их порядок, `data-p` корня и метки, `p-placeholder`, `p-select-label-empty`, `p-select-option-selected` (не при `checkmark`), `p-focus`, `data-p-focused`/`-highlight`/`-selected`/`-disabled` у вариантов.
- `aria-*`: `aria-haspopup`, `aria-expanded`, `aria-controls`, `aria-activedescendant`, `aria-label` метки (заданный или метка варианта), `aria-setsize`/`aria-posinset` без учёта заголовков групп, идентификаторы вариантов `{id}_{индекс}` по списку вместе с заголовками.
- Клавиатура: `onKeyDown` и `onFilterKeyDown` — стрелки, `Alt+ArrowUp`, `Home`/`End`, `PageUp`/`PageDown`, `Enter`/`Space`, `Escape`, `Tab`, `Delete`, поиск набором с накоплением символов на 500 мс; первая стрелка вниз открывает панель (фокус сразу на выбранном варианте, если он есть), следующие идут от него; без выбранного и фокуса стрелка вниз ведёт на первый вариант, стрелка вверх — на последний.
- Фокус при открытии — на выбранном варианте (`show()`), `auto_option_focus` — на первом; `focus_on_hover`, `select_on_focus`.
- Закрытие: повторный клик, клик снаружи, `Escape`, `Tab`, выбор; `reset_filter_on_hide`; `on_show`/`on_hide`/`on_change`/`on_filter`/`on_clear`/`on_click`.
- Сообщения пустого списка и пустого результата — из `empty_message`/`empty_filter_message` и перевода (`emptyMessage`, `emptyFilterMessage`), подпись списка — `aria.listLabel`.

## Отличия

| Отличие | Причина |
|---|---|
| Корень — `<div>`, а не хост `<p-select>` | Хост в Angular — деталь фреймворка. |
| `value: RwSignal<Option<String>>` вместо `ngModel`; без него внутреннее состояние (`default_value`); открытость панели — двусторонний `open` | Двусторонняя привязка без форм-инфраструктуры; `open` даёт программное управление и тесты открытой панели. |
| Варианты — `SelectOption { label, value, disabled }` и `SelectGroup { label, items }`, значения — строки; нет `optionLabel`, `optionValue`, `optionDisabled`, `optionGroupLabel`, `optionGroupChildren`, `dataKey` | Поля произвольного объекта заменены типизированными вариантами, как у SelectButton. |
| Панель всегда внутри компонента, без `appendTo` и `overlayOptions`; нет анимации появления | В проекте нет общего компонента Overlay: панель позиционируется через `prime-dom` (`align_overlay`) и получает z-index группы оверлеев; предок с `overflow` может её обрезать. |
| Поиск — подстрока метки без учёта регистра (пробелы по краям строки отбрасываются); нет `filterBy`, `filterFields`, `filterMatchMode`, `filterLocale`, `filterTemplate` | Объём первого change; PrimeNG по умолчанию делает то же самое. |
| Нет виртуальной прокрутки и ленивой загрузки, `editable`, шаблонов `header`, `footer`, `empty`, `loader`, `tooltip*`, `autofocus` | Вне объёма первого change по решению пользователя. |
| Закрытие по клику снаружи и `resize` слушает `window` только пока панель открыта | `ConnectedOverlayScrollHandler` и слушатели `p-overlay` PrimeNG заменены двумя подписками. |
| Последняя скрытая граница панели закрывает её и возвращает фокус на метку | В PrimeNG она отдаёт фокус обратно по сложной цепочке `getFirstFocusableElement`; на практике результат тот же. |
| Метка, подсказка и значок очистки читают значение напрямую из списка вариантов; значение, которого нет среди вариантов, считается невыбранным | В PrimeNG такое значение остаётся в модели и показывается пустой меткой. |
| Свойство `fluid` — `OptProp<bool>`: без значения берётся из контекста `Fluid` | Как `hasFluid = fluid() ?? !!pcFluid` в PrimeNG; см. `docs/field-wrappers.md`. |
| Идентификатор `pn_id_<n>` берётся из общего хелпера `prime_core::ids::next_unique` | Одинаков на сервере и в браузере. |
| Нет демонстраций `customfilter`, `editable`, `virtualscroll`, `lazyvirtualscroll`, `reactiveforms`, `templatedrivenforms`; в `template` вместо флагов с CDN коды стран | Объём change и независимость от CDN. |

Список перестраивается только при смене вариантов, групп или строки поиска; фокус варианта, выбор и состояние метки — отдельные замыкания, поэтому элементы метки и поля поиска не пересоздаются при навигации.

## Производительность

SSR: 230 аллокаций на экземпляр с закрытой панелью и пятью вариантами (бюджет 250); вклад в сырой wasm +630 КБ (бюджет 690 КБ) — клавиатура, поиск, позиционирование и список. Это самый тяжёлый компонент проекта.
