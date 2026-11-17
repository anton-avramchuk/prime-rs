# Tree

Port of PrimeNG 21.1.10 `p-tree` (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-tree::Tree`, `TreePt`, `TreeNode`, `TreeNodeEvent`, `TreeSelectionMode`, `TreeFilterMode`, `TreeLoadingMode` | `packages/primeng/src/tree/tree.ts` (`Tree` and `UITreeNode`), `tree/style/treestyle.ts` |
| Model (visible nodes, selection, propagation, filter, moves) | `prime-tree::state` — pure functions with table-driven tests |
| CSS | `@primeuix/styles/tree` and the Aura preset — `cargo xtask gen-theme` (there is no `/* For PrimeNG */` block) |
| Demos | `apps/showcase/doc/tree/*-doc.ts` (`basic`, `single`, `multiple`, `checkbox`, `controlled`, `event`, `filter`, `lazy`, `template`) |

Spec: `openspec/specs/tree`. State is stored not in the nodes but in key signals: `expanded_keys: RwSignal<HashSet<String>>` and `selection: RwSignal<Vec<String>>` (in `Single` — 0 or 1 key); initial expansion is taken from `TreeNode::expanded`.

```rust
let selection = RwSignal::new(Vec::<String>::new());
view! { <Tree value=nodes selection_mode=TreeSelectionMode::Checkbox selection=selection /> }
```

## What matches PrimeNG

- Markup and classes: `div.p-tree.p-component` (`p-tree-selectable`, `p-tree-loading`, `p-tree-flex-scrollable`) > mask, filter field, `div.p-tree-root` > `ul.p-tree-root-children[role=tree]` > `li.p-tree-node[role=treeitem]` (`p-tree-node-leaf`, `aria-*`, `data-id`) > `div.p-tree-node-content` (`-selectable`, `-selected`, indent `level × indentation` rem) > button, checkbox, icon, label + `ul.p-tree-node-children[role=group]`; `div.p-tree-empty-message`.
- Selection: `Single` (a repeated click deselects), `Multiple`, `Checkbox` with `propagate_selection_up/down` (an ancestor is selected when all its children are selected; the partial state is `data-p-partialchecked` and a minus in the checkbox), `meta_key_selection`, `selectable = false`, `highlight_on_select`; a click on the toggle button does not change the selection.
- Filter: a port of `_filter`/`isFilterMatched`/`findFilteredNodes` (`Lenient` and `Strict` modes, comma-separated `filter_by` fields), ancestors of matching nodes are expanded.
- Keyboard: roving `tabindex` (focused, otherwise selected, otherwise the first node), ↓ ↑ over visible nodes, → expands a collapsed node and moves to its first child (does nothing on an expanded one), ← collapses an expanded node or goes to the parent, `Enter`/`Space` select; keys are accepted only on the `li` itself.

## Differences

| Difference | Reason |
|---|---|
| Expansion and selection live in key signals; partial selection is computed from the selection | PrimeNG writes them into the nodes (`expanded`, `partialSelected`), ours keeps the model immutable. |
| The filter ignores diacritics, only case | No dependency for `removeAccents`. |
| Selection is shared between the full and the filtered tree (by keys) | PrimeNG looks the selection up in node copies. |
| `Home`/`End` move to the first/last visible node | PrimeNG has no such keys. |
| Lazy loading is on the user's side: `on_node_expand` updates the `value` signal (`loading`, `children`) | No `onLazyLoad` feed; `TreeNode::leaf(false)` makes a node expandable without children. |
| Node icons are a `ViewFn` (`icon`, `expanded_icon`, `collapsed_icon`) | No PrimeIcons. |
| No drag and drop, virtual scroll, `contextMenu`, `trackBy`, `filterOptions`, `header`/`footer`/`loader`/`checkboxicon`/`loadingicon`/`filtericon` templates, `Ripple`, `Bind`, `pTemplate`; touch (`nodeTouched`) is not handled | Out of the port's scope; a context menu is attached via `on:contextmenu` and `ContextMenu`. |
| One node template (`node`), the look is picked by `TreeNode::type_` | PrimeNG has a dictionary of templates by type. |

## Not verified

- Right-to-left layout and touch input.
- Screen reading: `aria-*` were checked against PrimeNG, screen reader behaviour was not.
- Large trees: the view is rebuilt when `value` and the filter change, there is no virtual scroll.

## Performance

See `benches/` (`tree-1`, `tree-1000`, `case-tree`) and `benches/*budgets.toml`.
