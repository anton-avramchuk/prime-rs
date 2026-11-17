# TreeSelect

Port of `p-treeselect` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-treeselect::TreeSelect`, `TreeSelectPt`, `TreeSelectDisplay` | `packages/primeng/src/treeselect/treeselect.ts`, `treeselect/style/treeselectstyle.ts` |
| Panel | a ready-made `prime_tree::Tree` (shared selection and expanded keys signal) |
| CSS | `@primeuix/styles/treeselect`, the Aura preset and the `/* For PrimeNG */` block — `cargo xtask gen-theme` (`treeselect` in `primeng_extra`) |
| Demos | `apps/showcase/doc/treeselect/*-doc.ts` (`basic`, `multiple`, `checkbox`, `filter`, `clearicon`, `disabled`, `invalid`, `template`) |

Spec: `openspec/specs/treeselect`. The value is `RwSignal<Vec<String>>` of the keys of selected nodes (in `Single` — 0 or 1 key); nodes are `prime_tree::TreeNode`.

```rust
let value = RwSignal::new(Vec::<String>::new());
view! { <TreeSelect options=nodes value=value selection_mode=TreeSelectionMode::Multiple display=TreeSelectDisplay::Chip /> }
```

## What matches PrimeNG

- Markup and classes: `div.p-treeselect.p-component.p-inputwrapper` (states: `-display-chip`, `p-disabled`, `p-invalid`, `p-focus`, `p-variant-filled`, `p-inputwrapper-filled|focus`, `-open`, `-clearable`, `-fluid`, `-sm`, `-lg`) > hidden field `input[role=combobox][readonly]` > `p-treeselect-label-container` > `p-treeselect-label` (comma, chips, placeholder) > clear icon > `p-treeselect-dropdown` > panel `p-treeselect-overlay` > `p-treeselect-tree-container` (`max-height` 400px) > `Tree`.
- A click on the field opens/closes the panel; `Single` closes the panel after selection and returns focus; on open the ancestors of selected nodes are expanded; an outside click, `resize` and `Escape` close; after a node is expanded the panel is re-aligned.
- Keyboard of the hidden field: Down opens and focuses the first node, `Space`/`Enter` open, `Escape` closes, `Tab` closes.
- Filter, selection modes, `meta_key_selection`, checkbox propagation and the empty message work via `Tree`.

## Differences

| Difference | Reason |
|---|---|
| The value is keys, not nodes (in PrimeNG `Single` is a node, otherwise an array of nodes) | The `Tree` model is immutable and stores the selection by keys. |
| No Angular forms (`ngModel`, reactive forms), `appendTo`, virtual scrolling, `resetFilterOnHide` (the filter is reset on every close because the panel is recreated) | Outside the port's scope. |
| No hidden panel focus traps; `Tab` on the field closes the panel | Simplification; focus inside the panel leaves via tree keys and click. |
| No `header`/`footer`/`empty`/icon templates; there are `value_view` (label replacement) and `node` (node label) | No `pTemplate`. |
| The label of an empty field without a placeholder is the word `empty` under `p-treeselect-label-empty` (hidden by the theme) | Same in PrimeNG. |
| The panel is inside the root (`position: absolute`), like `Select` | In PrimeNG — a separate `p-overlay`. |

## Not verified

- Right-to-left layout and touch input.
- Screen reading: `aria-*` was checked against PrimeNG, screen reader behavior was not.
- Panel position at the bottom/right edge of the window (shared `align_overlay`).

## Performance

See `benches/` (`treeselect-1`, `treeselect-1000`, `case-treeselect`) and `benches/*budgets.toml`.
