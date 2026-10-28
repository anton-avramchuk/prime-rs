## ADDED Requirements

### Requirement: Свойства Avatar
Компонент `Avatar` крейта `prime-avatar` SHALL принимать `label`, `icon` (содержимое Leptos), `image` (URL), `size` (`normal`, `large`, `xlarge`; по умолчанию `normal`), `shape` (`square`, `circle`; по умолчанию `square`), `style`, `style_class`, `aria_label`, `aria_labelledby`, `children`, `pt` и событие `on_image_error`. Все свойства необязательны; изменяемые во время работы SHALL принимать значение или сигнал.

#### Scenario: Метка
- **WHEN** `Avatar` отрисован с `label = "P"`
- **THEN** внутри есть `<span class="p-avatar-label">P</span>`

#### Scenario: Значения по умолчанию
- **WHEN** `Avatar` отрисован без свойств
- **THEN** у корня классы `p-avatar p-component`, а `data-p` равен `square normal`

### Requirement: Классы и атрибуты Avatar
Корень SHALL быть элементом с классами `p-avatar p-component` и модификаторами: `p-avatar-image` при заданном `image`, `p-avatar-circle` при `shape = circle`, `p-avatar-lg` при `size = large`, `p-avatar-xl` при `size = xlarge`. Атрибут `data-p` SHALL содержать через пробел форму и размер; `aria-label` и `aria-labelledby` SHALL переноситься на корень. `style_class` и `style` SHALL добавляться без потери собственных классов.

#### Scenario: Форма и размер
- **WHEN** заданы `shape = circle` и `size = xlarge`
- **THEN** у корня классы `p-avatar-circle` и `p-avatar-xl`, а `data-p` равен `circle xlarge`

#### Scenario: Изображение
- **WHEN** задан `image`
- **THEN** у корня есть `p-avatar-image`

#### Scenario: Доступность
- **WHEN** заданы `aria_label = "Фото"` и `aria_labelledby = "id1"`
- **THEN** у корня атрибуты `aria-label="Фото"` и `aria-labelledby="id1"`

### Requirement: Содержимое Avatar
Содержимое SHALL выводиться в порядке: `children`, затем метка `span.p-avatar-label`, если задана; иначе иконка `span.p-avatar-icon`, если задана; иначе изображение `<img>` с `src`, `aria-label` и `data-p`, если задан `image`. Каждый из этих элементов SHALL нести тот же `data-p`, что и корень.

#### Scenario: Приоритет
- **WHEN** заданы `label`, `icon` и `image`
- **THEN** выводится только метка

#### Scenario: Иконка вместо изображения
- **WHEN** заданы `icon` и `image`, но нет `label`
- **THEN** выводится `span.p-avatar-icon`, изображения нет

### Requirement: Ошибка загрузки изображения Avatar
При ошибке загрузки `<img>` SHALL вызываться `on_image_error` с исходным событием.

#### Scenario: Ошибка
- **WHEN** изображение не загрузилось
- **THEN** `on_image_error` вызван один раз

### Requirement: Pass-through Avatar
`Avatar` SHALL принимать `pt` для корня, метки, иконки и изображения; классы и атрибуты добавляются к собственным, не заменяя их.

#### Scenario: pt на метке
- **WHEN** передан `pt` с классом `extra` для метки
- **THEN** у `p-avatar-label` есть и `p-avatar-label`, и `extra`

### Requirement: Стоимость отрисовки Avatar
`Avatar` SHALL использовать статическую ветвь отрисовки, если все свойства статические, и реактивную иначе; обе ветви SHALL давать одинаковую разметку. Число аллокаций на экземпляр при SSR SHALL укладываться в бюджет `avatar` в `benches/alloc-budgets.toml`, вклад в wasm — в бюджет `avatar` в `benches/budgets.toml`.

#### Scenario: Паритет ветвей
- **WHEN** один и тот же набор значений передан статически и сигналами
- **THEN** HTML совпадает

#### Scenario: Бюджеты
- **WHEN** выполняются `cargo xtask alloc-budget` и `cargo xtask wasm-size`
- **THEN** значения Avatar не превышают бюджеты
