# contextmenu Specification

## Purpose
ContextMenu показывает меню с вложенными подменю у курсора по правому клику (или долгому касанию) на цели или на всём документе.

## ADDED Requirements

### Requirement: Разметка
`ContextMenu` крейта `prime-contextmenu` SHALL принимать пункты `prime_menu::MenuItem` и, пока меню показано, выводить `div.p-contextmenu.p-component` (`p-contextmenu-mobile`, пока окно не шире `breakpoint`; `id`, `style`, `style_class`, `pt`) с корневым списком `ul.p-contextmenu-root-list[role=menu]` (`tabindex`, `aria-label`, `aria-labelledby`, `aria-activedescendant`). Пункты, группы, разделители, `aria-*` и `data-p-*` SHALL выводиться так же, как у `TieredMenu`, с классами `p-contextmenu-item`, `-item-content`, `-item-link`, `-item-icon`, `-item-label`, `-submenu-icon`, `-submenu`, `-separator`. Пока меню не показано, в DOM не должно быть панели.

#### Scenario: Скрыто по умолчанию
- **WHEN** меню отрисовано и не показывалось
- **THEN** `div.p-contextmenu` в DOM отсутствует

#### Scenario: Состав
- **WHEN** меню показано, пункты — два, один из них группа
- **THEN** выведены `li.p-contextmenu-item`, у группы стрелка и `aria-haspopup="menu"`

### Requirement: Открытие
Меню SHALL показываться по событию `trigger_event` (по умолчанию `contextmenu`) на `target` либо, при `global`, на документе; на сенсорных устройствах — после касания дольше `press_delay` (500 мс), отпускание раньше отменяет показ. Показ SHALL отменять действие по умолчанию и всплытие события, сбрасывать путь и фокус, вызывать `on_show` и переводить фокус на корневой список. Меню SHALL позиционироваться в (`pageX + 1`, `pageY + 1`); если оно не помещается справа/снизу — сдвигаться на свою ширину/высоту влево/вверх; затем прижиматься к левому/верхнему краю окна. Повторное событие при показанном меню SHALL только пересчитывать положение. Слушатели триггера SHALL сниматься при размонтировании.

#### Scenario: Правый клик
- **WHEN** пользователь нажимает правую кнопку на `target`
- **THEN** меню показано у курсора, контекстное меню браузера не открывается

#### Scenario: У края окна
- **WHEN** правый клик сделан у правого нижнего угла окна
- **THEN** меню целиком в пределах окна

### Requirement: Закрытие и ref
`ContextMenuRef` SHALL предоставлять `show(event)`, `hide`, `toggle(event)`, `visible`. Меню SHALL закрываться кликом вне меню (кроме правой кнопки и клика с Ctrl), изменением размера окна, выбором листа, `Escape` и `Tab`; закрытие SHALL сбрасывать путь и фокус и вызывать `on_hide`. Слушатели окна/документа SHALL вешаться на время показа (клик — в следующем тике) и сниматься при скрытии и размонтировании. Панель SHALL получать слой группы `Menu` (`auto_z_index`, `base_z_index`).

#### Scenario: Клик вне
- **WHEN** меню показано и пользователь кликает левой кнопкой вне меню
- **THEN** меню скрыто, вызван `on_hide`

#### Scenario: Лист
- **WHEN** пользователь кликает по листу
- **THEN** вызван `command`, меню скрыто

### Requirement: Подменю и клавиатура
Раскрытие подменю мышью (клик, наведение с `dirty`), положение справа/слева по месту и клавиатура (↓ ↑ → ← `Home` `End` `Enter` `Space` `Escape` `Tab`, поиск по символам со сбросом через 500 мс) SHALL работать как у `TieredMenu`.

#### Scenario: Вправо
- **WHEN** фокус на группе и нажата →
- **THEN** подменю раскрыто, фокус на его первом допустимом пункте

### Requirement: Шаблоны
`item` SHALL заменять содержимое `div.p-contextmenu-item-content`, `submenu_icon` — стрелку группы.

#### Scenario: Свой пункт
- **WHEN** задан `item`
- **THEN** внутри `div.p-contextmenu-item-content` каждого пункта своё представление
