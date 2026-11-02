# listbox Specification

## Purpose
Listbox показывает список вариантов (в том числе сгруппированных) и позволяет выбрать одно или несколько значений мышью и клавишами, с поиском и флажками.

## Requirements

### Requirement: Разметка
`Listbox` крейта `prime-listbox` SHALL выводить `div.p-listbox.p-component` с `data-p`, внутри — `div.p-listbox-list-container` (высота `scroll_height`, по умолчанию `14rem`) > `ul.p-listbox-list[role=listbox]` с `tabindex`, `aria-label`, `aria-multiselectable` (при `multiple`), `aria-disabled`, `aria-activedescendant` (пока есть фокус) и вариантами `li.p-listbox-option[role=option]` с `id` вида `<id>_<индекс>`, `aria-label`, `aria-selected`, `aria-disabled`, `aria-setsize`, `data-p-selected`, `data-p-focused`, `data-p-disabled`. `style_class`, `style` и `pt` SHALL применяться к корню; `list_style`/`list_style_class` — к списку. Классы корня: `p-listbox-striped`, `p-disabled`, `p-invalid`, `p-listbox-fluid`. Классы варианта: `p-listbox-option-selected` (выбран и `highlight_on_select`), `p-focus`, `p-disabled`.

#### Scenario: Простой список
- **WHEN** отрисован `Listbox` с тремя вариантами
- **THEN** выведены три `li.p-listbox-option[role=option]` с метками, ни один не выбран

#### Scenario: Выбранное значение
- **WHEN** `value` равно значению одного из вариантов
- **THEN** у этого варианта `aria-selected=true` и класс `p-listbox-option-selected`

### Requirement: Группы и пустой список
Если заданы `groups`, `Listbox` SHALL выводить `li.p-listbox-option-group[role=option]` с заголовком и затем варианты группы; заголовки не учитываются в `aria-setsize`/`aria-posinset` и не выбираются. При отсутствии вариантов SHALL выводиться `div.p-listbox-empty-message` с `empty_message`, а при пустом результате поиска — с `empty_filter_message`.

#### Scenario: Группа
- **WHEN** заданы две группы по два варианта
- **THEN** выведены два заголовка и четыре варианта, `aria-setsize=4`

#### Scenario: Пустой результат поиска
- **WHEN** строка поиска не подходит ни под один вариант
- **THEN** выведено `div.p-listbox-empty-message` с `empty_filter_message`

### Requirement: Одиночный выбор
Без `multiple` клик по доступному варианту SHALL записывать его значение в `value` и вызывать `on_change`; повторный клик по выбранному — сбрасывать значение (`None`). При `meta_key_selection` повторный клик без `Ctrl`/`⌘` SHALL оставлять выбор, а с ними — снимать его. Недоступный вариант и режим `readonly`/`disabled` SHALL игнорировать клики.

#### Scenario: Выбор и сброс
- **WHEN** пользователь кликает по варианту, затем по нему же
- **THEN** `value` сначала равно значению варианта, затем `None`; `on_change` вызван дважды

#### Scenario: Недоступный вариант
- **WHEN** пользователь кликает по недоступному варианту
- **THEN** `value` не меняется, `on_change` не вызывается

### Requirement: Множественный выбор
При `multiple` клик SHALL добавлять вариант в `values` или убирать его оттуда. При `meta_key_selection` клик без `Ctrl`/`⌘` SHALL заменять выбор одним вариантом, а с ними — добавлять/убирать. `Shift`+↓/↑ SHALL выбирать диапазон от опорного варианта до фокусного, `Ctrl+Shift+Home/End` — до первого/последнего, `Ctrl+A` — все доступные видимые варианты.

#### Scenario: Добавление и снятие
- **WHEN** пользователь кликает по двум вариантам, затем по первому
- **THEN** `values` содержит только второй

#### Scenario: Диапазон
- **WHEN** фокус на первом варианте, нажато `Shift`+↓ дважды
- **THEN** `values` содержит первые три доступных варианта

#### Scenario: Все
- **WHEN** нажато `Ctrl+A`
- **THEN** `values` содержит все доступные видимые варианты

### Requirement: Флажки и «выбрать все»
При `checkbox` и `multiple` каждый вариант SHALL содержать флажок `Checkbox` с `p-listbox-option-check-icon`, а при `show_toggle_all` в шапке SHALL быть флажок «выбрать все»: он выбирает все доступные видимые варианты (с учётом поиска) либо снимает выбор, если все уже выбраны, и вызывает `on_select_all_change` и `on_change`. При `checkmark` выбранный вариант SHALL показывать значок-галочку (`p-listbox-option-check-icon`), невыбранный — пустой (`p-listbox-option-blank-icon`), а `p-listbox-option-selected` не добавляться.

