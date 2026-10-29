# togglebutton Specification

## Purpose
TBD - created by archiving change port-togglebutton. Update Purpose after archive.

## Requirements

### Requirement: Состояние и события ToggleButton
Компонент `ToggleButton` крейта `prime-togglebutton` SHALL принимать двусторонний `checked` (`RwSignal<bool>`); без него SHALL вести внутреннее состояние, начинающееся с `default_checked`. Клик по корню и клавиши Enter и Space SHALL переключать состояние, вызывать `on_change` с новым значением и исходным событием ровно один раз и отменять действие браузера по умолчанию для клавиш. При `disabled` состояние и `on_change` SHALL не меняться. При `allow_empty = false` включённую кнопку SHALL быть нельзя выключить. `on_focus` и `on_blur` SHALL вызываться при получении и потере фокуса.

#### Scenario: Клик
- **WHEN** пользователь кликает по выключенной кнопке
- **THEN** сигнал становится `true` и `on_change` вызывается один раз со значением `true`

#### Scenario: Клавиши
- **WHEN** фокус на кнопке и нажаты Enter или Space
- **THEN** состояние переключается, прокрутка страницы по Space не происходит

#### Scenario: Disabled
- **WHEN** задан `disabled` и пользователь кликает
- **THEN** сигнал и `on_change` не затронуты

#### Scenario: Нельзя снять выбор
- **WHEN** `allow_empty = false`, кнопка включена и пользователь кликает
- **THEN** кнопка остаётся включённой, `on_change` не вызывается

### Requirement: Структура и классы ToggleButton
Корень SHALL быть `<div class="p-togglebutton p-component">` с модификаторами `p-togglebutton-checked`, `p-invalid`, `p-disabled`, `p-togglebutton-sm p-inputfield-sm` (`Size::Small`), `p-togglebutton-lg p-inputfield-lg` (`Size::Large`), `p-togglebutton-fluid`. Внутри SHALL идти `span.p-togglebutton-content`; без `content_template` в нём SHALL быть иконка (только при заданной `on_icon` или `off_icon`) в `span.p-togglebutton-icon` с `p-togglebutton-icon-left` или `p-togglebutton-icon-right` по `icon_pos` и `span.p-togglebutton-label` с `on_label` во включённом состоянии и `off_label` в выключенном. Пустая метка SHALL отображаться как один пробел.

#### Scenario: Включена
- **WHEN** `checked = true`, `on_label = "On"`
- **THEN** у корня есть `p-togglebutton-checked`, метка равна «On»

#### Scenario: Метки по умолчанию
- **WHEN** метки не заданы
- **THEN** выключенная кнопка показывает «No», включённая — «Yes»

#### Scenario: Иконка и позиция
- **WHEN** задана `on_icon` и `icon_pos = Right`
- **THEN** иконка во включённом состоянии лежит в `span.p-togglebutton-icon.p-togglebutton-icon-right`

#### Scenario: Свой контент
- **WHEN** задан `content_template`
- **THEN** его содержимое получает текущее `checked`, а иконка и метка по умолчанию не выводятся

### Requirement: Атрибуты ToggleButton
Корень SHALL иметь `role="button"`, `aria-pressed="true|false"`, `tabindex` (по умолчанию `0`, при `disabled` `-1`, если `tabindex` не задан явно), `aria-label`, `aria-labelledby` (только заданные), `data-pc-name="togglebutton"`, `data-p-checked`, `data-p-disabled` и `data-p`, в котором через пробел идут `checked`, `invalid` и размер (`small` или `large`).

#### Scenario: Доступность
- **WHEN** задан `aria_label = "Confirm"` и кнопка включена
- **THEN** у корня `aria-pressed="true"`, `aria-label="Confirm"`, `role="button"`, `data-p="checked"`

#### Scenario: Недоступна
- **WHEN** задан `disabled`
- **THEN** `tabindex="-1"`, `data-p-disabled="true"`, класс `p-disabled`

### Requirement: Свойства и pass-through ToggleButton
Свойства `on_label`, `off_label`, `on_icon`, `off_icon`, `icon_pos`, `size`, `disabled`, `invalid`, `allow_empty`, `fluid`, `tabindex`, `aria_label`, `aria_labelledby`, `style`, `style_class` SHALL принимать значение или сигнал и обновлять разметку при смене сигнала. `pt` SHALL принимать части `root`, `content`, `icon`, `label`, добавляя классы, стиль и атрибуты без замены собственных.

#### Scenario: Реактивная метка
- **WHEN** `on_label` передан сигналом и меняется у включённой кнопки
- **THEN** текст метки обновляется

#### Scenario: Класс через pt
- **WHEN** в `pt.label` передан класс `custom`
- **THEN** у метки есть `custom` и `p-togglebutton-label`

### Requirement: Стоимость отрисовки ToggleButton
Число аллокаций на экземпляр при SSR SHALL укладываться в бюджет `togglebutton-1000` в `benches/alloc-budgets.toml`, вклад в wasm — в бюджет `togglebutton` в `benches/budgets.toml`.

#### Scenario: Бюджеты
- **WHEN** выполняются `cargo xtask alloc-budget` и `cargo xtask wasm-size`
- **THEN** значения ToggleButton не превышают бюджеты
