# Design

## Context

The motivation and measurements are in `proposal.md`, the requirements are in `specs/`. What is known:

- The previous change (`optimize-reactive-props`) brought the properties to `Prop`/`OptProp` and removed `Memo`; during SSR the attribute closures are almost free (5 allocations against 4 for ready strings), but in the browser each attribute closure is a `RenderEffect` (≈ 0.5 KB). A button has about 12 of them: in total 6.7 KB of wasm linear memory per instance against 0.33 KB for an icon (a measurement over 1000 instances, release).
- A single value "a string or a closure" cannot express an attribute: `Either` does not implement `AttributeValue`, and `AnyAttribute` is more expensive (7 allocations against 5). The working variant is branching at the level of the whole markup: `Either<A, B>` implements `Render` and `RenderHtml`, so the component can return different `view!`s depending on whether the properties are static.
- In `view!` the identifier before `{..attrs}` must be wrapped in `{}` (otherwise the macro reads it as a struct literal).

## Goals / Non-Goals

**Goals:**
- Static Badge/Button create no `RenderEffect` for attributes and blocks.
- The branches do not diverge: the markup is written once, the tests run both branches.
- Memory target: ≤ 3 KB per static button (it was 6.7 KB) — checked by a spike in the first group of tasks (go/no-go).

**Non-Goals:**
- The size of the wasm binary, the other components, the change of the API.

## Decisions

### D1. Choosing the branch when the instance is created
`let is_static = props.all_static();` (all `Prop::is_static()`; `ViewFn`, `children` and `pt` are always static). Then `if is_static { Either::Left(static_view) } else { Either::Right(dynamic_view) }`. The decision is made once: if a property later turns out to be a signal, the whole instance is recreated, not the branch.

### D2. The markup once — through `macro_rules!`
The macro `button_markup!` accepts the expressions of the attributes and blocks (`$class`, `$style`, `$disabled`, `$content`, …) and expands into one `view!`; the static branch passes ready values (`String`, `Option<&'static str>`, `bool`), the reactive one — `move ||` closures. A divergence of the branches in structure is excluded: only the type of the expressions changes.
*Alternatives*: two hand-written `view!`s (they diverge over time); a shared `Either` attribute (does not compile, see the D context); `AnyAttribute` (more expensive).

### D3. Static values are computed once
In the static branch the state and the strings (`class`, `data-p`, `style`, `type`, `aria-label`, `tabindex`) are computed once at the start; the blocks (icon or indicator, label, badge) are ordinary `Option<View>`, not closures. The same for Badge.

### D4. Checking parity
Parameterized tests: each SSR scenario (`ssr.rs`) runs twice — with static values and with the same values through an `RwSignal` — and compares the HTML. The browser event tests run in both branches.

### D5. Measurement
`e2e/tests/memory.spec.ts` gets the scenarios "static buttons" and "reactive buttons" (one of them passes `loading` as a signal); both rows are recorded in `docs/benchmarks.md`.

## Risks / Trade-offs

- [Two copies of the markup in the binary: wasm growth of +5–15 % of the contribution] → measure `wasm-size`, revise the budgets.
- [The static path does not reach the target of ≤ 3 KB] → a spike (task 1.2) with a go/no-go criterion: if the gain is < 40 %, the change is folded, and the result is recorded in `docs/benchmarks.md`.
- [The macro complicates reading] → one pair of macros per component, a comment, how to add an attribute; the rule for the next components is in `docs/button.md`.
- [A reactive property added later unexpectedly changes the branch] → the branch is selected by `Prop`, documented; the behaviour of both branches is the same by the tests.

## Migration Plan

No migration: the public API and the markup do not change. Rollback — `git revert`. The wasm size budget in `benches/budgets.toml` is revised after the measurement.

## Open Questions

None.
