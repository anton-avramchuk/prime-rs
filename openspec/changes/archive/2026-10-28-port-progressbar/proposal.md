# Proposal

## Why

ProgressBar is the last component of wave 1: a progress indicator (determinate and indeterminate). It has no client-side behavior, so it checks that the component template (two render branches, `pt`, `data-p`) also fits purely presentational components with a numeric value.

## What Changes

- New crate `prime-progressbar` with the `ProgressBar` component: `value` (a number, 0-100), `mode` (`determinate` by default, `indeterminate`), `show_value` (`true` by default), `unit` (`%` by default), `color`, `style`, `style_class`, `value_style_class`, the `content_template` slot (receives the value), `pt` (`root`, `value`, `label`).
- Markup and classes as in PrimeNG 21.1.10: root `role="progressbar"` with `aria-valuemin="0"`, `aria-valuemax="100"`, `aria-valuenow`, `aria-level` and classes `p-progressbar p-component p-progressbar-determinate|indeterminate`; in `determinate` mode - `div.p-progressbar-value` (`width: <value>%; display: flex`, with `color` - `background`) with `div.p-progressbar-label`; in `indeterminate` mode - only `div.p-progressbar-value`; `data-p` on the root and parts.
- The `/progressbar` page (`basic`, `dynamic`, `indeterminate`, `template`), e2e, benchmarks, budgets.
- **Not included:** update animations (theme CSS), form integration.

## Capabilities

### New Capabilities
- `progressbar`: the ProgressBar component - modes, value, label, classes, `pt`, performance.

### Modified Capabilities
<!-- None. -->

## Impact

- New crate `crates/prime-progressbar` (`publish = false`); the only dependency is `prime-core`.
- Changed: `showcase`, `benches/render`, `benches/wasm-size`, budgets, `e2e/tests`.
