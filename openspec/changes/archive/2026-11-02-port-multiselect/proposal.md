# Proposal

## Why

MultiSelect is the eleventh component of wave 3: selecting several values from a list in a dropdown panel. It combines two already finished components: the panel, positioning and label field come from Select, while multiple selection, ranges, checkboxes and "select all" come from Listbox. It is the last large "selecting" component before Paginator and Stepper.

## What Changes

- New crate `prime-multiselect` (port of `p-multiselect` from PrimeNG 21.1.10).
- `MultiSelectOption` (`label`, `value`, `disabled`) and `MultiSelectGroup` (`label`, `items`); values are strings, as in Select and Listbox.
- `MultiSelect`: root `div.p-multiselect.p-component.p-inputwrapper` > hidden `combobox` field (`p-hidden-accessible`) > `div.p-multiselect-label-container` > `div.p-multiselect-label` (`comma` mode — comma-separated text, `chip` mode — `div.p-multiselect-chip-item` with `Chip`), clear icon, `div.p-multiselect-dropdown` with an arrow or loading icon, panel `div.p-multiselect-overlay` (hidden focus boundaries, header `p-multiselect-header` with the "select all" checkbox and the search field in `p-multiselect-filter-container`, container `p-multiselect-list-container`, list `ul.p-multiselect-list[role=listbox]` with `li.p-multiselect-option` (checkbox inside), group headers `p-multiselect-option-group` and the `p-multiselect-empty-message` message).
- Properties: `options`, `groups`, `values` (two-way `RwSignal<Vec<String>>`) / `default_values`, `open` (two-way), `display` (`MultiSelectDisplay::Comma`/`Chip`), `max_selected_labels` (3), `selected_items_label` (`{0} items selected` from the translation), `display_selected_label` (yes), `selection_limit`, `show_toggle_all` (yes), `show_header` (yes), `filter` (yes), `filter_placeholder`, `reset_filter_on_hide`, `autofocus_filter`, `placeholder`, `show_clear`, `loading`, `highlight_on_select` (yes), `focus_on_hover` (yes), `select_on_focus`, `auto_option_focus` (no), `scroll_height` (`200px`), `disabled`, `readonly`, `invalid`, `variant`, `size`, `fluid`, `input_id`, `tabindex`, `aria_label`, `aria_labelledby`, `aria_filter_label`, `empty_message`, `empty_filter_message`, `style`, `style_class`, `panel_style`, `panel_style_class`, `dropdown_icon`, `clear_icon`, `chip_icon`, `loading_icon`, `pt`; views `item`, `group`, `selected_items`; events `on_change`, `on_filter`, `on_focus`, `on_blur`, `on_click`, `on_clear`, `on_show`, `on_hide`, `on_remove`, `on_select_all_change`.
- Behaviour: a click on an option adds/removes the value, the panel stays open; `selection_limit` blocks unselected options when the limit is reached (and hides "select all"); "select all" accounts for search and keeps selected disabled options; keys ↓ ↑ (with `Alt+↑` — select and close), Home End PageUp PageDown Enter Space Escape Tab, `Shift` ranges, `Ctrl+Shift+Home/End`, `Ctrl+A`, type-ahead search; closing on a click outside and `resize`; overlay group z-index; a chip is removed by clicking the icon (`on_remove`).
- Showcase page `/multiselect` with demos `basic`, `chips`, `clearicon`, `disabled`, `filled`, `filter`, `fluid`, `group`, `invalid`, `loadingstate`, `sizes`, `template`, `floatlabel`, `iftalabel`; e2e; benchmarks and budgets; `docs/multiselect.md`.
- Styles: `multiselect` is added to `theme.components`.
- **Out of scope:** virtual scrolling and lazy loading (`virtualScroll*`, `lazy`, `onLazyLoad`), `appendTo` and `overlayOptions` (the panel is always inside the root, as in Select), `@primeuix/motion`, Angular forms, `optionLabel`/`optionValue`/`optionDisabled`/`optionGroup*`/`dataKey`/`filterBy`/`filterFields`/`filterMatchMode`/`filterLocale`, `tooltip*`, `autofocus`, external `selectAll`, `header`, `footer`, `filter`, `empty`, `loader`, `headercheckboxicon` templates, `Bind`, `pTemplate`.

## Capabilities

### New Capabilities
- `multiselect`: selecting several values from a list in a dropdown panel with search, groups, chips, a selection limit and keyboard navigation.

### Modified Capabilities
<!-- None. -->

## Impact

- New crate `crates/prime-multiselect` (`publish = false`); dependencies: `prime-rs-core`, `prime-dom`, `prime-icons`, `prime-checkbox` (checkbox classes), `prime-chip`, `web-sys`.
- Changed: workspace `Cargo.toml`, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (browser job, three places).
- The list logic (flat list, filter, indices, selection, ranges) is copied from `prime-listbox` into `logic.rs` in the first version; extraction into a shared module is deferred until a third consumer (following the Drawer/Dialog example).
