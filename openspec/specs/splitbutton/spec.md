# splitbutton Specification

## Purpose
SplitButton объединяет кнопку с действием по умолчанию и кнопку-стрелку, которая раскрывает всплывающее меню дополнительных команд.

## Requirements

### Requirement: Разметка
`SplitButton` крейта `prime-splitbutton` SHALL выводить `div.p-splitbutton.p-component` с `data-p-severity` (при `severity`), модификаторами `p-splitbutton-raised`, `p-splitbutton-rounded`, `p-splitbutton-outlined`, `p-splitbutton-text`, `p-splitbutton-sm`/`p-splitbutton-lg` (по `size`), `style`, `style_class` и `pt`. Внутри в порядке: основная кнопка `button.p-button.p-splitbutton-button` (метка `label`, значок `icon` с `icon_pos`), кнопка раскрытия `button.p-button.p-splitbutton-dropdown.p-button-icon-only` со значком стрелки или `dropdown_icon`, всплывающее меню. Основная и дополнительная кнопки SHALL наследовать `severity`, `text`, `outlined`, `size` и `plain` от `SplitButton`; `raised` и `rounded` SHALL задаваться классами корня.

#### Scenario: Состав
- **WHEN** отрисован `SplitButton` с `label = "Save"` и `model`
- **THEN** выведены две кнопки: «Save» с классом `p-splitbutton-button` и стрелка с классом `p-splitbutton-dropdown`

#### Scenario: Модификаторы
- **WHEN** заданы `raised`, `rounded`, `size = small`, `severity = success`
- **THEN** корень имеет классы `p-splitbutton-raised`, `p-splitbutton-rounded`, `p-splitbutton-sm` и `data-p-severity="success"`, а обе кнопки — класс `p-button-success`

### Requirement: Основная кнопка
Клик по основной кнопке SHALL вызывать `on_click` с событием и закрывать меню, если оно открыто. `content` SHALL заменять содержимое основной кнопки (вместо метки и значка). `disabled` и `button_disabled` SHALL делать основную кнопку недоступной (`disabled`); `tabindex`, `autofocus` и `button_aria_label` SHALL применяться к ней.

#### Scenario: Клик
- **WHEN** пользователь кликает по основной кнопке при открытом меню
- **THEN** вызван `on_click`, меню закрыто

#### Scenario: Недоступность
- **WHEN** задан `button_disabled`
- **THEN** основная кнопка имеет `disabled`, а кнопка раскрытия остаётся доступной

### Requirement: Кнопка раскрытия
Кнопка раскрытия SHALL иметь `aria-haspopup="true"`, `aria-expanded` (`true` пока меню открыто), `aria-controls` (идентификатор меню) и `aria-label` из `expand_aria_label`. Клик по ней SHALL вызывать `on_dropdown_click` и переключать меню; нажатие ↓ или ↑ на ней SHALL переключать меню и отменять действие по умолчанию. `disabled` и `menu_button_disabled` SHALL делать её недоступной.

#### Scenario: Открытие
- **WHEN** пользователь кликает по кнопке раскрытия
- **THEN** меню открыто, `aria-expanded="true"`, вызваны `on_dropdown_click` и `on_menu_show`

#### Scenario: Клавиша
- **WHEN** фокус на кнопке раскрытия и нажата ↓
- **THEN** меню открыто, действие по умолчанию отменено

#### Scenario: Закрытие
- **WHEN** меню открыто и пользователь кликает вне компонента
- **THEN** меню закрыто, `aria-expanded="false"`, вызван `on_menu_hide`

### Requirement: Меню
Меню SHALL быть всплывающим `Menu` с `model`, `menu_style`, `menu_style_class` и привязываться к корню `SplitButton` (панель выравнивается под всем компонентом, а не под стрелкой). Выбор пункта меню SHALL закрывать меню и выполнять `command` пункта, как у `Menu`.

#### Scenario: Положение
- **WHEN** меню открыто
- **THEN** панель меню расположена под корнем `SplitButton` и не уже его

#### Scenario: Команда
- **WHEN** пользователь выбирает пункт меню с `command`
- **THEN** `command` выполнена, меню закрыто

### Requirement: Рендер без браузера
Вне браузера (SSR) `SplitButton` SHALL отрисовываться с обеими кнопками и закрытым меню (панель не выводится); `window` и `document` SHALL не использоваться.

#### Scenario: SSR
- **WHEN** страница со `SplitButton` отрисована на сервере
- **THEN** в разметке две кнопки, `aria-expanded="false"` и нет панели меню
