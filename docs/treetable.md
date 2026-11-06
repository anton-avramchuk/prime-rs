# TreeTable (этап 1)

Порт `p-treetable` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`). Это **этап 1**: иерархическая таблица
без фильтров, правки ячеек, изменения и перестановки столбцов, заморозки и ленивой загрузки (они — этапы 2+, список ниже).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-treetable::TreeTable<T>`, `TreeNode<T>`, `TreeSelectionMode`, `TreeTablePt` (столбцы — `prime_table::Column<T>`) | `packages/primeng/src/treetable/treetable.ts` (`TreeTable`, `TreeTableToggler`, `TTCheckbox`, `TTSortableColumn`), `treetable/style/treetablestyle.ts` |
| Сортировка по уровням, страница корней, развёртка в строки, флажки с распространением | `prime-treetable::tree` — чистая логика `sortNodes`, `serializePageNodes`, `serializeNodes`, `propagateSelectionUp/Down`, табличные тесты |
| Сортировка, выбор кликом, раскрытие | `prime_table::model` (общая с Table) |
| CSS | `@primeuix/styles/treetable` и блок `/* For PrimeNG */` (`treetable` в `theme.components` и `primeng_extra`) |
| Демонстрации | `apps/showcase/doc/treetable/*-doc.ts` (`basic`, `controlled`, `sortsinglecolumn`, `paginatorbasic`, `selectionsingle`, `selectionmultiple`, `selectioncheckbox`, `gridlines`/`size`, `loadingmask`) |

Спека: `openspec/specs/treetable`. Узлы определяются ключами (`TreeNode::key`, как `dataKey`); выбор и раскрытие хранят ключи.

```rust
let nodes = vec![TreeNode::new("docs", File::folder("Documents")).children(vec![TreeNode::new("cv", File::file("Resume.pdf"))])];
view! { <TreeTable value=nodes columns=columns selection_mode=Some(TreeSelectionMode::Checkbox) /> }
```

## Что совпадает с PrimeNG

- Разметка и классы: `p-treetable p-component` (`-gridlines`, `-hoverable-rows`, `-flex-scrollable`, `-sm`, `-lg`), `p-treetable-mask`, `-header`, `-table-container`, `-table`, `-thead`, `-tbody`, `-tfoot`, `-footer`, `p-treetable-toggler` (видимость по наличию детей, отступ `уровень × 16px`), `p-treetable-node-checkbox`, `p-treetable-row-selected`, `p-sortable-column` (`p-treetable-column-sorted`), `p-treetable-sort-icon`; `aria-expanded`, `aria-level`, `aria-posinset`, `aria-setsize`.
- Пагинатор считает **корневые** узлы (`serializePageNodes`); потомки раскрытых узлов выводятся в странице корня.
- Сортировка действует на соседей каждого уровня, одиночная и многоуровневая (Ctrl/⌘), возврат на первую страницу.
- Выбор: одиночный и множественный кликом (с `meta_key_selection`), флажки с распространением вниз и вверх и частичным состоянием.

## Отличия

| Отличие | Причина |
|---|---|
| Узлы — `TreeNode<T>` с ключом, данные `T` типизированы; раскрытие хранится ключами (`expanded_keys`), а не флагом `node.expanded` в данных | Данные приложения не мутируются; начальное раскрытие берётся из `TreeNode::expanded`, если сигнал не передан. |
| Столбцы — `prime_table::Column<T>` (используются столбцы данных); кнопка раскрытия и флажок выводятся в столбце `toggler_column` | Нет шаблонов `ng-template` с директивами. |
| Сортировка — функцией сравнения столбца, а не именем поля | Данные типизированы. |
| Нет диапазона выбора Shift, контекстного меню, `selectionKeys`, `selectionPageOnly`, флажка «выбрать все» в заголовке | Этап 2+. |
| Нет `lazy`/`onNodeExpand` с догрузкой потомков, `leaf` учитывается только для видимости кнопки | Этап 2+. |
| Нет `autoLayout`, изменения размера и перестановки столбцов, заморозки, виртуальной прокрутки, `stateStorage` | Этап 2+. |

## Не вошло в этап 1

Фильтры (по столбцам и глобальный), правка ячеек, изменение размера и перестановка столбцов, заморозка, виртуальная прокрутка, ленивая загрузка, флажок в заголовке, диапазон выбора, контекстное меню, клавиатурная навигация по строкам, `toggleButtonProps`/шаблон значка раскрытия.

## Что не проверялось

- Чтение с экрана: `aria-*` сверены с PrimeNG, поведение скринридеров — нет.
- Деревья с тысячами узлов (строятся целиком, без виртуальной прокрутки).

## Производительность

См. `benches/` (`treetable-1`, `treetable-1000`, `case-treetable`) и `benches/*budgets.toml`.
