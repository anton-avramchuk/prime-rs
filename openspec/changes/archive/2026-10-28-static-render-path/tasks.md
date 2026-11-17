# Tasks

## 1. Helper and spike (go/no-go)

- [x] 1.1 Add to `prime-core` a check "all properties are static" (a function or a macro over several `Prop`s) and check the scenarios of the `core-runtime` spec (all static, a signal is present) with unit tests, and the absence of allocations (dhat)
- [x] 1.2 A spike on `Badge`: a static branch with ready attributes and `Either`; build the showcase, measure in the browser (`memory.spec.ts`, 1000 badges) the growth of wasm memory before and after and record it in `docs/benchmarks.md`; **go** if the growth per instance is ≥ 40 % lower, otherwise stop and record the conclusion, without continuing groups 2–5

## 2. Badge

- [x] 2.1 Move the Badge markup into a macro, substitute it into the static and reactive branches, select the branch by `all_static`; check that the existing SSR tests of `prime-badge` pass without changes to the expected values
- [x] 2.2 Add parameterized parity tests (each scenario statically and through an `RwSignal`, the HTML matches); check `cargo test -p prime-badge --all-features`

## 3. Button

- [x] 3.1 Move the Button markup into a macro, add a static branch (ready strings, the icon/label/badge blocks as `Option<View>`), select the branch by `all_static`; check that all SSR tests of `prime-button` pass without changes to the expected values
- [x] 3.2 Add parameterized SSR parity tests and run the browser event tests (click, `disabled`/`loading`, focus, autofocus, ripple, rewriting of `class`) in both branches; check `cargo test -p prime-button --target wasm32-unknown-unknown --test web`

## 4. Measurements and budgets

- [x] 4.1 Add the scenarios "static buttons" and "reactive buttons" (with the `loading` signal) to the showcase and `memory.spec.ts`; check `npx playwright test memory` locally and record the values before and after in `docs/benchmarks.md`
- [x] 4.2 Measure `cargo xtask wasm-size` and `cargo bench -p render-bench --bench alloc`; revise `benches/budgets.toml` and `benches/alloc-budgets.toml`; check `cargo xtask wasm-size` and `alloc-budget`

## 5. Integration

- [x] 5.1 Update `docs/button.md` (the rule "one markup in a macro, two branches" for the next components) and `docs/benchmarks.md`; run `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace --all-features` (except the applications), `cargo deny check`, `openspec validate static-render-path`; make sure CI on `dev` is green and the weekly job publishes the new memory metrics

## Workflow follow-up

- Archive the change after CI passes.
- The next wave-1 components use `Prop` and the "static/reactive branch" pair as the template.
