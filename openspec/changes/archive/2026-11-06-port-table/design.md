# Design

## Context

Требования — в `specs/table`. Исходники PrimeNG 21.1.10: `table/table.ts` (6613 строк; этап 1 берёт `sort`, `sortSingle`, `sortMultiple`, `handleRowClick`, `toggleRowWithCheckbox`, `toggleRowsWithCheckbox`, `toggleRow`, `SortableColumn`, `SortIcon`), `table/style/tablestyle.ts` (классы и блок `For PrimeNG`), `@primeuix/styles/datatable`.

## Decisions

### D1. Столбцы — данные, не шаблоны
В PrimeNG заголовок и тело собираются шаблонами `ng-template` с директивами. У нас `Column<T>` (поле, заголовок, ячейка-функция, сортировка с `compare`, вид столбца); таблица строит заголовок, строки и подвал сама. Обобщённый `Table<T: Clone + Send + Sync + 'static>`.

### D2. Чистая модель (`model.rs`)
Сортировка (`toggle_sort`, `sort_rows`), выбор (`row_click`, `toggle_all`, `all_selected`) и раскрытие (`toggle_expanded`) — функции над ключами `String`. Ключ строки задаёт приложение (`row_key`), как `dataKey`.

### D3. Состояние
Сигналы `selection`, `expanded_keys`, `sort`, `first`, `rows` (внешние или внутренние). Заголовки, строки и подвал строятся в реактивных замыканиях; классы и значки сортировки читают сигнал внутри ячейки, поэтому клик по заголовку не перестраивает таблицу, меняется только порядок строк. Флажок в заголовке и радиокнопки согласуются с `selection` эффектами под `cfg(wasm32)`; флажки строк — `Checkbox` с `model` = `selection` и `value` = ключ.

### D4. Тема
`datatable` в `theme.components` и `primeng_extra` (блок `For PrimeNG` даёт отступы ячеек заголовка, полосатость и сортировку). `xtask gen-theme` получил `primeng_dir`: каталог исходников `table` вместо `datatable`.

### D5. Тесты
Модель — таблицы. SSR: классы, заголовки и значки, порядок строк, пагинатор, выбор, флажки/радио, раскрытие, пусто/загрузка/подвал. Браузерные: сортировка, выбор, флажки, радио, Ctrl, раскрытие, страницы. e2e: стили темы и клики.

## Risks / Trade-offs

- [Объём] Этап 1 — основа; расширения (фильтры, правка и т. д.) добавляются новыми столбцами/свойствами без смены модели.
- [Размер] Таблица тянет Paginator (Select, InputNumber), Checkbox, RadioButton, Badge, Button: около 1,9 МБ wasm.

## Migration Plan

Миграции нет. Откат — `git revert`.
