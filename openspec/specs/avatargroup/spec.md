# avatargroup Specification

## Purpose
TBD - created by archiving change port-avatargroup. Update Purpose after archive.

## Requirements

### Requirement: Разметка AvatarGroup
Компонент `AvatarGroup` крейта `prime-avatargroup` SHALL отрисовывать корень `<div class="p-avatar-group p-component">` с дочерним содержимым без обёрток; `style_class` и `style` SHALL добавляться без потери собственных классов.

#### Scenario: Дети
- **WHEN** внутри AvatarGroup переданы три элемента
- **THEN** они выводятся в корне в том же порядке

#### Scenario: Класс и стиль
- **WHEN** заданы `style_class = "mine"` и `style = "gap: 1px"`
- **THEN** корень имеет классы `p-avatar-group p-component mine` и стиль `gap: 1px`

### Requirement: Pass-through AvatarGroup
`pt` SHALL принимать `root`, добавляя классы и атрибуты без замены собственных.

#### Scenario: Класс через pt
- **WHEN** в `pt.root` передан класс `custom`
- **THEN** у корня есть и `custom`, и `p-avatar-group`

### Requirement: Стоимость отрисовки AvatarGroup
AvatarGroup со статическими свойствами SHALL использовать статическую ветвь; обе ветви SHALL давать одинаковую разметку. Число аллокаций на экземпляр при SSR SHALL укладываться в бюджет `avatargroup` в `benches/alloc-budgets.toml`, вклад в wasm — в бюджет `avatargroup` в `benches/budgets.toml`.

#### Scenario: Паритет ветвей
- **WHEN** один набор значений передан статически и сигналами
- **THEN** HTML совпадает

#### Scenario: Бюджеты
- **WHEN** выполняются `cargo xtask alloc-budget` и `cargo xtask wasm-size`
- **THEN** значения AvatarGroup не превышают бюджеты
