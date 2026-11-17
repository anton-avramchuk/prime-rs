# OrderList

Port of `p-orderlist` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-orderlist::OrderList`, `OrderListPt`, `ControlsPosition` | `packages/primeng/src/orderlist/orderlist.ts`, `orderlist/style/orderliststyle.ts` |
| Reordering of the selected items | `prime-orderlist::reorder` — pure logic instead of `moveUp/moveTop/moveDown/moveBottom`, table-driven tests |
| List, selection, search | `prime-listbox::Listbox` (in PrimeNG — `p-listbox`) |
| CSS | `@primeuix/styles/orderlist` and the `/* For PrimeNG */` block (`orderlist` in `theme.components` and `primeng_extra`) |
| Demos | `apps/showcase/doc/orderlist/*-doc.ts` (`basic`, `filter`; `controls` is ours) |

Spec: `openspec/specs/orderlist`. The order is a `RwSignal<Vec<ListboxOption>>`, the selection is a `RwSignal<Vec<String>>` (the values of the selected items).

```rust
let value = RwSignal::new(vec![ListboxOption::new("Apple", "a"), ListboxOption::new("Pear", "p")]);
view! { <OrderList value=value filter=true on_reorder=Callback::new(|sel: Vec<String>| log(sel)) /> }
```

## What matches PrimeNG

- Markup: `p-orderlist p-component` with `p-orderlist-controls-left|right`, the `p-orderlist-controls` panel of four buttons (up, to the top, down, to the bottom), unavailable without a selection or when `disabled`, then the list.
- The list — multiple selection, `striped_rows`, `meta_key_selection`, `scroll_height` (default `14rem`), search.
- Reordering over the full list, even if the search hides some items; the selection is kept after moving.

## Differences

| Difference | Reason |
|---|---|
| Selected items keep their relative order on any move | In PrimeNG, with `moveUp` adjacent items at the edge swap places, and `moveTop`/`moveBottom` depend on the click order — these are bugs; the behaviour is fixed on purpose. |
| No drag and drop (`dragdrop`) | In PrimeNG it is built on Angular CDK; the project has no common drag-and-drop solution yet. |
| No `header`, templates, `filterBy`/`filterMatchMode`/`filterLocale`, `responsive`/`breakpoint`, `buttonProps` | `Listbox` has no header slot; search goes by the label; the rest is outside the scope of the port. |
| The value is a list of `ListboxOption` (a label and a string value), not arbitrary objects | A typed model instead of `any`; the item view is `option`. |

## Not verified

- Moving from the keyboard inside the list (the standard `Listbox` behaviour).
- Screen readers: `aria-*` were checked against PrimeNG, screen reader behaviour was not.

## Performance

See `benches/` (`orderlist-1`, `orderlist-1000`, `case-orderlist`) and `benches/*budgets.toml`.
