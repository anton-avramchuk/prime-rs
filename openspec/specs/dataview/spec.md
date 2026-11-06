# dataview Specification

## Purpose
DataView — данные в виде списка или сетки с пагинатором, сортировкой и фильтром.

## Requirements

### Requirement: Отбор и страница
Элементы SHALL проходить фильтр (предикат), затем устойчивую сортировку (`sort_order` 1 — по возрастанию, -1 — по убыванию, 0 — без сортировки), затем выбирается страница `[first, first + rows)`, если включён пагинатор. В режиме `lazy` значение SHALL показываться как есть, а число записей пагинатора — браться из `total_records`. `first`, вышедший за предел, SHALL возвращаться к первой записи последней страницы.

#### Scenario: Сортировка
- **WHEN** `[5, 3, 8, 1]`, порядок -1
- **THEN** `[8, 5, 3, 1]`

#### Scenario: Страница
- **WHEN** 25 элементов, `rows = 10`, `first = 20`
- **THEN** показаны 5 последних элементов

#### Scenario: Выход за предел
- **WHEN** `first = 90` при 25 элементах и `rows = 10`
- **THEN** `first` становится 20

### Requirement: Разметка
`DataView` SHALL выводить `div.p-dataview.p-component.p-dataview-list` или `-grid` с блоками в порядке: загрузка (`p-dataview-loading` > `p-dataview-loading-overlay p-overlay-mask` > значок), заголовок, пагинатор сверху (`p-dataview-paginator-top`), `p-dataview-content` (шаблон `list` или `grid`, сообщение `p-dataview-empty-message` при пустых данных и отсутствии загрузки), пагинатор снизу, подвал.

#### Scenario: Пусто
- **WHEN** элементов нет
- **THEN** выведено сообщение «No results found» (или заданное `empty_message`/`empty`)

#### Scenario: Загрузка
- **WHEN** `loading = true`
- **THEN** есть маска со значком, сообщения о пустых данных нет

### Requirement: События и режим lazy
Смена страницы SHALL обновлять `first` и `rows` и вызывать `on_page`; в режиме `lazy` — и `on_lazy_load` (`first`, `rows`, `sort_order`). `on_lazy_load` SHALL вызываться при создании (если `lazy_load_on_init`) и при смене порядка сортировки; смена порядка возвращает на первую страницу.

#### Scenario: Lazy
- **WHEN** режим `lazy`, переход на вторую страницу
- **THEN** вызван `on_lazy_load` с `first = rows`
