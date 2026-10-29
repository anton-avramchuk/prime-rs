# Design

## Context

Требования — в `specs/select`. Исходники PrimeNG 21.1.10: `packages/primeng/src/select/select.ts` (2015 строк: `Select` и вложенный `SelectItem`), `select/style/selectstyle.ts`, демонстрации `apps/showcase/doc/select`. Образцы — `crates/prime-inputnumber` и `crates/prime-knob` (слушатели `window` и таймеры из обработчиков, `StoredValue<_, LocalStorage>`, `Effect` только под `cfg(target_arch = "wasm32")`, замыкания, не пересоздающие элементы), `crates/prime-selectbutton` (значения-строки, `SelectButtonOption`), `crates/prime-dom` (`position::absolute_position`, `dom::align_overlay`, `FocusTrap` с фокусируемыми границами) и `prime_core::zindex` (`ZGroup::Overlay`). Объём первого change выбран пользователем: ядро, фильтр и группы.

Наблюдения по исходнику, влияющие на решения:
- Панель — отдельный `p-overlay` (позиционирование, добавление в `body` или `self`, закрытие снаружи, анимация); у нас такого компонента нет, и Select — его первый потребитель.
- Индексы вариантов (`focusedOptionIndex`, id `{id}_{индекс}`) считаются по плоскому списку, в котором заголовки групп занимают позиции; допустимыми для фокуса считаются только не отключённые варианты (`isValidOption`). Поиск набором (`searchOptions`) копит символы 500 мс и ищет первый допустимый вариант с префиксом.
- Поиск `filter` в PrimeNG использует `FilterService` (режимы совпадения, поля, локаль); по умолчанию — «содержит». В группах отбираются группы с подходящими вариантами.
- `Tab` на открытой панели с фокусируемыми элементами уводит фокус на скрытый граничный элемент панели (ловушка фокуса), иначе выбирает вариант под фокусом и закрывает панель.
- Селекторы `isVisibleClearIcon` и `emptyMessage` зависят от перевода (`emptyMessage`, `emptyFilterMessage`, `aria.listLabel`); эти ключи уже есть в `prime-core::translation_defaults`.
- Блок `/* For PrimeNG */` в `selectstyle.ts` содержит нужное правило цвета подсказки `.p-select-label.p-placeholder { color: dt('select.placeholder.color') }` (плюс правила Angular Forms), поэтому `select` добавляется в `primeng_extra`.
- Поле поиска в PrimeNG — `p-iconfield` + `p-inputicon`; такие компоненты у нас не портированы, но их CSS (`iconfield`, `inputicon`) нужен для разметки поля поиска.

## Decisions

### D1. Данные и значение
`SelectOption { label: String, value: String, disabled: bool }`, `SelectGroup { label: String, items: Vec<SelectOption> }`; `options: Prop<Vec<SelectOption>>` и `groups: Prop<Vec<SelectGroup>>` (если групп нет — плоский список). Значения — строки (как у `SelectButton`). `value: Option<RwSignal<Option<String>>>`, иначе внутренний сигнал с `default_value`. Состояние открытости: `open: Option<RwSignal<bool>>`, иначе внутренний.

### D2. Плоский список (`options.rs`, чистые функции)
`Entry::Group(label)` и `Entry::Option { option, group }`; `visible_entries(options, groups, filter) -> Vec<Entry>` (фильтрация «содержит» без учёта регистра; группы без подходящих вариантов опускаются). Чистые функции индексов: `first_valid`, `last_valid`, `next_valid`, `prev_valid` (как `findNextOptionIndex` и `findPrevOptionIndex`: при отсутствии следующего индекс остаётся прежним), `selected_index`, `first_focused` (выбранный или первый допустимый), `search_match(entries, typed)`, `set_size`, `pos_in_set`. Табличные тесты на группы, отключённые варианты и границы.

### D3. Состояние и реактивность
`value`, `open`, `filter: RwSignal<String>`, `focused_index: RwSignal<Option<usize>>` (индекс в плоском списке), `focused: RwSignal<bool>` (метка в фокусе), `clicked: RwSignal<bool>` (открыто мышью: первая стрелка идёт на первый или последний вариант, а не на выбранный), буфер набора (`StoredValue`) с таймером 500 мс. Метка, значок очистки и кнопка читают только `value`, `open`, `loading` и свойства — панель в DOM есть лишь при `open`, поэтому замыкание панели можно пересоздавать, а метка (элемент, получающий фокус) не пересоздаётся (урок `port-selectbutton`). Внутри списка элементы строятся по плоскому списку; фокус варианта — класс и атрибуты через замыкания, без пересоздания `li`.

### D4. Панель (`overlay.rs`)
Панель — потомок корня (`position: relative` из CSS темы), абсолютно позиционируется `prime_dom::align_overlay` (под полем, над ним при нехватке места), z-index — `prime_core::zindex::generate(ZGroup::Overlay, base)` из конфигурации, снимается при закрытии. Подписки на `window` (клик/`mousedown` вне компонента, `resize`) создаются при открытии из обработчика или `Effect` под `cfg(wasm32)` и снимаются при закрытии и `on_cleanup`. Анимации нет (панель появляется и исчезает мгновенно), `appendTo` не поддерживается (всегда внутри корня). `scroll_height` — `max-height` контейнера списка, при открытии выбранный вариант прокручивается `scrollIntoView({ block: nearest })`.