#### Scenario: Выбрать все
- **WHEN** пользователь кликает флажок в шапке
- **THEN** выбраны все доступные варианты, флажок отмечен

#### Scenario: Выбрать все с поиском
- **WHEN** строка поиска оставляет два варианта и нажат флажок «выбрать все»
- **THEN** выбраны только эти два

#### Scenario: Отметка
- **WHEN** задан `checkmark` и вариант выбран
- **THEN** в варианте есть `.p-listbox-option-check-icon` и нет класса `p-listbox-option-selected`

### Requirement: Поиск
При `filter` шапка SHALL содержать `span.p-iconfield` с полем `input.p-listbox-filter[role=searchbox]` (`aria-owns`, `aria-activedescendant`, `placeholder` из `filter_placeholder`, `aria-label` из `aria_filter_label`) и значком `p-inputicon`. Ввод SHALL оставлять варианты, метка которых содержит строку (без учёта регистра, пробелы по краям отбрасываются); группы без подходящих вариантов опускаются; фокусный вариант сбрасывается; вызывается `on_filter`. Стрелки, `Home`/`End` (в поле — перемещение курсора) и `Enter` в поле SHALL работать как в PrimeNG.

#### Scenario: Фильтрация
- **WHEN** пользователь вводит «lon»
- **THEN** остаются варианты, содержащие «lon» в любом регистре, `on_filter` вызван со строкой

### Requirement: Клавиатура
Фокус на списке SHALL ставить фокусным выбранный вариант (при `auto_option_focus` — первый доступный), `Tab` не перехватывается. ↓/↑ SHALL переходить к следующему/предыдущему доступному варианту, `Home`/`End` — к первому/последнему, `PageUp`/`PageDown` — прокручивать, `Enter`/`Space` — выбирать фокусный вариант, печатный символ — переходить к следующему варианту, метка которого начинается с набранной строки (накопление символов 500 мс). При `focus_on_hover` и фокусе списка наведение мыши SHALL переносить фокусный вариант. При `select_on_focus` перемещение фокуса стрелками (без `Shift`) SHALL выбирать вариант. `on_focus`/`on_blur` SHALL вызываться при входе/выходе фокуса.

#### Scenario: Навигация
- **WHEN** список в фокусе и нажато ↓ дважды
- **THEN** фокусный вариант — третий (если он доступен), `aria-activedescendant` указывает на него

#### Scenario: Выбор клавишей
- **WHEN** фокусный вариант и нажат Enter
- **THEN** вариант выбран

#### Scenario: Поиск набором
- **WHEN** набрана буква, с которой начинается метка одного из вариантов
- **THEN** фокус переходит на него

### Requirement: Недоступность и валидность
При `disabled` корень SHALL получать `p-disabled`, список — `aria-disabled`, `tabindex=-1`; выбор и клавиши SHALL не работать. При `readonly` выбор SHALL не меняться, список остаётся доступным для фокуса. При `invalid` корень SHALL получать `p-invalid`. `fluid` без значения SHALL браться из контекста `Fluid`.

#### Scenario: Disabled
- **WHEN** `disabled=true` и пользователь кликает вариант
- **THEN** значение не меняется

### Requirement: События мыши
Клик по доступному варианту SHALL вызывать `on_click`; двойной клик — `on_dbl_click` (не при `readonly`/`disabled`/недоступном варианте). Клик SHALL переносить фокусный вариант на кликнутый.

#### Scenario: Двойной клик
- **WHEN** пользователь дважды кликает по варианту
- **THEN** вызван `on_dbl_click` со значением варианта

### Requirement: Свои представления
`option` SHALL заменять содержимое `li.p-listbox-option` (кроме флажка/отметки) и получать вариант и признак выбора; `group` — содержимое заголовка группы.

#### Scenario: Шаблон варианта
- **WHEN** задан `option`
- **THEN** внутри варианта переданное представление

### Requirement: Рендер без браузера
Вне браузера (SSR) `Listbox` SHALL отрисовываться полностью (включая выбранные значения и флажки); `window` и `document` SHALL не использоваться.

#### Scenario: SSR
- **WHEN** страница с `Listbox` отрисована на сервере
- **THEN** в разметке есть `ul.p-listbox-list` с вариантами, выбранные помечены
