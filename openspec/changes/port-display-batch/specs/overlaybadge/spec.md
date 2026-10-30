## Purpose
OverlayBadge накладывает бейдж на правый верхний угол произвольного содержимого (значок, аватар, кнопка), не меняя раскладку содержимого.

## ADDED Requirements

### Requirement: Разметка OverlayBadge
Компонент `OverlayBadge` крейта `prime-overlaybadge` SHALL выводить `<div class="p-overlaybadge">`, внутри которого сначала дочернее содержимое, затем `Badge`. `style_class` и `style` SHALL передаваться вложенному `Badge`, а не корню, как в PrimeNG. `pt.root` SHALL добавлять классы, стиль и атрибуты корню, `pt.pc_badge` — вложенному `Badge`.

#### Scenario: Порядок
- **WHEN** внутри `OverlayBadge` с `value = "2"` вложен значок
- **THEN** внутри `div.p-overlaybadge` сначала значок, затем `span.p-badge`

#### Scenario: Свойства бейджа
- **WHEN** заданы `severity = Danger`, `badge_size = Large`, `style_class = "mine"`
- **THEN** вложенный бейдж имеет классы `p-badge-danger`, `p-badge-lg` и `mine`, а корень `p-overlaybadge` — нет

#### Scenario: Без значения
- **WHEN** `value` не задан
- **THEN** вложенный бейдж отрисован точкой (`p-badge-dot`)

#### Scenario: Скрытие
- **WHEN** `badge_disabled = true`
- **THEN** вложенный бейдж скрыт, содержимое остаётся

#### Scenario: pt
- **WHEN** `pt.root` содержит атрибут `data-x="1"`, а `pt.pc_badge` — класс `custom`
- **THEN** корень имеет `data-x="1"`, а вложенный бейдж — класс `custom`

### Requirement: Положение бейджа
Положение бейджа SHALL задаваться CSS темы `overlaybadge`: корень позиционирован относительно, бейдж прижат к правому верхнему углу и смещён на половину своего размера наружу.

#### Scenario: Угол
- **WHEN** страница с подключённым CSS темы показывает значок с бейджем
- **THEN** центр бейджа совпадает с правым верхним углом корня

### Requirement: Стоимость отрисовки OverlayBadge
Число аллокаций на экземпляр при SSR SHALL укладываться в бюджет `overlaybadge-1000` в `benches/alloc-budgets.toml`, вклад в wasm — в бюджет `overlaybadge` в `benches/budgets.toml`.

#### Scenario: Бюджеты
- **WHEN** выполняются `cargo xtask alloc-budget` и `cargo xtask wasm-size`
- **THEN** значения OverlayBadge не превышают бюджеты
