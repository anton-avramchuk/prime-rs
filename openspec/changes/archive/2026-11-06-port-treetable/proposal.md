# Proposal

## Why

TreeTable — hierarchical data in a table: nodes with descendants, expansion, per-level sorting, selection. The last component of wave 7. Per the user decision — **stage 1** on the shared basis with Table (`prime_table::Column`, `prime_table::model`): without filters, editing, column resizing and reordering, freezing and lazy loading.

## What Changes

- New crate `prime-treetable` (port of `p-treetable` from PrimeNG 21.1.10, stage 1): generic `TreeTable<T>`, node `TreeNode<T>` (key, data, children, `leaf`, initial `expanded`).
- Pure model `tree.rs`: `sort_nodes` (per level, stable), `page_roots` (the paginator counts roots), `flatten` (flattening into rows with level, position and flags), `checkbox_toggle` (propagation down and up), `check_state` (selected/partial/none), `initial_expanded`; table-driven tests.
- Markup: `div.p-treetable.p-component` (`-gridlines`, `-hoverable-rows`, `-flex-scrollable`, `-sm`, `-lg`) > `p-treetable-mask`, `p-treetable-header`, paginator `p-treetable-paginator-top|bottom`, `p-treetable-table-container` > `table.p-treetable-table[role=treegrid]` (`thead`, `tbody`, `tfoot`), `p-treetable-footer`; expand button `p-treetable-toggler` with indent `level × 16px`, node checkbox `p-treetable-node-checkbox`.
- Behavior: expanding and collapsing by click (`expanded_keys` or node flags), sorting (single and multi-level, icons, counters, `aria-sort`), paginator over root nodes, selection by click (`single`, `multiple`, `meta_key_selection`) and by checkboxes with a partial state, empty message, loading indicator; node events.
- Page `/treetable`, e2e, benchmarks and budgets, `docs/treetable.md`; `treetable` in `theme.components` and `primeng_extra`.
- **Out of scope (stages 2+):** filters, cell editing, column resizing and reordering, freezing, virtual scroll, lazy loading, header checkbox, Shift range selection, context menu, keyboard navigation over rows.

## Capabilities

### New Capabilities
- `treetable`: a hierarchical table with expansion, sorting, a paginator and selection.

### Modified Capabilities
<!-- None. -->

## Impact

- New crate `crates/prime-treetable` (`publish = false`); dependencies: `prime-rs-core`, `prime-table` (columns and model), `prime-dataview` (`clamp_first`), `prime-paginator`, `prime-checkbox`, `prime-badge`, `prime-icons`.
- Changed: workspace `Cargo.toml`, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml`.
