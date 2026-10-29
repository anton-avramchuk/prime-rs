# iconfield Specification

## Purpose
IconField размещает значок внутри поля ввода слева или справа; InputIcon — контейнер этого значка.

## Requirements

### Requirement: Разметка IconField и InputIcon
Компонент `IconField` крейта `prime-iconfield` SHALL выводить `<span class="p-iconfield p-iconfield-left">` (при `icon_position = Right` — `p-iconfield-right`; по умолчанию `Left`) с дочерним содержимым. Компонент `InputIcon` SHALL выводить `<span class="p-inputicon">` с дочерним содержимым (значком). Оба SHALL принимать `style`, `style_class` (добавляется после собственных классов) и `pt` с частью `root`, добавляющий классы, стиль и атрибуты без замены собственных. Положение значка внутри поля SHALL обеспечиваться CSS темы `iconfield` по порядку дочерних элементов.

#### Scenario: Значок слева
- **WHEN** в `IconField` вложены `InputIcon` и поле
- **THEN** выводится `span.p-iconfield.p-iconfield-left`, внутри `span.p-inputicon` и поле в исходном порядке

#### Scenario: Значок справа
- **WHEN** `icon_position = Right`
- **THEN** у корня есть `p-iconfield-right` и нет `p-iconfield-left`

#### Scenario: Свой значок
- **WHEN** в `InputIcon` вложен `<svg>`
- **THEN** `<svg>` выводится внутри `span.p-inputicon` без изменений

#### Scenario: Классы и pt
- **WHEN** у `InputIcon` заданы `style_class = "pi"` и `pt.root` с классом `custom`
- **THEN** класс `p-inputicon pi custom`

### Requirement: Позиция значка в теме
CSS темы SHALL ставить `InputIcon`, идущий первым в `IconField`, у левого края поля, а идущий последним — у правого; поле, граничащее со значком слева или справа, SHALL получать внутренний отступ под значок.

#### Scenario: Положение
- **WHEN** страница показывает `IconField` со значком перед полем
- **THEN** значок лежит внутри границ поля слева, а текст поля сдвинут вправо от него

### Requirement: Стоимость отрисовки IconField
Число аллокаций на экземпляр при SSR (`IconField` со значком и полем) SHALL укладываться в бюджет `iconfield-1000` в `benches/alloc-budgets.toml`, вклад в wasm — в бюджет `iconfield` в `benches/budgets.toml`.

#### Scenario: Бюджеты
- **WHEN** выполняются `cargo xtask alloc-budget` и `cargo xtask wasm-size`
- **THEN** значения IconField не превышают бюджеты
