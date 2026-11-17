# TreeTable (stage 1)

Port of `p-treetable` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`). This is **stage 1**: a hierarchical table
without filters, cell editing, column resizing and reordering, freezing and lazy loading (those are stages 2+, listed below).

| Our component | PrimeNG sources |
|---|---|
| `prime-treetable::TreeTable<T>`, `TreeNode<T>`, `TreeSelectionMode`, `TreeTablePt` (columns are `prime_table::Column<T>`) | `packages/primeng/src/treetable/treetable.ts` (`TreeTable`, `TreeTableToggler`, `TTCheckbox`, `TTSortableColumn`), `treetable/style/treetablestyle.ts` |
| Per-level sorting, root page, flattening into rows, checkboxes with propagation | `prime-treetable::tree` — pure logic `sortNodes`, `serializePageNodes`, `serializeNodes`, `propagateSelectionUp/Down`, table-driven tests |
| Sorting, click selection, expansion | `prime_table::model` (shared with Table) |
| CSS | `@primeuix/styles/treetable` and the `/* For PrimeNG */` block (`treetable` in `theme.components` and `primeng_extra`) |
| Demos | `apps/showcase/doc/treetable/*-doc.ts` (`basic`, `controlled`, `sortsinglecolumn`, `paginatorbasic`, `selectionsingle`, `selectionmultiple`, `selectioncheckbox`, `gridlines`/`size`, `loadingmask`) |

Spec: `openspec/specs/treetable`. Nodes are identified by keys (`TreeNode::key`, like `dataKey`); selection and expansion store keys.

```rust
let nodes = vec![TreeNode::new("docs", File::folder("Documents")).children(vec![TreeNode::new("cv", File::file("Resume.pdf"))])];
view! { <TreeTable value=nodes columns=columns selection_mode=Some(TreeSelectionMode::Checkbox) /> }
```

## What matches PrimeNG

- Markup and classes: `p-treetable p-component` (`-gridlines`, `-hoverable-rows`, `-flex-scrollable`, `-sm`, `-lg`), `p-treetable-mask`, `-header`, `-table-container`, `-table`, `-thead`, `-tbody`, `-tfoot`, `-footer`, `p-treetable-toggler` (visibility by presence of children, indent `level × 16px`), `p-treetable-node-checkbox`, `p-treetable-row-selected`, `p-sortable-column` (`p-treetable-column-sorted`), `p-treetable-sort-icon`; `aria-expanded`, `aria-level`, `aria-posinset`, `aria-setsize`.
- The paginator counts **root** nodes (`serializePageNodes`); descendants of expanded nodes are rendered within the root page.
- Sorting applies to siblings of every level, single and multi-level (Ctrl/⌘), return to the first page.
- Selection: single and multiple by click (with `meta_key_selection`), checkboxes with propagation down and up and a partial state.

## Differences

| Difference | Reason |
|---|---|
| Nodes are `TreeNode<T>` with a key, data `T` is typed; expansion is stored by keys (`expanded_keys`), not by the `node.expanded` flag in the data | App data is not mutated; the initial expansion comes from `TreeNode::expanded` if the signal is not passed. |
| Columns are `prime_table::Column<T>` (data columns are used); the expand button and checkbox are rendered in the `toggler_column` column | No `ng-template` templates with directives. |
| Sorting uses the column comparison function, not a field name | Data is typed. |
| No Shift range selection, context menu, `selectionKeys`, `selectionPageOnly`, header "select all" checkbox | Stage 2+. |
| No `lazy`/`onNodeExpand` with loading descendants on demand, `leaf` only affects button visibility | Stage 2+. |
| No `autoLayout`, column resizing and reordering, freezing, virtual scroll, `stateStorage` | Stage 2+. |

## Not included in stage 1

Filters (per column and global), cell editing, column resizing and reordering, freezing, virtual scroll, lazy loading, header checkbox, range selection, context menu, keyboard navigation over rows, `toggleButtonProps`/expand icon template.

## Not verified

- Screen reading: `aria-*` were checked against PrimeNG, screen reader behavior was not.
- Trees with thousands of nodes (built in full, without virtual scroll).

## Performance

See `benches/` (`treetable-1`, `treetable-1000`, `case-treetable`) and `benches/*budgets.toml`.
