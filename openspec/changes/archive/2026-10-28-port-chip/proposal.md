# Proposal

## Why

Chip is the next component of wave 1 (Tag has been ported). Unlike Badge and Tag, it is interactive: it is removable (`removable`), reacts to the keyboard, has events and a state "hidden after removal", a localizable label, and an image with loading-error handling. This is the first component where the static render branch must coexist with an internal state, so it tests the `Prop` + two-branch template on behaviour, not only on markup.

## What Changes

- A new crate `prime-chip` with the `Chip` component: `label`, `icon`, `image`, `alt`, `disabled`, `removable`, `remove_icon`, `style`, `style_class`, `children`, `pt`, the events `on_remove`, `on_image_error`; the classes `p-chip*` and `data-p` as in PrimeNG 21.1.10.
- Removal by click and by the Enter and Backspace keys; after removal the Chip is hidden (`display: none`), as in PrimeNG; the `aria-label` of the remove button is taken from `PrimeConfig.locale` (`aria.removeLabel`).
- The static render branch is used when all properties are static and the Chip is not removable (a removable one needs internal visibility state, so it takes the reactive path).
- The `/chip` page in the showcase (`basic`, `icon`, `image`, `template`), e2e, benchmarks and budgets.
- **Not included:** `chipProps`, icon fonts, the `unstyled` mode.

## Capabilities

### New Capabilities
- `chip`: the Chip component — label, icon, image, removal, events, accessibility, `pt`, performance.

### Modified Capabilities
<!-- None. -->

## Impact

- A new crate `crates/prime-chip` (`publish = false`); depends on `prime-rs-core` and `prime-icons` (feature `timescircle`).
- Changes to `showcase`, `benches/render`, `benches/wasm-size`, `benches/budgets.toml`, `benches/alloc-budgets.toml`, `e2e/tests`.
