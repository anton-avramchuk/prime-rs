## Purpose
Slider позволяет выбрать число или диапазон из двух чисел между `min` и `max`, перемещая ручку мышью, касанием или клавиатурой.

## ADDED Requirements

### Requirement: Значение и диапазон
Компонент `Slider` крейта `prime-slider` SHALL принимать двусторонний `value` (`RwSignal<f64>`), а при `range` — `values` (`RwSignal<[f64; 2]>`); без сигнала SHALL вести внутреннее состояние, начинающееся с `default_value` (по умолчанию `0`) или `default_values` (по умолчанию `[0, 0]`). Значение SHALL ограничиваться отрезком `min..=max` (по умолчанию `0..=100`). Положение ручки и ширина или высота заполненной части SHALL соответствовать доле значения в отрезке; в диапазоне заполнена часть между ручками независимо от их порядка, а событие и записанное значение SHALL содержать меньшее значение первым.

#### Scenario: Положение
- **WHEN** `min = 0`, `max = 200`, значение `50`
- **THEN** ручка стоит на 25 % полосы, заполнено 25 %

#### Scenario: Диапазон
- **WHEN** `range`, значения `[20, 80]`
- **THEN** две ручки на 20 % и 80 %, заполнена часть между ними

#### Scenario: Внешнее изменение
- **WHEN** сигнал значения меняется
- **THEN** положение ручки и заполнение обновляются

### Requirement: Ввод клавиатурой
Ручка SHALL принимать фокус (`tabindex`, по умолчанию `0`; при `disabled` атрибута нет) и реагировать на клавиши: стрелки вправо и вверх увеличивают значение, влево и вниз уменьшают на `step` или, без него, на 1; `PageUp` и `PageDown` меняют значение на `step` или, без него, на 10 (в диапазоне на 1); `Home` и `End` устанавливают `min` и `max`. Каждая обработанная клавиша SHALL отменять действие браузера по умолчанию и вызывать `on_change` один раз; результат SHALL ограничиваться `min..=max`. В диапазоне клавиша SHALL менять значение ручки, на которой стоит фокус.

#### Scenario: Стрелка
- **WHEN** значение `50` и нажата стрелка вправо
- **THEN** значение `51`, `on_change` вызван один раз

#### Scenario: Шаг
- **WHEN** `step = 20`, значение `20` и нажата стрелка вправо
- **THEN** значение `40`

#### Scenario: Границы
- **WHEN** нажаты `End`, затем стрелка вправо
- **THEN** значение равно `max` и не превышает его

### Requirement: Ввод мышью и касанием
Нажатие на ручку и перемещение указателя SHALL менять значение по положению указателя вдоль полосы (при `orientation = Vertical` снизу вверх; при RTL справа налево), пока кнопка не отпущена, а отпускание SHALL вызывать `on_slide_end`; обработчики перемещения и отпускания SHALL существовать только на время перетаскивания. Клик по полосе SHALL перемещать на указанное место ручку, активную последней (в диапазоне; первая по умолчанию), и вызывать `on_slide_end`; клик по ручке SHALL NOT считаться кликом по полосе. Касание ручки SHALL двигать её на долю смещения пальца вдоль полосы с шагом в 1 % (`onDrag` PrimeNG). При `disabled` все эти действия SHALL не менять значение и не вызывать события. При заданном `step` итоговое значение SHALL получаться, как в PrimeNG, из `handleStepChange`: сдвигом от прежнего значения на целое число шагов в сторону указателя; без `step` значение округляется вниз до целого либо до числа знаков после запятой, как у `step`.

#### Scenario: Клик по полосе
- **WHEN** значение `0` и клик на середине горизонтальной полосы шириной 200 пикселей
- **THEN** значение `50`, `on_change` и `on_slide_end` вызваны

#### Scenario: Перетаскивание
- **WHEN** нажата ручка и указатель сдвинут на четверть полосы вправо, затем кнопка отпущена
- **THEN** значение выросло на 25, `on_slide_end` вызван один раз, на документе не осталось слушателей

#### Scenario: Недоступен
- **WHEN** задан `disabled`
- **THEN** клик, перетаскивание и клавиши не меняют значение и не вызывают события

### Requirement: Структура, классы и доступность
Корень SHALL быть `<div class="p-slider p-component">` с модификаторами `p-disabled`, `p-invalid`, `p-slider-horizontal` или `p-slider-vertical`, `p-slider-animate` (при `animate`, кроме времени перетаскивания), `style_class`; атрибуты `data-pc-name="slider"`, `data-pc-section="root"`, `data-p` (ориентация) и `data-p-sliding` (`true` во время перетаскивания). Внутри SHALL идти `span.p-slider-range` и ручки `span.p-slider-handle` с `position: absolute`: одна с `data-pc-section="handle"`, а в диапазоне две — `startHandler` и `endHandler`, активная с `p-slider-handle-active`. Каждая ручка SHALL иметь `role="slider"`, `aria-valuemin`, `aria-valuenow`, `aria-valuemax`, `aria-orientation`, а `aria-label` и `aria-labelledby` — только если заданы.

#### Scenario: Корень
- **WHEN** `orientation = Vertical`, `disabled`, `invalid`
- **THEN** у корня `p-slider p-component p-disabled p-invalid p-slider-vertical`, `data-p="vertical"`

#### Scenario: Ручка
- **WHEN** значение `30`, `min = 0`, `max = 100`
- **THEN** у ручки `role="slider"`, `aria-valuemin="0"`, `aria-valuenow="30"`, `aria-valuemax="100"`, `aria-orientation="horizontal"`

#### Scenario: Недоступна
- **WHEN** задан `disabled`
- **THEN** у ручки нет `tabindex`

### Requirement: Свойства и pass-through
Свойства `min`, `max`, `step`, `orientation`, `animate`, `range`, `disabled`, `invalid`, `tabindex`, `aria_label`, `aria_labelledby`, `style`, `style_class` SHALL принимать значение или сигнал и обновлять разметку при смене сигнала. `pt` SHALL принимать части `root`, `range`, `handle`, `start_handler`, `end_handler`, добавляя классы, стиль и атрибуты без замены собственных.

#### Scenario: Реактивные границы
- **WHEN** `max` передан сигналом и меняется
- **THEN** положение ручки пересчитывается

#### Scenario: Класс через pt
- **WHEN** в `pt.handle` передан класс `custom`
- **THEN** у ручки есть `custom` и `p-slider-handle`

### Requirement: Стоимость отрисовки Slider
Число аллокаций на экземпляр при SSR SHALL укладываться в бюджет `slider-1000` в `benches/alloc-budgets.toml`, вклад в wasm — в бюджет `slider` в `benches/budgets.toml`.

#### Scenario: Бюджеты
- **WHEN** выполняются `cargo xtask alloc-budget` и `cargo xtask wasm-size`
- **THEN** значения Slider не превышают бюджеты
