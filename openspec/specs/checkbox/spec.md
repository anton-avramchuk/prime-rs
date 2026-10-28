# checkbox Specification

## Purpose
TBD - created by archiving change port-checkbox. Update Purpose after archive.

## Requirements

### Requirement: Состояние и события Checkbox
Компонент `Checkbox` крейта `prime-checkbox` SHALL поддерживать режим `binary` (значение `RwSignal<bool>`) и режим набора (`value` и `RwSignal<Vec<String>>`), переключать состояние при изменении нативного `<input type="checkbox">`, вызывать `on_change` (с новым `checked` и исходным событием), `on_focus`, `on_blur`; при `readonly` и `disabled` состояние SHALL не меняться.

#### Scenario: Binary
- **WHEN** пользователь кликает по Checkbox в режиме `binary`
- **THEN** сигнал меняет значение и вызывается `on_change` с новым `checked`

#### Scenario: Набор значений
- **WHEN** в режиме набора кликают по Checkbox со `value = "a"`
- **THEN** `a` добавляется в сигнал, а повторный клик его удаляет

#### Scenario: Readonly
- **WHEN** `readonly` включён и пользователь кликает
- **THEN** сигнал и `on_change` не затронуты

### Requirement: Разметка и классы Checkbox
Корень SHALL быть `<div>` с классами `p-checkbox p-component` и модификаторами `p-checkbox-checked p-highlight` (отмечен), `p-disabled`, `p-invalid`, `p-variant-filled`, `p-checkbox-sm p-inputfield-sm`, `p-checkbox-lg p-inputfield-lg`; внутри SHALL идти `<input type="checkbox" class="p-checkbox-input">`, затем `<div class="p-checkbox-box">` с иконкой `check` (отмечен) или `minus` (`indeterminate`) и классом `p-checkbox-icon`. Корень SHALL иметь `data-p-checked`, `data-p-highlight`, `data-p-disabled` и `data-p` (через пробел `invalid`, `checked`, `disabled`, `filled` при варианте `filled`, размер).

#### Scenario: Отмечен
- **WHEN** `checked = true`
- **THEN** есть классы `p-checkbox-checked p-highlight`, у `<input>` атрибут `checked`, в боксе иконка `check`

#### Scenario: Indeterminate
- **WHEN** `indeterminate` включён
- **THEN** состояние не считается отмеченным, в боксе иконка `minus`

#### Scenario: Вариант из конфигурации
- **WHEN** `variant` не задан, а `PrimeConfig.input_variant = filled`
- **THEN** есть класс `p-variant-filled`

### Requirement: Атрибуты и pass-through Checkbox
`<input>` SHALL получать `id` (`input_id`), `name`, `value`, `tabindex`, `required`, `readonly`, `disabled`, `aria-label`, `aria-labelledby`, `input_class`, `input_style`; незаданные атрибуты SHALL отсутствовать. `pt` SHALL принимать `root`, `input`, `box`, `icon`, добавляя классы и атрибуты без замены собственных.

#### Scenario: Атрибуты
- **WHEN** заданы `input_id`, `name`, `disabled`
- **THEN** у `<input>` есть `id`, `name`, `disabled`, а `readonly` и `required` нет

### Requirement: Стоимость отрисовки Checkbox
Checkbox со статическими свойствами и без сигналов значения SHALL использовать статическую ветвь; обе ветви SHALL давать одинаковую разметку. Число аллокаций на экземпляр при SSR SHALL укладываться в бюджет `checkbox` в `benches/alloc-budgets.toml`, вклад в wasm — в бюджет `checkbox` в `benches/budgets.toml`.

#### Scenario: Паритет ветвей
- **WHEN** один набор значений передан статически и сигналами
- **THEN** HTML совпадает

#### Scenario: Бюджеты
- **WHEN** выполняются `cargo xtask alloc-budget` и `cargo xtask wasm-size`
- **THEN** значения Checkbox не превышают бюджеты
