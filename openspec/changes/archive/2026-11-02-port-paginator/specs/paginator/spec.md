# paginator Specification

## Purpose
Paginator показывает постраничную навигацию по набору записей: кнопки и ссылки на страницы, переход к странице, выбор числа строк и отчёт о текущей странице.

## ADDED Requirements

### Requirement: Разметка
`Paginator` крейта `prime-paginator` SHALL выводить `div.p-paginator.p-component` с `style`/`style_class`/`pt` на корне. В порядке: `div.p-paginator-content-start` (при `template_left`), `span.p-paginator-current` (при `show_current_page_report`), кнопка `p-paginator-first` (при `show_first_last_icon`), кнопка `p-paginator-prev`, `span.p-paginator-pages` со ссылками (при `show_page_links`), выпадающий список перехода (при `show_jump_to_page_dropdown`), кнопки `p-paginator-next` и `p-paginator-last` (последняя при `show_first_last_icon`), поле перехода (при `show_jump_to_page_input`), выпадающий список числа строк (при непустом `rows_per_page_options`), `div.p-paginator-content-end` (при `template_right`). Кнопки SHALL иметь `type="button"` и `aria-label` из перевода (`firstPageLabel`, `prevPageLabel`, `nextPageLabel`, `lastPageLabel`) и значок (`first_page_link_icon` и остальные заменяют стандартные).

#### Scenario: Состав по умолчанию
- **WHEN** отрисован `Paginator` с `total_records = 120` и `rows = 10`
- **THEN** выведены четыре кнопки, пять ссылок на страницы и нет отчёта, списков и поля перехода

### Requirement: Ссылки на страницы
Ссылки SHALL показывать окно из `min(page_link_size, число страниц)` соседних страниц с текущей по возможности посередине; у окна у начала и конца списка границы сдвигаются так, чтобы показывалось ровно `min(page_link_size, число страниц)` ссылок. Текущая ссылка SHALL иметь класс `p-paginator-page-selected` и `aria-current="page"`; у каждой ссылки `aria-label` из `aria.pageLabel` с подстановкой `{page}`. Клик по ссылке SHALL переходить на её страницу.

#### Scenario: Окно у начала
- **WHEN** 12 страниц, текущая первая, `page_link_size = 5`
- **THEN** ссылки 1–5, первая выбрана

#### Scenario: Окно в середине
- **WHEN** 12 страниц, текущая шестая
- **THEN** ссылки 4–8

#### Scenario: Окно в конце
- **WHEN** 12 страниц, текущая двенадцатая
- **THEN** ссылки 8–12

### Requirement: Навигация кнопками
Кнопка «первая» SHALL переходить на страницу 1, «предыдущая» — на `страница − 1`, «следующая» — на `страница + 1`, «последняя» — на последнюю. На первой странице «первая» и «предыдущая», на последней «следующая» и «последняя», а при пустом списке все SHALL получать класс `p-disabled`; «предыдущая», «следующая» и «последняя» SHALL получать ещё и атрибут `disabled`. Каждый переход SHALL записывать новое `first` (`rows × страница`) и вызывать `on_page_change` с `PaginatorState { page, first, rows, page_count }` один раз. Переход на несуществующую страницу SHALL не выполняться.

#### Scenario: Следующая страница
- **WHEN** первая страница и клик по «следующей»
- **THEN** `first` равно `rows`, `on_page_change` вызван с `page = 1`

#### Scenario: Граница
- **WHEN** последняя страница
- **THEN** «следующая» и «последняя» недоступны

### Requirement: Отчёт
При `show_current_page_report` `span.p-paginator-current` SHALL показывать `current_page_report_template` с подстановкой `{currentPage}` (с единицы, 0 при пустом списке), `{totalPages}`, `{first}` (с единицы, 0 при пустом списке), `{last}` (`min(first + rows, total_records)`), `{rows}`, `{totalRecords}`; каждый заполнитель заменяется один раз (первое вхождение).

#### Scenario: Шаблон
- **WHEN** шаблон «Showing {first} to {last} of {totalRecords}», `first = 10`, `rows = 10`, `total_records = 120`
- **THEN** отчёт «Showing 11 to 20 of 120»

### Requirement: Переход к странице
При `show_jump_to_page_dropdown` SHALL выводиться `Select` с классом `p-paginator-jtp-dropdown`, вариантами «1»…«N», выбранной текущей страницей и подписью-отчётом вместо метки; выбор SHALL переходить на страницу. При `show_jump_to_page_input` SHALL выводиться `InputNumber` с классом `p-paginator-jtp-input`, значением «текущая страница» (с единицы); ввод числа SHALL переходить на страницу `число − 1`, если она существует. Оба элемента SHALL быть недоступны при пустом списке.

#### Scenario: Список страниц
- **WHEN** выбрана пятая страница в списке
- **THEN** `first` равно `rows × 4`

### Requirement: Число строк
При непустом `rows_per_page_options` SHALL выводиться `Select` с классом `p-paginator-rpp-dropdown`, вариантами из опций (число — подпись и значение; `ShowAll(подпись)` — вариант со значением `total_records`), выбранным текущим `rows` и `aria-label` `rowsPerPageLabel`. Выбор SHALL записывать новое `rows` и переходить на страницу, в которой остаётся прежний `first` (если страница не существует — на последнюю существующую), вызывая `on_page_change`.

#### Scenario: Смена числа строк
- **WHEN** `rows = 10`, `first = 20` и выбрано 20
- **THEN** `rows` равно 20, `first` равно 20, страница 2, вызван `on_page_change`

### Requirement: Управление извне
`first` и `rows` SHALL быть двусторонними сигналами; внешнее изменение `first`, `rows` или `total_records` SHALL перерисовывать окно ссылок, отчёт и состояние кнопок. Если после уменьшения `total_records` `first` оказывается не меньше `total_records`, а страница больше первой, компонент SHALL переходить на предыдущую страницу (не дальше последней существующей), пока `first` не окажется в пределах.

#### Scenario: Уменьшение числа записей
- **WHEN** `first = 110`, `rows = 10`, затем `total_records` уменьшено со 120 до 50
- **THEN** `first` становится 40

### Requirement: Скрытие
При `always_show = false` и единственной странице корень SHALL получать `display: none`.

#### Scenario: Одна страница
- **WHEN** `always_show = false`, `total_records = 5`, `rows = 10`
- **THEN** корень скрыт

### Requirement: Пустой список и крайние значения
При `total_records = 0` страниц SHALL быть ноль, ссылок нет, все кнопки недоступны; при `rows = 0` страниц SHALL быть ноль (без деления на ноль).

#### Scenario: Пусто
- **WHEN** `total_records = 0`
- **THEN** ссылок нет, все кнопки недоступны, отчёт «0 of 0»

### Requirement: Рендер без браузера
Вне браузера (SSR) `Paginator` SHALL отрисовываться полностью; `window` и `document` SHALL не использоваться.

#### Scenario: SSR
- **WHEN** страница с `Paginator` отрисована на сервере
- **THEN** в разметке есть кнопки и ссылки на страницы, текущая выбрана
