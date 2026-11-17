# Tasks

## 1. Core

- [x] 1.1 `ClassNames`: stack buffer, a "dynamic" flag, `finish_cow()` with interning (limit 4096); unit tests (same result, a repeated call gives the same `&'static str`, dynamic strings are not interned, buffer overflow)
- [x] 1.2 `DataP::finish_cow()`; measurement on an empty scenario: a repeated call does not allocate (a test with an allocation counter in `render-bench`)

## 2. Components

- [x] 2.1 Switch `class`/`data-p` to `Cow<'static, str>` in `prime-avatargroup` and `prime-progressbar`, remove superfluous `Arc`s in the static branch; verify the allocation count and HTML
- [x] 2.2 The same in `prime-badge`, `prime-tag`, `prime-chip`, `prime-avatar`, `prime-divider`, `prime-card`
- [x] 2.3 The same in `prime-button`, `prime-inputtext`, `prime-checkbox`, `prime-radiobutton`, `prime-toggleswitch`, `prime-message`

## 3. Budgets and integration

- [x] 3.1 Recompute `benches/alloc-budgets.toml` from measurements (headroom ≤ 15 %), extend `docs/benchmarks.md` with a before/after table
- [x] 3.2 `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace --exclude showcase --exclude wasm-size --all-features`, browser tests, `cargo deny check`, `cargo machete`, `openspec validate optimize-allocs`; CI `dev` green

## Workflow follow-up

- Archive after CI.
