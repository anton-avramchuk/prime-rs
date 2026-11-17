# Proposal

## Why

After `optimize-reactive-props` Button makes 20 allocations during SSR, but in the browser it still takes ≈ 6.7 KB of wasm linear memory per instance (an icon — 0.33 KB; the measurement: 1000 buttons = +6.75 MB). The reason is a `RenderEffect` for each attribute closure in `view!`: a button has about twelve of them (`class`, `style`, `type`, `aria-label`, `tabindex`, `disabled`, three `data-p*` and the dynamic label, icon and badge blocks), even when all the properties are constants and nothing can change. Most buttons on real pages are static, so there is no point paying for reactivity that is not needed. The goal of the project is "no unnecessary memory use"; this is the largest remaining item.

A prototype in the previous change showed that "a string or a closure" cannot be expressed as one attribute value (`Either` does not implement `AttributeValue`), so the static path is done at the level of the whole markup: the component chooses one of two `view!` branches.

## What Changes

- `Badge` and `Button`: if all properties are static (`Prop::is_static`), the component renders the markup with ready attribute and block values (without closures and `RenderEffect`); otherwise the current reactive path. The choice is made once, when the instance is created.
- The markup is written once in a `macro_rules!` and substituted into both branches, so that the branches do not diverge; both branches are covered by the same tests.
- `prime-rs-core`: a helper for checking "all properties are static" (`all_static!`/a function) and the common building of the static attribute values.
- Benchmarks: the browser memory scenario is split into "static buttons" and "reactive buttons" (`loading` is a signal); a wasm memory budget for 1000 static buttons is added (it does not block PRs, it is recorded by the weekly job).
- The behaviour of the components does not change: classes, attributes, events, `pt`, reactivity when a signal is passed.
- **Not included:** optimization of the wasm binary size, the other components.

## Capabilities

### New Capabilities
<!-- None. -->

### Modified Capabilities
- `core-runtime`: a requirement for the helper of the static checking of properties.
- `badge`: a requirement for the static render path.
- `button`: the same for Button.

## Impact

- Changes to `crates/prime-rs-core` (the helper), `crates/prime-badge`, `crates/prime-button`, `showcase` (the "reactive buttons" scenario), `e2e/tests/memory.spec.ts`, `docs/benchmarks.md`, `docs/button.md`.
- Growth of the wasm size: the markup code exists in two branches (expected +5–15 % to the component's contribution); the budget in `benches/budgets.toml` is revised by the measurement.
- Risk: the two branches diverging in behaviour. It is covered by a single source of markup (a macro) and by parameterized tests that run each scenario in both branches.
