# Table (этап 1)

Порт `p-table` (DataTable) из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`). Это **этап 1**: основная таблица
без фильтров, правки ячеек, перестановки и изменения размера столбцов, заморозки, виртуальной прокрутки и группировки
(они — этапы 2+, список ниже).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-table::Table<T>`, `Column<T>`, `ColumnKind`, `TablePt`, `PaginatorPosition`, `PageEvent` | `packages/primeng/src/table/table.ts` (`Table`, `SortableColumn`, `SortIcon`, `TableCheckbox`, `TableHeaderCheckbox`, `TableRadioButton`, `RowToggler`), `table/style/tablestyle.ts` |
| Сортировка, выбор, раскрытие | `prime-table::model` — чистая логика `sort`, `multisortField`, `handleRowClick`, `toggleRowsWithCheckbox`, `toggleRow`, табличные тесты |
| Пагинатор, срез страницы | `prime-paginator::Paginator`, `prime_dataview::data::{page, clamp_first}` |
| CSS | `@primeuix/styles/datatable` (`datatable` в `theme.components`) |
| Демонстрации | `apps/showcase/doc/table/*-doc.ts` (`basic`, `singlecolumnsort`, `multiplecolumnssort`, `paginatorbasic`, `singleselection`, `multipleselection`, `checkboxselection`, `radiobuttonselection`, `rowexpansion`, `striped`/`gridlines`/`size`, `verticalscroll`, `loadingmask`) |

Спека: `openspec/specs/table`. Строки определяются ключами (`row_key`, как `dataKey`); выбор и раскрытие хранят ключи.

```rust
let columns = vec![
    Column::text("name", "Name", |p: &Product| p.name.clone()).sortable(|a, b| a.name.cmp(&b.name)),
    Column::text("qty", "Quantity", |p: &Product| p.quantity.to_string()),
];
view! { <Table value=products columns=columns row_key=Callback::new(|p: Product| p.code) paginator=true default_rows=10usize /> }
```

## Что совпадает с PrimeNG

- Разметка и классы: `p-datatable p-component` (`-hoverable`, `-striped`, `-gridlines`, `-sm`, `-lg`, `-scrollable`, `-flex-scrollable`), `p-datatable-mask`, `p-datatable-header`, `p-datatable-table-container` (`overflow: auto`, `max-height`), `p-datatable-table`, `p-datatable-thead`/`tbody`/`tfoot`, `p-datatable-footer`, `p-datatable-empty-message`, `p-datatable-row-expansion`, `p-datatable-sortable-column` (`-column-sorted`), `p-datatable-sort-icon`, `p-sortable-column-badge`, `p-datatable-selectable-row` (`-row-selected`), `p-datatable-row-toggle-button`.
- Сортировка: одиночная (повторный клик меняет порядок) и многоуровневая (Ctrl/⌘ добавляет столбец, счётчики порядка), клавиши `Enter`/`Space` на заголовке, возврат на первую страницу при сортировке.
- Выбор: одиночный и множественный кликом (с `meta_key_selection` и без), столбцы флажков и радиокнопок, флажок «выбрать все» (по всем строкам либо только страницы).
- Раскрытие строк (в том числе только одной) и содержимое раскрытия; пагинатор, `striped_rows`, `show_gridlines`, `size`, вертикальная прокрутка, индикатор загрузки, пустое сообщение, `caption`, `summary`, подвал столбцов.

## Отличия

| Отличие | Причина |
|---|---|
| Столбцы описываются данными (`Column<T>`: `field`, `header`, `body`, `sortable` + `compare`, `footer`), а не шаблонами `ng-template` с директивами `pSortableColumn`, `pSelectableRow` | В Leptos нет шаблонов Angular; таблица собирает заголовок, строки и подвал сама. |
| Сортировка — функцией сравнения столбца, а не именем поля (`resolveFieldData`) | Строки типизированы. |
| Сортируется копия данных, `value` не изменяется | Сигналы приложения не мутируются. |
| Выбор хранит ключи строк (`Vec<String>`), а не объекты; `row_key` обязателен | Тип `T` произвольный, ключ задаёт приложение. |
| Нет диапазона выбора по Shift, `selectionAutoFocus`, контекстного меню, `rowSelectable`, `compareSelectionBy` | Этап 2+. |
| Нет `removableSort` | Его нет и в исходнике 21.1.10. |
| Нет `lazy`, `stateStorage`, `customSort`, `resetScrollTop`, `getBlockableElement` | Этап 2+ / вне объёма порта. |

## Не вошло в этап 1

Фильтры (по столбцам и глобальный), правка ячеек и строк, изменение размера и перестановка столбцов, перетаскивание строк, заморозка столбцов и строк, виртуальная прокрутка, группировка строк (в том числе по `rowspan`), `lazy`, экспорт CSV, меню переключения столбцов, сохранение состояния, `p-table` внутри `ContextMenu`, клавиатурная навигация по строкам.

## Что не проверялось

- Чтение с экрана: `aria-*` сверены с PrimeNG, поведение скринридеров — нет.
- Таблицы с тысячами строк (без виртуальной прокрутки они строятся целиком).

## Производительность

См. `benches/` (`table-1`, `table-1000`, `case-table`) и `benches/*budgets.toml`.
