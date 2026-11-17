# Proposal

## Why

OrderList — a list with a changeable order: the user selects items and moves them with the buttons up, to the top, down, to the bottom. The fourth component of wave 6. The list itself, selection and search already exist in `Listbox`; here — the pure reordering logic and the button panel.

## What Changes

- A new crate `prime-orderlist` (a port of `p-orderlist` from PrimeNG 21.1.10).
- A pure module `reorder.rs`: `move_up`, `move_top`, `move_down`, `move_bottom` over the list of keys and the set of selected ones; table tests.
- `OrderList`: `div.p-orderlist.p-component` (`p-orderlist-controls-left|right`) > `div.p-orderlist-controls` (four `Button`s with `severity=secondary`: up, to the top, down, to the bottom; `aria-label` from the translation; unavailable without a selection or when `disabled`) and a `Listbox` (multiple selection, `striped`, `meta_key_selection`, the search `filter`, `scroll_height`, `aria_label`, `tabindex`).
- The value — `RwSignal<Vec<ListboxOption>>` (the order of the items), the selection — `RwSignal<Vec<String>>` (the values of the selected ones). The events `on_reorder` (the moved items), `on_selection_change`.
- The page `/orderlist` (`basic`, `filter`, `controls`), e2e, benchmarks and budgets, `docs/orderlist.md`, `orderlist` in `theme.components` and `primeng_extra`.
- **Not included:** drag-and-drop (`dragdrop`, CDK), `header`, Angular templates, the adaptive `<style>` `responsive`/`breakpoint`, `filterBy`/`filterMatchMode` (the search of `Listbox` by label), `buttonProps`.
- **Difference in behaviour:** moving several selected items keeps their mutual order (in PrimeNG with `moveUp` adjacent items at the edge swap places, while `moveTop`/`moveBottom` depend on the order of the clicks).

## Capabilities

### New Capabilities
- `orderlist`: a list with a changeable order of items.

### Modified Capabilities
<!-- None. -->

## Impact

- A new crate `crates/prime-orderlist` (`publish = false`); dependencies: `prime-rs-core`, `prime-listbox`, `prime-button`, `prime-icons`.
- Changes to `Cargo.toml` of the workspace, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml`.
