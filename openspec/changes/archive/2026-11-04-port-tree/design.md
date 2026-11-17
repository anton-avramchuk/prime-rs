# Design

## Context

Requirements are in `specs/tree`. PrimeNG 21.1.10 sources: `tree/tree.ts` (`UITreeNode` — node and keyboard, `Tree` — selection, filter, expansion), `tree/style/treestyle.ts`; demos `apps/showcase/doc/tree/{basic,single,multiple,checkbox,controlled,event,filter,lazy,template}-doc.ts`. Reference implementations in the project are `prime-panelmenu` (expanded keys, focus by key, `PanelMenuRef`), `prime-multiselect` (checkbox and filter markup), `prime-iconfield`/`prime-inputtext`.

In PrimeNG state (`expanded`, `partialSelected`) is written into the nodes themselves, and focus is kept in the DOM (`tabindex` is moved manually). Ours keeps the model immutable and the state lives in signals.

## Goals / Non-Goals

**Goals:** the same DOM, classes, `aria-*`, selection, filter and keys.

**Non-Goals:** drag and drop, virtual scroll, `contextMenu`, `trackBy`, `header`/`footer`/`loader` templates, `filterOptions`.

## Decisions

### D1. State
`expanded: RwSignal<HashSet<String>>` (keys; initial ones from `TreeNode::expanded`, the external `expanded_keys` signal is two-way) and `selection: RwSignal<Vec<String>>` (keys; the external signal is two-way). Partial selection is computed from the selection (`partial(node) = not selected and at least one descendant is selected`), not stored.

### D2. Pure model (`state.rs`)
A flat list of nodes with key, parent, level and flags (`leaf`, `selectable`), built from `value`. Functions without `web_sys` and with table-driven tests: `visible(nodes, expanded)`, `toggle_select(...)` for each mode (including `meta`), `propagate_up/down`, `is_partial`, `filter_tree(nodes, text, by, strict)` (a port of `_filter`/`isFilterMatched`/`findFilteredNodes`; returns the filtered tree and the keys to expand), `next/prev/first/last`, `arrow_left/right`, `tab_stop`.

### D3. Filter
The field is an `InputText` in an `IconField` with `SearchIcon`; the text is a signal; the filtered tree is a `Memo` of `value` and the text. Keys expanded by the filter are added to `expanded` on input (like `node.expanded = true` on PrimeNG copies).

### D4. Markup and focus
A recursive per-level function; focus is a key signal, the `li` `tabindex` is reactive (`0` on the "anchor" node: the focused one, else the selected one, else the first); focus is moved with `focus()` by `id` (`<tree id>_<key>`) under `cfg(wasm32)`. Signals are read while building the view only via `*_untracked` (otherwise the whole tree is rebuilt and focus is lost).

### D5. Checkbox
The `p-checkbox` markup inside the node is our own (via `prime_checkbox::{CheckboxState, write_classes, data_p}`), as in MultiSelect; the partial state is `MinusIcon`, `data-p-partialchecked`.

### D6. Lazy loading
There is no built-in loader: `on_node_expand` receives the node and the user updates the `value` signal (`loading`, `children`). A spinner instead of the chevron — with `loading_mode = Icon`, a mask — when the tree itself is `loading`.

### D7. Tests
SSR: markup, ARIA, selection modes, expansion from the model, empty, templates, `pt`. Browser `tests/web.rs` as one sequential test: expansion, selection in all modes, checkbox propagation, filter, keyboard, events; after a key `dispatch` wait `sleep(10)`. e2e: theme layout, per-level indents, keyboard, filter, checkboxes.

## Risks / Trade-offs

- [Own model instead of mutable nodes] Divergence in partial selection → it is derived from the selection, as the result of `propagateUp`, table-driven tests.
- [The filter copies nodes] In PrimeNG selection works on the filtered copies; ours uses keys, so the selection is shared between the full and the filtered tree.
- [Large trees] The view is rebuilt when `value` and the filter change; expansion and selection are reactive at the attribute level. No virtual scroll.

## Open Questions

- How PrimeNG behaves on `Enter` on a node with `selection_mode = None` (nothing) — to verify.

## Migration Plan

No migration. Rollback — `git revert`.
