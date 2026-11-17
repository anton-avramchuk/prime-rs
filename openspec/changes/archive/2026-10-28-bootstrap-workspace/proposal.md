# Proposal

## Why

The prime-rs project is a port of PrimeNG components (21.1.10, commit `fe82b291d6`) to Rust/Leptos 0.8 on the "one component, one crate" principle. The repository is empty so far (only `openspec/`). Before porting the first component, infrastructure is needed ("wave 0"): the workspace structure, shared core, theme, icons, CI, benchmarks and protection against `target/` bloat. Without it every component would reinvent these things, and disk, memory and performance constraints would have to be imposed retroactively.

## What Changes

- Cargo workspace: `crates/`, `showcase/`, `xtask/`, `benches/`, a shared `[workspace.dependencies]`, `rust-toolchain.toml` (stable, wasm32).
- Crate `prime-rs-core`: config (`PrimeConfig` via Leptos context), `ClassNames`, z-index manager, locale, base props (`disabled`, `invalid`, `variant`, `size`), a shared `pt` (pass-through) helper for the root element.
- Crate `prime-dom`: a port of the needed part of `DomHandler` (positioning, focus, scroll), `focustrap`, `ripple`, portal; `web-sys` only, no `leptos-use`.
- Crate `prime-theme` and `xtask gen-theme`: generating static CSS from the Aura preset tokens and component styles (the npm packages `@primeuix/themes` and `@primeuix/styles` used by PrimeNG) with an `xtask` command; the result is committed.
- Crate `prime-icons` and `xtask gen-icons`: 53 PrimeNG SVG icons, one component per icon behind a feature flag.
- `upstream.toml` and `xtask upstream-diff`: pinning to the PrimeNG version and finding changes in the sources.
- `xtask gen-spec`: an OpenSpec spec stub of a component from the PrimeNG TS sources.
- CSR and SSR/hydrate modes via the `ssr`/`hydrate` features in all crates; `showcase` on cargo-leptos.
- Measures against disk growth: `debug = "line-tables-only"`, `rust-lld`, incremental builds only in dev, `cargo-sweep`, `xtask clean`, minimal dependencies.
- CI (GitHub Actions): on PRs — fmt, clippy, nextest, wasm-bindgen-test, deny, benchmarks (divan/dhat), wasm size budget, affected crates only; weekly on a schedule and manually — e2e (Playwright), screenshot comparison with PrimeNG, browser memory benchmarks.
- Licensing: MIT, `NOTICE` with the PrimeNG copyright.
- Publishing to crates.io is **not included** in the change.

## Capabilities

### New Capabilities
- `workspace-layout`: Cargo workspace structure, `prime-<name>` crate naming, `ssr`/`hydrate` feature flags, toolchain, license and `NOTICE`.
- `core-runtime`: `prime-rs-core` and `prime-dom` — config, context, ClassNames, z-index, pass-through, DOM helpers.
- `theme-css`: generating static CSS from the Aura tokens and including it.
- `icon-set`: the `prime-icons` crate and generating icons from PrimeNG.
- `upstream-tracking`: pinning the PrimeNG version, `upstream-diff`, generating spec stubs.
- `disk-footprint`: limits on `target/` size and build profiles.
- `ci-pipeline`: the set of PR checks and weekly checks (e2e, screenshots, browser benchmarks).
- `benchmarking`: SSR render and allocation benchmarks, wasm size budgets, regression threshold.

### Modified Capabilities
<!-- None: there are no existing specs in openspec/specs/. -->

## Impact

- All repository code is created from scratch; `Cargo.toml`, `rust-toolchain.toml`, `.cargo/config.toml`, `.github/workflows/`, `.gitignore` (add `nul`), `NOTICE`, `LICENSE`, `upstream.toml` are affected.
- Dependencies: `leptos` 0.8, `web-sys`, `wasm-bindgen`; dev: `divan`, `dhat`, `wasm-bindgen-test`. On the CI side: `cargo-nextest`, `cargo-deny`, `cargo-sweep`, Playwright (weekly job only).
- `C:\Projects\primeng` is read (read-only); it is not copied into the repository.
- Components (Button and onward) are not part of this change: each goes in a separate change.
