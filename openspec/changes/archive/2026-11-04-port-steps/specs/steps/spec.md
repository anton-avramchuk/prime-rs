# steps Specification

## Purpose
Steps показывает нумерованные шаги мастера в ряд, отмечает текущий и при необходимости позволяет переходить между шагами.

## ADDED Requirements

### Requirement: Разметка
`Steps` крейта `prime-steps` SHALL выводить `nav.p-steps.p-component` (`p-readonly` при `readonly`; `style`, `style_class`, `pt`; `data-pc-name="steps"`) > `ul.p-steps-list` > `li.p-steps-item` на каждый пункт модели (`p-steps-item-active` у активного, `p-disabled` у отключённого, `aria-current="step"` у активного, `id`, `style`) > `a.p-steps-item-link[role=link]` (`href` из `url`, `target`, `tabindex`, `aria-expanded`, `aria-disabled`) с `span.p-steps-item-number` (номер пункта, считая с единицы) и `span.p-steps-item-label` (подпись). Пункт SHALL считаться отключённым, если у него `disabled` или включён `readonly`, а пункт не активен. Ссылка SHALL иметь `tabindex="-1"` у отключённого пункта, `tabindex` пункта (или `0`) у активного и `tabindex` пункта (или `-1`) у остальных; `aria-expanded` SHALL быть истинным у активного пункта, `aria-disabled` — равным признаку отключённости (у пункта с `disabled` или у неактивного при `readonly`).

#### Scenario: Состав
- **WHEN** отрисован `Steps` из трёх пунктов с `active_index = 1`
- **THEN** выведены три `li`, у второго `p-steps-item-active` и `aria-current="step"`, номера 1, 2, 3

#### Scenario: Только чтение
- **WHEN** `readonly = true` (по умолчанию)
- **THEN** у корня `p-readonly`, у неактивных пунктов `p-disabled` и `aria-disabled="true"`

### Requirement: Клик
В режиме `readonly` или по отключённому пункту клик SHALL отменять переход по ссылке и ничего не вызывать. В интерактивном режиме клик по доступному пункту SHALL вызывать `on_active_index_change(индекс)`, обновлять переданный сигнал `active_index_signal` (если задан), отменять переход по ссылке, если у пункта нет `url`, и вызывать `command` пункта с событием, пунктом и индексом.

#### Scenario: Интерактивный клик
- **WHEN** `readonly = false` и пользователь кликает по третьему пункту
- **THEN** вызваны `on_active_index_change(2)` и `command` с индексом 2

#### Scenario: Отключённый пункт
- **WHEN** пользователь кликает по пункту с `disabled`
- **THEN** ничего не вызывается

### Requirement: Клавиатура
На ссылках пунктов → и ← SHALL переносить фокус на следующий и предыдущий пункты, `Home` и `End` — на первый и последний; фокус SHALL перемещаться и `tabindex="0"` переходить к сфокусированной ссылке (прежняя получает `-1`), на границах фокус остаётся. `Tab` SHALL возвращать `tabindex="0"` ссылке активного пункта. `Enter` и `Space` SHALL действовать как клик. Обработанные клавиши SHALL отменять действие по умолчанию.

#### Scenario: Стрелка вправо
- **WHEN** фокус на первой ссылке и нажата →
- **THEN** фокус и `tabindex="0"` у второй ссылки, у первой `tabindex="-1"`

#### Scenario: Конец списка
- **WHEN** фокус на последнем пункте и нажата →
- **THEN** фокус остаётся на нём
