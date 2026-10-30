## Purpose
MeterGroup показывает составную шкалу: несколько цветных отрезков, доли которых отсчитываются от диапазона `min`–`max`, и подписи к ним.

## ADDED Requirements

### Requirement: Корень MeterGroup
Компонент `MeterGroup` крейта `prime-metergroup` SHALL выводить корень с `role="meter"`, классами `p-metergroup p-component p-metergroup-<orientation>` (`horizontal` по умолчанию или `vertical`), `aria-valuemin` и `aria-valuemax` (`min`, по умолчанию 0, и `max`, по умолчанию 100), `aria-valuenow` — суммарный процент, и `data-p` с ориентацией. `style_class` SHALL добавляться после собственных классов, `style` — задавать встроенный стиль, `pt.root` — добавлять классы, стиль и атрибуты.

#### Scenario: Значения по умолчанию
- **WHEN** отрисован `MeterGroup` без свойств, кроме `value`
- **THEN** корень имеет `role="meter"`, `aria-valuemin="0"`, `aria-valuemax="100"`, класс `p-metergroup-horizontal`

#### Scenario: Суммарное значение
- **WHEN** отрезки имеют значения 20 и 30 при `min = 0`, `max = 100`
- **THEN** `aria-valuenow="50"`

#### Scenario: Вертикальная ориентация
- **WHEN** `orientation = Vertical`
- **THEN** класс корня `p-metergroup-vertical`, `data-p="vertical"`

### Requirement: Отрезки MeterGroup
Внутри `div.p-metergroup-meters` SHALL выводиться `span.p-metergroup-meter` для каждого элемента со значением больше нуля, в исходном порядке; элементы с нулевым значением пропускаются. Процент отрезка SHALL вычисляться как `(value - min) / (max - min) * 100`, ограничиваться диапазоном 0–100 и округляться до целого; при `min == max` он равен 100. Горизонтально процент задаёт ширину, вертикально — высоту; цвет отрезка — фон.

#### Scenario: Ширина
- **WHEN** `min = 0`, `max = 200`, значение элемента 50, цвет `#f00`
- **THEN** отрезок имеет `width: 25%` и `background-color: #f00`

#### Scenario: Ограничение
- **WHEN** значение элемента 150 при `max = 100`, и другое −10
- **THEN** первый отрезок имеет ширину 100%, второй не выводится

#### Scenario: Вертикально
- **WHEN** `orientation = Vertical`, значение 40
- **THEN** отрезок имеет `height: 40%` и не имеет ширины в процентах

#### Scenario: Свой отрезок
- **WHEN** задан шаблон `meter`
- **THEN** вместо стандартного отрезка выводится результат шаблона для каждого элемента

### Requirement: Подписи MeterGroup
По умолчанию SHALL выводиться `ol.p-metergroup-label-list` (с модификатором `-vertical` или `-horizontal` по `label_orientation`, по умолчанию горизонтально) с `li.p-metergroup-label` на каждый элемент: маркер `p-metergroup-label-marker` с цветом фона (или значок `p-metergroup-label-icon`, если у элемента есть `icon`) и текст `p-metergroup-label-text` вида `Метка (процент%)`. При `label_position = End` (по умолчанию) подписи SHALL идти после отрезков, при `Start` — перед ними.

#### Scenario: Подпись
- **WHEN** элемент `{label: "Приложения", value: 16, color: "#34d399"}` при диапазоне 0–100
- **THEN** подпись содержит маркер с фоном `#34d399` и текст `Приложения (16%)`

#### Scenario: Позиция
- **WHEN** `label_position = Start`
- **THEN** список подписей находится перед `p-metergroup-meters`

#### Scenario: Ориентация подписей
- **WHEN** `label_orientation = Vertical`
- **THEN** список подписей имеет класс `p-metergroup-label-list-vertical`

#### Scenario: Значок элемента
- **WHEN** у элемента задан `icon`
- **THEN** вместо маркера выводится значок с цветом элемента

### Requirement: Шаблоны MeterGroup
Шаблон `label` SHALL заменять весь список подписей, `start` и `end` — выводиться до и после отрезков, `icon` — заменять маркер подписи. Шаблоны SHALL получать элементы, суммарный процент и накопленные проценты.

#### Scenario: Свои подписи
- **WHEN** задан шаблон `label`
- **THEN** стандартный список подписей не выводится

### Requirement: Стоимость отрисовки MeterGroup
Число аллокаций на экземпляр при SSR SHALL укладываться в бюджет `metergroup-1000` в `benches/alloc-budgets.toml`, вклад в wasm — в бюджет `metergroup` в `benches/budgets.toml`.

#### Scenario: Бюджеты
- **WHEN** выполняются `cargo xtask alloc-budget` и `cargo xtask wasm-size`
- **THEN** значения MeterGroup не превышают бюджеты
