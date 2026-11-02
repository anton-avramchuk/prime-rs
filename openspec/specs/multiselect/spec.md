# multiselect Specification

## Purpose
MultiSelect позволяет выбрать несколько значений из списка вариантов в выпадающей панели с поиском, группами, флажками, режимом чипов и управлением с клавиатуры.

## Requirements

### Requirement: Значения и метка
`MultiSelect` крейта `prime-multiselect` SHALL принимать двусторонний `values` (`RwSignal<Vec<String>>`); без него SHALL вести внутреннее состояние, начинающееся с `default_values`. Метка `div.p-multiselect-label` в режиме `comma` SHALL показывать метки выбранных вариантов через «, » в порядке выбора, а при выбранных значениях больше `max_selected_labels` — `selected_items_label` (по умолчанию «{0} items selected» из перевода, `{0}` — число выбранных). Без выбора метка SHALL показывать `placeholder` с классом `p-placeholder`, а без `placeholder` — пустую метку с классом `p-multiselect-label-empty`. При `display_selected_label = false` метка SHALL показывать `placeholder`. `selected_items` SHALL заменять содержимое метки. Значения, которых нет среди вариантов, SHALL не попадать в метку.

#### Scenario: Список меток
- **WHEN** выбраны «New York» и «London», `max_selected_labels` равен 3
- **THEN** метка показывает «New York, London»

#### Scenario: Сводка
- **WHEN** выбрано четыре значения, `max_selected_labels` равен 3
- **THEN** метка показывает «4 items selected»

#### Scenario: Подсказка
- **WHEN** ничего не выбрано и задан `placeholder = "Select Cities"`
- **THEN** метка показывает «Select Cities» с классом `p-placeholder`

### Requirement: Режим чипов
При `display = Chip` корень SHALL получать класс `p-multiselect-display-chip`, а метка — по одному `div.p-multiselect-chip-item` с `Chip` (`p-multiselect-chip`) на выбранный вариант, не больше `max_selected_labels`; при превышении предела SHALL показываться `selected_items_label`. У чипа, если поле доступно (не `disabled`, не `readonly`), SHALL быть значок удаления (`chip_icon` или стандартный); клик по нему SHALL убирать значение, вызывать `on_remove` и `on_change`, не открывая панель.

#### Scenario: Чипы
- **WHEN** выбраны два значения в режиме `chip`
- **THEN** выведены два `.p-multiselect-chip-item` с метками

#### Scenario: Удаление чипа
- **WHEN** пользователь кликает по значку удаления чипа
- **THEN** значение убрано из `values`, панель не открылась, вызваны `on_remove` и `on_change`

### Requirement: Панель
Панель `div.p-multiselect-overlay` SHALL открываться по клику на поле, стрелкам вниз и вверх, `Enter`, `Space`, `Home`/`End` и набору печатного символа, а также программно через двусторонний `open`; закрываться по повторному клику, `Escape`, `Tab`, клику вне компонента, `resize` и `Alt+↑`. Выбор варианта SHALL оставлять панель открытой. Открытая панель SHALL быть расположена под полем (над ним, если снизу не хватает места) с z-index группы оверлеев, а `aria-expanded` поля SHALL быть `true`. При `filter` и `autofocus_filter` фокус SHALL переходить в поле поиска. Закрытие по `Escape` SHALL возвращать фокус на поле. При `disabled`, `readonly` и `loading` панель SHALL не открываться. `on_show` и `on_hide` SHALL вызываться при открытии и закрытии; `reset_filter_on_hide` SHALL сбрасывать строку поиска при закрытии.

#### Scenario: Открытие кликом
- **WHEN** пользователь кликает по закрытому полю
- **THEN** панель открыта, `aria-expanded="true"`, вызван `on_show`

#### Scenario: Выбор не закрывает
- **WHEN** панель открыта и пользователь кликает по варианту
- **THEN** значение добавлено, панель осталась открытой

#### Scenario: Недоступно
- **WHEN** задан `disabled`
- **THEN** клик по полю панель не открывает

### Requirement: Выбор вариантов
Клик по варианту `li.p-multiselect-option[role=option]` SHALL добавлять его значение в `values` или убирать его и вызывать `on_change` один раз со списком значений и исходным событием. Недоступные варианты SHALL не выбираться и пропускаться клавиатурой. Выбранный вариант SHALL иметь `aria-selected="true"`, флажок с классом `p-checkbox-checked` и (при `highlight_on_select`) класс `p-multiselect-option-selected`; вариант под фокусом — класс `p-focus` и `data-p-focused`; `focus_on_hover` SHALL переносить фокус на вариант под указателем. `aria-setsize`/`aria-posinset` SHALL не учитывать заголовки групп. `item` SHALL заменять содержимое варианта (после флажка), получая вариант и признак выбора.

#### Scenario: Добавление и снятие
- **WHEN** пользователь кликает по двум вариантам, затем по первому
- **THEN** `values` содержит только второй, `on_change` вызван трижды

#### Scenario: Недоступный вариант
- **WHEN** пользователь кликает по варианту с `disabled`
- **THEN** `values` не меняются

