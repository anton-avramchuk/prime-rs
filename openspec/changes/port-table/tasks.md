# Tasks

## 1. Основа

- [x] 1.1 `datatable` в `theme.components` и `primeng_extra`; `xtask gen-theme`: каталог исходников `table`; `cargo xtask gen-theme`
- [x] 1.2 Свериться с `sort`, `handleRowClick`, `toggleRowsWithCheckbox`, `toggleRow`, `SortableColumn`, `SortIcon` исходника

## 2. Крейт `prime-table`

- [x] 2.1 Модель `model.rs` с табличными тестами
- [x] 2.2 Компонент `Table<T>`, `Column<T>`, `TablePt`: разметка, заголовок, строки, подвал, пагинатор, SSR-тесты, rustdoc
- [x] 2.3 Сортировка (одиночная, многоуровневая, клавиши), выбор (клик, флажки, радио, заголовок), раскрытие, события
- [x] 2.4 Браузерные `tests/web.rs`

## 3. Showcase и e2e

- [x] 3.1 Страница `/table`, меню, маршрут, крейт в workspace и showcase
- [x] 3.2 `e2e/tests/pages.json` и `table.spec.ts`

## 4. Бенчмарки и бюджеты

- [x] 4.1 Сценарии `-1` и `-1000`, `case-table`, бюджеты

## 5. Интеграция

- [x] 5.1 `docs/table.md`; `prime-table` в браузерный job `ci.yml` (три места)
- [x] 5.2 fmt, clippy, тесты, deny, machete, `openspec validate port-table`

## Workflow follow-up

- Волна 7: после каждого компонента — коммит и архив локально, CI один раз в конце волны.
