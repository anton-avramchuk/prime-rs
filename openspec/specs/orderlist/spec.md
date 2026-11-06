# orderlist Specification

## Purpose
OrderList — список с изменяемым порядком элементов.

## Requirements

### Requirement: Перестановка
Модель SHALL переставлять выбранные элементы, сохраняя их взаимный порядок и порядок остальных: `move_up` сдвигает каждый выбранный на одну позицию вверх, если выше не выбранный элемент; `move_down` — вниз; `move_top` и `move_bottom` переносят все выбранные в начало и в конец. Элемент, уже стоящий у края (или упирающийся в выбранный блок у края), SHALL оставаться на месте.

#### Scenario: Вверх
- **WHEN** список `A B C D`, выбраны `C`
- **THEN** результат `A C B D`

#### Scenario: Блок у края
- **WHEN** список `A B C`, выбраны `A B`, `move_up`
- **THEN** порядок не меняется

#### Scenario: Несколько
- **WHEN** список `A B C D E`, выбраны `B D`, `move_up`
- **THEN** результат `B A D C E`

#### Scenario: В конец
- **WHEN** список `A B C D`, выбраны `A C`, `move_bottom`
- **THEN** результат `B D A C`

### Requirement: Разметка
`OrderList` SHALL выводить `div.p-orderlist.p-component` (`p-orderlist-controls-left` или `-right` по `controls_position`) с `div.p-orderlist-controls` (четыре кнопки в порядке «вверх», «в начало», «вниз», «в конец» с `aria-label` «Move Up», «Move Top», «Move Down», «Move Bottom») и списком `Listbox` с множественным выбором. Кнопки SHALL быть недоступны, если нет выбранных элементов или `disabled`.

#### Scenario: Без выбора
- **WHEN** ничего не выбрано
- **THEN** все четыре кнопки `disabled`

### Requirement: Действия
Клик по кнопке SHALL менять порядок значения `value` и вызывать `on_reorder` с выбранными элементами; выбор SHALL сохраняться; поиск `filter` SHALL фильтровать только отображение, перестановка идёт по полному списку.

#### Scenario: Клик
- **WHEN** выбран второй элемент, клик «вверх»
- **THEN** он первый в `value`, выбор прежний, вызван `on_reorder`
