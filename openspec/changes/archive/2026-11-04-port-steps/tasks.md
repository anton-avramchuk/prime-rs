# Tasks

## 1. Foundation

- [x] 1.1 Add `steps` to `theme.components` (`upstream.toml`), `cargo xtask gen-theme`; check that the CSS of other components did not change and that the `p-steps*` classes are present

## 2. Crate `prime-steps`

- [x] 2.1 `state.rs` (classes, disabled state, `tabindex`, `aria-disabled`, click outcome, key target index, activation keys) with table-driven tests
- [x] 2.2 `Steps`, `StepsItem`, `StepsCommand`, `StepsPt` component: root, list, items, numbers and labels, SSR tests, rustdoc
- [x] 2.3 Click: `readonly`, disabled items, `on_active_index_change`, `active_index_signal`, `command` with the index, navigation cancelled without `url`
- [x] 2.4 Keyboard: ←/→/`Home`/`End`, roving `tabindex`, `Tab`, `Enter`/`Space`
- [x] 2.5 Browser `tests/web.rs` (one sequential test)

## 3. Showcase and e2e

- [x] 3.1 `/steps` page (`basic`, `controlled`, `interactive`), menu, route, `stylesheet(&[..])`, the crate in the workspace and showcase
- [x] 3.2 `e2e/tests/pages.json` and `steps.spec.ts` (styles, click, keys, buttons of the controlled demo); wait for `networkidle`, restart the showcase server; `PW_CHANNEL=chrome npx playwright test steps`

## 4. Benchmarks and budgets

- [x] 4.1 Scenarios `-1` and `-1000`, `case-steps` and budgets (first `cargo bench -p render-bench --bench alloc`; wasm measurement — in the background, with a separate budgets file)

## 5. Integration

- [x] 5.1 `docs/steps.md` (differences from PrimeNG, what was not verified); `prime-steps` in the browser job of `ci.yml` (three places)
- [x] 5.2 `cargo fmt --all -- --check`, clippy with CI exclusions and `--all-features` (and for the wasm target), tests (SSR one by one: `--exact`; effects only under `cfg(wasm32)`), deny, machete, `openspec validate port-steps`; CI `dev` is green

## Workflow follow-up

- Archive after CI.
- Next: ContextMenu.
