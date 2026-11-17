# Proposal

## Why

Avatar is the next component of wave 1 (after Button, Badge, Tag, Chip). It represents a person with a label, an icon or an image and follows the proven template (`Prop`, static and reactive branches, parity tests) without internal state; in addition it adds `Prop` for enumerations with implicit default values (`size = normal`, `shape = square`), which go into `data-p`.

## What Changes

- A new crate `prime-avatar` with the `Avatar` component: `label`, `icon`, `image`, `size` (`normal`, `large`, `xlarge`), `shape` (`square`, `circle`), `style`, `style_class`, `aria_label`, `aria_labelledby`, `children`, `pt`, the event `on_image_error`; the classes `p-avatar*` and `data-p` as in PrimeNG 21.1.10.
- The `/avatar` page in the showcase (`label`, `icon`, `image`, `shape`, `size`, `template`), e2e, benchmarks and budgets.
- **Not included:** `AvatarGroup` (a separate PrimeNG component, a separate change), icon fonts, the `unstyled` mode.

## Capabilities

### New Capabilities
- `avatar`: the Avatar component — label, icon, image, size, shape, accessibility, `pt`, performance.

### Modified Capabilities
<!-- None. -->

## Impact

- A new crate `crates/prime-avatar` (`publish = false`); dependencies `prime-core`, `web-sys`.
- Changes to `showcase`, `benches/render`, `benches/wasm-size`, `benches/budgets.toml`, `benches/alloc-budgets.toml`, `e2e/tests`.