### Requirement: Предел выбора
При `selection_limit` и выбранных значениях, равных пределу, все невыбранные варианты SHALL считаться недоступными (класс `p-disabled`, не выбираются, пропускаются клавиатурой), выбранные SHALL оставаться доступными для снятия, а флажок «выбрать все» SHALL не выводиться.

#### Scenario: Предел достигнут
- **WHEN** `selection_limit = 2` и выбраны два значения
- **THEN** остальные варианты недоступны, снятие выбранного возможно

### Requirement: Выбрать все
Шапка `div.p-multiselect-header` (при `show_header`) SHALL при `show_toggle_all` и отсутствии `selection_limit` содержать флажок «выбрать все». Он SHALL выбирать все доступные видимые варианты (с учётом поиска, не теряя уже выбранные вне результата поиска и выбранные недоступные), а если все доступные видимые уже выбраны — снимать их (выбранные недоступные остаются). Флажок SHALL быть отмечен, когда выбраны все доступные видимые варианты. Нажатие SHALL вызывать `on_select_all_change` и `on_change`.

#### Scenario: Выбрать все с поиском
- **WHEN** поиск оставляет два варианта и нажат флажок
- **THEN** выбраны эти два и ранее выбранные

#### Scenario: Снять все
- **WHEN** все доступные видимые варианты выбраны и нажат флажок
- **THEN** выбор снят, кроме недоступных выбранных

### Requirement: Поиск
При `filter` шапка SHALL содержать `p-iconfield.p-multiselect-filter-container` с полем `input.p-multiselect-filter[role=searchbox]` (`aria-owns`, `aria-activedescendant`, `placeholder` из `filter_placeholder`, `aria-label` из `aria_filter_label`) и значком `p-inputicon`. Ввод SHALL оставлять варианты, метка которых содержит строку (без учёта регистра); группы без подходящих вариантов опускаются; фокусный вариант сбрасывается; вызывается `on_filter`; при пустом результате SHALL выводиться `li.p-multiselect-empty-message` с `empty_filter_message`, при пустом списке — с `empty_message`.

#### Scenario: Фильтрация
- **WHEN** пользователь вводит «lon»
- **THEN** остаются варианты, содержащие «lon» в любом регистре, `on_filter` вызван со строкой

#### Scenario: Пустой результат
- **WHEN** строка не подходит ни под один вариант
- **THEN** выведено `li.p-multiselect-empty-message`

### Requirement: Клавиатура
Скрытое поле `combobox` SHALL принимать фокус (`tabindex`, по умолчанию `0`; при `disabled` равен `-1`) и иметь `aria-activedescendant` вариант под фокусом. ↓/↑ SHALL открывать панель и переносить фокус на следующий/предыдущий доступный вариант (`Shift` — выбирать диапазон от опорного); `Alt+↑` SHALL выбирать вариант под фокусом и закрывать панель; `Home`/`End` — на первый/последний доступный (`Ctrl+Shift` — диапазон); `PageUp`/`PageDown` — прокручивать к началу/концу; `Enter`/`Space` на закрытой панели открывать её, на открытой — переключать вариант под фокусом (`Shift` — диапазон); `Escape` — закрывать; `Tab` — выбирать вариант под фокусом, если он не выбран, и закрывать панель; `Ctrl+A` — выбирать все доступные видимые варианты; печатный символ — открывать панель и переходить к варианту, метка которого начинается с набранной строки (накопление 500 мс). При `select_on_focus` перенос фокуса клавишами SHALL выбирать вариант.

#### Scenario: Навигация и выбор
- **WHEN** поле в фокусе, нажато ↓ (панель открывается), затем ↓ и Enter
- **THEN** выбран второй доступный вариант, панель открыта

#### Scenario: Escape
- **WHEN** панель открыта и нажат Escape
- **THEN** панель закрыта, фокус на поле

### Requirement: Очистка, загрузка, блокировки
При `show_clear` и выбранных значениях SHALL выводиться значок `p-multiselect-clear-icon`; клик по нему SHALL сбрасывать `values`, вызывать `on_change` и `on_clear` и не открывать панель. При `loading` кнопка раскрытия SHALL показывать значок загрузки `p-multiselect-loading-icon`. Корень SHALL получать `p-disabled`, `p-invalid`, `p-variant-filled`, `p-focus`, `p-inputwrapper-filled`, `p-inputwrapper-focus`, `p-multiselect-open`, `p-multiselect-fluid`, `p-multiselect-sm`/`-lg` по состоянию; `fluid` без значения SHALL браться из контекста `Fluid`. При `readonly` и `disabled` выбор, очистка и удаление чипов SHALL не работать.

#### Scenario: Очистка
- **WHEN** пользователь кликает по значку очистки
- **THEN** `values` пусты, панель не открылась, вызваны `on_change` и `on_clear`

### Requirement: Рендер без браузера
Вне браузера (SSR) `MultiSelect` SHALL отрисовываться полностью с закрытой панелью (метка, чипы, значки); `window` и `document` SHALL не использоваться.

#### Scenario: SSR
- **WHEN** страница с `MultiSelect` и выбранными значениями отрисована на сервере
- **THEN** в разметке есть корень, метка с выбранными значениями и нет панели
