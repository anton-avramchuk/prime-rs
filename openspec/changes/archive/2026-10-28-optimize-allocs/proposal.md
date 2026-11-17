# Proposal

## Why

`render-bench` measurements show that the Leptos "floor" is 2 allocations for a `div` with a `span`, while ready components spend noticeably more: an empty `AvatarGroup` — 4, `Checkbox` — 34, `Message` — 36. A significant part is class and `data-p` strings that are rebuilt for every instance although the component has a finite number of states, and superfluous `Arc<Props>`/`Arc<Pt>` in the static branch. The budgets in `benches/alloc-budgets.toml` are set with headroom over the current values.

## What Changes

- `prime-core`: `ClassNames` assembles the string on the stack (no allocation), a `finish_cow()` is added that returns an interned `&'static str` for strings without user classes (a global table of bounded size), and a `String` for strings with `style_class`/`pt`. `DataP` gets `finish_cow()`.
- In all components `class`/`data-p` are returned as `Cow<'static, str>`; in the static branch `Arc<Props>`/`Arc<Pt>` are not created if closures and handlers do not use them.
- The budgets in `alloc-budgets.toml` are lowered according to the new measurements.
- **Out of scope:** changing the public API of the components and the markup (HTML stays byte-for-byte the same), wasm size optimization.

## Capabilities

### New Capabilities
<!-- None. -->

### Modified Capabilities
- `benchmarking`: allocation budgets are lowered, and a requirement is added that the shared assembly of classes and `data-p` does not allocate for typical states.

## Impact

- Changed: `crates/prime-core` and the states/components of all `prime-*` crates, `benches/alloc-budgets.toml`, `docs/benchmarks.md`.
