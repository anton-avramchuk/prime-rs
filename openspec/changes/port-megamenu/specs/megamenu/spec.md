# megamenu Specification

## Purpose
MegaMenu показывает панель пунктов, у каждого из которых раскрывается широкое наложение с колонками подменю.

## ADDED Requirements

### Requirement: Модель
`prime-megamenu` SHALL принимать `Vec<MegaMenuItem>`: корневой пункт имеет общие поля пункта меню и `columns: Vec<Vec<MegaMenuSubmenu>>` (колонка — список подменю); `MegaMenuSubmenu` имеет `label`, `visible`, классы и `items: Vec<prime_menu::MenuItem>`. Пункт с непустыми колонками SHALL считаться группой.

#### Scenario: Группа
- **WHEN** у корневого пункта есть хотя бы одна колонка с подменю
- **THEN** у него выводятся стрелка и наложение

### Requirement: Разметка
`MegaMenu` SHALL выводить `div.p-megamenu.p-component` (`p-megamenu-horizontal` либо `p-megamenu-vertical`; `p-megamenu-mobile`, пока окно не шире `breakpoint`; `p-megamenu-mobile-active` при открытой кнопке; `id`, `style`, `style_class`), слоты `div.p-megamenu-start` и `div.p-megamenu-end`, кнопку `a.p-megamenu-button[role=button]` (только при непустой модели; `aria-haspopup`, `aria-expanded`, `aria-controls`, `aria-label`) и корневой список `ul.p-megamenu-root-list[role=menubar]` (`aria-orientation`, `tabindex`, `aria-activedescendant`). Корневой пункт SHALL быть `li.p-megamenu-item[role=menuitem]` (`data-p-active|focused|disabled`, `aria-level`, `aria-setsize`, `aria-posinset`, `aria-haspopup`, `aria-expanded`, `p-megamenu-item-active`, `p-focus`, `p-disabled`) с `div.p-megamenu-item-content` > `a.p-megamenu-item-link` (значок, подпись, стрелка: вниз в горизонтальной ориентации и на мобильном, вправо в вертикальной) и, у группы, наложением `div.p-megamenu-overlay` > `div.p-megamenu-grid` > колонки `div.p-megamenu-col-N`. Подменю SHALL быть `ul.p-megamenu-submenu[role=menu]` с заголовком `li.p-megamenu-submenu-label[role=presentation]` и пунктами; разделитель — `li.p-megamenu-separator[role=separator]`. Класс колонки SHALL зависеть от числа колонок пункта: 2 → `col-6`, 3 → `col-4`, 4 → `col-3`, 6 → `col-2`, иначе и на мобильном — `col-12`.

#### Scenario: Колонки
- **WHEN** у пункта три колонки
- **THEN** каждая колонка получает класс `p-megamenu-col-4`

#### Scenario: Ориентация
- **WHEN** `orientation = Vertical`
- **THEN** у корня `p-megamenu-vertical`, у списка `aria-orientation="vertical"`, у группы стрелка вправо

### Requirement: Раскрытие мышью
Клик по корневой группе SHALL раскрывать её наложение, повторный клик — закрывать; клик по листу SHALL вызывать `command` (если пункт не отключён) и закрывать наложение. Наведение SHALL раскрывать группу после первого клика/нажатия по меню; на узком окне путь не меняется. Клик вне меню SHALL закрывать наложение (и мобильную панель); `resize` SHALL закрывать его.

#### Scenario: Клик
- **WHEN** пользователь кликает по корневому пункту с колонками
- **THEN** у пункта `p-megamenu-item-active` и `aria-expanded="true"`, наложение показано

#### Scenario: Лист
- **WHEN** пользователь кликает по листу внутри наложения
- **THEN** `command` вызван, наложение закрыто

### Requirement: Клавиатура
Фокус SHALL принадлежать корневому списку (`aria-activedescendant`, `p-focus`). В горизонтальной ориентации ← → SHALL переходить между корневыми пунктами, ↓ — раскрывать группу и входить в первый пункт наложения; внутри наложения ↓ ↑ SHALL идти по пунктам, ← → — между колонками, ↑ на первом пункте — возвращать на родителя; в вертикальной ориентации направления корня меняются местами (↓ ↑ по корню, → раскрывает и входит, ← возвращает). `Home`/`End`, `Enter`/`Space` (клик по пункту), `Escape` (закрыть и вернуть фокус на родителя), `Tab` и поиск по началу подписи SHALL работать как в PrimeNG. Недоступные пункты SHALL пропускаться.

#### Scenario: Вход в наложение
- **WHEN** фокус на раскрываемом корневом пункте и нажата ↓
- **THEN** наложение раскрыто, фокус на первом допустимом пункте первой колонки

#### Scenario: Escape
- **WHEN** фокус внутри наложения и нажата `Escape`
- **THEN** наложение закрыто, фокус на корневом пункте

### Requirement: Мобильный режим
Пока окно не шире `breakpoint`, корневой список SHALL быть скрыт, а кнопка — показывать/скрывать его (`p-megamenu-mobile-active`, `aria-expanded`); открытый список получает слой группы `Menu` (`auto_z_index`, `base_z_index`), закрытие возвращает фокус на кнопку.

#### Scenario: Кнопка
- **WHEN** на узком окне пользователь кликает по кнопке
- **THEN** `p-megamenu-mobile-active` добавлен и `aria-expanded="true"`, повторный клик закрывает

### Requirement: Шаблоны
`start` и `end` SHALL выводиться в своих слотах; `item` SHALL заменять содержимое `div.p-megamenu-item-content`; `submenu_icon` — стрелку группы; `menu_icon` — значок кнопки.

#### Scenario: Слоты
- **WHEN** заданы `start` и `end`
- **THEN** они выведены до кнопки и после списка
