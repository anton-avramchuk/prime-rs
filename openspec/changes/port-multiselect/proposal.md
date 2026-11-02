# Proposal

## Why

MultiSelect — одиннадцатый компонент волны 3: выбор нескольких значений из списка в выпадающей панели. Он объединяет два уже готовых компонента: панель, позиционирование и поле метки берутся из Select, а множественный выбор, диапазоны, флажки и «выбрать все» — из Listbox. Это последний крупный «выбирающий» компонент перед Paginator и Stepper.

## What Changes

- Новый крейт `prime-multiselect` (порт `p-multiselect` из PrimeNG 21.1.10).
- `MultiSelectOption` (`label`, `value`, `disabled`) и `MultiSelectGroup` (`label`, `items`); значения — строки, как у Select и Listbox.
- `MultiSelect`: корень `div.p-multiselect.p-component.p-inputwrapper` > скрытое поле `combobox` (`p-hidden-accessible`) > `div.p-multiselect-label-container` > `div.p-multiselect-label` (режим `comma` — текст через запятую, режим `chip` — `div.p-multiselect-chip-item` с `Chip`), значок очистки, `div.p-multiselect-dropdown` со стрелкой или значком загрузки, панель `div.p-multiselect-overlay` (скрытые границы фокуса, шапка `p-multiselect-header` с флажком «выбрать все» и полем поиска в `p-multiselect-filter-container`, контейнер `p-multiselect-list-container`, список `ul.p-multiselect-list[role=listbox]` с `li.p-multiselect-option` (флажок внутри), заголовками групп `p-multiselect-option-group` и сообщением `p-multiselect-empty-message`).
- Свойства: `options`, `groups`, `values` (двусторонний `RwSignal<Vec<String>>`) / `default_values`, `open` (двусторонний), `display` (`MultiSelectDisplay::Comma`/`Chip`), `max_selected_labels` (3), `selected_items_label` (`{0} items selected` из перевода), `display_selected_label` (да), `selection_limit`, `show_toggle_all` (да), `show_header` (да), `filter` (да), `filter_placeholder`, `reset_filter_on_hide`, `autofocus_filter`, `placeholder`, `show_clear`, `loading`, `highlight_on_select` (да), `focus_on_hover` (да), `select_on_focus`, `auto_option_focus` (нет), `scroll_height` (`200px`), `disabled`, `readonly`, `invalid`, `variant`, `size`, `fluid`, `input_id`, `tabindex`, `aria_label`, `aria_labelledby`, `aria_filter_label`, `empty_message`, `empty_filter_message`, `style`, `style_class`, `panel_style`, `panel_style_class`, `dropdown_icon`, `clear_icon`, `chip_icon`, `loading_icon`, `pt`; представления `item`, `group`, `selected_items`; события `on_change`, `on_filter`, `on_focus`, `on_blur`, `on_click`, `on_clear`, `on_show`, `on_hide`, `on_remove`, `on_select_all_change`.
- Поведение: клик по варианту добавляет/снимает значение, панель остаётся открытой; `selection_limit` блокирует невыбранные варианты при достижении предела (и скрывает «выбрать все»); «выбрать все» учитывает поиск и сохраняет выбранные недоступные варианты; клавиши ↓ ↑ (с `Alt+↑` — выбрать и закрыть), Home End PageUp PageDown Enter Space Escape Tab, `Shift`-диапазоны, `Ctrl+Shift+Home/End`, `Ctrl+A`, поиск набором; закрытие по клику вне и `resize`; z-index группы оверлеев; чип удаляется кликом по значку (`on_remove`).
- Страница showcase `/multiselect` с демонстрациями `basic`, `chips`, `clearicon`, `disabled`, `filled`, `filter`, `fluid`, `group`, `invalid`, `loadingstate`, `sizes`, `template`, `floatlabel`, `iftalabel`; e2e; бенчмарки и бюджеты; `docs/multiselect.md`.
- Стили: `multiselect` добавляется в `theme.components`.
- **Не входит:** виртуальная прокрутка и ленивая загрузка (`virtualScroll*`, `lazy`, `onLazyLoad`), `appendTo` и `overlayOptions` (панель всегда внутри корня, как в Select), `@primeuix/motion`, формы Angular, `optionLabel`/`optionValue`/`optionDisabled`/`optionGroup*`/`dataKey`/`filterBy`/`filterFields`/`filterMatchMode`/`filterLocale`, `tooltip*`, `autofocus`, внешнее `selectAll`, шаблоны `header`, `footer`, `filter`, `empty`, `loader`, `headercheckboxicon`, `Bind`, `pTemplate`.

## Capabilities

### New Capabilities
- `multiselect`: выбор нескольких значений из списка в выпадающей панели с поиском, группами, чипами, пределом выбора и навигацией клавишами.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-multiselect` (`publish = false`); зависимости: `prime-core`, `prime-dom`, `prime-icons`, `prime-checkbox` (классы флажка), `prime-chip`, `web-sys`.
- Изменяются `Cargo.toml` workspace, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (браузерный job, три места).
- Логика списка (плоский список, фильтр, индексы, выбор, диапазоны) в первой версии копируется из `prime-listbox` в `logic.rs`; вынос в общий модуль откладывается до третьего потребителя (по образцу Drawer/Dialog).
