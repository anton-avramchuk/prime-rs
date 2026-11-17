# Proposal

## Why

Tag is the next wave 1 component after Button and Badge: it categorizes content (value, severity, icon, rounding). Its logic is simple and fits entirely into the proven template (`Prop`, two render branches, parity tests), so it serves as a check that the template carries over to a new component without infrastructure changes.

## What Changes

- New crate `prime-tag` with the `Tag` component: properties `value`, `severity`, `rounded`, `icon`, `style`, `style_class`, `children`, `pt`; classes `p-tag*`, `data-p` as in PrimeNG 21.1.10.
- Static and reactive render branches, properties are `Prop`/`OptProp`, allocation and wasm size budgets.
- Page `/tag` in the showcase with the PrimeNG demos (`basic`, `icon`, `pill`, `severity`, `template`) and screenshot comparison in the weekly job.
- **Out of scope:** PrimeIcons font icons (the icon is a `ViewFn`), `unstyled` mode.

## Capabilities

### New Capabilities
- `tag`: the Tag component: value, severity, rounding, icon, content, `pt`, performance.

### Modified Capabilities
<!-- None. -->

## Impact

- New crate `crates/prime-tag` (`publish = false`).
- Changed: `showcase` (page, menu), `benches/render` (scenarios), `benches/wasm-size` (`case-tag`), `benches/budgets.toml`, `benches/alloc-budgets.toml`, `e2e/tests/pages.json`, `e2e/tests` (Tag spec), `crates/prime-theme` (the `tag` CSS is already generated).
- Dependencies: `prime-rs-core`; no new external ones.
