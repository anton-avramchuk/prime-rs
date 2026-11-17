# Proposal

## Why

ProgressSpinner is an indicator of an indeterminate wait: one SVG with a rotating circle and no client-side logic. It is needed as a standalone component and as a template for future loading indicators (Button, Table, Dropdown). The port is small and closes one more pattern: the SVG markup with configurable circle attributes.

## What Changes

- A new crate `prime-progressspinner` with the `ProgressSpinner` component: `stroke_width` (default `"2"`), `fill` (`"none"`), `animation_duration` (`"2s"`), `aria_label`, `style`, `style_class`, `pt` (the parts `root`, `spin`, `circle`).
- Markup and classes as in PrimeNG 21.1.10 (`progressspinnerstyle.ts`): the root `div.p-progressspinner` with `role="progressbar"`, `aria-busy="true"` and `aria-label`; inside it `svg.p-progressspinner-spin` (`viewBox="25 25 50 50"`, `animation-duration` in the `style`) and `circle.p-progressspinner-circle` (`cx="50" cy="50" r="20"`, `fill`, `stroke-width`, `stroke-miterlimit="10"`).
- The `/progressspinner` page (`basic`, `custom`), e2e, benchmarks, budgets, `docs/progressspinner.md`, the CSS of the theme `progressspinner`.
- **Not included:** the selector synonym `p-progressSpinner`, `BlockableUI`; the outdated `styleClass` is replaced by `style_class`.

## Capabilities

### New Capabilities
- `progressspinner`: the ProgressSpinner component — structure and classes, ARIA, the properties of the circle and the animation, `pt`, performance.

### Modified Capabilities
<!-- None. -->

## Impact

- A new crate `crates/prime-progressspinner` (`publish = false`); dependency `prime-core`.
- Changes to the workspace `Cargo.toml`, `upstream.toml` (`progressspinner` in `theme.components`), `showcase`, `benches/*`, `e2e/tests`. There are no browser tests, the `ci.yml` job does not change.
