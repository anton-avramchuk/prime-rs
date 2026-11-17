# Tasks

## 1. The `Prop<T>` property type

- [x] 1.1 Implement `Prop<T>`/`OptProp<T>` in `prime-rs-core` (`From` constructors, `Default`, `get`, `with`, `get_untracked`, `is_static`); verify the `core-runtime` spec scenarios with unit tests (a static value creates no primitives, a signal reacts, the default value) and by compiling the calls `value="5"`, `RwSignal`, `Memo`
- [x] 1.2 Document `Prop` in rustdoc with examples and check `cargo test --doc -p prime-rs-core`; if the `Prop<Option<T>>` coercions do not compile — move `OptProp<T>` into a separate type and record the decision in `design.md`

## 2. Badge

- [x] 2.1 Move `Badge` to `Prop`/`OptProp`, remove `Memo` and `class_memo`, compute attributes in closures; check that all existing SSR tests of `prime-badge` pass without changing expected values
- [x] 2.2 Add tests of both branches (static and reactive, change of `value` and `severity`) and measure `badge-1000` via `cargo bench -p render-bench --bench alloc`; record the value before and after

## 3. Button

- [x] 3.1 Move `Button` to `Prop`/`OptProp` (all properties, including `badge`, `severity`, `loading`, `disabled`), remove superfluous `Memo`s, compute attributes in closures; check that all SSR and browser tests of `prime-button` pass without changing expected values
- [x] 3.2 Measure `button-1000` and `button-full-1000`; if a single state `Memo` on the dynamic path gives a noticeable gain, keep it and record the decision in `design.md` (D3); check the test "class is not rewritten when an unrelated property changes"

## 4. Allocation budgets

- [x] 4.1 Implement `cargo xtask alloc-budget` and `benches/alloc-budgets.toml` (`badge`, `button`, `button-full`: maximum allocations per instance); check with unit tests of parsing and excess and by running on a real `target/bench/alloc.json`, including an artificially lowered budget (non-zero exit code)
- [x] 4.2 Add `alloc-budget` to the `apps` job of the `ci.yml` workflow and a description in `docs/benchmarks.md`; check the YAML syntax and a CI run on GitHub

## 5. Verification in the app

- [x] 5.1 Build the showcase (`cargo leptos build`) and run e2e (`npx playwright test`) with the `/button` and `/badge` pages; make sure the look and behavior did not change (including the Loading page)
- [x] 5.2 Add a "1000 buttons" scenario to `e2e/tests/memory.spec.ts`, record the `heap_bytes`/`dom_nodes` values before and after in `docs/benchmarks.md` and estimate the cost of the attributes' `RenderEffect` (D2)

## 6. Integration

- [x] 6.1 Update `docs/benchmarks.md` and `docs/button.md` (new allocation values and the `Prop` template and the conclusion about closure attributes for the next components); run `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace --all-features` (excluding apps), `cargo deny check`, `openspec validate optimize-reactive-props`; make sure CI on `dev` is green

## Workflow follow-up

- Archive the change after CI passes.
- The next wave 1 components (Tag, Chip, Avatar, Divider, Card, …) take `Prop` as the template.
