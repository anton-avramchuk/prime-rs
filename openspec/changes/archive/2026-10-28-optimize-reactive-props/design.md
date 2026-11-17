# Design

## Context

The motivation and the measurement table are in `proposal.md`, the requirements are in `specs/`. What was found out:

- `exp-raw`: a "bare" `<span class data-p>` ×1000 = 2 allocations per instance; `Pt::new().merge_class()` + `attrs()` add 0; one `Memo` — +9; one `MaybeProp<String>` (from a value) read in a closure — +3.
- Badge: 5 properties (`value`, `severity`, `size`, `badge_disabled`, `style_class`) + `Memo<BadgeState>` + `class_memo` ≈ 41. Button: ≈ 25 signal properties + two `Memo`s ≈ 67. This is consistent: the cost is almost entirely the reactive primitives of the properties, not the markup.
- `Signal<T>` and `MaybeProp<T>` by default (`#[prop(optional)]`) also create arena entries per instance.
- In the browser (CSR/hydrate) every attribute closure in `view!` creates a `RenderEffect` even without dependencies; during SSR the closure runs once without primitives.

## Goals / Non-Goals

**Goals:**
- A static Badge/Button creates no signals or `Memo`s (property values and state), the allocation count during SSR is close to bare markup.
- Call compatibility is preserved (`value="5"`, `loading=signal`, `severity=ButtonSeverity::Danger`), as are the behavior and all existing tests.
- The template the other components will follow: `Prop<T>` + a static fast path.

**Non-Goals:**
- wasm size (separately: `opt-level`, `wasm-opt`, the set of web-sys features).
- Changing component APIs, behavior specs and markup.

## Decisions

### D1. The `Prop<T>` type in `prime-core`
```rust
pub enum Prop<T> { Static(T), Reactive(Signal<T>) }
```
Constructors via `From`: `T` → `Static`; `Signal<T>`, `ReadSignal<T>`, `RwSignal<T>`, `Memo<T>` → `Reactive`; for strings — `&str` → `Prop<String>`. `Default` is `Static(T::default())`. Methods: `get()` (a tracking read; for `Static` a clone), `get_untracked()`, `is_static()`. For optional values — the alias `OptProp<T> = Prop<Option<T>>` with `From<T>` (`Some`). The coercions `From<T> for Prop<T>` and `From<Signal<T>> for Prop<T>` do not overlap (T cannot equal `Signal<T>`); for `Prop<Option<U>>` likewise by the occurs check. If the compiler refuses, `OptProp<T>` becomes a separate type.
*Alternatives*: Leptos's `MaybeSignal` (removed in 0.8, replaced by `Signal` with an arena entry even for a value); generics per component (`Badge<V: Into<...>>`) — break type inference and `view!`.

### D2. Attributes are closures, no "fast path"
A prototype on `render-bench` (1000 instances, allocations per instance): closure attributes 5, ready static strings 4, `AnyAttribute` 7. So during SSR the static fast path wins only ≈ 1 allocation, and "a string or a closure" cannot be expressed as one value: `Either` does not implement `AttributeValue`, and `AnyAttribute` is more expensive. Decision: attributes are always closures (`class=move || …`); the savings come from `Prop` and dropping `Memo`.
In the browser each attribute closure creates a `RenderEffect`; their cost is measured by the browser "1000 components" scenario (task 5.2). If it turns out to be noticeable, a fast path will be added in a separate change.
*Rejected*: `Either<String, closure>` (does not compile), `Vec<AnyAttribute>` (more allocations).

### D3. No `Memo`, properties in a single `Arc`
`BadgeState`/`ButtonState` borrow values (`Option<&str>`) and are assembled inside the attribute closures; classes and `data-p` are functions of the state. There are no extra DOM updates: verified by the browser test "`class` is not rewritten when an unrelated property changes". A `Memo` was not needed on the dynamic path: the decision is recorded, the question is closed.
The measurement showed that `Prop::Static(String)` copies the string on every `.clone()`, so all of an instance's properties are put into a single `Arc<Props>`, and closures clone only the reference count. This reduced allocations by another 1–5 per instance and Badge's peak memory by 40 %.

### D4. Allocation budgets
`benches/alloc-budgets.toml`: `max_blocks_per_instance` for `badge`, `button`, `button-full`; the `cargo xtask alloc-budget` command reads `target/bench/alloc.json` and fails on excess (like `wasm-size`). Measured after the migration (allocations per instance, SSR): `badge` 8, `button` 20, `button-full` 50 (was 41, 67 and 128). Budgets are set with ≈ 20 % headroom: `badge` 10, `button` 24, `button-full` 60.

### D5. Browser verification
The memory scenario from `e2e/tests/memory.spec.ts` (1000 icons) is extended with a "1000 Buttons" scenario; `browser.mount-*.heap_bytes` and `dom_nodes` are compared before and after.

## Risks / Trade-offs

- [The extra `Prop` type complicates signatures and IDE hints] → a single type in all components, documented examples; `Into` keeps the call shape.
- [`Prop<Option<T>>` coercions may fail to compile due to coherence rules] → a separate `OptProp<T>`, see D1.
- [In the browser a `RenderEffect` per attribute remains] → measured in 5.2; if the cost is noticeable — a separate change.
- [Reactivity regression] → the existing reactivity tests (SSR and browser) stay unchanged and must pass.

## Migration Plan

No migration for users: the original call is compatible. Rollback — `git revert`. Allocation budgets are enabled in CI after the new values appear.

## Open Questions

- Is a state `Memo` needed on the dynamic path (D3)? Decided by measurement, does not affect the specs.
