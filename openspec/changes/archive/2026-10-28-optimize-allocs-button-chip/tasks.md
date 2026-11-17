# Tasks

## 1. Implementation

- [x] 1.1 Chip: `Props`/`Pt` without `Arc` in the static branch, blocks without `.into_any()`, `Pt::merge_style_parts`; verify with SSR and browser tests
- [x] 1.2 Button: `Props`/`Pt` without `Arc` in the static branch, `NodeRef` only on `wasm32`, `EitherOf4`/`Option<impl IntoView>`, `icon_classes` → `Cow`; verify with SSR, parity and browser tests

## 2. Budgets and integration

- [x] 2.1 Recompute the `button-*`, `chip-*` budgets, extend `docs/benchmarks.md`
- [x] 2.2 clippy, tests, wasm-size, deny, machete, `openspec validate optimize-allocs-button-chip`; CI `dev` green

## Workflow follow-up

- Archive after CI.
