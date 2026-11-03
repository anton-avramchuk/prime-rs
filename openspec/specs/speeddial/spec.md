# speeddial Specification

## Purpose
SpeedDial показывает плавающую кнопку, которая по нажатию раскрывает набор быстрых действий по прямой или по дуге вокруг неё.

## Requirements

### Requirement: Разметка
`SpeedDial` крейта `prime-speeddial` SHALL выводить `div.p-speeddial.p-component.p-speeddial-<type>` (`type` — `linear`, `circle`, `semi-circle`, `quarter-circle`; для типов кроме `circle` — ещё `p-speeddial-direction-<direction>`; `p-speeddial-open` при показе; `p-disabled` при `disabled`; `style`, `style_class`, `pt`) с кнопкой переключения `button.p-button.p-button-icon-only.p-speeddial-button.p-button-rounded` (`p-speeddial-rotate`, если вращение включено и нет `hide_icon`; `aria-expanded`, `aria-haspopup="true"`, `aria-controls="<id>_list"`, `aria-label`, `aria-labelledby`, `disabled`) со значком «плюс» или `show_icon`/`hide_icon`, и списком `ul.p-speeddial-list[role=menu]` (`id="<id>_list"`, `tabindex="-1"`, `aria-activedescendant` пока список в фокусе и есть пункт под фокусом). Каждый пункт модели SHALL выводиться как `li.p-speeddial-item[role=menuitem]` с `id="<id>_<индекс>"`, `p-hidden` (у `visible = false`), `p-focus`, `data-p-active` и кнопкой `p-speeddial-action` (вторичная, круглая, малая) со значком, `aria-label` из `label` и `disabled`. Идентификатор `id` SHALL по умолчанию быть уникальным. При `mask` и показе SHALL выводиться `div.p-speeddial-mask.p-overlay-mask`.

#### Scenario: Состав
- **WHEN** отрисован `SpeedDial` с тремя пунктами
- **THEN** выведены кнопка и список из трёх `li.p-speeddial-item`, у корня `p-speeddial-linear` и `p-speeddial-direction-up`

#### Scenario: Круг
- **WHEN** задан `type = Circle`
- **THEN** класса `p-speeddial-direction-*` нет

### Requirement: Раскладка
Для `linear` SHALL выставляться только `flex-direction` корня и списка по направлению (`up` — `column-reverse`, `down` — `column`, `left` — `row-reverse`, `right` — `row`). Для `circle` каждый пункт SHALL получить `left` и `top` вида `calc(<x>px + var(--item-diff-x, 0px))` с углом `2π·i/n`; для `semi-circle` — шаг `π/(n-1)` и пару сторон по направлению (`up`: `left`/`bottom`, `down`: `left`/`top`, `left`: `right`/`top`, `right`: `left`/`top`); для `quarter-circle` — шаг `π/(2(n-1))` и пару сторон по диагональному направлению (`up-left`: `right`/`bottom`, `up-right`: `left`/`bottom`, `down-left`: `right`/`top`, `down-right`: `left`/`top`). Радиус SHALL быть `radius`, а при `0` — `20 × n`. Каждый пункт SHALL получать `transition-delay`: при показе `i · transition_delay` мс, при скрытии `(n − i − 1) · transition_delay`. После показа в браузере для типов кроме `linear` SHALL ставиться `--item-diff-x`/`--item-diff-y` списка — половина разницы ширины/высоты кнопки и первого пункта.

#### Scenario: Задержки
- **WHEN** список показан, `transition_delay = 30`, пунктов три
- **THEN** задержки равны 0, 30 и 60 мс

#### Scenario: Полукруг вверх
- **WHEN** `type = SemiCircle`, `direction = Up`, пунктов три, радиус 80
- **THEN** у первого пункта `left` считается от `80·cos 0`, `bottom` — от `80·sin 0`

### Requirement: Показ, команды и закрытие
Клик по кнопке SHALL переключать показ, вызывать `on_click`, `on_visible_change(bool)` и `on_show`/`on_hide`; свойство `visible` SHALL быть двусторонним. Клик по пункту SHALL вызывать `command` (если пункт не отключён), скрывать список и не скрывать его повторно кликом вне. При `hide_on_click_outside` (по умолчанию да) клик вне компонента при показе SHALL скрывать список; слушатель SHALL вешаться на время показа (в следующем тике) и сниматься при скрытии и размонтировании. `Escape` на кнопке и в списке SHALL скрывать список и возвращать фокус на кнопку. Отключённый компонент SHALL не реагировать на клик по кнопке.

#### Scenario: Команда пункта
- **WHEN** показан список и нажат пункт с `command`
- **THEN** `command` вызван, список скрыт

#### Scenario: Клик вне
- **WHEN** список показан и пользователь кликает вне компонента
- **THEN** список скрыт

### Requirement: Клавиатура
На кнопке `ArrowDown` и `ArrowLeft` SHALL открывать список и переводить фокус на следующий пункт, `ArrowUp` и `ArrowRight` — на предыдущий (последний при пустом фокусе), с `preventDefault`. В списке стрелки SHALL вести по пунктам согласно направлению: для `up` ↑ — следующий, ↓ — предыдущий; для `down` наоборот; для `left`, `up-right`, `down-left` ← — следующий, → — предыдущий; для `right`, `up-left`, `down-right` наоборот; для остальных значений ↑ и → — следующий, ↓ и ← — предыдущий. `Home` SHALL вести к первому пункту, `End` — к последнему; `Enter` и `Space` SHALL активировать пункт под фокусом и возвращать фокус на кнопку. Отключённые и скрытые пункты SHALL пропускаться; на границах фокус SHALL оставаться.

#### Scenario: Открытие стрелкой
- **WHEN** на кнопке нажата `ArrowDown`
- **THEN** список показан, первый доступный пункт под фокусом (`p-focus`, `aria-activedescendant`)

#### Scenario: Направление вниз
- **WHEN** `direction = Down` и в списке нажата `ArrowDown`
- **THEN** фокус переходит на следующий пункт

### Requirement: Подсказки и шаблоны
При `tooltip_options` каждый пункт SHALL получать подсказку с текстом `label` и параметрами шаблона. Представление `button_content` SHALL заменять кнопку и получать обратный вызов переключения; `item_content` SHALL заменять кнопку внутри `li` и получать пункт, индекс и обратный вызов клика; `icon_content` SHALL заменять значок кнопки.

#### Scenario: Свой пункт
- **WHEN** задан `item_content`
- **THEN** внутри каждого `li.p-speeddial-item` выведено своё представление вместо кнопки действия
