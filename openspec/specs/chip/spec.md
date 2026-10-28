# chip Specification

## Purpose
Компонент Chip (порт p-chip из PrimeNG): метка, иконка или изображение, удаление мышью и клавиатурой, события, локализуемая подпись, pass-through.

## Requirements

### Requirement: Свойства Chip
Компонент `Chip` крейта `prime-chip` SHALL принимать `label`, `icon` (содержимое Leptos), `image` (URL), `alt`, `disabled`, `removable`, `remove_icon` (содержимое Leptos), `style`, `style_class`, `children`, `pt` и события `on_remove`, `on_image_error`. Все свойства необязательны; изменяемые во время работы SHALL принимать значение или сигнал.

#### Scenario: Метка
- **WHEN** `Chip` отрисован с `label = "Amy Elsner"`
- **THEN** внутри есть `<div class="p-chip-label">Amy Elsner</div>`

#### Scenario: Без метки
- **WHEN** `label` не задана
- **THEN** элемента `p-chip-label` нет

### Requirement: Классы и атрибуты Chip
Корень SHALL быть элементом с классами `p-chip p-component` и `p-disabled` при `disabled`; атрибут `data-p` SHALL равняться `removable` при `removable` и быть пустым иначе. `style_class` и `style` SHALL добавляться без потери собственных классов.

#### Scenario: Disabled и removable
- **WHEN** заданы `disabled` и `removable`
- **THEN** у корня есть `p-disabled`, а `data-p` равен `removable`

### Requirement: Содержимое Chip
Содержимое SHALL выводиться в порядке: `children`, затем изображение `<img class="p-chip-image">` с `src` и `alt`, если задан `image`, иначе иконка в `<span class="p-chip-icon">`, если задана, затем метка, затем элемент удаления, если `removable`.

#### Scenario: Изображение вместо иконки
- **WHEN** заданы `image` и `icon`
- **THEN** выводится `img.p-chip-image`, а `p-chip-icon` нет

#### Scenario: Иконка
- **WHEN** задана только `icon`
- **THEN** выводится `span.p-chip-icon`

### Requirement: Удаление Chip
При `removable` SHALL выводиться элемент `span.p-chip-remove-icon` с `role="button"`, `tabindex="0"` (`-1` при `disabled`) и `aria-label` из `PrimeConfig.locale` (`aria.removeLabel`, по умолчанию `Remove`); внутри — `remove_icon` или `TimesCircleIcon`. Клик по нему, а также клавиши Enter и Backspace SHALL скрывать Chip (`display: none`) и вызывать `on_remove` ровно один раз.

#### Scenario: Клик
- **WHEN** пользователь нажимает на элемент удаления
- **THEN** Chip скрыт (`display: none`) и `on_remove` вызван один раз

#### Scenario: Клавиатура
- **WHEN** фокус на элементе удаления и нажата Enter или Backspace
- **THEN** Chip скрыт и `on_remove` вызван один раз

#### Scenario: Подпись из локали
- **WHEN** в `PrimeConfig.locale` задано `aria.removeLabel = "Удалить"`
- **THEN** у элемента удаления `aria-label="Удалить"`

#### Scenario: Не удаляемый
- **WHEN** `removable` не задан
- **THEN** элемента удаления нет

### Requirement: Ошибка загрузки изображения
При ошибке загрузки `img.p-chip-image` SHALL вызываться `on_image_error` с исходным событием.

#### Scenario: Ошибка
- **WHEN** изображение не загрузилось
- **THEN** `on_image_error` вызван один раз

### Requirement: Pass-through Chip
`Chip` SHALL принимать `pt` для корня, изображения, иконки, метки и элемента удаления; классы и атрибуты добавляются к собственным, не заменяя их.

#### Scenario: pt на метке
- **WHEN** передан `pt` с классом `extra` для метки
- **THEN** у `p-chip-label` есть и `p-chip-label`, и `extra`

### Requirement: Стоимость отрисовки Chip
`Chip` SHALL использовать статическую ветвь отрисовки, если все свойства статические и он не удаляемый, и реактивную иначе; обе ветви SHALL давать одинаковую разметку. Число аллокаций на экземпляр при SSR SHALL укладываться в бюджет `chip` в `benches/alloc-budgets.toml`, вклад в wasm — в бюджет `chip` в `benches/budgets.toml`.

#### Scenario: Паритет ветвей
- **WHEN** один и тот же набор значений передан статически и сигналами
- **THEN** HTML совпадает

#### Scenario: Бюджеты
- **WHEN** выполняются `cargo xtask alloc-budget` и `cargo xtask wasm-size`
- **THEN** значения Chip не превышают бюджеты
