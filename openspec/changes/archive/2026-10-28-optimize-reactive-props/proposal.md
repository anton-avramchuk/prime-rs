# Proposal

## Why

One of the project's goals is no excess memory consumption and high component performance. Measurements of Badge and Button (`docs/benchmarks.md`) showed: there are 41 (Badge) and 67 (Button) allocations per instance in SSR rendering, whereas a "bare" `<span>` is 2. An experiment on `render-bench` broke the difference down:

| Variant | Allocations per instance |
|---|---:|
| `<span>` without reactivity | 2 |
| + one `Memo` | 11 (+9) |
| + one `MaybeProp` property | 5 (+3) |
| Badge now | 41 |
| Button now | 67 |

Every `Signal<T>` or `MaybeProp<T>` property creates an entry in the Leptos arena even when a constant or the default value is passed; plus a `Memo` for the state and a `class_memo` for the classes. Changing the components after another ten to twenty move to this template will be many times more expensive, so we optimize now, while there are two examples.

## What Changes

- A property type `Prop<T>` appears in `prime-rs-core`: either a value (`Static`) or a signal (`Reactive`). A static value creates no reactive primitives; default values are static.
- `Badge` and `Button` accept `Prop<T>` instead of `Signal<T>`/`MaybeProp<T>`: calling code does not change (`value="5"`, `loading=signal`, `severity=ButtonSeverity::Danger` keep working).
- The `Memo` for state and `class_memo` are removed from the components: classes and attributes are computed in reactive attribute closures (during SSR each runs once, in the browser — when dependencies change).
- Verifiable allocation-per-instance targets are added for `badge-1000` and `button-1000` (a budget in `benches`), the comparison in CI already exists (`bench-compare`).
- Component behavior (classes, attributes, events, reactivity) does not change; updating `class` only when the inputs change is preserved.
- **Out of scope:** wasm size optimization (separately, after this change), migrating the other components.

## Capabilities

### New Capabilities
<!-- None: the requirements of existing capabilities change. -->

### Modified Capabilities
- `core-runtime`: a new requirement for a "value or signal" property type with no reactive cost for static values.
- `badge`: a new render cost requirement (allocations per instance, no reactive primitives with static properties).
- `button`: the same for Button.

## Impact

- Changed: `crates/prime-rs-core` (the new `Prop`), `crates/prime-badge`, `crates/prime-button`, `benches/render`, `benches/budgets.toml` (or a separate allocation budget), `docs/benchmarks.md`, `docs/button.md`.
- The public API of the components is compatible at the call-site source level (`#[prop(into)]` accepts the same expressions); property types in signatures change to `Prop<T>`.
- Risk: building `showcase` and the demos; covered by building them and e2e (`button.spec.ts`).
