# Proposal

## Why

The infrastructure is ready (`bootstrap-workspace` is archived), but there is not a single component yet. Button is the first port of wave 1: the simplest in logic, but it touches all the cross-cutting mechanisms (classes `p-*`, `pt`, ripple, icons, configuration, theme, benchmarks, size budgets), so it becomes the reference by which the other components are ported. Inside `p-button` `p-badge` is used, so Badge is ported together with it.

## What Changes

- A new crate `prime-badge` with the `Badge` component: the same set of properties, the classes `p-badge*` and `data-p`, as in PrimeNG 21.1.10.
- A new crate `prime-button` with the `Button` component: properties, events, the classes `p-button*`, `data-p*`, the `loading`/`disabled` states, icon, label, badge, `fluid`, ripple, basic `pt`.
- The icon is passed as a `ViewFn` (for example `<CheckIcon />`), not as a class name string: there is no PrimeIcons font in the project.
- Cross-cutting additions: the `Pt` type is extended to the set of parts of a component; `Ripple` is connected by `PrimeConfig.ripple`; `upstream.toml` records the sources from which the components are ported.
- Infrastructure: the `render-bench` scenarios, the cases `case-button` and `case-badge` in `wasm-size` and the budgets, the Button and Badge pages in the showcase, an entry in `e2e/tests/pages.json` for comparison with PrimeNG.
- **Not included:** the `pButton` directive and its helpers `ButtonLabel`/`ButtonIcon`, `buttonProps`, the `Fluid` context, the `unstyled` mode, `SplitButton`, `ButtonGroup`.

## Capabilities

### New Capabilities
- `badge`: the Badge component — value, severity, size, dot and circle, hiding.
- `button`: the Button component — appearance, states, icon, label, badge, events, accessibility, ripple, `pt`.

### Modified Capabilities
<!-- None: the existing specs (core-runtime, theme-css, …) do not change; Button uses them as they are. -->

## Impact

- New crates `crates/prime-badge`, `crates/prime-button` (`publish = false`).
- Changed: `prime-core` (an extension of `Pt`, without changing the requirements), `upstream.toml` (the list of CSS components already includes `button` and `badge`), `benches/render`, `benches/wasm-size`, `benches/budgets.toml`, `showcase`, `e2e/tests/pages.json`.
- Dependencies: `prime-core`, `prime-dom` (ripple, autofocus), `prime-icons` (`spinner`), `prime-theme` (`button`, `badge`), `prime-badge`. No new external dependencies.
- Components that depend on it in the future: `Chip`, `Tag`, `SplitButton` etc. will take from here the `pt` scheme, the work with icons and the tests.
