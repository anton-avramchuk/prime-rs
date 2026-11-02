# menu Specification

## Purpose
Menu показывает список команд и ссылок с группами и разделителями, статичный или всплывающий у элемента-цели, и управляется мышью и клавишами.

## Requirements

### Requirement: Разметка списка
`Menu` крейта `prime-menu` SHALL выводить `div.p-menu.p-component` с `data-p`, внутри — `ul.p-menu-list[role=menu]` с `tabindex`, `aria-label`, `aria-labelledby`, `aria-activedescendant` (пока есть фокус) и пунктами `li.p-menu-item[role=menuitem]` с `id`, `aria-label`, `aria-disabled` и `data-p-disabled`, в которых `div.p-menu-item-content` > `a.p-menu-item-link` (`tabindex=-1`, `href` из `url`, `target`, `title`) со значком `span.p-menu-item-icon` (если есть `icon`) и `span.p-menu-item-label`. `style_class`, `style` и `pt` SHALL применяться к корню; `style_class`/`style` пункта — к `li`.

#### Scenario: Простая модель
- **WHEN** отрисован `Menu` с пунктами «New» и «Search»
- **THEN** выведены два `li.p-menu-item` с подписями в `.p-menu-item-label`

#### Scenario: Пункт со ссылкой
- **WHEN** у пункта задан `url`
- **THEN** `a.p-menu-item-link` имеет этот `href`

### Requirement: Группы и разделители
Пункт с `items` SHALL выводить `li.p-menu-submenu-label[role=none]` с подписью и затем свои подпункты; пункт с `separator` SHALL выводиться как `li.p-menu-separator[role=separator]`. Невидимые (`visible=false`) пункты и группы SHALL не выводиться.

#### Scenario: Группа
- **WHEN** модель содержит группу «Documents» с двумя пунктами
- **THEN** выведены заголовок «Documents» и два `li.p-menu-item`

#### Scenario: Невидимый пункт
- **WHEN** у пункта `visible=false`
- **THEN** его нет в разметке

### Requirement: Команда пункта
Клик по пункту SHALL вызывать его `command` с событием и пунктом; у пункта без `url` переход по ссылке SHALL отменяться. Клик по отключённому пункту SHALL отменять переход и не вызывать `command`. Пункт с `disabled` SHALL иметь `p-disabled`.

#### Scenario: Команда
- **WHEN** пользователь кликает по пункту с `command`
- **THEN** `command` вызван один раз с этим пунктом

#### Scenario: Отключённый пункт
- **WHEN** пользователь кликает по пункту с `disabled`
- **THEN** `command` не вызван

### Requirement: Навигация клавишами
Список SHALL принимать фокус; в статичном режиме при получении фокуса фокусируется первый доступный пункт (`p-focus`, `aria-activedescendant`). ↓/↑ SHALL переходить к следующему/предыдущему доступному пункту (отключённые пропускаются), Home/End — к первому/последнему, Enter и Space — активировать пункт (клик по его ссылке). При потере фокуса `p-focus` и `aria-activedescendant` SHALL сниматься. Вызывать `on_focus`/`on_blur`.

#### Scenario: Стрелка вниз
- **WHEN** список в фокусе на первом пункте и нажата ↓
- **THEN** `p-focus` у второго доступного пункта

#### Scenario: Enter
- **WHEN** сфокусирован пункт с `command` и нажат Enter
- **THEN** `command` вызван

### Requirement: Всплывающий режим
При `popup` меню SHALL не выводиться, пока оно не показано; `MenuRef::toggle(event)`/`show(event)` SHALL показывать его у `currentTarget` события (`position: absolute`, класс `p-menu-overlay`, выравнивание по цели), фокусировать список и вызывать `on_show`; `hide()`, клик по пункту, клик вне меню и цели, resize окна (кроме сенсорных устройств) и прокрутка вложенного предка цели SHALL скрывать его с вызовом `on_hide`. Escape и Tab SHALL скрывать меню и возвращать фокус на цель; Alt+↑ — то же.

#### Scenario: Показ
- **WHEN** вызван `toggle` от клика по кнопке
- **THEN** появился `.p-menu.p-menu-overlay` рядом с кнопкой, список в фокусе

#### Scenario: Клик вне
- **WHEN** меню показано, пользователь кликает вне него и вне цели
- **THEN** меню скрыто

#### Scenario: Escape
- **WHEN** меню показано и нажат Escape
- **THEN** меню скрыто, фокус на цели

### Requirement: Слой
При `auto_z_index` показанное всплывающее меню SHALL получать `z-index` группы `Menu` со смещением `base_z_index`; после скрытия индекс SHALL освобождаться.

#### Scenario: Слой
- **WHEN** меню показано, затем скрыто
- **THEN** `z-index` был задан и освобождён

### Requirement: Свои представления
`item` SHALL заменять содержимое `div.p-menu-item-content` пункта и получать пункт; `submenu_header` — содержимое заголовка группы; `start` и `end` — выводиться в `div.p-menu-start` до списка и `div.p-menu-end` после.

#### Scenario: Шаблон пункта
- **WHEN** задан `item`
- **THEN** внутри `.p-menu-item-content` только переданное представление

### Requirement: Рендер без браузера
Вне браузера (SSR) статичный `Menu` SHALL отрисовываться полностью, всплывающий — не выводиться; `window` и `document` SHALL не использоваться.

#### Scenario: SSR
- **WHEN** страница со статичным `Menu` отрисована на сервере
- **THEN** в разметке есть `ul.p-menu-list` с пунктами
