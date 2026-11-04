# AutoComplete

Порт `p-autocomplete` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-autocomplete::AutoComplete`, `AutoCompletePt`, `AutoCompleteOption`, `AutoCompleteCompleteEvent`, `AutoCompleteDropdownMode` | `packages/primeng/src/autocomplete/autocomplete.ts`, `autocomplete/style/autocompletestyle.ts` |
| Модель (плоский список, навигация, подпись значения, совпадение, разделитель) | `prime-autocomplete::state` — чистые функции с табличными тестами |
| CSS | `@primeuix/styles/autocomplete`, пресет Aura и блок `/* For PrimeNG */` — `cargo xtask gen-theme` (`autocomplete` в `primeng_extra`) |
| Демонстрации | `apps/showcase/doc/autocomplete/*-doc.ts` (`basic`, `dropdown`, `multiple`, `forceselection`, `group`, `clearicon`, `disabled`, `invalid`, `template`) |

Спека: `openspec/specs/autocomplete`. Варианты — `AutoCompleteOption` (`label`, `value` — по умолчанию подпись, `disabled`, `items` — вложенные варианты группы). Одиночный режим: `value: RwSignal<String>` (набранный текст либо значение выбранного варианта); `multiple`: `values: RwSignal<Vec<String>>`.

```rust
let (value, suggestions) = (RwSignal::new(String::new()), RwSignal::new(Vec::new()));
view! {
    <AutoComplete
        suggestions=suggestions
        value=value
        on_complete=Callback::new(move |e: AutoCompleteCompleteEvent| suggestions.set(search(&e.query)))
    />
}
```

## Что совпадает с PrimeNG

- Разметка и классы: `div.p-autocomplete.p-component.p-inputwrapper` > `input.p-inputtext.p-autocomplete-input[role=combobox]` либо `ul.p-autocomplete-input-multiple` с чипами `p-autocomplete-chip-item` > `Chip.p-autocomplete-chip` и `li.p-autocomplete-input-chip`; значок очистки, спиннер `p-autocomplete-loader`, кнопка `p-autocomplete-dropdown`; панель `p-autocomplete-overlay` > `p-autocomplete-list-container` > `ul.p-autocomplete-list[role=listbox]` > `li.p-autocomplete-option` (`-selected`, `p-focus`, `p-disabled`, `aria-*`, `data-p-*`), `li.p-autocomplete-option-group`, `li.p-autocomplete-empty-message`.
- Запрос: ввод от `min_length` символов через `delay` вызывает `on_complete`; до обновления `suggestions` виден спиннер; новые варианты открывают панель (в том числе повторно тот же набор); пустой результат — сообщение; кнопка-стрелка шлёт пустой запрос или текущий текст; `complete_on_focus`.
- Выбор и клавиши по `onKeyDown`: ↓ ↑ по допустимым вариантам, `Alt+↑` выбирает и закрывает, `Enter` выбирает, `Escape` закрывает с фокусом, `Tab` выбирает сфокусированный, `Backspace` в пустом `multiple` удаляет последний чип, ← → по чипам, `force_selection` (очистка несовпавшего ввода по `change` и закрытию), `unique`, `typeahead = false` (добавление набранного по `Enter`, разделителю, `Tab`, потере фокуса и вставке).

## Отличия

| Отличие | Причина |
|---|---|
| Варианты — `AutoCompleteOption` (строки), без объектов, `optionLabel`/`optionValue`/`dataKey` | Нет произвольных объектов; значение — строка. |
| Нет виртуальной прокрутки и `lazy`, `appendTo`, формы Angular, `onKeyUp`/`onInputKeydown`, сообщений для скринридеров (`aria-live`), `selectOnFocus`, `autoHighlight` | Вне объёма порта. |
| Панель выводится только при открытии и лежит внутри корня (`position: absolute` по CSS темы, корень `position: relative`) | В PrimeNG — отдельный `p-overlay`. |
| `Home`/`End` в поле не трогают каретку (браузер делает это сам) | PrimeNG ставит `setSelectionRange` вручную, итог тот же. |
| Шаблоны — свойства `item`, `group_view`, `selected_item`; нет `header`/`footer`/`loader`/иконок | Нет `pTemplate`. |
| Контейнер чипов обрабатывает только события на себе (стрелки поля не доходят до него) | В PrimeNG для этого вызывается `stopPropagation` в каждом обработчике. |

## Что не проверялось

- Раскладка справа налево и сенсорный ввод.
- Чтение с экрана: `aria-*` сверены с PrimeNG, поведение скринридеров — нет.
- Долгий список вариантов (без виртуальной прокрутки панель прокручивается стандартно).

## Производительность

См. `benches/` (`autocomplete-1`, `autocomplete-1000`, `case-autocomplete`) и `benches/*budgets.toml`.
