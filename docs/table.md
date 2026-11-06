# Table (этап 2, пакет 1)

Порт `p-table` (DataTable) из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`). Этап 1 — основная таблица;
**этап 2, пакет 1** добавил фильтры (по столбцам и глобальный), правку ячеек и строк и ленивую загрузку. Изменение
размера, перестановка и заморозка столбцов, группировка, виртуальная прокрутка и перестановка строк — пакеты 2–3
(список ниже).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-table::Table<T>`, `Column<T>`, `ColumnKind`, `TablePt`, `PaginatorPosition`, `PageEvent`, `EditMode`, `EditEvent<T>`, `LazyLoadEvent` | `packages/primeng/src/table/table.ts` (`Table`, `SortableColumn`, `SortIcon`, `TableCheckbox`, `TableHeaderCheckbox`, `TableRadioButton`, `RowToggler`), `table/style/tablestyle.ts` |
| Сортировка, выбор, раскрытие, фильтры | `prime-table::model` — чистая логика `sort`, `multisortField`, `handleRowClick`, `toggleRowsWithCheckbox`, `toggleRow`, `filter`/`executeLocalFilter` (`MatchMode`, `FilterMeta`, `matches`, `filter_rows`), табличные тесты |
| Пагинатор, срез страницы | `prime-paginator::Paginator`, `prime_dataview::data::{page, clamp_first}` |
| CSS | `@primeuix/styles/datatable` (`datatable` в `theme.components`) |
| Демонстрации | `apps/showcase/doc/table/*-doc.ts` (`basic`, `singlecolumnsort`, `multiplecolumnssort`, `paginatorbasic`, `singleselection`, `multipleselection`, `checkboxselection`, `radiobuttonselection`, `rowexpansion`, `striped`/`gridlines`/`size`, `verticalscroll`, `loadingmask`; пакет 1: `filterrow`, `filterglobal`, `celledit`, `rowedit`, `lazyload`) |

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

### Фильтры, правка, ленивая загрузка (пакет 1)

```rust
let columns = vec![
    Column::text("name", "Name", |p: &Product| p.name.clone())
        .filterable(|p| p.name.clone())                       // поле в строке фильтров, режим по умолчанию startsWith
        .edit_text(|p| p.name.clone(), |p, v| Product { name: v, ..p.clone() }), // редактор ячейки
    Column::text("qty", "Qty", |p: &Product| p.quantity.to_string()).filterable(|p| p.quantity.to_string()).filter_match_mode(MatchMode::Gte),
    Column::row_editor(),
];
view! { <Table value=rows columns=columns row_key=key edit_mode=Some(EditMode::Cell) on_edit_complete=save global_filter=search /> }
```

- Фильтры: поле ввода у столбцов `filterable` во второй строке `thead` (`p-datatable-filter`), режимы `startsWith`, `contains`, `notContains`, `endsWith`, `equals`, `notEquals`, `lt`, `lte`, `gt`, `gte` (числовые — по числу), кнопка очистки, возврат на первую страницу; `filters` — двусторонний сигнал. Глобальный фильтр — `global_filter` (режим «содержит») по `global_filter_fields` или по столбцам с текстом для фильтра (`filterable`/`searchable`).
- Правка: `edit_mode = Cell` — клик по ячейке с `editable` открывает редактор (`p-editable-column`, `p-cell-editing`), `Enter`/потеря фокуса завершают, `Escape` отменяет; `edit_mode = Row` — столбец `row_editor` с кнопками «изменить / сохранить / отменить» (`p-datatable-row-editor-*`), `editing_keys` — двусторонний сигнал.
- `lazy`: сортировку, фильтры и страницы выполняет приложение по `on_lazy_load(LazyLoadEvent)`; число записей — `total_records`.

## Отличия

| Отличие | Причина |
|---|---|
| Столбцы описываются данными (`Column<T>`: `field`, `header`, `body`, `sortable` + `compare`, `footer`), а не шаблонами `ng-template` с директивами `pSortableColumn`, `pSelectableRow` | В Leptos нет шаблонов Angular; таблица собирает заголовок, строки и подвал сама. |
| Сортировка — функцией сравнения столбца, а не именем поля (`resolveFieldData`) | Строки типизированы. |
| Сортируется копия данных, `value` не изменяется | Сигналы приложения не мутируются. |
| Выбор хранит ключи строк (`Vec<String>`), а не объекты; `row_key` обязателен | Тип `T` произвольный, ключ задаёт приложение. |
| Правка не меняет `value`: результат приходит в `EditEvent { original, row, field }`, приложение обновляет данные само; редактор получает черновик строки (`RwSignal<T>`) | Данные приложения не мутируются. |
| Фильтр столбца — только строка в `thead` с полем ввода (`display="row"`), режим сравнения задаёт `Column::filter_match_mode` без меню; фильтр применяется сразу, без `filterDelay` | Меню фильтра — следующие пакеты. |
| Значение для фильтра берётся функцией столбца (`filterable`/`searchable`), а не `resolveFieldData` | Строки типизированы. |
| Нет диапазона выбора по Shift, `selectionAutoFocus`, контекстного меню, `rowSelectable`, `compareSelectionBy` | Этап 2+. |
| Нет `removableSort` | Его нет и в исходнике 21.1.10. |
| Нет `stateStorage`, `customSort`, `resetScrollTop`, `getBlockableElement` | Этап 2+ / вне объёма порта. |

## Не вошло

Меню фильтра (`display="menu"`) с несколькими условиями и операторами И/ИЛИ, режимы `in`, `between`, `is`, `before`/`after`, датовые, фильтр-выпадающий список и `filterDelay`; шаблонные редакторы по умолчанию (число, дата, список), `onEditInit` с отменой, клавиша Tab между ячейками; изменение размера и перестановка столбцов, перетаскивание строк, заморозка столбцов и строк, виртуальная прокрутка, группировка строк (в том числе по `rowspan`), `lazyLoadOnInit`, экспорт CSV, меню переключения столбцов, сохранение состояния, `p-table` внутри `ContextMenu`, клавиатурная навигация по строкам.

## Что не проверялось

- Чтение с экрана: `aria-*` сверены с PrimeNG, поведение скринридеров — нет.
- Таблицы с тысячами строк (без виртуальной прокрутки они строятся целиком).

## Производительность

См. `benches/` (`table-1`, `table-1000`, `case-table`) и `benches/*budgets.toml`.
