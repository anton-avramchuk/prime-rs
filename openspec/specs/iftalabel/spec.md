# iftalabel Specification

## Purpose
IftaLabel показывает метку над значением внутри поля, оставляя её на месте (стиль «inside field, top aligned»).

## Requirements

### Requirement: Разметка IftaLabel
Компонент `IftaLabel` крейта `prime-iftalabel` SHALL выводить `<span class="p-iftalabel">` с дочерним содержимым (поле и его `<label>`). `style_class` SHALL добавляться после собственных классов, `style` — задавать встроенный стиль, `pt.root` — добавлять классы, стиль и атрибуты без замены собственных. Положение метки над значением и отступы поля SHALL обеспечиваться CSS темы `iftalabel`.

#### Scenario: Корень
- **WHEN** в `IftaLabel` вложены поле и метка
- **THEN** выводится один `span.p-iftalabel` с обоими элементами в исходном порядке

#### Scenario: Классы и стиль
- **WHEN** заданы `style_class = "mine"` и `style = "width: 14rem"`
- **THEN** класс корня `p-iftalabel mine`, атрибут `style="width: 14rem;"`

#### Scenario: Положение метки
- **WHEN** страница с подключённым CSS темы показывает `IftaLabel` с `InputText`
- **THEN** метка лежит внутри границ поля над значением

#### Scenario: Недопустимое значение
- **WHEN** вложенное поле имеет класс `p-invalid`
- **THEN** метка получает цвет недопустимого значения из темы

### Requirement: Стоимость отрисовки IftaLabel
Число аллокаций на экземпляр при SSR SHALL укладываться в бюджет `iftalabel-1000` в `benches/alloc-budgets.toml`, вклад в wasm — в бюджет `iftalabel` в `benches/budgets.toml`.

#### Scenario: Бюджеты
- **WHEN** выполняются `cargo xtask alloc-budget` и `cargo xtask wasm-size`
- **THEN** значения IftaLabel не превышают бюджеты
