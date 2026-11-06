# Design

## Context

Требования — в `specs/table`. Исходники PrimeNG 21.1.10: `table/table.ts` (`filter`, `filterGlobal`, `_filter`, `executeLocalFilter`, `ColumnFilter`, `EditableColumn`, `CellEditor`, `EditableRow`, `RowEditor`, `initRowEdit/saveRowEdit/cancelRowEdit`, `createLazyLoadMetadata`), `api/filtermatchmode.ts`, `table/style/tablestyle.ts`.

## Decisions

### D1. Чистая фильтрация в `model.rs`
`FilterMeta { field, value, match_mode }`, `MatchMode` (десять режимов), `matches(text, value, mode)`, `set_filter`, `filter_rows(rows, filters, global, fields, text_of)`. Значение поля берётся функцией столбца (`Column::filterable(text_of)`). Регистр игнорируется; числовые режимы разбирают оба значения как `f64`.

### D2. Фильтры — данные столбца
`Column::filterable(text_of)` задаёт функцию значения; `filter_match_mode` — режим. Строка фильтров строится под заголовком, поле — `input.p-inputtext`. Сигнал `filters: RwSignal<Vec<FilterMeta>>`.

### D3. Правка без мутации
`Column::editable(editor)`, где редактор — `Callback<RwSignal<T>, AnyView>`, сигнал — черновик строки; редактор пишет в черновик, завершение отдаёт приложению новую строку. Готовый `Column::edit_text(get, set)` — текстовое поле. Правка ячеек — одна пара ключ+поле в состоянии; правка строк — `editing_keys` и черновики по ключам.

### D4. Lazy
`lazy` отключает внутренние сортировку, фильтрацию и срез; `LazyLoadEvent` собирается из сигналов `first`, `rows`, `sort`, `filters`, `global_filter`; `Effect` под `cfg(wasm32)` вызывает `on_lazy_load` при изменении, в том числе при монтировании.

### D5. Тесты
Модель — таблицы. SSR: строка фильтров, фильтрация, начальное состояние редакторов, lazy. Браузерные: ввод, очистка, правка ячейки (Enter/Escape/blur), правка строки, lazy. e2e и бенчмарки как для остальных компонентов.

## Risks / Trade-offs

- [Размер] Новых зависимостей нет: поля — обычные `input` с классами `p-inputtext`.
- [Отличие] Нет меню фильтра, `in`/`between`/датовых режимов — позже.

## Migration Plan

Миграции нет: новые свойства необязательны. Откат — `git revert`.
