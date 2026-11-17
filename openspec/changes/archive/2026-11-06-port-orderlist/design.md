# Design

## Context

The requirements are in `specs/orderlist`. PrimeNG 21.1.10 sources: `orderlist/orderlist.ts`, `style/orderliststyle.ts` (+ the `For PrimeNG` block), `@primeuix/styles/orderlist`. In PrimeNG OrderList is a wrapper over `p-listbox`; ours is the same on top of `prime-listbox`.

## Goals / Non-Goals

**Goals:** the same DOM and behaviour of the buttons; the elements of the value are reordered.

**Non-Goals:** drag-and-drop, `header`, templates, `responsive`.

## Decisions

### D1. Reordering (`reorder.rs`)
Functions over a `Vec<String>` of keys and a `&[String]` of the selected ones, returning the new order. `move_up`/`move_down` — a single pass with the check "the neighbour is not selected" (preserves the order of the block); `move_top`/`move_bottom` — a stable partition into the selected and the rest.

### D2. Component
`value: RwSignal<Vec<ListboxOption>>`, `selection: RwSignal<Vec<String>>` (internal if not set). The list — `Listbox` with `multiple`, `options` from `value`, `values` = `selection`; `Listbox` changes `selection` itself. The buttons — `Button` with the icons `AngleUp`, `AngleDoubleUp`, `AngleDown`, `AngleDoubleDown`.

### D3. Theme
`orderlist` in `theme.components` and `primeng_extra` (the block `.p-orderlist-controls-right .p-orderlist-controls {order: 2}`).

### D4. Tests
The logic — tables. SSR: markup, the positions of the buttons, the state of the buttons. Browser `tests/web.rs`: selection, four buttons, `on_reorder`, `disabled`, the filter. e2e: styles and real clicks.

## Risks / Trade-offs

- [Behaviour] Moving several items was deliberately fixed; recorded in the docs.

## Open Questions

- None.

## Migration Plan

No migration. Rollback — `git revert`.
