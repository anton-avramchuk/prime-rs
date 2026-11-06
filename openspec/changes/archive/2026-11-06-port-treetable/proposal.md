# Proposal

## Why

TreeTable — иерархические данные в таблице: узлы с потомками, раскрытие, сортировка по уровням, выбор. Последний компонент волны 7. По решению пользователя — **этап 1** на общей основе с Table (`prime_table::Column`, `prime_table::model`): без фильтров, правки, изменения и перестановки столбцов, заморозки и ленивой загрузки.

## What Changes

- Новый крейт `prime-treetable` (порт `p-treetable` из PrimeNG 21.1.10, этап 1): обобщённый `TreeTable<T>`, узел `TreeNode<T>` (ключ, данные, дети, `leaf`, начальное `expanded`).
- Чистая модель `tree.rs`: `sort_nodes` (по уровням, устойчиво), `page_roots` (пагинатор считает корни), `flatten` (развёртка в строки с уровнем, позицией и признаками), `checkbox_toggle` (распространение вниз и вверх), `check_state` (выбран/частично/нет), `initial_expanded`; табличные тесты.
- Разметка: `div.p-treetable.p-component` (`-gridlines`, `-hoverable-rows`, `-flex-scrollable`, `-sm`, `-lg`) > `p-treetable-mask`, `p-treetable-header`, пагинатор `p-treetable-paginator-top|bottom`, `p-treetable-table-container` > `table.p-treetable-table[role=treegrid]` (`thead`, `tbody`, `tfoot`), `p-treetable-footer`; кнопка раскрытия `p-treetable-toggler` с отступом `уровень × 16px`, флажок узла `p-treetable-node-checkbox`.
- Поведение: раскрытие и сворачивание кликом (`expanded_keys` или флаги узлов), сортировка (одиночная и многоуровневая, значки, счётчики, `aria-sort`), пагинатор по корневым узлам, выбор кликом (`single`, `multiple`, `meta_key_selection`) и флажками с частичным состоянием, пустое сообщение, индикатор загрузки; события узлов.
- Страница `/treetable`, e2e, бенчмарки и бюджеты, `docs/treetable.md`; `treetable` в `theme.components` и `primeng_extra`.
- **Не входит (этапы 2+):** фильтры, правка ячеек, изменение размера и перестановка столбцов, заморозка, виртуальная прокрутка, ленивая загрузка, флажок в заголовке, диапазон выбора Shift, контекстное меню, клавиатурная навигация по строкам.

## Capabilities

### New Capabilities
- `treetable`: иерархическая таблица с раскрытием, сортировкой, пагинатором и выбором.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-treetable` (`publish = false`); зависимости: `prime-core`, `prime-table` (столбцы и модель), `prime-dataview` (`clamp_first`), `prime-paginator`, `prime-checkbox`, `prime-badge`, `prime-icons`.
- Изменяются `Cargo.toml` workspace, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml`.
