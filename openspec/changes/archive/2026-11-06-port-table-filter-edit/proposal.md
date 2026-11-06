# Proposal

## Why

Этап 1 Table даёт сортировку, выбор, раскрытие и пагинатор. Для рабочих таблиц не хватает фильтрации, правки данных и серверной (ленивой) загрузки. По решению пользователя этап 2 делается тремя пакетами; это **пакет 1**: фильтры, правка ячеек и строк, `lazy`.

## What Changes

- Фильтры по столбцам в строке под заголовком (`filter_display = row`): поле ввода, режим сравнения (`startsWith`, `contains`, `notContains`, `endsWith`, `equals`, `notEquals`, `lt`, `lte`, `gt`, `gte`), кнопка очистки; двусторонний сигнал `filters`.
- Глобальный фильтр: сигнал `global_filter` + `global_filter_fields`; режим `contains`.
- Чистая модель `model.rs`: `FilterMeta`, `MatchMode`, `matches`, `filter_rows`; табличные тесты.
- Правка ячеек (`edit_mode = cell`): `Column::editable(...)` с редактором; клик по ячейке открывает редактор, `Enter`/потеря фокуса завершают, `Escape` отменяет; события `on_edit_init`, `on_edit_complete`, `on_edit_cancel`.
- Правка строк (`edit_mode = row`): столбец `Column::row_editor()` с кнопками «изменить / сохранить / отменить»; события `on_row_edit_init|save|cancel`.
- Данные не мутируются: результат правки приходит в событии с новой строкой, приложение обновляет `value`.
- `lazy`: `lazy`, `total_records`, `on_lazy_load(LazyLoadEvent { first, rows, sort, filters, global_filter })`; при `lazy` таблица не сортирует, не фильтрует и не режет страницы сама.
- Страница `/table` — новые демонстрации, e2e, бенчмарки, `docs/table.md`.
- **Не входит (пакеты 2–3 и далее):** меню фильтра (`display = menu`) с несколькими условиями, `in`/`between`/датовые режимы, фильтр-выпадающий список; изменение размера, перестановка, заморозка столбцов; группировка, виртуальная прокрутка, перестановка строк.

## Capabilities

### Modified Capabilities
- `table`: фильтры, правка ячеек и строк, ленивая загрузка.

## Impact

- `crates/prime-table` (`model.rs`, `lib.rs`, тесты), showcase, `e2e`, `benches/*` (бюджеты пересчитать), `docs/table.md`.
