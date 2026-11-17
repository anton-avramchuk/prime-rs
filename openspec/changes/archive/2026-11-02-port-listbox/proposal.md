# Proposal

## Why

Listbox is the tenth component of wave 3: a list of options from which one or several values are selected, with search, groups and checkboxes. It is needed as the basis for MultiSelect (shared list selection, `Ctrl`/`Shift`, "select all") and covers the typical "list beside the form" forms without an overlay panel. Markup and keys largely repeat the list inside Select, but selecting several values and ranges is a new part.

## What Changes

- New crate `prime-listbox` (port of `p-listbox` from PrimeNG 21.1.10).
- `ListboxOption` (`label`, `value`, `disabled`) and `ListboxGroup` (`label`, `items`); values are strings, as in Select and SelectButton.
- `Listbox`: root `div.p-listbox.p-component` > optional header `div.p-listbox-header` ("select all" checkbox with `checkbox` and `multiple`, search field `p-listbox-filter` in `p-iconfield`) > `div.p-listbox-list-container` (`scroll_height`, default `14rem`) > `ul.p-listbox-list[role=listbox]` with `li.p-listbox-option[role=option]`, group headers `li.p-listbox-option-group` and the empty list message `div.p-listbox-empty-message`.
- Properties: `options`, `groups`, `value` (one value, `RwSignal<Option<String>>`) and `values` (several, `RwSignal<Vec<String>>`) with `multiple`, `multiple`, `checkbox`, `filter`, `checkmark`, `striped`, `highlight_on_select` (yes), `meta_key_selection`, `select_on_focus`, `auto_option_focus` (yes), `focus_on_hover` (yes), `show_toggle_all` (yes), `readonly`, `disabled`, `invalid`, `fluid` (`OptProp<bool>`), `scroll_height`, `tabindex` (0), `style`, `style_class`, `list_style`, `list_style_class`, `filter_placeholder`, `empty_message`, `empty_filter_message`, `aria_label`, `aria_filter_label`, `id`, `pt`; views `option` (instead of the option content) and `group`; events `on_change`, `on_click`, `on_dbl_click`, `on_filter`, `on_focus`, `on_blur`, `on_select_all_change`.
- Behavior: a click selects/clears an option (single and multiple modes, `Ctrl`/`⌘` with `meta_key_selection`); keys ↓ ↑ Home End PageUp PageDown Enter Space, `Shift`+arrows and `Ctrl+Shift+Home/End` select a range, `Ctrl+A` selects all, printable characters do type-ahead search; focus (`p-focus`, `aria-activedescendant`); double click; "select all" respecting the filter; search is a case-insensitive label substring.
- Showcase page `/listbox` with demos `basic`, `multiple`, `group`, `filter`, `checkbox`, `checkmark`, `disabled`, `invalid`, `template`; e2e; benchmarks and budgets; `docs/listbox.md`.
- Styles: `listbox` is added to `theme.components`; the `/* For PrimeNG */` block (header, search field, container height) goes through `primeng_extra`.
- **Out of scope:** virtual scroll and lazy loading (`virtualScroll*`, `lazy`, `onLazyLoad`), `dragdrop` and `dropListData` (CDK), Angular forms integration, `optionLabel`/`optionValue`/`optionDisabled`/`optionGroup*`/`dataKey`/`filterBy`/`filterFields`/`filterMatchMode`/`filterLocale`/`searchLocale` (replaced by typed options), hidden focus boundaries `p-hidden-focusable`, `header`, `footer`, `filter`, `empty`, `loader` templates, hidden `aria-live` messages (`searchMessage`, `selectionMessage`, etc.), externally controlled "select all" (`selectAll`), `Bind`, `pTemplate`.

## Capabilities

### New Capabilities
- `listbox`: a list of options with single and multiple selection, groups, search, checkboxes and keyboard navigation.

### Modified Capabilities
<!-- None. -->

## Impact

- New crate `crates/prime-listbox` (`publish = false`); dependencies: `prime-rs-core`, `prime-icons`, `prime-checkbox` (only `CheckboxState`, classes and `data-p` of the checkbox), `web-sys`.
- Changed: workspace `Cargo.toml`, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (browser job, three places).
