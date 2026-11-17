# Proposal

## Why

PickList — two lists (source and target) and buttons for moving items between them, plus reordering in each list. The fifth component of wave 6. The lists, selection and search already exist in `Listbox`, reordering is in `prime-orderlist::reorder`; what is new here is the transfer and the button panels.

## What Changes

- New crate `prime-picklist` (port of `p-picklist` from PrimeNG 21.1.10).
- Pure module `transfer.rs`: `move_selected` (selected ones in selection order, without duplicates by value) and `move_all` (only those visible under the search); table tests.
- `PickList`: `div.p-picklist.p-component` > `.p-picklist-controls.p-picklist-source-controls` (up, to the top, down, to the bottom), `.p-picklist-list-container.p-picklist-source-list-container` (`Listbox`), `.p-picklist-controls.p-picklist-transfer-controls` (to target, all to target, to source, all to source), `.p-picklist-list-container.p-picklist-target-list-container`, `.p-picklist-controls.p-picklist-target-controls`.
- Data — `source` and `target`: `RwSignal<Vec<ListboxOption>>`; the selection in each list is an internal signal; events `on_move_to_target`, `on_move_all_to_target`, `on_move_to_source`, `on_move_all_to_source`, `on_source_reorder`, `on_target_reorder`, `on_source_select`, `on_target_select`.
- Properties: `source_header`, `target_header`, `filter` (with `show_source_filter`/`show_target_filter`), `striped_rows`, `meta_key_selection`, `disabled`, `scroll_height`, `show_source_controls`, `show_target_controls`, `keep_selection`, search placeholders, `source_style`/`target_style`, `style`, `style_class`, `option`.
- A double click on an item moves the selected ones (`onSourceItemDblClick`/`onTargetItemDblClick`).
- Page `/picklist` (`basic`, `filter`), e2e, benchmarks and budgets, `docs/picklist.md`, `picklist` in `theme.components`.
- A `header_content` property (header slot) was added to `prime-listbox`.
- **Out of scope:** drag and drop, Angular templates, `responsive`/`breakpoint` (vertical view), `filterBy`/`filterMatchMode`/`filterLocale` (`Listbox` search is by label), `buttonProps`.

## Capabilities

### New Capabilities
- `picklist`: moving items between two lists with reordering.

### Modified Capabilities
<!-- None: the `header_content` property of Listbox does not change existing behaviour. -->

## Impact

- New crate `crates/prime-picklist` (`publish = false`); dependencies: `prime-core`, `prime-listbox`, `prime-orderlist` (reordering), `prime-button`, `prime-icons`.
- Changed: workspace `Cargo.toml`, `upstream.toml`, `prime-listbox`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml`.
