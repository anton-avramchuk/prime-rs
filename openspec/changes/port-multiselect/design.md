# Design

## Context

Требования — в `specs/multiselect`. Исходники PrimeNG 21.1.10: `multiselect/multiselect.ts`, `multiselect/style/multiselectstyle.ts`; демонстрации `apps/showcase/doc/multiselect/*-doc.ts`. Образцы в проекте:
- `prime-select` — корень, панель внутри корня, `align_overlay(Relative)`, z-index группы `Overlay`, слушатели `window` на время открытия, эффект открытия под `cfg(wasm32)`, значок очистки/загрузки, фокус поля поиска, поиск набором, `realign` после смены списка;
- `prime-listbox` — модель вариантов, `visible_entries`, индексы, `select_multiple`, `select_range`, `select_all`, флажок как разметка `Checkbox` с состоянием из замыканий, клавиши с диапазонами.

Наблюдения по исходнику:
- Фокус клавиатуры — на скрытом `input[role=combobox]` в `p-hidden-accessible`; он держит `aria-expanded`, `aria-controls`, `aria-activedescendant`.
- Метка: `label()` = метки выбранных через «, » либо `selectedItemsLabel` при превышении `maxSelectedLabels` (по умолчанию 3), иначе `placeholder`. В режиме `chip` — `chipSelectedItems()` (первые `maxSelectedLabels`) в `p-chip` с `removable`.
- `isOptionDisabled`: при `selectionLimit` и достигнутом пределе невыбранные варианты недоступны.
- `onToggleAll`: итог = выбранные до поиска (если поиск и не все выбраны) + выбранные недоступные + видимые (все выбранные/допустимые, либо только выбранные-недопустимые, если всё выбрано) — без повторов.
- Клавиши: как в Listbox, плюс открытие панели ↓ ↑ Home End и набором, `Alt+↑` — выбрать и закрыть, `Tab` — выбрать фокусный (если не выбран) и закрыть, `Escape` — закрыть.
- В отличие от Select, выбор не закрывает панель; фокус остаётся на скрытом поле, список не принимает фокус.
- `filter` по умолчанию `true`.

## Goals / Non-Goals

**Goals:** тот же DOM, классы и роли; поведение выбора, предела, «выбрать все», клавиш и закрытия; режим чипов; `values`/`open` как двусторонние сигналы.

**Non-Goals:** виртуальная прокрутка, `appendTo`/`overlayOptions`, `@primeuix/motion`, формы Angular, `optionLabel` и родственные, `tooltip*`, шаблоны `header`/`footer`/`filter`/`empty`/`loader`, внешнее `selectAll`.

## Decisions

### D1. Крейт и модель
`prime-multiselect` с `MultiSelect`, `MultiSelectOption`, `MultiSelectGroup`, `MultiSelectDisplay { Comma, Chip }`, `MultiSelectPt`. Типы совпадают по форме с `ListboxOption`/`ListboxGroup`, но свои — зависимость от `prime-listbox` ради структур тянула бы весь Listbox в бинарник.

### D2. Чистая логика (`logic.rs`)
Копия нужных функций Listbox (плоский список, фильтр, индексы, поиск набором, диапазоны, `select_all`) плюс специфичные:
- `option_disabled(option, selected, limit)` — учитывает предел выбора;
- `entries_with_limit(...)` или параметр `limit` в `is_valid` — недоступность невыбранных при пределе;
- `toggle_all(all_options, visible, selected, filter_active) -> Vec<String>` — формула PrimeNG без повторов;
- `all_selected(visible, selected)`;
- `label_text(selected_labels, max, summary_template, placeholder, display_selected_label)` и `chip_labels(selected_labels, max)`;
- `summary(template, n)` — подстановка `{0}`;
- `root_classes`, `label_classes`, `option_classes`, `data_p`.
Табличные тесты на каждую. Дублирование с Listbox намеренно оставлено до третьего потребителя.

### D3. Состояние и разметка
`values`, `open`, `filter_text`, `focused_index: Option<usize>`, `start_range`, `focused` (поле), `clicked` (панель открыта мышью). Корневые классы и `data-p` — реактивные замыкания; метка читает `values` и варианты; список — как в Select (`ul` стабилен, элементы перестраиваются от вариантов, групп и строки поиска); состояние варианта (выбран, фокус, недоступен по пределу) — реактивные атрибуты, чтобы клик и навигация не пересоздавали элементы и не теряли прокрутку.

