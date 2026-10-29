# Proposal

## Why

Select — выбор одного значения из списка в выпадающей панели; самый востребованный компонент форм, на котором держатся Dropdown-подобные виджеты. Это последний кандидат волны и первый компонент проекта с оверлеем: он задаёт образец панели (позиционирование, z-index, закрытие по клику снаружи и по Escape) для будущих Popover, Menu и других.

## What Changes

- Новый крейт `prime-select` с компонентом `Select`: список `options` (`SelectOption { label, value, disabled }`) или группы `groups` (`SelectGroup { label, items }`), двусторонний `value` (`RwSignal<Option<String>>`) или внутреннее состояние с `default_value`; двусторонний `open` (`RwSignal<bool>`) для программного управления панелью; `placeholder`, `show_clear`, `checkmark`, `filter` (поле поиска, `filter_placeholder`, `reset_filter_on_hide`, `autofocus_filter`), `empty_message`, `empty_filter_message`, `loading`, `focus_on_hover`, `select_on_focus`, `auto_option_focus`, `scroll_height` (200px), `disabled`, `readonly`, `invalid`, `required`, `variant`, `size`, `fluid`, `input_id`, `tabindex`, `aria_label`, `aria_labelledby`, `aria_filter_label`, `style`, `style_class`, `panel_style`, `panel_style_class`; значки `dropdown_icon`, `clear_icon`, `loading_icon` (`ViewFn`) и слоты `item_template`, `selected_item_template`, `group_template`; события `on_change`, `on_filter`, `on_focus`, `on_blur`, `on_click`, `on_show`, `on_hide`, `on_clear`; `pt`.
- Разметка как в PrimeNG 21.1.10 (`select.ts`, `selectstyle.ts`): корень `<div class="p-select p-component p-inputwrapper">`, метка `span.p-select-label[role=combobox]`, значок очистки, кнопка `div.p-select-dropdown`, панель `div.p-select-overlay` с полем поиска (`p-select-filter` в `p-iconfield`), списком `ul.p-select-list[role=listbox]` (`li.p-select-option`, `li.p-select-option-group`, `li.p-select-empty-message`) и скрытыми фокусируемыми границами.
- Панель позиционируется под полем (над ним, если места не хватает) через `prime-dom`, получает z-index группы оверлеев, закрывается по клику снаружи, `Escape`, `Tab` и выбору.
- Клавиатура как в PrimeNG: стрелки, `Home`/`End`, `PageUp`/`PageDown`, `Enter`/`Space`, `Escape`, `Tab`, `Alt+ArrowUp`, `Delete` (очистка), поиск по набору символов (500 мс), навигация и в поле поиска.
- Страница `/select` (`basic`, `checkmark`, `clearicon`, `disabled`, `filled`, `filter`, `fluid`, `group`, `invalid`, `loadingstate`, `sizes`, `template`), e2e, браузерные тесты, бенчмарки, бюджеты, `docs/select.md`, CSS темы `select` и `iconfield` (поле поиска использует классы `p-iconfield` и `p-inputicon`, которые входят в `iconfield`), блок `/* For PrimeNG */` (цвет подсказки) через `primeng_extra`.
- **Не входит** (по решению пользователя об объёме первого change): виртуальная прокрутка (`virtualScroll`, `virtualScrollItemSize`, `virtualScrollOptions`) и ленивая загрузка (`lazy`, `onLazyLoad`), `editable`, настройка `overlayOptions` и `appendTo` (панель всегда внутри компонента), анимация показа и скрытия панели, `filterBy`/`filterFields`/`filterMatchMode`/`filterLocale` (поиск — подстрока метки без учёта регистра), `filterTemplate` и прочие шаблоны (`header`, `footer`, `empty`, `loader`), `tooltip*`, `dataKey`, `optionLabel`/`optionValue`/`optionDisabled`/`optionGroupLabel`/`optionGroupChildren` (вместо них типизированные `SelectOption` и `SelectGroup`), `autofocus`; Angular Forms; демонстрации `customfilter`, `editable`, `virtualscroll`, `lazyvirtualscroll`, `floatlabel`, `iftalabel`, `reactiveforms`, `templatedrivenforms`.

## Capabilities

### New Capabilities
- `select`: компонент Select — значение и метка, панель с вариантами, выбор мышью и клавиатурой, поиск, группы, очистка, структура и доступность, `pt`, производительность.

### Modified Capabilities
<!-- Нет: в `prime-core` добавляется только публичная константа ключей перевода, если потребуется; поведение существующих компонентов не меняется. -->

## Impact

- Новый крейт `crates/prime-select` (`publish = false`); зависимости `prime-core`, `prime-dom`, `prime-icons` (`chevrondown`, `search`, `times`, `check`, `spinner`).
- Изменяются `Cargo.toml` workspace, `upstream.toml` (`select`, `iconfield` в `theme.components`, `select` в `primeng_extra`), `showcase`, `benches/*`, `e2e/tests`, `ci.yml` (браузерный job для `tests/web.rs`).
