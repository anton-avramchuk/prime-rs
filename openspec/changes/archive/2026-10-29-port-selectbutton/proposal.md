# Proposal

## Why

SelectButton — selection of one or several options from a list with buttons; it is built from `ToggleButton`, which is already ported (`port-togglebutton`). This is the next small port and the first component on top of another prime-rs component.

## What Changes

- New crate `prime-selectbutton` with the `SelectButton` component and the `SelectButtonOption` type (`label`, `value`, `disabled`): an `options` list, single selection via a two-way `value` (`RwSignal<Option<String>>`) or multiple (`multiple`) via `values` (`RwSignal<Vec<String>>`); without a signal the component keeps internal state (`default_value`, `default_values`).
- `allow_empty`, `disabled`, `invalid`, `fluid`, `size`, `aria_labelledby`, `style` (root), `style_class` (as in PrimeNG, passed to each button, not to the root), the `item_template` slot (receives the option and index), events `on_change` (new value and original event), `on_option_click` (option, index, original event), `pt` (`root` and `toggle_button` for the nested `ToggleButton`s).
- Markup as in PrimeNG 21.1.10 (`selectbutton.ts`, `selectbuttonstyle.ts`): root `<div class="p-selectbutton p-component" role="group" aria-labelledby data-p>`, inside a `ToggleButton` per option with `on_label = off_label = label`.
- Page `/selectbutton` (`basic`, `disabled`, `fluid`, `invalid`, `multiple`, `sizes`, `template`), e2e, browser tests, benchmarks, budgets, `docs/selectbutton.md`, `selectbutton` theme CSS.
- **Out of scope:** Angular Forms (`ngModel`, reactive forms); the deprecated `unselectable` (replaced by `allow_empty`); `autofocus`; `tabindex` (not used in the PrimeNG template, each button has its own `tabindex`); `option_label`/`option_value`/`option_disabled`/`data_key` (arbitrary object fields) — a typed `SelectButtonOption` instead; the `/* For PrimeNG */` block (`.ng-invalid.ng-dirty`) is not ported; the `reactiveforms` and `templatedrivenforms` demos.

## Capabilities

### New Capabilities
- `selectbutton`: the SelectButton component — options and value, single and multiple selection, events, structure and attributes, `pt`, performance.

### Modified Capabilities
<!-- None. -->

## Impact

- New crate `crates/prime-selectbutton` (`publish = false`); dependencies `prime-rs-core`, `prime-togglebutton`.
- Changed: workspace `Cargo.toml`, `upstream.toml` (`selectbutton` in `theme.components`), `showcase`, `benches/*`, `e2e/tests`, `ci.yml` (browser job for `tests/web.rs`).
- `ToggleButton` does not change.
