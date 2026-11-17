# Proposal

## Why

After `optimize-allocs` Button (19 allocations per instance) and Chip (10, removable — 35) remained the most expensive: event handlers held `Arc<Props>` in the static branch too, `Arc<Pt>` was always created, `NodeRef` — during SSR, and blocks were assembled via `.into_any()`.

## What Changes

- Button and Chip: `Props` and `Pt` are plain values; `Arc` is created only in the reactive branch. The click handler of a static button captures a ready `bool`.
- Button: `NodeRef` and `node_ref` are created only in the browser (`wasm32`); the indicator, label and badge blocks are `EitherOf4`/`Option<impl IntoView>` instead of `AnyView`; `icon_classes` returns `Cow<'static, str>`.
- Chip: the image, label and remove blocks without `.into_any()`; the root style is assembled as one string (`Pt::merge_style_parts`).
- The budgets in `benches/alloc-budgets.toml` for `button-*` and `chip-*` are lowered according to the new measurements.
- **Out of scope:** public API and markup (HTML does not change).

## Capabilities

### New Capabilities
<!-- None. -->

### Modified Capabilities
- `benchmarking`: a requirement is added that there be no `NodeRef` and `Arc` in the static branch during SSR.

## Impact

- `crates/prime-button`, `crates/prime-chip`, `crates/prime-rs-core` (`Pt::merge_style_parts`), `benches/alloc-budgets.toml`, `docs/benchmarks.md`.
