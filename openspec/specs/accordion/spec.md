# accordion Specification

## Purpose
TBD - created by archiving change port-accordion. Update Purpose after archive.

## Requirements

### Requirement: Структура и классы Accordion
`Accordion` крейта `prime-accordion` SHALL отрисовывать `<div class="p-accordion p-component">`, содержащий `AccordionPanel`. `AccordionPanel` SHALL отрисовывать `<div class="p-accordionpanel">` с `p-accordionpanel-active` у активной панели и `p-disabled` у отключённой, а также `data-p-active` и `data-p-disabled`. `AccordionHeader` SHALL отрисовывать `p-accordionheader` с содержимым и иконкой `p-accordionheader-toggle-icon` (шеврон вверх у активной панели, вниз у неактивной, `aria-hidden="true"`) и `data-p="active"` у активной. `AccordionContent` SHALL отрисовывать `p-accordioncontent` с вложенными `p-motion`, `p-accordioncontent-wrapper` и `p-accordioncontent-content`. `style_class` каждой части SHALL добавляться без потери собственных классов.

#### Scenario: Базовая структура
- **WHEN** отрисованы три панели, активна первая
- **THEN** у первой `p-accordionpanel-active`, а у остальных нет; заголовок первой содержит шеврон вверх, остальных — вниз

#### Scenario: Отключённая панель
- **WHEN** панель имеет `disabled`
- **THEN** у неё `p-disabled` и `data-p-disabled="true"`

### Requirement: Значение и режим multiple
`Accordion` SHALL принимать двусторонний `value` (`RwSignal<Vec<String>>`) либо `default_value` для собственного состояния. Панель активна, если её `value` входит в список. Клик по заголовку не отключённой панели SHALL в одиночном режиме открывать её и закрывать остальные, а повторный клик по активной — закрывать её; при `multiple` SHALL добавлять или убирать значение, не затрагивая остальные.

#### Scenario: Одиночный режим
- **WHEN** открыта панель `0` и нажат заголовок панели `1`
- **THEN** `value` становится `["1"]`, панель `0` закрывается

#### Scenario: Режим multiple
- **WHEN** `multiple`, открыта панель `0` и нажат заголовок панели `1`
- **THEN** `value` содержит `0` и `1`

#### Scenario: Закрытие активной
- **WHEN** нажат заголовок единственной активной панели
- **THEN** `value` становится пустым

#### Scenario: Отключённая панель
- **WHEN** нажат заголовок отключённой панели
- **THEN** `value` не меняется

### Requirement: События Accordion
`Accordion` SHALL вызывать `on_open` с исходным событием и значением панели, когда панель стала активной после клика, и `on_close`, когда стала неактивной; при открытии одной панели закрытие остальных в одиночном режиме событием `on_close` не сопровождается.

#### Scenario: Открытие
- **WHEN** пользователь открывает неактивную панель
- **THEN** вызван `on_open` со значением этой панели

#### Scenario: Закрытие
- **WHEN** пользователь закрывает активную панель
- **THEN** вызван `on_close` со значением этой панели

### Requirement: Доступность и клавиатура Accordion
`AccordionHeader` SHALL иметь `role="button"`, `aria-expanded`, `aria-controls` (идентификатор содержимого), `aria-disabled`, `tabindex="0"` (`-1` у отключённой) и идентификатор `{id}_accordionheader_{value}`. `AccordionContent` SHALL иметь `role="region"`, идентификатор `{id}_accordioncontent_{value}` и `aria-labelledby` с идентификатором заголовка. Enter, Space и NumpadEnter на заголовке SHALL переключать панель; ArrowDown и ArrowUp SHALL переносить фокус на заголовок следующей и предыдущей не отключённой панели с переходом по кругу; Home и End — на первый и последний. При `select_on_focus` получение фокуса не отключённым заголовком SHALL переключать панель.

#### Scenario: Связь заголовка и содержимого
- **WHEN** панель со значением `a` отрисована
- **THEN** `aria-controls` заголовка равен `id` содержимого, а `aria-labelledby` содержимого — `id` заголовка

#### Scenario: Стрелки пропускают отключённые
- **WHEN** фокус на заголовке первой панели, вторая отключена, нажата ArrowDown
- **THEN** фокус переходит на заголовок третьей панели

#### Scenario: Круговая навигация
- **WHEN** фокус на заголовке последней панели и нажата ArrowDown
- **THEN** фокус переходит на заголовок первой панели

#### Scenario: Клавиатурное переключение
- **WHEN** фокус на заголовке и нажат Enter
- **THEN** панель переключается один раз

### Requirement: Анимация Accordion
Содержимое неактивной панели SHALL быть скрыто с первого рендера. При смене состояния в браузере содержимое SHALL анимироваться классами `p-collapsible-enter-active` и `p-collapsible-leave-active`, а после завершения скрываться или показываться; фокусируемые потомки скрытого содержимого SHALL получать `tabindex="-1"`.

#### Scenario: Начальное состояние
- **WHEN** панель неактивна при первом рендере
- **THEN** её содержимое скрыто сразу, без анимации

#### Scenario: Раскрытие
- **WHEN** неактивная панель открывается в браузере
- **THEN** контейнер получает `p-collapsible-enter-active`, затем класс снимается и содержимое видимо

### Requirement: Слоты и pass-through Accordion
`toggle_icon_template` заголовка SHALL получать `active` и заменять стандартную иконку. `pt` аккордеона SHALL принимать части `root`, `panel`, `header`, `toggle_icon`, `content_container`, `content_wrapper`, `content`, добавляя классы, стиль и атрибуты без замены собственных.

#### Scenario: Свой значок
- **WHEN** задан `toggle_icon_template`
- **THEN** вместо шеврона выводится слот с актуальным `active`

#### Scenario: Класс через pt
- **WHEN** в `pt.header` передан класс `custom`
- **THEN** у каждого заголовка есть `custom` и `p-accordionheader`

### Requirement: Стоимость отрисовки Accordion
Число аллокаций SSR на аккордеон из трёх панелей SHALL укладываться в бюджет `accordion-1000` в `benches/alloc-budgets.toml`, вклад в wasm — в бюджет `accordion` в `benches/budgets.toml`.

#### Scenario: Бюджеты
- **WHEN** выполняются `cargo xtask alloc-budget` и `cargo xtask wasm-size`
- **THEN** значения Accordion не превышают бюджеты
