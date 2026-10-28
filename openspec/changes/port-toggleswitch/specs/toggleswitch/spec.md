## ADDED Requirements

### Requirement: Состояние и события ToggleSwitch
Компонент `ToggleSwitch` крейта `prime-toggleswitch` SHALL принимать двусторонний `checked` (`RwSignal<bool>`), переключать его по клику на любой части корня (в том числе на нативном `<input>`), вызывать `on_change` с новым значением и исходным событием ровно один раз на клик, возвращать фокус на `<input>`, вызывать `on_focus` и `on_blur`. При `disabled` и `readonly` состояние SHALL не меняться.

#### Scenario: Клик
- **WHEN** пользователь кликает по ToggleSwitch
- **THEN** сигнал меняет значение и `on_change` вызывается один раз с новым значением

#### Scenario: Readonly и disabled
- **WHEN** включён `readonly` или `disabled` и пользователь кликает
- **THEN** сигнал, нативное состояние `<input>` и `on_change` не затронуты

### Requirement: Разметка и классы ToggleSwitch
Корень SHALL быть `<div>` с классами `p-toggleswitch p-component` и модификаторами `p-toggleswitch-checked`, `p-disabled`, `p-invalid`, со `style="position: relative"` (к нему присоединяются `style` и `pt.root`); внутри SHALL идти `<input type="checkbox" role="switch" class="p-toggleswitch-input">` с `aria-checked`, затем `<div class="p-toggleswitch-slider">` с `<div class="p-toggleswitch-handle">`. Корень SHALL иметь `data-p-checked`, `data-p-disabled`, а корень, слайдер и ручка — `data-p` (через пробел `checked`, `disabled`, `invalid`).

#### Scenario: Включён
- **WHEN** `checked = true`
- **THEN** есть класс `p-toggleswitch-checked`, `aria-checked="true"`, `data-p-checked="true"`, `data-p="checked"`, у `<input>` атрибут `checked`

#### Scenario: Слот ручки
- **WHEN** задан `handle_template`
- **THEN** его содержимое внутри `p-toggleswitch-handle`, шаблон получает текущее `checked`

### Requirement: Атрибуты и pass-through ToggleSwitch
`<input>` SHALL получать `id` (`input_id`), `name`, `tabindex`, `required`, `disabled`, `aria-label`, `aria-labelledby`; незаданные атрибуты SHALL отсутствовать. `pt` SHALL принимать `root`, `input`, `slider`, `handle`, добавляя классы и атрибуты без замены собственных.

#### Scenario: Атрибуты
- **WHEN** заданы `input_id`, `name`
- **THEN** у `<input>` есть `id`, `name`, а `required` и `tabindex` нет

### Requirement: Стоимость отрисовки ToggleSwitch
ToggleSwitch со статическими свойствами и без `checked` SHALL использовать статическую ветвь; обе ветви SHALL давать одинаковую разметку. Число аллокаций на экземпляр при SSR SHALL укладываться в бюджет `toggleswitch` в `benches/alloc-budgets.toml`, вклад в wasm — в бюджет `toggleswitch` в `benches/budgets.toml`.

#### Scenario: Паритет ветвей
- **WHEN** один набор значений передан статически и сигналами
- **THEN** HTML совпадает

#### Scenario: Бюджеты
- **WHEN** выполняются `cargo xtask alloc-budget` и `cargo xtask wasm-size`
- **THEN** значения ToggleSwitch не превышают бюджеты
