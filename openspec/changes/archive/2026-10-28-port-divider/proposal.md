# Proposal

## Why

Divider - the next component of wave 1 (after Button, Badge, Tag, Chip, Avatar). It is the "purest" component by logic: markup depends only on properties (direction, line type, content alignment), with no events or state, but with a non-trivial alignment class table and an inline `justify-content`/`align-items` style. It is a good check of the template against conditional class and style logic.

## What Changes

- New crate `prime-divider` with the `Divider` component: `layout` (`horizontal`, `vertical`), `type` (`solid`, `dashed`, `dotted`), `align` (`left`, `center`, `right`, `top`, `bottom`), `style`, `style_class`, `children`, `pt`; `p-divider*` classes, `role="separator"`, `aria-orientation`, `data-p` and the inline alignment style as in PrimeNG 21.1.10.
- Page `/divider` in showcase (`basic`, `content`, `type`, `vertical`, `login`), e2e, benchmarks and budgets.
- **Out of scope:** `unstyled` mode.

## Capabilities

### New Capabilities
- `divider`: the Divider component - direction, type, alignment, content, accessibility, `pt`, performance.

### Modified Capabilities
<!-- None. -->

## Impact

- New crate `crates/prime-divider` (`publish = false`); dependency `prime-core`.
- Changed: `showcase`, `benches/render`, `benches/wasm-size`, `benches/budgets.toml`, `benches/alloc-budgets.toml`, `e2e/tests`.
