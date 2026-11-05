# colorpicker Specification

## Purpose
ColorPicker — выбор цвета на панели с полем насыщенности/яркости и шкалой оттенка.

## ADDED Requirements

### Requirement: Преобразования цветов
Модуль SHALL переводить между HSB, RGB и HEX по формулам PrimeNG: `h` 0–360, `s`, `b` 0–100, `r`, `g`, `b` 0–255 (округление до целого); HEX короче шести цифр SHALL дополняться нулями слева; неразбираемая строка SHALL давать чёрный.

#### Scenario: HSB в HEX
- **WHEN** HSB `{h: 0, s: 100, b: 100}`
- **THEN** HEX `ff0000`

#### Scenario: HEX в HSB
- **WHEN** HEX `#00ff00`
- **THEN** HSB `{h: 120, s: 100, b: 100}`

#### Scenario: Ограничение
- **WHEN** HSB `{h: 400, s: -5, b: 120}`
- **THEN** результат ограничения `{h: 360, s: 0, b: 100}`

### Requirement: Разметка
`ColorPicker` SHALL выводить `div.p-colorpicker.p-component` (`p-colorpicker-overlay` без `inline`) с `input.p-colorpicker-preview` (`readonly`, `type=text`, `tabindex`, `id`, `aria-label`, `disabled` и `p-disabled` при блокировке; цвет фона — текущий цвет) и панелью `div.p-colorpicker-panel` (`p-colorpicker-panel-inline` при `inline`, `p-disabled` при блокировке) > `.p-colorpicker-content` > `.p-colorpicker-color-selector` (> `.p-colorpicker-color-background` > `.p-colorpicker-color-handle`) и `.p-colorpicker-hue` (> `.p-colorpicker-hue-handle`). При `inline` образца нет и панель видна всегда; иначе панель выводится только при открытии.

#### Scenario: Закрытый
- **WHEN** `inline = false`, панель закрыта
- **THEN** в DOM есть образец, нет панели

#### Scenario: Встроенный
- **WHEN** `inline = true`
- **THEN** нет образца, панель присутствует с `p-colorpicker-panel-inline`

### Requirement: Значение и формат
Значение SHALL быть `ColorValue` по `format`: `hex` — строка `#rrggbb`, `rgb` — `{r, g, b}`, `hsb` — `{h, s, b}`. Пустое значение SHALL показывать `default_color` (по умолчанию `ff0000`). Внешнее изменение `value` SHALL обновлять образец и положение указателей; указатели стоят в `left = floor(150·s/100)` px, `top = floor(150·(100−b)/100)` px (поле) и `top = floor(150 − 150·h/360)` px (шкала); фон поля — цвет с `s = 100, b = 100` текущего оттенка.

#### Scenario: Формат rgb
- **WHEN** `format = rgb`, пользователь выбирает чистый красный
- **THEN** значение `Rgb {r: 255, g: 0, b: 0}`

#### Scenario: Положение указателей
- **WHEN** значение HSB `{h: 0, s: 100, b: 100}`
- **THEN** указатель поля в `left: 150px; top: 0px`, указатель шкалы в `top: 0px`

### Requirement: Открытие и закрытие
Клик по образцу и клавиша `Space` SHALL переключать панель (`Space` с `preventDefault`); `Escape` и `Tab` на образце SHALL закрывать; клик вне корня и resize окна SHALL закрывать; `ColorPickerRef::show`/`hide` — программно. `on_show` и `on_hide` SHALL вызываться при реальной смене состояния; при `disabled` образец не открывает панель.

#### Scenario: Space
- **WHEN** фокус на образце, нажат `Space`
- **THEN** панель открыта; повторное нажатие закрывает

#### Scenario: Клик снаружи
- **WHEN** панель открыта, клик вне корня
- **THEN** панель закрыта, вызван `on_hide`

### Requirement: Выбор цвета
`mousedown` и `touchstart` по полю SHALL задавать `s = floor(100·clamp(x, 0, 150)/150)` и `b = floor(100·(150 − clamp(y, 0, 150))/150)`; по шкале — `h = floor(360·(150 − clamp(y, 0, 150))/150)` (x, y — от левого верхнего угла области). Жест SHALL продолжаться при движении мыши по окну до `mouseup` (для касания — `touchmove`/`touchend` области, с `preventDefault`). Каждое изменение SHALL обновлять значение и вызывать `on_change`; во время жеста корень SHALL иметь `p-colorpicker-dragging`. При `disabled` жесты игнорируются.

#### Scenario: Клик по полю
- **WHEN** клик в правом верхнем углу поля
- **THEN** `s = 100`, `b = 100`

#### Scenario: Перетаскивание
- **WHEN** `mousedown` по шкале и движение мыши ниже её нижней границы
- **THEN** оттенок ограничен `0`; после `mouseup` движение значение не меняет

#### Scenario: Заблокирован
- **WHEN** `disabled`, `mousedown` по полю
- **THEN** значение и `on_change` не затронуты
