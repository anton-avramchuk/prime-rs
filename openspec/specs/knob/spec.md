# knob Specification

## Purpose
Knob позволяет ввести число от `min` до `max`, поворачивая круговую шкалу мышью, касанием или клавишами.

## Requirements

### Requirement: Значение и шкала
Компонент `Knob` крейта `prime-knob` SHALL принимать двусторонний `value` (`RwSignal<f64>`); без него SHALL вести внутреннее состояние, начинающееся с `default_value`, а при его отсутствии с `min`. Шкала SHALL быть дугой от нижнего левого угла к нижнему правому через верх (от −120° до +120° относительно верха, разрыв внизу): `path.p-knob-range` рисует всю дугу, `path.p-knob-value` — дугу от нуля шкалы (при `min > 0` от `min`) до значения. Положение конца дуги значения SHALL соответствовать доле значения в отрезке `min..max`; значение вне отрезка SHALL отображаться прижатым к ближайшему краю, но в тексте и `aria-valuenow` показываться как есть. При `show_value` внутри SHALL выводиться `text.p-knob-text` со значением, подставленным в `value_template` вместо первого `{value}`.

#### Scenario: Середина шкалы
- **WHEN** `min = 0`, `max = 100`, значение `50`
- **THEN** конец дуги значения лежит на вертикальной оси над центром

#### Scenario: Шаблон
- **WHEN** `value_template = "{value}%"`, значение `60`
- **THEN** текст равен «60%»

#### Scenario: Внешнее изменение
- **WHEN** сигнал значения меняется
- **THEN** дуга значения, текст и `aria-valuenow` обновляются

### Requirement: Ввод клавиатурой
SVG SHALL принимать фокус (`tabindex`, по умолчанию `0`; при `readonly` и `disabled` равен `-1`) и реагировать на клавиши: стрелки вправо и вверх увеличивают значение на 1, влево и вниз уменьшают на 1, `PageUp` и `PageDown` меняют на 10, `Home` и `End` устанавливают `min` и `max`; результат SHALL ограничиваться отрезком `min..=max`. Каждая обработанная клавиша SHALL отменять действие браузера по умолчанию и вызывать `on_change` один раз. При `readonly` и `disabled` клавиши SHALL не менять значение.

#### Scenario: Стрелка
- **WHEN** значение `50` и нажата стрелка вправо
- **THEN** значение `51`, `on_change` вызван один раз со значением `51`

#### Scenario: Границы
- **WHEN** нажаты `End`, затем стрелка вверх
- **THEN** значение равно `max` и не превышает его

#### Scenario: Только чтение
- **WHEN** задан `readonly` и нажата стрелка
- **THEN** значение и `on_change` не затронуты

### Requirement: Ввод мышью и касанием
Клик по шкале SHALL устанавливать значение по углу точки клика относительно центра SVG; нажатие кнопки мыши и перемещение указателя SHALL менять значение по углу, пока кнопка не отпущена, а обработчики перемещения и отпускания SHALL существовать только на время перетаскивания (для касания — перемещения и окончания касания, при одном касании). Точки в разрыве внизу шкалы SHALL игнорироваться. Угол SHALL отображаться на значение с округлением до ближайшего шага `step` от `min` и ограничением `min..=max`. Нажатие SHALL ставить фокус на SVG. При `readonly` и `disabled` указатель SHALL не менять значение и не вызывать `on_change`.

#### Scenario: Клик
- **WHEN** клик по верхней точке шкалы при `0..100`
- **THEN** значение около 50

#### Scenario: Разрыв
- **WHEN** клик по нижней точке шкалы, в разрыве
- **THEN** значение не меняется

#### Scenario: Перетаскивание
- **WHEN** нажата шкала, указатель перемещён по дуге, кнопка отпущена
- **THEN** значение следует за указателем, а после отпускания перемещение значение не меняет

#### Scenario: Шаг
- **WHEN** `step = 10` и клик на углу, соответствующем значению 47
- **THEN** значение равно 50

### Requirement: Структура, классы и доступность
Корень SHALL быть `<div class="p-knob p-component">` с `p-disabled` при `disabled` и `style_class`. SVG SHALL иметь `viewBox="0 0 100 100"`, `role="slider"`, ширину и высоту `size` пикселей, `aria-valuemin`, `aria-valuemax`, `aria-valuenow`, `required` (только при `required`), `aria-label` и `aria-labelledby` (только заданные). Дуги SHALL иметь `stroke-width` из `stroke_width` и `stroke` из `range_color` и `value_color`, текст — `fill` из `text_color`, `x="50"`, `y="57"`, `text-anchor="middle"` и `name` (если задан); цвета по умолчанию — переменные темы `--p-knob-range-background`, `--p-knob-value-background`, `--p-knob-text-color`.

#### Scenario: Корень
- **WHEN** `disabled`
- **THEN** у корня `p-knob p-component p-disabled`, у SVG `tabindex="-1"`

#### Scenario: Размер и цвета
- **WHEN** `size = 200`, `stroke_width = 5`, `value_color = "SlateGray"`
- **THEN** SVG шириной и высотой 200 px, `stroke-width="5"` у дуг, `stroke="SlateGray"` у дуги значения

#### Scenario: Подписи
- **WHEN** `aria_label = "Volume"`, `min = 0`, `max = 100`, значение `30`
- **THEN** у SVG `aria-label="Volume"`, `aria-valuemin="0"`, `aria-valuemax="100"`, `aria-valuenow="30"`

### Requirement: Свойства и pass-through
Свойства `min`, `max`, `step`, `size`, `stroke_width`, `show_value`, `value_template`, цвета, `readonly`, `disabled`, `required`, `name`, `tabindex`, `aria_label`, `aria_labelledby`, `style`, `style_class` SHALL принимать значение или сигнал и обновлять разметку при смене сигнала. `pt` SHALL принимать части `root`, `svg`, `range`, `value`, `text`, добавляя классы, стиль и атрибуты без замены собственных.

#### Scenario: Реактивные границы
- **WHEN** `max` передан сигналом и меняется
- **THEN** дуга значения пересчитывается

#### Scenario: Класс через pt
- **WHEN** в `pt.text` передан класс `custom`
- **THEN** у текста есть `custom` и `p-knob-text`

### Requirement: Стоимость отрисовки Knob
Число аллокаций на экземпляр при SSR SHALL укладываться в бюджет `knob-1000` в `benches/alloc-budgets.toml`, вклад в wasm — в бюджет `knob` в `benches/budgets.toml`.

#### Scenario: Бюджеты
- **WHEN** выполняются `cargo xtask alloc-budget` и `cargo xtask wasm-size`
- **THEN** значения Knob не превышают бюджеты
