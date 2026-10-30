## Purpose
Tabs переключает представления вкладками с навигацией по клавиатуре, индикатором активной вкладки и прокруткой.

## ADDED Requirements

### Requirement: Разметка Tabs
`Tabs` крейта `prime-tabs` SHALL выводить `div.p-tabs.p-component` (с `p-tabs-scrollable` при `scrollable`) и `id` (по умолчанию уникальный). `TabList` SHALL выводить `div.p-tablist` с `div.p-tablist-content.p-tablist-viewport` > `div.p-tablist-tab-list[role=tablist]`, содержащим вкладки и `span.p-tablist-active-bar[role=presentation][data-pc-section=activebar]`. `Tab` SHALL выводить `div.p-tab[role=tab]` с `id` `<tabs-id>_tab_<value>`, `aria-controls` `<tabs-id>_tabpanel_<value>`, `aria-selected`, `data-p-active`, `p-tab-active` у активной и `p-disabled`, `aria-disabled`, `data-p-disabled` у отключённой. `TabPanels` SHALL выводить `div.p-tabpanels`, `TabPanel` — `div.p-tabpanel[role=tabpanel]` с `id` `<tabs-id>_tabpanel_<value>`, `aria-labelledby`, `data-p-active`, `p-tabpanel-active` у активной и атрибутом `hidden` у неактивной. `style`, `style_class` и `pt` SHALL применяться к корню.

#### Scenario: Активная вкладка
- **WHEN** `value` равно «1»
- **THEN** вкладка «1» имеет `p-tab-active`, `aria-selected="true"`, панель «1» без `hidden`, остальные панели с `hidden`

#### Scenario: Связь вкладки и панели
- **WHEN** отрисованы вкладка и панель со значением «a»
- **THEN** `aria-controls` вкладки равен `id` панели, `aria-labelledby` панели равен `id` вкладки

### Requirement: Значение
`value` SHALL быть двусторонним сигналом `Option<String>`; без него состояние собственное, начальное — `default_value`. Клик по доступной вкладке SHALL делать её значение активным. При `select_on_focus` фокус на доступной вкладке SHALL делать её активной.

#### Scenario: Клик
- **WHEN** пользователь нажимает на вкладку «2»
- **THEN** `value` равно «2», панель «2» видна

#### Scenario: Отключённая вкладка
- **WHEN** нажата отключённая вкладка
- **THEN** `value` не меняется

### Requirement: Клавиатура
`tabindex` вкладки SHALL быть `tabindex` группы у активной доступной вкладки и −1 у остальных. На вкладке `ArrowRight`/`ArrowLeft` SHALL переносить фокус на следующую/предыдущую доступную вкладку (по кругу), `Home`/`End` — на первую/последнюю доступную, `PageUp`/`PageDown` — прокручивать список к первой/последней вкладке, `Enter`, `NumpadEnter` и `Space` — активировать вкладку; обработанные клавиши SHALL отменять действие по умолчанию, а событие SHALL не всплывать дальше вкладки. Фокус SHALL не активировать вкладку без `select_on_focus`.

#### Scenario: Стрелка по кругу
- **WHEN** фокус на последней доступной вкладке и нажата `ArrowRight`
- **THEN** фокус на первой доступной вкладке

#### Scenario: Пропуск отключённой
- **WHEN** следующая вкладка отключена
- **THEN** фокус переходит через неё

### Requirement: Индикатор и прокрутка
Полоса `p-tablist-active-bar` SHALL получать `width` и `left` активной вкладки относительно списка; пересчёт SHALL выполняться при смене значения и изменении размера списка. При `show_navigators` (по умолчанию) `TabList` SHALL выводить кнопку `p-tablist-prev-button` (`aria-label` «Previous»), когда содержимое прокручено от начала, и `p-tablist-next-button` («Next»), когда справа есть скрытая часть; клик SHALL прокручивать содержимое на ширину видимой области (для следующей — за вычетом кнопок). `prev_icon` и `next_icon` SHALL заменять значки.

#### Scenario: Индикатор
- **WHEN** активна вторая вкладка
- **THEN** ширина полосы равна ширине этой вкладки, `left` — её смещению от начала списка

#### Scenario: Кнопки прокрутки
- **WHEN** вкладки не помещаются и содержимое прокручено вправо
- **THEN** выведены обе кнопки

### Requirement: Ленивые панели
При `lazy` у `Tabs` или `TabPanel` содержимое панели SHALL не выводиться до первой активации, а после неё SHALL оставаться (скрытым при деактивации). Без `lazy` содержимое выводится всегда.

#### Scenario: До активации
- **WHEN** `lazy` и панель неактивна
- **THEN** внутри `div.p-tabpanel` ничего нет

#### Scenario: После активации
- **WHEN** вкладка стала активной и затем неактивной
- **THEN** содержимое остаётся в DOM, панель с `hidden`
