# Proposal

## Why

Rating is star rating selection, often needed in forms and cards; in complexity it is a medium-sized component built on a group of native radio buttons (like RadioButton) with no overlays and no shared logic. The next port in the queue after SelectButton.

## What Changes

- A new crate `prime-rating` with the `Rating` component: a two-way `value` (`RwSignal<Option<u32>>`) or internal state with `default_value`, `stars` (5 by default), `readonly`, `disabled`, `invalid`, `name`, `required`, `style`, `style_class`, `on_icon`/`off_icon` (`ViewFn`) to replace the default stars, the `on_icon_template` and `off_icon_template` slots (receive the star number and the icon class), the events `on_rate` (new value and the original event), `on_focus`, `on_blur`, `pt`.
- Markup as in PrimeNG 21.1.10 (`rating.ts`, `ratingstyle.ts`): root `<div class="p-rating" data-p>` with `p-readonly` and `p-disabled`; for each star a `div.p-rating-option` (`p-rating-option-active`, `p-focus-visible`) with a hidden `<input type="radio">` in a `span.p-hidden-accessible` and the icon `p-rating-icon p-rating-on-icon` or `p-rating-off-icon` (`p-invalid`). Default stars are `StarFillIcon` and `StarIcon` from `prime-icons`. Selecting the current value again resets it to `None`; star navigation is by arrows, as in a native radio button group. Each star's `aria-label` is from the locale (`aria.star`, `aria.stars`).
- `prime-rs-core` gets a shared unique number generator (extracting `next_unique` from `collapsible`), which also gives the default radio group name (`pn_id_<n>_name`).
- The `/rating` page (`basic`, `disabled`, `numberofstars`, `readonly`, `template`), e2e, browser tests, benchmarks, budgets, `docs/rating.md`, the `rating` theme CSS.
- **Out of scope:** Angular Forms (`ngModel`, reactive forms); `autofocus`; `icon_on_class`/`icon_off_class`/`icon_on_style`/`icon_off_style` (icon font classes: `on_icon`/`off_icon` and templates instead); the `/* For PrimeNG */` block (the `.ng-invalid.ng-dirty` rule); the `reactiveforms` and `templatedrivenforms` demos; the `withoutcancel` demo (in 21.1.10 Rating already has no cancel button, the demo duplicates `basic`).

## Capabilities

### New Capabilities
- `rating`: the Rating component: value and events, mouse and keyboard selection, structure and classes, accessibility, `pt`, performance.

### Modified Capabilities
<!-- None: `prime-rs-core` gets an internal shared helper with no behaviour change. -->

## Impact

- A new crate `crates/prime-rating` (`publish = false`); dependencies `prime-rs-core`, `prime-icons` (`star`, `starfill`), `prime-dom` is not needed.
- Changed: `prime-rs-core` (`ids`, `collapsible`), workspace `Cargo.toml`, `upstream.toml` (`rating` in `theme.components`), `showcase`, `benches/*`, `e2e/tests`, `ci.yml` (browser job for `tests/web.rs`).
