# panelmenu Specification

## Purpose
PanelMenu показывает вертикальный список панелей-заголовков с раскрывающимися вложенными ветками (гибрид Accordion и Tree).

## ADDED Requirements

### Requirement: Разметка
`PanelMenu` крейта `prime-panelmenu` SHALL принимать `prime_menu::MenuItem` и выводить `div.p-panelmenu.p-component` (`id`, `style`, `style_class`). Каждый видимый корневой пункт (`visible != false`) SHALL выводиться как `div.p-panelmenu-panel` с заголовком `div.p-panelmenu-header` (`p-panelmenu-header-active` у раскрытой группы, `p-disabled` у отключённого; `aria-expanded`, `aria-controls`, `aria-disabled`, `data-p-highlight`) > `div.p-panelmenu-header-content` > `a.p-panelmenu-header-link` (шеврон `p-panelmenu-submenu-icon` у группы — вправо свёрнутой, вниз раскрытой; `p-panelmenu-header-icon`; `p-panelmenu-header-label`) и с областью `div.p-panelmenu-content-container` (`p-panelmenu-expanded` у раскрытой) > `div.p-panelmenu-content-wrapper` > `div.p-panelmenu-content` > `ul.p-panelmenu-root-list[role=tree]` (`tabindex`, `aria-activedescendant`, `aria-hidden`). Вложенные пункты SHALL выводиться как `li.p-panelmenu-item[role=treeitem]` (`aria-label`, `aria-level`, `aria-setsize`, `aria-posinset`, `aria-expanded` у групп, `data-p-focused`, `data-p-disabled`, `p-focus`, `p-disabled`) > `div.p-panelmenu-item-content` > `a.p-panelmenu-item-link` (`p-panelmenu-item-icon`, `p-panelmenu-item-label`, шеврон группы) и вложенным `ul.p-panelmenu-submenu`; разделители — `li.p-menuitem-separator[role=separator]`.

#### Scenario: Состав
- **WHEN** отрисовано меню с группой из двух уровней
- **THEN** выведены панели, заголовки, области содержимого и деревья с `role=tree`/`treeitem` и `aria-level` по глубине

#### Scenario: Скрытые пункты
- **WHEN** у пункта `visible = Some(false)`
- **THEN** его нет в разметке

### Requirement: Раскрытие
Клик по заголовку группы или по пункту-группе SHALL переключать её раскрытие. Начальное состояние SHALL браться из `expanded` пунктов. При `multiple = false` (по умолчанию) раскрытие корневой панели SHALL сворачивать прежнюю раскрытую корневую панель; при `multiple = true` панели независимы. Вложенные ветки SHALL быть независимы: раскрытие одной не сворачивает сестёр. Раскрытие и сворачивание SHALL анимироваться через `prime_core::collapsible`.

#### Scenario: Одиночный режим
- **WHEN** раскрыта панель A, пользователь кликает по заголовку B
- **THEN** B раскрыта, A свёрнута

#### Scenario: Множественный режим
- **WHEN** `multiple = true` и раскрыта A, клик по B
- **THEN** раскрыты обе

#### Scenario: Начальное состояние
- **WHEN** у пункта `expanded = Some(true)`
- **THEN** он выведен раскрытым без анимации

### Requirement: Команды и отключённые пункты
Клик по листу SHALL вызывать `command` пункта, если он не отключён; клик по отключённому пункту или заголовку SHALL ничего не делать. Отключённые пункты SHALL пропускаться при навигации.

#### Scenario: Команда
- **WHEN** пользователь кликает по листу с `command`
- **THEN** `command` вызван один раз с пунктом

#### Scenario: Отключённый
- **WHEN** пользователь кликает по отключённому заголовку
- **THEN** раскрытие не меняется

### Requirement: Управляемое раскрытие
`PanelMenuRef` SHALL позволять раскрыть все группы, свернуть все и переключить все (`expand_all`, `collapse_all`, `toggle_all`); состояние SHALL храниться по ключам пунктов (`key` или позиция), чтобы демонстрация `controlled` была воспроизводима.

#### Scenario: Раскрыть всё
- **WHEN** вызван `expand_all`
- **THEN** все группы раскрыты и `aria-expanded="true"`

### Requirement: Клавиатура
Заголовок панели SHALL быть фокусируемым (`tabindex="0"`): ↓ SHALL переводить в дерево раскрытой панели, иначе на следующий заголовок; ↑ — на предыдущий (в его дерево, если он раскрыт); `Home`/`End` — на первый/последний заголовок; `Enter`/`Space` — переключать панель. Отключённые заголовки пропускаются, переход идёт по кругу. В дереве панели (подсветка — `aria-activedescendant` и `p-focus`) ↓/↑ SHALL переходить к следующему/предыдущему видимому допустимому пункту, а за краями — к соседнему заголовку / заголовку своей панели; → SHALL раскрывать ветку, а на раскрытой — переходить к первому потомку; ← SHALL сворачивать раскрытую ветку, иначе переходить к родителю; `Home`/`End` — к первому/последнему пункту; `Enter`/`Space` — выполнять клик по пункту; печатные символы — поиск по началу подписи. `Tab` SHALL уводить фокус из компонента.

#### Scenario: Стрелки
- **WHEN** фокус на раскрытом пункте-группе и нажата →, затем ↓
- **THEN** активен первый потомок, затем второй

#### Scenario: Сворачивание
- **WHEN** фокус на раскрытой группе и нажата ←
- **THEN** группа свёрнута

#### Scenario: Enter
- **WHEN** фокус на листе с `command` и нажат `Enter`
- **THEN** `command` вызван

### Requirement: Шаблоны
`item` SHALL заменять содержимое `div.p-panelmenu-item-content`; `submenu_icon` — шеврон группы. Заголовок корневой панели SHALL оставаться стандартным.

#### Scenario: Свой пункт
- **WHEN** задан `item`
- **THEN** в `p-panelmenu-item-content` выведено его содержимое вместо ссылки
