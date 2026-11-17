# Design

## Context

Requirements are in `specs/selectbutton`. PrimeNG 21.1.10 sources: `packages/primeng/src/selectbutton/selectbutton.ts`, `selectbutton/style/selectbuttonstyle.ts`, demos `apps/showcase/doc/selectbutton`. References — `crates/prime-togglebutton` (`docs/togglebutton.md`) and the `prime-radiobutton` group (a single value signal for the whole group). SelectButton in PrimeNG is a `p-togglebutton` per option, with `[ngModel]="isSelected(option)"`, `onLabel = offLabel = label`, `allowEmpty = getAllowEmpty()`, `size`, `fluid`, `styleClass` and `pt = ptm('pcToggleButton')`; there is no own `tabindex` or arrow navigation in the template.

## Decisions

### D1. Options and value
`SelectButtonOption { label: String, value: String, disabled: bool }` (`Clone`, `Default` for `disabled`, constructor `new(label, value)`), `options: Prop<Vec<SelectButtonOption>>`. Values are strings, like the RadioButton group. Arbitrary object fields (`optionLabel`, `optionValue`, `optionDisabled`, `dataKey`) are not ported: a typed option replaces them. Values are compared by string equality (in PrimeNG `equals` from `@primeuix/utils`).

### D2. Value model
Two modes by `multiple: Prop<bool>`: single — `value: Option<RwSignal<Option<String>>>`, multiple — `values: Option<RwSignal<Vec<String>>>`. Without a signal the component creates an internal one with `default_value` / `default_values`. We do not take one signal for both modes: `Option<String>` and `Vec<String>` read more easily than "a list of ≤1 element". Only the signal of the active mode is read. `on_change` receives `SelectButtonChange { value: SelectButtonValue, original_event }`, where `SelectButtonValue` is `Single(Option<String>)` or `Multiple(Vec<String>)`.

### D3. Selection
The option handler (like `onOptionSelect`): do nothing when the component or the option is `disabled`. Single mode: a selected option with `allow_empty == Some(false)` does not change, otherwise the value becomes `None`; an unselected one — the option's value. Multiple: a selected one is removed from the list, an unselected one is appended to the end; `allow_empty = false` does not allow removing the only value (`getAllowEmpty`). After the change `on_change` is called, then `on_option_click { option, index, original_event }`. The deprecated `unselectable` is not ported.

### D4. Relation to ToggleButton
`ToggleButton` keeps `checked` itself and toggles its signal before calling `on_change`; so that it does not diverge from the SelectButton value (for example, with `allow_empty = false` or an unavailable option), a `RwSignal<bool>` is created per option, which an `Effect` synchronizes with "is the option selected", and the handler forcibly aligns it after its work (`set(is_selected)`), because when the value is unchanged the `Effect` does not fire. The `allow_empty` and `disabled` restrictions are passed to the button with the same values (the button's `allow_empty` is `getAllowEmpty()`), so blocking already happens in it. `ToggleButton` does not change.

Alternative — add a "read-only" mode to `ToggleButton`: `checked` as `Prop<bool>` and state only from outside. Rejected: it changes the accepted API and the `togglebutton` spec for a single consumer.

### D5. Markup
The root is a `<div>`; classes `p-selectbutton p-component`, `p-invalid`, `p-selectbutton-fluid`, then `pt.root`. Attributes `role="group"`, `aria-labelledby`, `data-p` (`invalid`). Child `ToggleButton`s: `on_label = off_label = label`, `disabled = disabled || option.disabled`, `size`, `fluid`, `style_class` (buttons only), `pt = pt.toggle_button`, `content_template` from `item_template`. `invalid` is not passed to the buttons (as in PrimeNG). The list is built by a closure over `options` and is redrawn entirely on change; a key by label (like `track getOptionLabel`) is not needed.

### D6. CSS
`@primeuix/styles/selectbutton` and the preset — `selectbutton` in `theme.components` (`upstream.toml`), `cargo xtask gen-theme`. The `/* For PrimeNG */` block contains only the Angular Forms `.ng-invalid.ng-dirty` rule, so `primeng_extra` is not extended. The styles of the nested `p-togglebutton`s come from the `togglebutton` CSS, which is included on pages together with `selectbutton`.

### D7. Demos
`basic`, `disabled`, `fluid`, `invalid`, `multiple`, `sizes`, `template`. In `template`, instead of `pi-align-*` (there are no such icons in `prime-icons`), the template renders a `<strong>` with the label; demos with forms are skipped. Additionally an `echo` of the value for e2e.

### D8. Tests
SSR: structure, labels and order, the selected option, `disabled`, root classes and `data-p`, `size`/`fluid`/`style_class` on the buttons, `item_template`, `pt`, `aria_labelledby`. Browser (`tests/web.rs`): select, deselect, `allow_empty`, multiple selection and order, `disabled` on an option and on the component, Enter and Space keys, one `on_change` per press, `aria-pressed` synchronization after a rejected click, reaction to an external signal and to an `options` change. The crate with `tests/web.rs` is added to the browser job of `ci.yml` (3 places).

## Risks / Trade-offs

- [Extra signals and an `Effect` per option raise the allocation count relative to ToggleButton] → budget by measurement; if exceeded — simplify the synchronization (see the alternative in D4).
- [Divergence between the button signal and the value on rare update paths] → a browser test for a rejected click and an external value change.
- [String values are less flexible than PrimeNG's arbitrary object] → a deliberate difference, recorded in `docs/selectbutton.md`; generalizing to `T` is possible in a separate change.

## Migration Plan

No migration: a new crate. Rollback — `git revert`.

## Open Questions

None.
