# Tree

Порт `p-tree` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-tree::Tree`, `TreePt`, `TreeNode`, `TreeNodeEvent`, `TreeSelectionMode`, `TreeFilterMode`, `TreeLoadingMode` | `packages/primeng/src/tree/tree.ts` (`Tree` и `UITreeNode`), `tree/style/treestyle.ts` |
| Модель (видимые, выбор, распространение, фильтр, переходы) | `prime-tree::state` — чистые функции с табличными тестами |
| CSS | `@primeuix/styles/tree` и пресет Aura — `cargo xtask gen-theme` (блока `/* For PrimeNG */` нет) |
| Демонстрации | `apps/showcase/doc/tree/*-doc.ts` (`basic`, `single`, `multiple`, `checkbox`, `controlled`, `event`, `filter`, `lazy`, `template`) |

Спека: `openspec/specs/tree`. Состояние хранится не в узлах, а в сигналах ключей: `expanded_keys: RwSignal<HashSet<String>>` и `selection: RwSignal<Vec<String>>` (в `Single` — 0 или 1 ключ); начальное раскрытие берётся из `TreeNode::expanded`.

```rust
let selection = RwSignal::new(Vec::<String>::new());
view! { <Tree value=nodes selection_mode=TreeSelectionMode::Checkbox selection=selection /> }
```

## Что совпадает с PrimeNG

- Разметка и классы: `div.p-tree.p-component` (`p-tree-selectable`, `p-tree-loading`, `p-tree-flex-scrollable`) > маска, поле фильтра, `div.p-tree-root` > `ul.p-tree-root-children[role=tree]` > `li.p-tree-node[role=treeitem]` (`p-tree-node-leaf`, `aria-*`, `data-id`) > `div.p-tree-node-content` (`-selectable`, `-selected`, отступ `уровень × indentation` rem) > кнопка, флажок, значок, подпись + `ul.p-tree-node-children[role=group]`; `div.p-tree-empty-message`.
- Выбор: `Single` (повторный клик снимает), `Multiple`, `Checkbox` с `propagate_selection_up/down` (предок выбран, когда выбраны все дети; частичное состояние — `data-p-partialchecked` и минус в флажке), `meta_key_selection`, `selectable = false`, `highlight_on_select`; клик по кнопке раскрытия выбор не меняет.
- Фильтр: порт `_filter`/`isFilterMatched`/`findFilteredNodes` (режимы `Lenient` и `Strict`, поля `filter_by` через запятую), предки совпавших узлов раскрываются.
- Клавиатура: roving `tabindex` (сфокусированный, иначе выбранный, иначе первый узел), ↓ ↑ по видимым узлам, → раскрывает свёрнутый узел и переходит к его первому ребёнку (у раскрытого ничего не делает), ← сворачивает раскрытый либо идёт к родителю, `Enter`/`Space` выбирают; клавиши принимаются только на самом `li`.

## Отличия

| Отличие | Причина |
|---|---|
| Раскрытие и выбор — в сигналах ключей; частичный выбор вычисляется из выбора | В PrimeNG они пишутся в узлы (`expanded`, `partialSelected`), у нас модель неизменяема. |
| Фильтр не учитывает диакритику, только регистр | Нет зависимости для `removeAccents`. |
| Выбор общий для полного и отфильтрованного дерева (по ключам) | В PrimeNG выбор ищется в копиях узлов. |
| `Home`/`End` переходят к первому/последнему видимому узлу | В PrimeNG таких клавиш нет. |
| Ленивая загрузка — на стороне пользователя: `on_node_expand` обновляет сигнал `value` (`loading`, `children`) | Нет `onLazyLoad` ленты; `TreeNode::leaf(false)` делает узел раскрываемым без детей. |
| Значки узла — `ViewFn` (`icon`, `expanded_icon`, `collapsed_icon`) | Нет PrimeIcons. |
| Нет перетаскивания, виртуальной прокрутки, `contextMenu`, `trackBy`, `filterOptions`, шаблонов `header`/`footer`/`loader`/`checkboxicon`/`loadingicon`/`filtericon`, `Ripple`, `Bind`, `pTemplate`; касание (`nodeTouched`) не учитывается | Вне объёма порта; контекстное меню подключается через `on:contextmenu` и `ContextMenu`. |
| Шаблон узла один (`node`), вид выбирается по `TreeNode::type_` | У PrimeNG словарь шаблонов по типам. |

## Что не проверялось

- Раскладка справа налево и сенсорный ввод.
- Чтение с экрана: `aria-*` сверены с PrimeNG, поведение скринридеров — нет.
- Большие деревья: вид перестраивается при смене `value` и фильтра, виртуальной прокрутки нет.

## Производительность

См. `benches/` (`tree-1`, `tree-1000`, `case-tree`) и `benches/*budgets.toml`.
