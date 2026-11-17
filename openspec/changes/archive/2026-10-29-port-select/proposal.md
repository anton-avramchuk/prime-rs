# Proposal

## Why

Select is a single-value choice from a list in a dropdown panel; the most in-demand form component, on which Dropdown-like widgets rest. This is the last candidate of the wave and the project's first component with an overlay: it sets the pattern for a panel (positioning, z-index, closing on outside click and on Escape) for future Popover, Menu and others.

## What Changes

- New crate `prime-select` with the `Select` component: an `options` list (`SelectOption { label, value, disabled }`) or `groups` (`SelectGroup { label, items }`), two-way `value` (`RwSignal<Option<String>>`) or internal state with `default_value`; two-way `open` (`RwSignal<bool>`) for programmatic panel control; `placeholder`, `show_clear`, `checkmark`, `filter` (search field, `filter_placeholder`, `reset_filter_on_hide`, `autofocus_filter`), `empty_message`, `empty_filter_message`, `loading`, `focus_on_hover`, `select_on_focus`, `auto_option_focus`, `scroll_height` (200px), `disabled`, `readonly`, `invalid`, `required`, `variant`, `size`, `fluid`, `input_id`, `tabindex`, `aria_label`, `aria_labelledby`, `aria_filter_label`, `style`, `style_class`, `panel_style`, `panel_style_class`; icons `dropdown_icon`, `clear_icon`, `loading_icon` (`ViewFn`) and slots `item_template`, `selected_item_template`, `group_template`; events `on_change`, `on_filter`, `on_focus`, `on_blur`, `on_click`, `on_show`, `on_hide`, `on_clear`; `pt`.
- Markup as in PrimeNG 21.1.10 (`select.ts`, `selectstyle.ts`): root `<div class="p-select p-component p-inputwrapper">`, label `span.p-select-label[role=combobox]`, clear icon, button `div.p-select-dropdown`, panel `div.p-select-overlay` with a search field (`p-select-filter` in `p-iconfield`), list `ul.p-select-list[role=listbox]` (`li.p-select-option`, `li.p-select-option-group`, `li.p-select-empty-message`) and hidden focusable boundaries.
- The panel is positioned below the field (above it if there is not enough room) via `prime-dom`, gets the overlay group z-index, and closes on outside click, `Escape`, `Tab` and selection.
- Keyboard as in PrimeNG: arrows, `Home`/`End`, `PageUp`/`PageDown`, `Enter`/`Space`, `Escape`, `Tab`, `Alt+ArrowUp`, `Delete` (clear), type-ahead search (500 ms), navigation in the search field too.
- The `/select` page (`basic`, `checkmark`, `clearicon`, `disabled`, `filled`, `filter`, `fluid`, `group`, `invalid`, `loadingstate`, `sizes`, `template`), e2e, browser tests, benchmarks, budgets, `docs/select.md`, theme CSS `select` and `iconfield` (the search field uses the `p-iconfield` and `p-inputicon` classes, which are part of `iconfield`), the `/* For PrimeNG */` block (placeholder color) via `primeng_extra`.
- **Out of scope** (by the user's decision on the scope of the first change): virtual scrolling (`virtualScroll`, `virtualScrollItemSize`, `virtualScrollOptions`) and lazy loading (`lazy`, `onLazyLoad`), `editable`, the `overlayOptions` setting and `appendTo` (the panel is always inside the component), show and hide animation of the panel, `filterBy`/`filterFields`/`filterMatchMode`/`filterLocale` (search is a case-insensitive substring of the label), `filterTemplate` and other templates (`header`, `footer`, `empty`, `loader`), `tooltip*`, `dataKey`, `optionLabel`/`optionValue`/`optionDisabled`/`optionGroupLabel`/`optionGroupChildren` (typed `SelectOption` and `SelectGroup` instead), `autofocus`; Angular Forms; demos `customfilter`, `editable`, `virtualscroll`, `lazyvirtualscroll`, `floatlabel`, `iftalabel`, `reactiveforms`, `templatedrivenforms`.

## Capabilities

### New Capabilities
- `select`: the Select component: value and label, panel with options, selection by mouse and keyboard, search, groups, clear, structure and accessibility, `pt`, performance.

### Modified Capabilities
<!-- None: `prime-rs-core` only gets a public constant of translation keys if needed; the behaviour of existing components does not change. -->

## Impact

- New crate `crates/prime-select` (`publish = false`); dependencies `prime-rs-core`, `prime-dom`, `prime-icons` (`chevrondown`, `search`, `times`, `check`, `spinner`).
- Changed: workspace `Cargo.toml`, `upstream.toml` (`select`, `iconfield` in `theme.components`, `select` in `primeng_extra`), `showcase`, `benches/*`, `e2e/tests`, `ci.yml` (browser job for `tests/web.rs`).
