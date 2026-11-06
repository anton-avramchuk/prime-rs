# Proposal

## Why

Table (DataTable) — главный компонент данных PrimeNG: таблица с сортировкой, выбором, раскрытием строк и пагинатором. Второй компонент волны 7. По решению пользователя делается **этап 1** (основная таблица); фильтры, правка, изменение и перестановка столбцов, заморозка, виртуальная прокрутка и группировка — этапы 2+.

## What Changes

- Новый крейт `prime-table` (порт `p-table` из PrimeNG 21.1.10, этап 1): обобщённый `Table<T>`.
- Столбцы описываются данными: `Column<T>` (`field`, `header`, `footer`, `body`, `sortable` + `compare`, `style`, `style_class`, `kind`), виды: данные, флажок выбора, радиокнопка выбора, кнопка раскрытия.
- Чистая модель `model.rs`: `toggle_sort` (одиночная и многоуровневая сортировка, Ctrl/⌘), `sort_rows` (устойчивая, многоуровневая), `row_click` (выбор кликом, `meta_key_selection`), `toggle_all`, `all_selected`, `toggle_expanded`; табличные тесты.
- Разметка: `div.p-datatable.p-component` (`-hoverable`, `-striped`, `-gridlines`, `-sm`, `-lg`, `-scrollable`, `-flex-scrollable`) > `p-datatable-mask`, `p-datatable-header`, пагинатор `p-datatable-paginator-top|bottom`, `p-datatable-table-container` > `table.p-datatable-table` (`thead`, `tbody`, `tfoot`), `p-datatable-footer`.
- Поведение: сортировка кликом и `Enter`/`Space` по заголовку (значки, счётчики порядка, `aria-sort`), пагинатор с возвратом на первую страницу при сортировке, выбор строк кликом (одиночный, множественный, с Ctrl/⌘), флажки и радиокнопки, флажок «выбрать все», раскрытие строк (в том числе только одной), пустое сообщение, индикатор загрузки.
- Страница `/table`, e2e, бенчмарки и бюджеты, `docs/table.md`; `datatable` в `theme.components` и `primeng_extra`; `xtask gen-theme` умеет каталог исходников, отличный от имени модуля стилей (`datatable` → `table`).
- **Не входит (этапы 2+):** фильтры, правка ячеек и строк, изменение размера и перестановка столбцов, перетаскивание строк, заморозка, виртуальная прокрутка, группировка, `lazy`, экспорт, меню столбцов, состояние, диапазон выбора Shift, клавиатурная навигация по строкам.

## Capabilities

### New Capabilities
- `table`: таблица данных с сортировкой, выбором, раскрытием строк и пагинатором.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-table` (`publish = false`); зависимости: `prime-core`, `prime-dataview` (срез страницы), `prime-paginator`, `prime-checkbox`, `prime-radiobutton`, `prime-badge`, `prime-button`, `prime-icons`.
- Изменяются `Cargo.toml` workspace, `upstream.toml`, `xtask/src/gen_theme.rs`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml`.
