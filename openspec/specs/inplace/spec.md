# inplace Specification

## Purpose
Inplace переключает область между «видом» и «содержимым» по действию пользователя.

## Requirements

### Requirement: Разметка Inplace
Компонент `Inplace` крейта `prime-inplace` SHALL выводить корень `div.p-inplace.p-component` с `aria-live="polite"`. В неактивном состоянии корень SHALL содержать `div.p-inplace-display` с `tabindex="0"`, `role="button"` и содержимым `display`; при `disabled` — класс `p-disabled` и атрибут `data-p-disabled`. В активном состоянии корень SHALL содержать `div.p-inplace-content` с содержимым `content`. `style`, `style_class` и `pt` SHALL применяться к корню.

#### Scenario: Вид по умолчанию
- **WHEN** компонент отрисован без свойств
- **THEN** есть `.p-inplace-display[role=button][tabindex="0"]`, `.p-inplace-content` нет

#### Scenario: Активное состояние
- **WHEN** `active` равно `true`
- **THEN** есть `.p-inplace-content`, `.p-inplace-display` нет

#### Scenario: Недоступен
- **WHEN** `disabled`
- **THEN** вид имеет класс `p-disabled` и `data-p-disabled`

### Requirement: Активация
Клик по виду и клавиша `Enter` на нём SHALL делать компонент активным и вызывать `on_activate`; `Enter` SHALL отменять действие по умолчанию. Клик SHALL игнорироваться при `prevent_click`. При `disabled` активация SHALL не выполняться. Двусторонний сигнал `active` SHALL отражать состояние и позволять менять его снаружи.

#### Scenario: Клик
- **WHEN** пользователь нажимает на вид
- **THEN** показывается содержимое, `on_activate` вызван, сигнал `active` равен `true`

#### Scenario: Клавиша
- **WHEN** на виде нажат `Enter`
- **THEN** содержимое показано

#### Scenario: prevent_click
- **WHEN** `prevent_click` и нажат вид
- **THEN** состояние не меняется

### Requirement: Закрытие
При `closable` в содержимом SHALL выводиться кнопка `Button` (`type=button`, значок `TimesIcon` или `close_icon`, `aria-label` из `close_aria_label`), по клику на которую компонент SHALL возвращаться к виду и вызывать `on_deactivate`. Функция закрытия, переданная в `content`, SHALL делать то же. При `prevent_click` кнопка закрытия SHALL ничего не делать. При `disabled` закрытие SHALL не выполняться.

#### Scenario: Кнопка закрытия
- **WHEN** `closable` и активное состояние, нажата кнопка
- **THEN** показан вид, `on_deactivate` вызван

#### Scenario: Колбэк в содержимом
- **WHEN** содержимое вызывает переданную функцию закрытия
- **THEN** показан вид
