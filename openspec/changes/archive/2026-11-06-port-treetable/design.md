# Design

## Context

Requirements are in `specs/treetable`. PrimeNG 21.1.10 sources: `treetable/treetable.ts` (`serializeNodes`, `serializePageNodes`, `sortNodes`, `toggleNodeWithCheckbox`, `propagateSelectionUp/Down`, `TreeTableToggler`, `TTCheckbox`), `style/treetablestyle.ts` (all the CSS is in the `For PrimeNG` block; the `@primeuix/styles/treetable` module provides tokens). The basis is Table (`prime_table::Column`, `prime_table::model`).

## Decisions

### D1. Nodes and keys
`TreeNode<T>` with a key; selection and expansion are key vectors (as in Table). The initial expansion comes from the `expanded` flags if `expanded_keys` is not passed.

### D2. Pure model (`tree.rs`)
`sort_nodes` recursively applies `prime_table::model::sort_rows` to siblings; `page_roots` + `flatten` produce the visible rows; `checkbox_toggle` and `check_state` implement selection propagation down/up and the partial state. Table-driven tests.

### D3. Component
Columns are `prime_table::Column<T>`; in the `toggler_column` column the expand button and (in `checkbox` mode) the node checkbox are rendered before the content — a `Checkbox` with `checked` and `indeterminate`, kept in sync with `selection` by an effect per row under `cfg(wasm32)`. Headers, rows and the footer are built in reactive closures; selection classes and sort icons read signals inside cells.

### D4. Theme
`treetable` in `theme.components` and `primeng_extra`.

### D5. Tests
Model — tables. SSR: collapsed tree, buttons and indents, sorting, root paginator, selection, checkboxes, empty/loading/footer. Browser: expand/collapse, per-level sorting, selection by click and Ctrl, checkboxes with propagation, paginator. e2e: styles and clicks.

## Risks / Trade-offs

- [Scope] Stage 1 is the foundation; filters, editing and lazy loading are added as new properties without changing the model.
- [Size] About 1.7 MB of wasm because of Paginator, Checkbox, Badge and the shared part of Table.

## Migration Plan

No migration. Rollback — `git revert`.