### D5. Клавиатура и фокус
`keydown` метки (и поля поиска) разбирает `KeyboardEvent.code`: `ArrowDown`, `ArrowUp` (+`Alt`), `Home`, `End`, `PageUp`, `PageDown`, `Space`, `Enter`/`NumpadEnter`, `Escape`, `Tab`, `Delete`, `Backspace` (в поле поиска), печатные символы (поиск набором). Правила — как в `onKeyDown` и `onFilterKeyDown`. Заблокированное (`disabled`, `readonly`, `loading`) игнорирует клавиши. `Tab`: при открытой панели с фокусируемыми элементами фокус уходит на скрытую границу (ловушка из `prime-dom`), иначе вариант под фокусом выбирается и панель закрывается. Фокус панели возвращается на метку при закрытии (`Escape`, выбор, `Tab`).

### D6. Поиск
Поле поиска — `input.p-inputtext p-component p-select-filter` внутри `span.p-iconfield` со `SearchIcon` в `span.p-inputicon`; `on:input` пишет `filter`, сбрасывает `focused_index`, зовёт `on_filter` и после перерисовки пересчитывает положение панели. `reset_filter_on_hide` сбрасывает строку при закрытии. Сообщение пустого результата — `empty_filter_message` или перевод `emptyFilterMessage`, пустого списка — `empty_message` или `emptyMessage` (как в PrimeNG: `emptyMessageLabel || emptyFilterMessageLabel`).

### D7. Разметка, классы, атрибуты
Классы и `data-p` — по `selectstyle.ts` (`state.rs`, табличные тесты): корень `p-select p-component p-inputwrapper` + модификаторы; метка `p-select-label` (+`p-placeholder`, `p-select-label-empty`); панель `p-select-overlay p-component-overlay p-component`; вариант `p-select-option` (+`p-select-option-selected` без `checkmark`, `p-disabled`, `p-focus`). Идентификатор `id` — `pn_id_<n>` из `prime_core::ids::next_unique`, варианты — `{id}_{индекс}`, список — `{id}_list`. Метка — `span[role=combobox]`, а не `<input>`. Лишние атрибуты элементов собираются в `Vec<AnyAttribute>` при превышении 26 (урок `port-inputnumber`).

### D8. Значки и шаблоны
По умолчанию `ChevronDownIcon`, `TimesIcon`, `SpinnerIcon` (с вращением), `CheckIcon`, `BlankIcon`, `SearchIcon`; `dropdown_icon`, `clear_icon`, `loading_icon` заменяют их (`ViewFn`). Шаблоны: `item_template: Callback<SelectOption, AnyView>`, `selected_item_template: Callback<SelectOption, AnyView>`, `group_template: Callback<SelectGroup, AnyView>`.

### D9. CSS
`select`, `iconfield` в `theme.components` и `select` в `primeng_extra` (`upstream.toml`), `cargo xtask gen-theme`; `@primeuix/styles` отдельного модуля `inputicon` не содержит: классы `.p-inputicon` входят в `iconfield`; нужно проверить, что CSS других компонентов не изменился.

### D10. Демонстрации
`basic`, `checkmark`, `clearicon`, `disabled`, `filled`, `filter`, `fluid`, `group`, `invalid`, `loadingstate`, `sizes`, `template` (двенадцать). В `template` вместо флагов страны (картинки с CDN) — символьные метки; в `group` — группы городов. В `basic` — `echo` значения для e2e.

### D11. Тесты
Чистые: `options.rs` (индексы, фильтр, группы, набор) и `state.rs`. SSR: закрытая разметка, метка и подсказка, значок очистки, загрузка, классы и `data-p`, открытая панель (через `open`): варианты, группы, `aria-*`, галочки, поле поиска, пустое сообщение, `pt`, паритет ветвей. Браузерные (`tests/web.rs`): открытие и закрытие (клик, Escape, клик снаружи, Tab), выбор мышью и клавишей, пропуск отключённых, набор символов, фильтр, группы, очистка, `disabled`/`readonly`/`loading`, фокус возвращается на метку и метка не пересоздаётся, внешний `value`, `open`, реактивные варианты. e2e: реальная клавиатура и мышь, положение панели под полем.

## Risks / Trade-offs

- [Первый оверлей: положение, z-index, закрытие снаружи, обрезание родителями с `overflow`] → панель внутри корня и позиционирование из `prime-dom`; документируем отсутствие `appendTo` и анимации.
- [Большой объём клавиатурной логики] → чистые функции индексов с таблицами, поведение из `onKeyDown` закрепляется браузерными тестами.
- [Фильтр проще PrimeNG (только «содержит» по метке)] → зафиксировано в спеке и `docs/select.md`.
- [Значение-строка и типизированные варианты вместо `optionLabel`/`optionValue`] → осознанное отличие, как у SelectButton.

## Migration Plan

Миграции нет: новый крейт и данные темы. Откат — `git revert`.

## Open Questions

Нет.
