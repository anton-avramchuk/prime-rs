## Purpose
Select позволяет выбрать одно значение из списка вариантов в выпадающей панели с поиском, группами и полным управлением с клавиатуры.

## ADDED Requirements

### Requirement: Значение и метка
Компонент `Select` крейта `prime-select` SHALL принимать двусторонний `value` (`RwSignal<Option<String>>`); без него SHALL вести внутреннее состояние, начинающееся с `default_value`. Метка (`span.p-select-label`) SHALL показывать `label` варианта (в группах — среди вариантов всех групп) с выбранным значением, при отсутствии выбора — `placeholder` с классом `p-placeholder`, а без `placeholder` — пустую метку с классом `p-select-label-empty`. `selected_item_template` SHALL заменять содержимое метки для выбранного варианта. При внешнем изменении `value` метка SHALL обновляться; значение, которого нет среди вариантов, SHALL считаться невыбранным.

#### Scenario: Выбранное значение
- **WHEN** значение равно значению варианта «New York»
- **THEN** метка показывает «New York» без класса `p-placeholder`

#### Scenario: Подсказка
- **WHEN** значения нет и задан `placeholder = "Select a City"`
- **THEN** метка показывает «Select a City» с классом `p-placeholder`

#### Scenario: Неизвестное значение
- **WHEN** значение не совпадает ни с одним вариантом
- **THEN** метка показывает подсказку

### Requirement: Панель
Панель `div.p-select-overlay` SHALL открываться по клику на поле, стрелкам вниз и вверх, `Enter`, `Space`, `Home`/`End` и набору печатного символа, а также программно через двусторонний `open`; закрываться по повторному клику, `Escape`, `Tab`, выбору варианта, клику вне компонента и изменению размера окна. Открытая панель SHALL быть расположена под полем (над ним, если снизу не хватает места) с z-index группы оверлеев, а `aria-expanded` метки и кнопки SHALL быть `true`. При открытии выбранный вариант SHALL прокручиваться в видимую область, а при `filter` фокус SHALL переходить в поле поиска (кроме `autofocus_filter = false`). Закрытие по `Escape` и выбору SHALL возвращать фокус на метку. При `disabled`, `readonly` и `loading` панель SHALL не открываться. События `on_show` и `on_hide` SHALL вызываться при открытии и закрытии.

#### Scenario: Открытие кликом
- **WHEN** пользователь кликает по закрытому полю
- **THEN** панель открыта, `aria-expanded="true"`, вызван `on_show`

#### Scenario: Закрытие снаружи
- **WHEN** панель открыта и пользователь нажимает вне компонента
- **THEN** панель закрыта, вызван `on_hide`

#### Scenario: Недоступно
- **WHEN** задан `disabled`
- **THEN** клик по полю панель не открывает

### Requirement: Выбор вариантов
Клик по варианту SHALL выбирать его значение, закрывать панель и вызывать `on_change` один раз с новым значением и исходным событием; клик по уже выбранному варианту SHALL только закрывать панель. Недоступные варианты (`disabled`) SHALL не выбираться и пропускаться клавиатурой. Выбранный вариант SHALL иметь `aria-selected="true"` и класс `p-select-option-selected` (при `checkmark` — галочку `p-select-option-check-icon`, у остальных пустой значок `p-select-option-blank-icon`). Вариант под фокусом SHALL иметь класс `p-focus` и `data-p-focused`; `focus_on_hover` (по умолчанию включён) SHALL переносить фокус на вариант под указателем. `item_template` SHALL заменять содержимое варианта, получая вариант.

#### Scenario: Выбор
- **WHEN** панель открыта и пользователь кликает по «London»
- **THEN** значение равно значению «London», панель закрыта, `on_change` вызван один раз

#### Scenario: Недоступный вариант
- **WHEN** пользователь кликает по варианту с `disabled`
- **THEN** значение и состояние панели не меняются

