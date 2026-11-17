# Proposal

## Why

Card — a container with optional areas (a header with an image, a title, a subtitle, content, a footer). It is the first wave 1 component with `ViewFn` slots, i.e. it checks how the `Prop` template + two render branches work for a component whose markup is mostly nested optional blocks. The `advanced` demo uses Button (already ported), so Card shows component composition.

## What Changes

- New crate `prime-card` with the `Card` component: `header` and `subheader` (text), slots `header_template`, `title_template`, `subtitle_template`, `footer_template` (`ViewFn`), `children` (content), `style`, `style_class`, `pt`; `p-card*` classes as in PrimeNG 21.1.10.
- Page `/card` in showcase (`basic`, `advanced`), e2e, benchmarks and budgets.
- **Out of scope:** `BlockableUI` (an Angular blocking interface), `unstyled` mode.

## Capabilities

### New Capabilities
- `card`: the Card component — areas, slots, content, `pt`, performance.

### Modified Capabilities
<!-- None. -->

## Impact

- New crate `crates/prime-card` (`publish = false`); dependency `prime-rs-core`.
- Changed: `showcase` (the page uses Button), `benches/render`, `benches/wasm-size`, `benches/budgets.toml`, `benches/alloc-budgets.toml`, `e2e/tests`.
