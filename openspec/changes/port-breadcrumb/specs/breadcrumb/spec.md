## Purpose
Breadcrumb показывает путь по иерархии страниц: необязательный пункт «домой», пункты модели и разделители между ними.

## ADDED Requirements

### Requirement: Разметка Breadcrumb
Компонент `Breadcrumb` крейта `prime-breadcrumb` SHALL выводить `<nav class="p-breadcrumb p-component">` с одним `<ol class="p-breadcrumb-list">`. `style_class` SHALL добавляться после собственных классов, `style` — задавать встроенный стиль, `pt.root` и `pt.list` — добавлять классы, стиль и атрибуты соответствующим элементам без замены собственных.

#### Scenario: Корень
- **WHEN** отрисован `Breadcrumb` без свойств
- **THEN** выводится `nav.p-breadcrumb.p-component` с пустым `ol.p-breadcrumb-list`

#### Scenario: Классы и стиль
- **WHEN** заданы `style_class = "mine"` и `style = "width: 20rem"`
- **THEN** класс корня `p-breadcrumb p-component mine`, стиль `width: 20rem;`

#### Scenario: pt
- **WHEN** `pt.root` содержит атрибут `data-x="1"`, а `pt.list` — класс `custom`
- **THEN** `nav` имеет `data-x="1"`, а `ol` — `p-breadcrumb-list custom`

### Requirement: Пункты Breadcrumb
Для каждого пункта модели с `visible` не равным `false` SHALL выводиться `li.p-breadcrumb-item` (с `p-disabled` у отключённых, `id`, `style_class` и `style` пункта) со ссылкой `a.p-breadcrumb-item-link`. Пункты SHALL идти в порядке модели.

#### Scenario: Подписи
- **WHEN** модель содержит пункты `Electronics` и `Computer`
- **THEN** выводятся два `li.p-breadcrumb-item` с подписями в этом порядке

#### Scenario: Скрытый пункт
- **WHEN** у пункта `visible = false`
- **THEN** он не выводится, и разделитель после него тоже

#### Scenario: Отключённый пункт
- **WHEN** у пункта `disabled = true`
- **THEN** у `li` есть класс `p-disabled`, а у ссылки нет `tabindex`

### Requirement: Ссылка пункта Breadcrumb
Ссылка SHALL иметь `href` из `url` (без `url` — без `href`), `target`, `title`, `tabindex` пункта (по умолчанию `0`, у отключённых — без `tabindex`) и класс `link_class`. Внутри SHALL идти значок `span.p-breadcrumb-item-icon` при наличии, подпись `span.p-breadcrumb-item-label` при непустой `label` и `Badge` при заданном `badge`.

#### Scenario: Ссылка
- **WHEN** у пункта `url = "/a"`, `target = "_blank"`, `title = "T"`
- **THEN** ссылка имеет `href="/a"`, `target="_blank"`, `title="T"`, `tabindex="0"`

#### Scenario: Пункт без url
- **WHEN** у пункта нет `url`
- **THEN** ссылка выводится без атрибута `href`

#### Scenario: Значок и бейдж
- **WHEN** у пункта заданы значок и `badge = "3"`
- **THEN** внутри ссылки сначала значок, затем подпись, затем `span.p-badge` со значением `3`

### Requirement: Пункт «домой» Breadcrumb
При заданном `home` с `visible` не равным `false` SHALL выводиться первым `li.p-breadcrumb-home-item` со ссылкой тех же свойств, что у обычного пункта, и классом `p-breadcrumb-item-link`. Без своего значка SHALL выводиться `HomeIcon` с классом `p-breadcrumb-item-icon`. Ссылка SHALL иметь `aria-label` из `home_aria_label`, если он задан.

#### Scenario: Значок по умолчанию
- **WHEN** задан `home` без значка и подписи
- **THEN** первый `li` — `p-breadcrumb-home-item` со значком `home`

#### Scenario: aria-label
- **WHEN** `home_aria_label = "Home"`
- **THEN** ссылка пункта «домой» имеет `aria-label="Home"`

### Requirement: Разделители Breadcrumb
Между соседними пунктами SHALL выводиться `li.p-breadcrumb-separator` со значком `chevron-right` (`pt.separator_icon`); после пункта «домой» разделитель SHALL выводиться, если есть хотя бы один пункт модели; после последнего пункта — нет. Шаблон `separator` SHALL заменять стандартный значок.

#### Scenario: Количество
- **WHEN** есть `home` и три пункта модели
- **THEN** выводятся три разделителя

#### Scenario: Без домой
- **WHEN** `home` не задан и в модели три пункта
- **THEN** выводятся два разделителя

#### Scenario: Свой разделитель
- **WHEN** задан шаблон `separator`, выводящий `/`
- **THEN** внутри каждого разделителя находится `/`, значка `chevron-right` нет

### Requirement: Шаблон пункта Breadcrumb
Шаблон `item` SHALL заменять содержимое `li` пункта (включая «домой»), получая пункт; стандартная ссылка при этом не выводится, а обёртка `li` с классами остаётся.

#### Scenario: Свой пункт
- **WHEN** задан шаблон `item`, выводящий `<b>` с подписью
- **THEN** внутри каждого `li.p-breadcrumb-item` находится `<b>`, ссылки `p-breadcrumb-item-link` нет

### Requirement: Нажатие на пункт Breadcrumb
При нажатии на ссылку пункта SHALL отменяться действие по умолчанию, если пункт отключён или у него нет `url`. У неотключённого пункта SHALL вызываться его `command`, затем `on_item_click` компонента, оба с событием и пунктом; у отключённого — ни один.

#### Scenario: Команда
- **WHEN** пользователь нажимает на пункт без `url` с `command`
- **THEN** вызываются `command` и `on_item_click`, переход не происходит

#### Scenario: Отключённый
- **WHEN** пользователь нажимает на отключённый пункт
- **THEN** ни `command`, ни `on_item_click` не вызываются

#### Scenario: Ссылка с url
- **WHEN** пользователь нажимает на пункт с `url`
- **THEN** действие по умолчанию не отменяется

### Requirement: Стоимость отрисовки Breadcrumb
Число аллокаций на экземпляр при SSR SHALL укладываться в бюджет `breadcrumb-1000` в `benches/alloc-budgets.toml`, вклад в wasm — в бюджет `breadcrumb` в `benches/budgets.toml`.

#### Scenario: Бюджеты
- **WHEN** выполняются `cargo xtask alloc-budget` и `cargo xtask wasm-size`
- **THEN** значения Breadcrumb не превышают бюджеты
