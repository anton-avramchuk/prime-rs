# Tasks

## 1. Foundation

- [x] 1.1 Add `stepper` to `theme.components` and `primeng_extra` (`upstream.toml`), `cargo xtask gen-theme`; check that the CSS of the other components did not change, that the `p-step*`, `p-steppanel*`, `p-stepitem*`, `p-stepper-separator` classes and the `.p-steppanel .p-motion` rule are present

## 2. Crate

- [x] 2.1 `crates/prime-stepper`: `state.rs` (`step_id`, `panel_id`, `is_active`, `step_disabled`, `is_last`, classes) with table-driven tests
- [x] 2.2 `Stepper`, `StepList`, `Step`, `StepContext`, `StepperPt`: root, value, steps with states and separators, step registry, SSR tests, rustdoc
- [x] 2.3 `StepPanels`, `StepPanel` (horizontal layout), `content`, `activate` transitions, linear mode; browser `tests/web.rs`
- [x] 2.4 `StepItem` and vertical layout: item value for `Step` and `StepPanel`, panel separator, hiding and expansion animation

## 3. Showcase and e2e

- [x] 3.1 The `/stepper` page (`basic`, `linear`, `vertical`, `template`, `stepsonly`), menu, route, `stylesheet(&[..])`, crate in the workspace and showcase
- [x] 3.2 `e2e/tests/pages.json` and `stepper.spec.ts` (markup, click on a step, linear mode, "Next"/"Back" buttons, vertical mode); wait for `networkidle`, restart the showcase server; `PW_CHANNEL=chrome npx playwright test stepper`

## 4. Benchmarks and budgets

- [x] 4.1 Scenarios `-1` and `-1000`, `case-stepper` and budgets (first `cargo bench -p render-bench --bench alloc`; the wasm measurement in the background, with a separate budgets file)

## 5. Integration

- [x] 5.1 `docs/stepper.md` (differences from PrimeNG, what was not verified); `prime-stepper` in the browser job of `ci.yml` (three places)
- [x] 5.2 `cargo fmt --all -- --check`, clippy with the CI exclusions and `--all-features` (and for the wasm target), tests, deny, machete, `openspec validate port-stepper`; CI `dev` green

## Workflow follow-up

- Archive after CI.
- Wave 3 is done; next - at the user's decision.
