# fieldset Specification

## Purpose
TBD - created by archiving change port-fieldset. Update Purpose after archive.

## Requirements

### Requirement: Структура и классы Fieldset
Компонент `Fieldset` крейта `prime-fieldset` SHALL отрисовывать `<fieldset class="p-fieldset p-component">` с `<legend class="p-fieldset-legend">` и контейнером `p-fieldset-content-container` с обёрткой `p-fieldset-content-wrapper` и содержимым `p-fieldset-content`. Без `toggleable` легенда SHALL содержать `span.p-fieldset-legend-label` с текстом `legend` и слот `header_template`. При `toggleable` корень SHALL получать `p-fieldset-toggleable`, а при свёрнутом состоянии — `p-fieldset-collapsed`; `data-p="toggleable"` SHALL быть на корне и легенде. `style_class` и `style` SHALL добавляться без потери собственных классов.

#### Scenario: Простой Fieldset
- **WHEN** отрисован `<Fieldset legend="Header">` с содержимым
- **THEN** HTML содержит `<fieldset`, `<legend`, `p-fieldset-legend-label` с текстом `Header`, `p-fieldset-content`, а кнопка-переключатель отсутствует

#### Scenario: Сворачиваемый Fieldset
- **WHEN** задан `toggleable` и `collapsed = true`
- **THEN** корень имеет `p-fieldset-toggleable p-fieldset-collapsed`, а при `collapsed = false` класса `p-fieldset-collapsed` нет

### Requirement: Переключатель в легенде Fieldset
При `toggleable` легенда SHALL содержать `<button class="p-fieldset-toggle-button">` с иконкой `p-fieldset-toggle-icon` («плюс» при свёрнутом, «минус» при развёрнутом; слоты `expand_icon_template` и `collapse_icon_template` заменяют их) и содержимым легенды (`legend`, `header_template`) после иконки. Клик по кнопке (в том числе Enter и Space) SHALL инвертировать `collapsed`, записывая значение в `RwSignal<bool>` (без сигнала — собственное состояние, начальное `default_collapsed`). Кнопка SHALL иметь `aria-controls`, `aria-expanded` и `aria-label` (текст `legend`).

#### Scenario: Клик
- **WHEN** пользователь нажимает кнопку развёрнутого Fieldset
- **THEN** сигнал `collapsed` становится `true`, `aria-expanded` — `false`, иконка меняется на «плюс»

#### Scenario: Свои иконки
- **WHEN** заданы `expand_icon_template` и `collapse_icon_template`
- **THEN** вместо стандартных иконок выводится слот, соответствующий состоянию

### Requirement: События и доступность Fieldset
Fieldset SHALL вызывать `on_before_toggle` с исходным событием и прежним `collapsed` до смены состояния и `on_after_toggle` с новым `collapsed` после окончания анимации. Область содержимого SHALL иметь `id`, `role="region"`, `aria-labelledby` (идентификатор кнопки), `aria-hidden` (при `toggleable`) и `tabindex="-1"` только при свёрнутом состоянии; фокусируемые потомки свёрнутой области SHALL получать `tabindex="-1"`. Свёрнутое содержимое SHALL быть скрыто с первого рендера, а смена состояния в браузере SHALL анимироваться классами `p-collapsible-enter-active`/`p-collapsible-leave-active`.

#### Scenario: Связь кнопки и области
- **WHEN** Fieldset со `toggleable` отрисован
- **THEN** `aria-controls` кнопки равен `id` области, а `aria-labelledby` области — `id` кнопки

#### Scenario: До и после
- **WHEN** Fieldset переключается
- **THEN** `on_before_toggle` вызван с прежним значением, а `on_after_toggle` — после анимации с новым

#### Scenario: Начальное состояние
- **WHEN** Fieldset отрисован с `collapsed = true`
- **THEN** область скрыта сразу, `aria-hidden="true"`, `tabindex="-1"`

### Requirement: Pass-through Fieldset
`pt` SHALL принимать части `root`, `legend`, `legend_label`, `toggle_button`, `toggle_icon`, `content_container`, `content_wrapper`, `content`, добавляя классы, стиль и атрибуты без замены собственных.

#### Scenario: Класс через pt
- **WHEN** в `pt.legend` передан класс `custom`
- **THEN** у легенды есть `custom` и `p-fieldset-legend`

### Requirement: Стоимость отрисовки Fieldset
Fieldset без `toggleable` и со статическими свойствами SHALL использовать статическую ветвь; обе ветви SHALL давать одинаковую разметку. Число аллокаций на экземпляр при SSR SHALL укладываться в бюджет `fieldset-1000` в `benches/alloc-budgets.toml`, вклад в wasm — в бюджет `fieldset` в `benches/budgets.toml`.

#### Scenario: Паритет ветвей
- **WHEN** один набор значений передан статически и сигналами
- **THEN** HTML совпадает

#### Scenario: Бюджеты
- **WHEN** выполняются `cargo xtask alloc-budget` и `cargo xtask wasm-size`
- **THEN** значения Fieldset не превышают бюджеты
