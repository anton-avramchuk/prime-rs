# Proposal

## Why

CascadeSelect is a field for picking a value from a nested structure: a panel with a root list, from which lists of nested groups unfold sideways. The shell (label, clear, button, panel inside the root) is the same as in `Select`/`TreeSelect`; what is new is the cascade of lists, the path of active groups and keyboard handling with transitions between levels.

## What Changes

- New crate `prime-cascadeselect` (port of `p-cascadeselect` from PrimeNG 21.1.10).
- Model: `CascadeOption` — `label`, `value` (`label` by default), `disabled`, `children`. The value is `RwSignal<Option<String>>` (the leaf value).
- `CascadeSelect`: root `div.p-cascadeselect.p-component.p-inputwrapper` (`p-cascadeselect-clearable|mobile|open|fluid|sm|lg`, `p-disabled`, `p-invalid`, `p-focus`, `p-variant-filled`, `p-inputwrapper-filled|focus`) > hidden input `input[role=combobox][readonly]` (`aria-haspopup="tree"`, `aria-expanded`, `aria-controls`, `aria-activedescendant`) > `span.p-cascadeselect-label` (label of the selected leaf or `placeholder`) > clear icon > `div.p-cascadeselect-dropdown` (arrow or loading spinner) > panel `div.p-cascadeselect-overlay` > `div.p-cascadeselect-list-container` > `ul.p-cascadeselect-list[role=tree]` > `li.p-cascadeselect-option[role=treeitem]` (`-option-group`, `-option-active`, `-option-selected`, `p-focus`, `p-disabled`; `aria-level|setsize|posinset|label|selected`, `id`) > `div.p-cascadeselect-option-content` > `span.p-cascadeselect-option-text` + `span.p-cascadeselect-group-icon` (for groups) and a nested `ul.p-cascadeselect-list.p-cascadeselect-overlay.p-cascadeselect-option-list[role=group]` for the active group.
- Behavior: a click on the field opens/closes the panel; a click on a group expands its list, a repeated one collapses it; a click on a leaf selects its value and closes the panel; hover expands groups (before the first click — root ones, after — any); on open the path to the selected one is expanded; an outside click and `resize` close; `show_clear` clears; in a narrow window (`breakpoint`, `960px`) hover does not change the path.
- Keyboard on the hidden input: ↓ ↑ over the level list (opening the group under focus), → expands the group and enters it, ← closes the group and returns to it, `Alt+↑` closes, `Home`/`End`, `Enter`/`Space`, `Escape`, `Tab`, `Backspace` (clear with `show_clear`), search by label prefix.
- Properties: `options`, `value`, `placeholder`, `show_clear`, `disabled`, `invalid`, `variant`, `size`, `fluid`, `loading`, `breakpoint`, `focus_on_hover`, `select_on_focus`, `auto_option_focus`, `input_id`, `tabindex`, `aria_label`, `aria_labelledby`, `id`, `style`, `style_class`, `panel_style_class`; events `on_change`, `on_group_change`, `on_show`, `on_hide`, `on_clear`, `on_focus`, `on_blur`; views `value_view` and `option`.
- Page `/cascadeselect` (`basic`, `clearicon`, `disabled`, `invalid`, `loading`, `template`), e2e, benchmarks and budgets, `docs/cascadeselect.md`, `cascadeselect` in `theme.components` and `primeng_extra`.
- **Out of scope:** arbitrary option objects (`optionLabel`/`optionValue`/`optionGroupLabel`/`optionGroupChildren`/`dataKey`), Angular forms, `appendTo`, `Ripple`, `Bind`, `pTemplate`, `header`/`footer`/icon templates, mirroring of the nested list at the window edge (`position()`), `onBeforeShow`/`onBeforeHide`, screen reader messages (`aria-live`).

## Capabilities

### New Capabilities
- `cascadeselect`: picking a value from a cascade of nested lists.

### Modified Capabilities
<!-- None. -->

## Impact

- New crate `crates/prime-cascadeselect` (`publish = false`); dependencies: `prime-rs-core`, `prime-dom`, `prime-icons` (chevrondown, angleright, times, spinner).
- Changed: workspace `Cargo.toml`, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml`.