#### Scenario: Галочка
- **WHEN** задан `checkmark`
- **THEN** у выбранного варианта есть `p-select-option-check-icon`, у остальных `p-select-option-blank-icon`

### Requirement: Клавиатура
Метка SHALL принимать фокус (`tabindex`, по умолчанию `0`; при `disabled` равен `-1`). Стрелка вниз SHALL открывать панель, а в открытой — переносить фокус на следующий доступный вариант; стрелка вверх — на предыдущий (с `Alt` выбирает вариант под фокусом и закрывает панель); `Home` и `End` — на первый и последний доступный вариант; `PageUp` и `PageDown` — прокручивать к началу и концу списка; `Enter` и `Space` на закрытой панели открывать её, а на открытой выбирать вариант под фокусом и закрывать; `Escape` закрывать; `Tab` на открытой панели выбирать вариант под фокусом (если он есть) и закрывать панель; `Delete` при `show_clear` очищать значение. Набор печатных символов SHALL открывать панель и переносить фокус на первый вариант, метка которого начинается с набранного (накопленные символы сбрасываются через 500 мс). При `select_on_focus` перенос фокуса SHALL выбирать вариант без закрытия панели. Метка SHALL иметь `aria-activedescendant`, указывающий на вариант под фокусом.

#### Scenario: Навигация
- **WHEN** панель открыта, фокус на первом варианте и нажата стрелка вниз
- **THEN** фокус на втором, а недоступные варианты пропущены

#### Scenario: Выбор Enter
- **WHEN** фокус на варианте и нажат `Enter`
- **THEN** вариант выбран, панель закрыта, фокус на метке

#### Scenario: Поиск набором
- **WHEN** набраны символы «Lo»
- **THEN** фокус на первом варианте, чья метка начинается с «lo» без учёта регистра

#### Scenario: Закрытие Escape
- **WHEN** панель открыта и нажат `Escape`
- **THEN** панель закрыта, значение не изменилось

### Requirement: Поиск
При `filter` панель SHALL содержать поле поиска (`input.p-select-filter` в `p-iconfield` со значком поиска и подсказкой `filter_placeholder`), а список SHALL показывать варианты, метка которых содержит введённую строку без учёта регистра; в группах SHALL остаться только группы с подходящими вариантами. Ввод SHALL сбрасывать фокус варианта, вызывать `on_filter` со строкой и пересчитывать положение панели. Если подходящих вариантов нет, SHALL выводиться `li.p-select-empty-message` с `empty_filter_message` (по умолчанию «No results found»). Клавиши в поле поиска SHALL работать как в списке (стрелки, `Enter`, `Escape`, `Tab`), а `Home`, `End` и стрелки влево и вправо — перемещать курсор. При `reset_filter_on_hide` строка SHALL сбрасываться при закрытии.

#### Scenario: Фильтрация
- **WHEN** введено «or»
- **THEN** в списке только варианты, содержащие «or» без учёта регистра

#### Scenario: Нет совпадений
- **WHEN** введено «zzz»
- **THEN** список содержит только сообщение «No results found»

### Requirement: Группы и пустой список
При заданных `groups` список SHALL показывать для каждой группы `li.p-select-option-group` с `span.p-select-option-group-label` (или `group_template`), за которой идут её варианты; заголовки групп SHALL не выбираться и не получать фокус клавиатуры. Без вариантов SHALL выводиться `li.p-select-empty-message` с `empty_message` (по умолчанию «No results found»). Каждый вариант SHALL иметь `role="option"`, `aria-label`, `aria-setsize` (число выбираемых вариантов) и `aria-posinset` (номер среди выбираемых).

#### Scenario: Группа
- **WHEN** заданы две группы по три варианта
- **THEN** список содержит два заголовка и шесть вариантов в порядке групп, `aria-setsize="6"`

#### Scenario: Пустой список
- **WHEN** вариантов нет
- **THEN** панель содержит сообщение «No results found»

