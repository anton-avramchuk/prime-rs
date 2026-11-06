# Tasks

## 1. Модель

- [x] 1.1 `model.rs`: `MatchMode`, `FilterMeta`, `matches`, `filter_rows`, табличные тесты
- [x] 1.2 Свериться с `executeLocalFilter`, `FilterMatchMode`, `createLazyLoadMetadata` исходника

## 2. Компонент

- [x] 2.1 Фильтры столбцов и глобальный фильтр: `Column::filterable`, строка фильтров, сигналы, события
- [x] 2.2 Правка ячеек: `Column::editable`, `edit_text`, события
- [x] 2.3 Правка строк: `Column::row_editor`, `editing_keys`, события
- [x] 2.4 `lazy`, `total_records`, `on_lazy_load`
- [x] 2.5 SSR-тесты и браузерные `tests/web.rs`

## 3. Showcase и e2e

- [x] 3.1 Демонстрации на `/table`: фильтры, глобальный фильтр, правка ячеек, правка строк, lazy
- [x] 3.2 `e2e/tests/table.spec.ts`

## 4. Бенчмарки и бюджеты

- [x] 4.1 Пересчитать бюджеты `table-1`, `table-1000`, `case-table`, wasm

## 5. Интеграция

- [x] 5.1 `docs/table.md`, `openspec/specs/table`
- [x] 5.2 fmt, clippy, тесты, deny, machete, `openspec validate port-table-filter-edit`
