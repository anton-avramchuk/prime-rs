# Tasks

## 1. Fluid context and consumers

- [x] 1.1 `prime-rs-core::fluid`: `FluidContext`, `provide_fluid`, `in_fluid`, `resolve_fluid`; tests (outside the context, inside, nesting, explicit value takes precedence)
- [x] 1.2 `Button`, `InputText`, `Textarea`, `InputNumber`, `Select`: `fluid: OptProp<bool>` and resolution via `resolve_fluid` (static and reactive branches); existing tests stay green, new SSR tests for inheritance in each crate

## 2. New crates

- [x] 2.1 `crates/prime-buttongroup` (`ButtonGroup`, `ButtonGroupPt`): crate, component, SSR tests, rustdoc
- [x] 2.2 `crates/prime-fluid` (`Fluid`, `FluidPt`): crate, component with `provide_fluid`, SSR tests with all consumers, rustdoc
- [x] 2.3 `crates/prime-iconfield` (`IconField`, `InputIcon`, `IconPosition`, `IconFieldPt`, `InputIconPt`): crate, components, SSR tests, rustdoc
- [x] 2.4 `crates/prime-floatlabel` (`FloatLabel`, `FloatLabelVariant`, `FloatLabelPt`, `write_classes` with a table-driven test): crate, component, SSR tests, rustdoc
- [x] 2.5 `crates/prime-iftalabel` (`IftaLabel`, `IftaLabelPt`): crate, component, SSR tests, rustdoc

## 3. Theme, showcase, e2e

- [x] 3.1 Add `buttongroup`, `floatlabel`, `iftalabel` to `theme.components` (`upstream.toml`), `cargo xtask gen-theme`, make sure the CSS of other components did not change, attach the CSS
- [x] 3.2 Pages `/buttongroup`, `/fluid`, `/iconfield`, `/floatlabel`, `/iftalabel`, menu, routes, `stylesheet(&[..])`
- [x] 3.3 `floatlabel` and `iftalabel` demos on the InputText, InputNumber, Select, Textarea pages (per their PrimeNG documentation)
- [x] 3.4 `e2e/tests/pages.json` and spec files (`buttongroup`, `fluid`, `iconfield`, `floatlabel`, `iftalabel`) plus checks of the new demos on existing pages; wait for `networkidle`, restart the showcase server before the run; `npx playwright test`

## 4. Benchmarks and budgets

- [x] 4.1 Scenarios `-1` and `-1000`, `case-*` and budgets for `buttongroup`, `fluid`, `iconfield`, `floatlabel`, `iftalabel` (first `cargo bench -p render-bench --bench alloc`)

## 5. Integration

- [x] 5.1 `docs/field-wrappers.md` (differences from PrimeNG: the Fluid context, demos without the missing icons, the `/* For PrimeNG */` rules); clippy with `--all-features` without exclusions, tests, deny, machete, `openspec validate port-field-wrappers`; CI `dev` green

## Workflow follow-up

- Archive after CI.
