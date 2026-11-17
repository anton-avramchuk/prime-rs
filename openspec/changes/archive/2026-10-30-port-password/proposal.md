# Proposal

## Why

Password is the last component of the wave 2 plan: a password input field with a strength indicator and a show toggle. It is assembled from the already ported `InputText`, icons and the overlay mechanism of `Select` (alignment, `Overlay` group z-index, closing on an outside click).

## What Changes

- New crate `prime-password` with the `Password` component (port of `p-password` from PrimeNG 21.1.10): root `span.p-password.p-component.p-inputwrapper` with an `InputText` field (`p-password-input`, `type=password`), optional clear and show/hide icons and a hint panel `div.p-password-overlay` with a strength meter (`p-password-meter`, `p-password-meter-label`, `p-password-meter-text`).
- Properties: `value` (`RwSignal<String>`), `feedback` (`true` by default), `toggle_mask`, `show_clear`, `prompt_label`, `weak_label`, `medium_label`, `strong_label` (from the locale by default), `strength` (custom strength estimate), `placeholder`, `autocomplete`, `input_id`, `input_style`, `input_style_class`, `max_length`, `disabled`, `readonly`, `invalid`, `required`, `variant`, `size`, `fluid`, `aria_label`, `style`, `style_class`, `pt`; templates `header`, `footer`, `content`, `show_icon`, `hide_icon`, `clear_icon` — `ViewFn`; callbacks `on_focus`, `on_blur`, `on_clear`.
- Strength: empty → prompt; non-empty → weak; "medium" — at least 6 characters and two kinds of {lowercase, uppercase, digits}; "strong" — at least 8 characters and all three kinds (same as the default PrimeNG regular expressions, but without `mediumRegex`/`strongRegex`).
- Panel: opens on focus and input, closes on `Escape` and focus loss; aligned below the field (`align_overlay`), `Overlay` group z-index, animation by the classes `p-password-enter`/`p-password-leave`.
- Showcase page `/password` with the demos `basic`, `meter`, `togglemask`, `clearicon`, `template`, `locale`, `filled`, `sizes`, `fluid`, `disabled`, `invalid`, `floatlabel`, `iftalabel`; e2e; benchmarks and budgets; `docs/password.md`.
- Styles: `password` is added to `theme.components` and `primeng_extra` (the `/* For PrimeNG */` block with animations and `min-width`).
- **Out of scope:** the field's `tabindex` and `aria-labelledby` (`InputText` does not have them), the `pPassword` directive (not needed next to `InputText`), `mediumRegex`/`strongRegex` (`strength` instead), `appendTo` other than `self`, `overlayOptions`, `motionOptions` and the deprecated `showTransitionOptions`/`hideTransitionOptions`, Angular Forms bindings (`ngModel`, reactive forms), per-part `pt` with context.

## Capabilities

### New Capabilities
- `password`: a password field with a strength indicator, show toggle and clearing.

### Modified Capabilities
<!-- None. -->

## Impact

- New crate `crates/prime-password` (`publish = false`); dependencies: `prime-core`, `prime-dom`, `prime-inputtext`, `prime-icons` (`eye`, `eyeslash`, `times`), `web-sys`.
- Changed: workspace `Cargo.toml`, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (browser job, three places).
