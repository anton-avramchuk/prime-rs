# Proposal

## Why

ToggleButton is a button with a boolean state (on/off), the first step towards SelectButton, which is built from such buttons. The value model and the events are the same as for Checkbox and ToggleSwitch, there is no native `<input>`, no shared logic in `prime-rs-core` is required, so the component suits well as the next small port.

## What Changes

- A new crate `prime-togglebutton` with the `ToggleButton` component: a two-way `checked` (`RwSignal<bool>`) or an internal state with `default_checked`, `on_label`/`off_label` (default "Yes"/"No"), `on_icon`/`off_icon` (`ViewFn`), `icon_pos`, `size`, `disabled`, `invalid`, `allow_empty`, `fluid`, `tabindex`, `aria_label`, `aria_labelledby`, `style`, `style_class`, the slots `icon_template` and `content_template`, the events `on_change`, `on_focus`, `on_blur`, `pt`.
- Markup as in PrimeNG 21.1.10 (`togglebutton.ts`, `togglebuttonstyle.ts`): the root `<div class="p-togglebutton p-component" role="button" aria-pressed tabindex>` with `data-pc-name`, `data-p-checked`, `data-p-disabled`, `data-p`; inside it `span.p-togglebutton-content` with the icon and `span.p-togglebutton-label`. Toggling by click and by the Enter and Space keys.
- `xtask gen-theme` accepts the marker `/* For PrimeNG (…) */` (in `togglebuttonstyle.ts` it has an explanation), `togglebutton` is added to `primeng_extra`.
- The `/togglebutton` page (`basic`, `customized`, `disabled`, `fluid`, `invalid`, `sizes`), e2e, browser tests, benchmarks, budgets, `docs/togglebutton.md`, the CSS of the theme `togglebutton`.
- **Not included:** integration with Angular Forms (`ngModel`, reactive forms); ripple; `autofocus`; the `reactiveforms` and `templatedrivenforms` demos; the PrimeIcons class icon (`ViewFn` is used instead).

## Capabilities

### New Capabilities
- `togglebutton`: the ToggleButton component — state and events, the keyboard, structure and classes, accessibility attributes, `pt`, performance.

### Modified Capabilities
<!-- None. -->

## Impact

- A new crate `crates/prime-togglebutton` (`publish = false`); dependencies `prime-rs-core`.
- Changes to the workspace `Cargo.toml`, `upstream.toml` (`togglebutton` in `theme.components` and `primeng_extra`), `xtask/src/gen_theme.rs`, `showcase`, `benches/*`, `e2e/tests`, `ci.yml` (the browser job for `tests/web.rs`).
