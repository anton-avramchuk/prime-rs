# panel Specification

## Purpose
TBD - created by archiving change port-panel. Update Purpose after archive.

## Requirements

### Requirement: Структура и классы Panel
Компонент `Panel` крейта `prime-panel` SHALL отрисовывать корень `<div class="p-panel p-component">`, шапку `p-panel-header` (если `show_header`, по умолчанию да), в ней заголовок `p-panel-title` (если задан `header`) и блок `p-panel-header-actions`, затем контейнер `p-panel-content-container` с обёрткой `p-panel-content-wrapper`, содержимым `p-panel-content` и подвалом `p-panel-footer` (только при `footer_template`). При `toggleable` корень SHALL получать `p-panel-toggleable` и `p-panel-expanded` либо `p-panel-collapsed`. Блок действий SHALL получать `p-panel-icons-start`, `p-panel-icons-end` (по умолчанию) или `p-panel-icons-center` по `icon_pos`. `style_class` и `style` SHALL добавляться без потери собственных классов.

#### Scenario: Простая панель
- **WHEN** отрисована `<Panel header="Header">` с содержимым
- **THEN** HTML содержит `p-panel-header`, `p-panel-title` с текстом `Header`, `p-panel-content`, а `p-panel-footer` и переключатель отсутствуют

#### Scenario: Сворачиваемая панель
- **WHEN** задан `toggleable` и `collapsed = true`
- **THEN** корень имеет `p-panel-toggleable p-panel-collapsed`, а при `collapsed = false` — `p-panel-expanded`

#### Scenario: Без шапки
- **WHEN** `show_header = false`
- **THEN** `p-panel-header` отсутствует, содержимое отрисовано

### Requirement: Переключение Panel
При `toggleable` Panel SHALL отрисовывать кнопку `p-panel-toggle-button` (текстовая, круглая, вторичная) с иконкой «минус» при развёрнутом и «плюс» при свёрнутом состоянии. Клик по кнопке (`toggler = icon`, по умолчанию) либо по шапке (`toggler = header`) SHALL инвертировать `collapsed`, записывая новое значение в `RwSignal<bool>` (двусторонняя привязка; без сигнала состояние внутреннее, начальное — свёрнуто при `collapsed = true`). Enter и Space на кнопке SHALL переключать панель. Без `toggleable` кнопки нет, клики ничего не меняют.

#### Scenario: Клик по иконке
- **WHEN** пользователь нажимает кнопку развёрнутой панели
- **THEN** сигнал `collapsed` становится `true`, иконка меняется на «плюс»

#### Scenario: Переключатель в шапке
- **WHEN** `toggler = header` и пользователь кликает по шапке
- **THEN** панель переключается, клик по самой кнопке при этом не приводит к двойному переключению

#### Scenario: Клавиатура
- **WHEN** фокус на кнопке и нажат Enter либо Space
- **THEN** панель переключается

### Requirement: События переключения Panel
Panel SHALL вызывать `on_before_toggle` с исходным событием и текущим `collapsed` до смены состояния и `on_after_toggle` с новым `collapsed` после окончания анимации раскрытия или сворачивания.

#### Scenario: До и после
- **WHEN** панель переключается
- **THEN** `on_before_toggle` вызван с прежним значением, а `on_after_toggle` — после завершения анимации с новым

### Requirement: Доступность Panel
Кнопка-переключатель SHALL иметь `type="button"`, `aria-label` (текст `header`), `aria-controls` (id контейнера содержимого) и `aria-expanded` (`true` при развёрнутой панели). Контейнер содержимого SHALL иметь `id`, `role="region"`, `aria-labelledby`, `aria-hidden` (`true` при свёрнутой) и `tabindex="-1"` только при свёрнутой панели. При сворачивании фокусируемые элементы внутри содержимого SHALL получать `tabindex="-1"`, при разворачивании — терять этот атрибут. Идентификаторы SHALL быть производными от `id` панели и уникальными между экземплярами.

#### Scenario: aria-expanded
- **WHEN** панель развёрнута, затем свёрнута
- **THEN** `aria-expanded` кнопки меняется с `true` на `false`, `aria-hidden` контейнера — с `false` на `true`

#### Scenario: Связь кнопки и области
- **WHEN** панель отрисована
- **THEN** `aria-controls` кнопки равен `id` контейнера содержимого, а `aria-labelledby` контейнера указывает на `id` заголовка

#### Scenario: Фокус скрытого содержимого
- **WHEN** панель свёрнута, а внутри есть кнопка или ссылка
- **THEN** у них `tabindex="-1"`

### Requirement: Анимация раскрытия Panel
Свёрнутое содержимое SHALL быть скрыто. При переключении в браузере контейнер SHALL получать класс `p-collapsible-enter-active` (раскрытие) или `p-collapsible-leave-active` (сворачивание) на время анимации и после её окончания (`animationend`) SHALL возвращаться в итоговое состояние. При SSR и первом рендере анимации нет.

#### Scenario: Раскрытие
- **WHEN** свёрнутая панель разворачивается в браузере
- **THEN** контейнер получает `p-collapsible-enter-active`, затем класс снимается и содержимое видимо

#### Scenario: Начальное состояние
- **WHEN** панель отрисована с `collapsed = true`
- **THEN** содержимое скрыто с самого начала, анимации нет

### Requirement: Слоты и pass-through Panel
`header_template` SHALL выводиться в шапке после заголовка, `icons_template` — в блоке действий перед переключателем, `header_icons_template` SHALL заменять иконку переключателя и получать текущий `collapsed`, `footer_template` — в подвале. `pt` SHALL принимать части `root`, `header`, `title`, `header_actions`, `toggle_button`, `content_container`, `content_wrapper`, `content`, `footer`, добавляя классы, стиль и атрибуты без замены собственных.

#### Scenario: Свой значок переключателя
- **WHEN** задан `header_icons_template`
- **THEN** внутри кнопки выводится он, а не «минус»/«плюс», и получает актуальный `collapsed`

#### Scenario: Класс через pt
- **WHEN** в `pt.header` передан класс `custom`
- **THEN** у шапки есть `custom` и `p-panel-header`

### Requirement: Стоимость отрисовки Panel
Panel без `toggleable` и со статическими свойствами SHALL использовать статическую ветвь; обе ветви SHALL давать одинаковую разметку. Число аллокаций на экземпляр при SSR SHALL укладываться в бюджет `panel` в `benches/alloc-budgets.toml`, вклад в wasm — в бюджет `panel` в `benches/budgets.toml`.

#### Scenario: Паритет ветвей
- **WHEN** один набор значений передан статически и сигналами
- **THEN** HTML совпадает

#### Scenario: Бюджеты
- **WHEN** выполняются `cargo xtask alloc-budget` и `cargo xtask wasm-size`
- **THEN** значения Panel не превышают бюджеты