### Requirement: Очистка и состояние загрузки
При `show_clear`, выбранном значении и не `disabled` SHALL выводиться значок очистки `p-select-clear-icon`; клик по нему SHALL сбрасывать значение в `None`, не открывая панель, и вызывать `on_clear`. При `loading` кнопка `p-select-dropdown` SHALL показывать значок загрузки `p-select-loading-icon` вместо стрелки, а ввод и открытие панели SHALL быть заблокированы.

#### Scenario: Очистка
- **WHEN** выбран вариант, `show_clear` и клик по значку
- **THEN** значение `None`, метка показывает подсказку, панель не открылась, вызван `on_clear`

#### Scenario: Загрузка
- **WHEN** задан `loading`
- **THEN** вместо стрелки значок загрузки, клик по полю панель не открывает

### Requirement: Структура, классы и атрибуты
Корень SHALL быть `<div class="p-select p-component p-inputwrapper">` с модификаторами `p-disabled`, `p-variant-filled`, `p-focus`, `p-invalid`, `p-inputwrapper-filled`, `p-inputwrapper-focus` (в фокусе или при открытой панели), `p-select-open`, `p-select-fluid`, `p-select-sm p-inputfield-sm`, `p-select-lg p-inputfield-lg`, `style_class` и атрибутами `id` и `data-p` (`invalid`, `disabled`, `focus`, `fluid`, `filled`, размер). Метка SHALL иметь `role="combobox"`, `aria-haspopup="listbox"`, `aria-expanded`, `aria-controls` (на список при открытой панели), `aria-disabled`, `aria-label` (заданный или метка варианта), `aria-labelledby`, `aria-required`, `required`, `disabled`, `id` (`input_id`). Кнопка SHALL иметь `role="button"`, `aria-label="dropdown trigger"`, `aria-haspopup="listbox"`, `aria-expanded`. Список SHALL иметь `role="listbox"`, `aria-label` из перевода `aria.listLabel`, а идентификаторы вариантов — `{id}_{индекс}`, где индекс считается по списку вместе с заголовками групп.

#### Scenario: Корень открытого поля
- **WHEN** панель открыта, задан `size = Large`
- **THEN** у корня `p-select-open p-inputwrapper-focus p-select-lg p-inputfield-lg`

#### Scenario: Недоступное поле
- **WHEN** задан `disabled`
- **THEN** у корня `p-disabled`, у метки `tabindex="-1"`, `aria-disabled="true"`, `disabled`

### Requirement: Свойства и pass-through
Свойства `options`, `groups`, `placeholder`, `show_clear`, `checkmark`, `filter`, `loading`, `disabled`, `readonly`, `invalid`, `variant`, `size`, `fluid`, `scroll_height`, сообщения, подписи, `style`, `style_class`, `panel_style`, `panel_style_class` SHALL принимать значение или сигнал и обновлять разметку при смене сигнала. `pt` SHALL принимать части `root`, `label`, `clear_icon`, `dropdown`, `dropdown_icon`, `loading_icon`, `overlay`, `header`, `filter`, `list_container`, `list`, `option_group`, `option_group_label`, `option`, `option_label`, `option_check_icon`, `option_blank_icon`, `empty_message`, добавляя классы, стиль и атрибуты без замены собственных.

#### Scenario: Реактивные варианты
- **WHEN** `options` передан сигналом и пополняется при открытой панели
- **THEN** список обновляется без закрытия панели

#### Scenario: Класс через pt
- **WHEN** в `pt.option` передан класс `custom`
- **THEN** у каждого варианта есть `custom` и `p-select-option`

### Requirement: Стоимость отрисовки Select
Число аллокаций на экземпляр при SSR (панель закрыта) SHALL укладываться в бюджет `select-1000` в `benches/alloc-budgets.toml`, вклад в wasm — в бюджет `select` в `benches/budgets.toml`.

#### Scenario: Бюджеты
- **WHEN** выполняются `cargo xtask alloc-budget` и `cargo xtask wasm-size`
- **THEN** значения Select не превышают бюджеты
