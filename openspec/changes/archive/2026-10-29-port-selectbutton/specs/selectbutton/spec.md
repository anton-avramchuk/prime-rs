## Purpose
SelectButton выбирает одно или несколько значений из списка вариантов, показанных кнопками, и сообщает об изменении выбора.

## ADDED Requirements

### Requirement: Варианты и отображение
Компонент `SelectButton` крейта `prime-selectbutton` SHALL выводить по одной кнопке `ToggleButton` на каждый вариант из `options` в заданном порядке; метка кнопки SHALL быть `label` варианта в обоих состояниях. Выбранные варианты SHALL быть включены (`aria-pressed="true"`), остальные выключены. Вариант с `disabled`, а также все варианты при `disabled` у компонента SHALL быть недоступны. При смене `options` и значения разметка SHALL обновляться.

#### Scenario: Кнопки по вариантам
- **WHEN** передано три варианта
- **THEN** выводятся три кнопки с их метками в том же порядке

#### Scenario: Выбранный вариант
- **WHEN** значение равно значению второго варианта
- **THEN** вторая кнопка включена, остальные выключены

#### Scenario: Недоступный вариант
- **WHEN** у варианта `disabled = true`
- **THEN** его кнопка имеет `p-disabled`, клик по ней не меняет значение

### Requirement: Одиночный выбор
Без `multiple` компонент SHALL принимать двусторонний `value` (`RwSignal<Option<String>>`); без него SHALL вести внутреннее состояние, начинающееся с `default_value`. Клик по кнопке, а также Enter и Space на ней SHALL выбирать её значение, снимая выбор с остальных, и вызывать `on_change` один раз с новым значением и исходным событием, а затем `on_option_click` с вариантом, его индексом и исходным событием. Повторный клик по выбранной кнопке SHALL сбрасывать значение в `None`, если `allow_empty` не равен `false`; при `allow_empty = false` значение SHALL остаться, а `on_change` и `on_option_click` не вызываться.

#### Scenario: Выбор
- **WHEN** выбран вариант «One-Way» и пользователь кликает по «Return»
- **THEN** значение становится значением «Return», «One-Way» выключается, `on_change` вызван один раз

#### Scenario: Снятие выбора
- **WHEN** выбранную кнопку кликают повторно и `allow_empty` не задан
- **THEN** значение становится `None`

#### Scenario: Нельзя снять выбор
- **WHEN** `allow_empty = false` и выбранную кнопку кликают повторно
- **THEN** кнопка остаётся включённой, `on_change` не вызывается

### Requirement: Множественный выбор
При `multiple` компонент SHALL принимать двусторонний `values` (`RwSignal<Vec<String>>`); без него SHALL вести внутреннее состояние, начинающееся с `default_values`. Клик SHALL добавлять значение варианта в конец списка либо удалять его, если оно уже выбрано. При `allow_empty = false` единственное выбранное значение SHALL быть нельзя удалить.

#### Scenario: Добавление и удаление
- **WHEN** выбрано `[1]` и пользователь кликает по вариантам 2 и затем 1
- **THEN** значения последовательно равны `[1, 2]` и `[2]`

#### Scenario: Последнее значение
- **WHEN** `allow_empty = false` и выбрано одно значение, по нему кликают
- **THEN** список не меняется, `on_change` не вызывается

### Requirement: Структура и атрибуты SelectButton
Корень SHALL быть `<div class="p-selectbutton p-component">` с модификаторами `p-invalid` и `p-selectbutton-fluid`, `role="group"`, `aria-labelledby` (только заданный) и `data-p`, равным `invalid` при `invalid`. `size` и `fluid` SHALL передаваться каждой кнопке; `style_class` SHALL добавляться каждой кнопке, а не корню (как в PrimeNG). `item_template`, если задан, SHALL заменять содержимое кнопки, получая вариант и его индекс. `pt` SHALL принимать части `root` и `toggle_button` (передаётся каждой вложенной кнопке) и добавлять классы, стиль и атрибуты без замены собственных.

#### Scenario: Корень
- **WHEN** `invalid = true`, `fluid = true`, `aria_labelledby = "lbl"`
- **THEN** у корня `p-selectbutton p-component p-invalid p-selectbutton-fluid`, `role="group"`, `aria-labelledby="lbl"`, `data-p="invalid"`

#### Scenario: Размер
- **WHEN** задан `size = Small`
- **THEN** у каждой кнопки есть `p-togglebutton-sm`

#### Scenario: Класс кнопок
- **WHEN** задан `style_class = "mine"`
- **THEN** класс `mine` есть у каждой кнопки и нет у корня

#### Scenario: Свой контент
- **WHEN** задан `item_template`
- **THEN** кнопки содержат результат шаблона, метка по умолчанию не выводится

### Requirement: Стоимость отрисовки SelectButton
Число аллокаций на экземпляр при SSR SHALL укладываться в бюджет `selectbutton-1000` в `benches/alloc-budgets.toml`, вклад в wasm — в бюджет `selectbutton` в `benches/budgets.toml`.

#### Scenario: Бюджеты
- **WHEN** выполняются `cargo xtask alloc-budget` и `cargo xtask wasm-size`
- **THEN** значения SelectButton не превышают бюджеты
