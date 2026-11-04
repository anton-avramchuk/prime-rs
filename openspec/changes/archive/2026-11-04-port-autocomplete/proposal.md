# Proposal

## Why

AutoComplete — поле ввода с подсказками: пользователь печатает, компонент запрашивает варианты (`on_complete`) и показывает их в панели. Нужен режим множественного выбора с чипами, кнопка-стрелка, очистка, группы и `force_selection`. Оболочка и панель — как у `Select`/`TreeSelect`, поле — на классах `InputText`, чипы — `Chip`.

## What Changes

- Новый крейт `prime-autocomplete` (порт `p-autocomplete` из PrimeNG 21.1.10).
- Модель: `AutoCompleteOption` — `label`, `value` (по умолчанию `label`), `disabled`, `items` (вложенные варианты — группа при `group = true`). Значение в одиночном режиме — `RwSignal<String>` (набранный текст или значение выбранного варианта), в множественном — `RwSignal<Vec<String>>`.
- `AutoComplete`: корень `div.p-autocomplete.p-component.p-inputwrapper` (`p-invalid`, `p-focus`, `p-inputwrapper-filled|focus`, `p-autocomplete-open|clearable|fluid`) > одиночный режим: `input.p-autocomplete-input.p-inputtext[role=combobox][aria-autocomplete=list]`; множественный: `ul.p-autocomplete-input-multiple[role=listbox]` > `li.p-autocomplete-chip-item` (`Chip.p-autocomplete-chip` с кнопкой удаления) и `li.p-autocomplete-input-chip` > `input.p-autocomplete-input`; значок очистки `p-autocomplete-clear-icon`, спиннер `p-autocomplete-loader`, кнопка `button.p-autocomplete-dropdown`; панель `div.p-autocomplete-overlay` > `div.p-autocomplete-list-container` (`max-height` = `scroll_height`, `200px`) > `ul.p-autocomplete-list[role=listbox]` > `li.p-autocomplete-option[role=option]` (`-selected`, `p-focus`, `p-disabled`; `aria-label|selected|disabled|setsize|posinset`, `data-p-selected|focused`), `li.p-autocomplete-option-group`, `li.p-autocomplete-empty-message`.
- Поведение: ввод от `min_length` символов через `delay` (300 мс) вызывает `on_complete` с запросом и включает спиннер до обновления `suggestions`; новые варианты открывают панель (при пустом списке — сообщение, если `show_empty_message`); выбор варианта записывает значение (в `multiple` добавляет чип, `unique`) и закрывает панель; кнопка-стрелка запрашивает варианты (`dropdown_mode`: пустой запрос либо текущий текст); `complete_on_focus`; `force_selection` очищает несовпавший ввод; очистка; клик вне закрывает; `typeahead = false` в `multiple` добавляет набранное по `Enter`, разделителю, `Tab` (`add_on_tab`), потере фокуса (`add_on_blur`) и вставке (`separator`).
- Клавиатура: ↓ ↑ по вариантам (`Alt+↑` выбирает и закрывает), `Enter` выбирает, `Escape` закрывает, `Tab` выбирает сфокусированный и закрывает, `Home`/`End` — каретка, `PageDown`/`PageUp` — прокрутка, `Backspace` на пустом поле `multiple` удаляет последний чип; ← на пустом поле `multiple` переводит на чипы (← → по ним, `Backspace` удаляет).
- Свойства: `suggestions`, `value`/`values`, `multiple`, `dropdown`, `dropdown_mode`, `show_clear`, `force_selection`, `complete_on_focus`, `auto_option_focus`, `focus_on_hover`, `min_length`, `delay`, `placeholder`, `scroll_height`, `typeahead`, `unique`, `group`, `show_empty_message`, `empty_message`, `add_on_tab`, `add_on_blur`, `separator`, `disabled`, `readonly`, `invalid`, `variant`, `size`, `fluid`, `input_type`, `input_id`, `tabindex`, `aria_label`, `aria_labelledby`, `id`, `style`, `style_class`, `input_style_class`, `panel_style_class`; события `on_complete`, `on_select`, `on_unselect`, `on_add`, `on_clear`, `on_dropdown_click`, `on_show`, `on_hide`, `on_focus`, `on_blur`; представления `item`, `group_view`, `selected_item`.
- Страница `/autocomplete` (`basic`, `dropdown`, `multiple`, `forceselection`, `group`, `clearicon`, `disabled`, `invalid`, `template`), e2e, бенчмарки и бюджеты, `docs/autocomplete.md`, `autocomplete` в `theme.components` и `primeng_extra`.
- **Не входит:** объекты как варианты (`optionLabel`/`optionValue`/`dataKey`), виртуальная прокрутка и `lazy`, формы Angular, `appendTo`, `Ripple`, `Bind`, `pTemplate`, шаблоны `header`/`footer`/`loader`/иконок, сообщения для скринридеров (`aria-live`), `onKeyUp`/`onInputKeydown`.

## Capabilities

### New Capabilities
- `autocomplete`: поле ввода с подсказками, одиночным и множественным режимом.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-autocomplete` (`publish = false`); зависимости: `prime-core`, `prime-dom`, `prime-inputtext` (классы), `prime-chip`, `prime-icons` (chevrondown, times, timescircle, spinner).
- Изменяются `Cargo.toml` workspace, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml`.