### D4. Панель и окно
По образцу Select: эффект открытия под `cfg(wasm32)` (`opened`/`closed`), `align_overlay(Relative)`, z-index `ZGroup::Overlay`, слушатели `click` и `resize` окна на время открытия, `realign` после ввода в поиск и после смены числа чипов, `release` в `on_cleanup`. Клик внутри корня панель не закрывает. Скрытые границы фокуса — как в Select.

### D5. Чипы
`Chip` из `prime-chip` с `removable`; при `chip_icon` — свой значок удаления. Клик по удалению: `stop_propagation`, удаление значения, `on_remove`, `on_change`. Если у `Chip` нет нужных точек расширения — разметка чипа собирается вручную с теми же классами (`p-chip`, `p-chip-label`, `p-chip-remove-icon`), решение фиксируется в docs.

### D6. Выбор и предел
`select_option` (клик/Enter): `option.disabled` или предел → ничего; иначе переключение значения в `values`, `on_change`. Предел — `Prop<Option<usize>>`; недоступность невыбранных при `values.len() >= limit`. Все выборы идут через `update_model(values, event)`.

### D7. Клавиши
Два обработчика (`key_down(e, in_filter)`) — скрытое поле и поле поиска; таблица `match code`. Открытие панели из любой навигационной клавиши как в PrimeNG. Диапазоны и `Ctrl+A` берутся из логики. `PageUp`/`PageDown` — постраничная прокрутка контейнера, как в Listbox.

### D8. Представления
`item: Option<Callback<(MultiSelectOption, bool), AnyView>>`, `group: Option<Callback<MultiSelectGroup, AnyView>>`, `selected_items: Option<Callback<Vec<MultiSelectOption>, AnyView>>`; значки — `Option<ViewFn>`.

### D9. CSS
`multiselect` в `theme.components` (блок `/* For PrimeNG */` содержит только селекторы `ng-invalid` Angular и не нужен, `primeng_extra` не используется); `iconfield` и `chip` уже подключены. Проверить `p-multiselect-*` в пресете.

### D10. Тесты
SSR: разметка корня и метки (режимы `comma`/`chip`, сводка, подсказка), классы корня по состояниям, значки очистки/загрузки, `data-p`, `pt`, `style_class`, отсутствие панели. Браузерные `tests/web.rs` (слушатели окна общие → один последовательный прогон для тестов с открытой панелью): открытие/закрытие, выбор не закрывает, предел, «выбрать все» с поиском, чипы и удаление, очистка, клавиши (в т. ч. диапазоны, `Ctrl+A`, `Alt+↑`, `Tab`, `Escape`), поиск набором, фильтр, внешний `open`/`values`, z-index и положение, блокировки, отписка. e2e: настоящая мышь и клавиши, чипы, положение панели. Крейт добавляется в три места `ci.yml`.

### D11. Бенчмарки
Сценарии `multiselect-1`/`-1000` (пять вариантов, закрытая панель, два выбранных) и `case-multiselect`; сначала `cargo bench -p render-bench --bench alloc`; для wasm — отдельный файл бюджетов `--budgets` только с этим случаем, затем бюджет в `benches/budgets.toml`.

## Risks / Trade-offs

- [Размер wasm] Select + Listbox + Chip в одном компоненте — бюджет подбирается по замеру.
- [Фокус] Перестройка списка при выборе роняла бы прокрутку → только реактивные атрибуты у вариантов.
- [Общие слушатели окна] Несколько MultiSelect в одном тесте → один последовательный прогон.
- [Дублирование] Логика списка частично повторяет Listbox → вынос в общий модуль при третьем потребителе.
- [Чип] Если `Chip` не позволяет подменить значок удаления — разметка вручную.

## Open Questions

- Нужна ли анимация показа: проверить, есть ли в теме `@keyframes` для `p-anchored-overlay`; если нет — без анимации (как в Select).

## Migration Plan

Миграции нет. Откат — `git revert`.
