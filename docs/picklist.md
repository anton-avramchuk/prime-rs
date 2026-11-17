# PickList

Port of `p-picklist` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-picklist::PickList`, `PickListPt` | `packages/primeng/src/picklist/picklist.ts`, `picklist/style/pickliststyle.ts` |
| Transfer between lists | `prime-picklist::transfer` — pure logic instead of `moveRight/AllRight/Left/AllLeft`, table tests |
| Reordering | `prime_orderlist::reorder` (shared with OrderList) |
| Lists, selection, search | `prime-listbox::Listbox` (in PrimeNG — `p-listbox`); a `header_content` property was added to `Listbox` for the headers |
| CSS | `@primeuix/styles/picklist` (`picklist` in `theme.components`) |
| Demos | `apps/showcase/doc/picklist/*-doc.ts` (`basic`, `filter`) |

Spec: `openspec/specs/picklist`. Source and target are `RwSignal<Vec<ListboxOption>>`, selection is `RwSignal<Vec<String>>`.

```rust
let (source, target) = (RwSignal::new(items), RwSignal::new(Vec::new()));
view! { <PickList source=source target=target source_header="Available" target_header="Selected" filter=true /> }
```

## What matches PrimeNG

- Markup: five blocks (`p-picklist-source-controls`, `-source-list-container`, `-transfer-controls`, `-target-list-container`, `-target-controls`), four buttons in each panel, `aria-label` from the translation.
- Transfer of the selected items in selection order without duplicates, "all" — only those visible under the search, clearing the source selection, `keep_selection`, double click on an item.
- Button disabled rules and `disabled`.

## Differences

| Difference | Reason |
|---|---|
| Selected items keep their relative order when reordering (see OrderList) | A fix for a PrimeNG bug. |
| No drag and drop (`dragdrop`) between lists | Built on Angular CDK; there is no shared drag-and-drop solution in the project yet. |
| No vertical view (`responsive`/`breakpoint`), icons do not change to "down/up" | Outside the port scope. |
| No templates (`item`, `filter`, `empty`…), `filterBy`/`filterMatchMode`/`filterLocale`, `buttonProps`, `trackBy`, `dataKey` | `Listbox` searches by label; the rest is outside the port scope. |
| `show_source_controls`/`show_target_controls` are read at creation, not reactively | The button view cannot be rebuilt from a signal. |
| With `keep_selection` and "all to target/source" the transferred items are selected, not the previous source selection | In PrimeNG `moveAllRight` appends to `selectedItemsSource` (a bug), `moveLeft` — a nested array. |

## What was not verified

- Keyboard transfer inside the lists (standard `Listbox` behaviour).
- Screen reading: `aria-*` were checked against PrimeNG, screen reader behaviour was not.

## Performance

See `benches/` (`picklist-1`, `picklist-1000`, `case-picklist`) and `benches/*budgets.toml`.
