# Tasks

## 1. Workspace skeleton and toolchain

- [x] 1.1 Create the root `Cargo.toml` (workspace, `resolver = "3"`, `[workspace.dependencies]`, `[workspace.lints]`) and `rust-toolchain.toml` (stable 1.98, wasm32, rustfmt, clippy); check that `cargo metadata` has no errors
- [x] 1.2 Add `.cargo/config.toml` (rust-lld for windows-msvc and linux-gnu) and the profiles from design D10; check `cargo build -p xtask` and the size of `target/` after the build
- [x] 1.3 Create empty crates `prime-core`, `prime-dom`, `prime-theme`, `prime-icons` with the `ssr`/`hydrate` features; check `cargo check --workspace` and `cargo check -p prime-core --target wasm32-unknown-unknown --features hydrate`
- [x] 1.4 Add `LICENSE` (MIT), `NOTICE` (PrimeNG 21.1.10), a `nul` line in `.gitignore`, `deny.toml`; check `cargo deny check`
- [x] 1.5 Write `README.md` and `docs/build.md` (structure, `CARGO_TARGET_DIR` for Windows and Linux, xtask commands); check that the described commands run as written

## 2. xtask and upstream-tracking

- [x] 2.1 Create `xtask` with stub subcommands and the `cargo xtask` alias in `.cargo/config.toml`; check `cargo xtask --help`
- [x] 2.2 Create `upstream.toml` (version, commit, path, `@primeuix/*` versions) and a reading module with `PRIMENG_PATH` override; check reading and override with a unit test
- [x] 2.3 Implement `xtask upstream-diff` and tests on a temporary git repository ("changed" and "unchanged" scenarios)
- [x] 2.4 Implement `xtask gen-spec <name>` with a section template (props, events, DOM/classes, a11y, keyboard, performance, link to the source and commit); check with a snapshot test (`insta`) on `button` and with an error for a nonexistent component
- [x] 2.5 Implement `xtask clean` (cargo-sweep + a report of the freed size) and document it in `docs/build.md`; check by running on a real `target/`

## 3. prime-core

- [x] 3.1 Implement `PrimeConfig` and a provider via Leptos context with defaults; check the "not set" and "overridden" scenarios with unit tests
- [x] 3.2 Implement `ClassNames` (one allocation, no empty or repeated classes) and a `Memo` wrapper; check with unit tests and an allocation count assertion via `dhat`
- [x] 3.3 Implement the shared props `disabled`, `invalid`, `variant`, `size` (values as in PrimeNG) and the `Pt` type for the root; check with a root render test with `pt`
- [x] 3.4 Implement a z-index manager by groups with release; check with a "two overlays in a row" unit test and a release test
- [x] 3.5 Document the public `prime-core` API (rustdoc with examples) and check `cargo test --doc -p prime-core`

## 4. prime-dom

- [x] 4.1 Implement the positioning port (`absolutePosition`, `relativePosition`) with flip rules as in PrimeNG; check with `wasm-bindgen-test` on the "enough room" and "does not fit below" scenarios
- [x] 4.2 Implement a focus trap (Tab and Shift+Tab cycling); check with `wasm-bindgen-test` on the first and last elements
- [x] 4.3 Implement a portal into `body` and ripple as a directive; check with `wasm-bindgen-test` that the content ends up in `body` and is removed on unmount
- [x] 4.4 Make sure the whole public API is SSR-safe; check with `cargo test -p prime-dom --features ssr` tests on the native target without panics
- [x] 4.5 Document the `prime-dom` API in rustdoc; check `cargo test --doc -p prime-dom`

## 5. prime-icons

- [x] 5.1 Implement the shared `Icon` component (size, `spin`, class, a11y attributes) in `prime-icons`; check both a11y scenarios with tests
- [x] 5.2 Implement `xtask gen-icons` (parsing `d` and `viewBox`, an error on a template mismatch); check with a snapshot test on `check`
- [x] 5.3 Generate 53 icons with a feature flag on each and commit; check by comparing `d` and `viewBox` with all PrimeNG sources and regenerating with an empty `git diff`
- [x] 5.4 Check that a disabled icon does not end up in wasm (size of a minimal app with one icon and with all) and record the numbers in `docs/icons.md`

## 6. prime-theme

- [x] 6.1 Write a Node script that extracts the Aura tokens and component CSS from the pinned `@primeuix/themes` and `@primeuix/styles` (via `npm pack` into a cache outside the repository); check by running it and getting valid JSON as output
- [x] 6.2 Implement `xtask gen-theme`: JSON → `base.css` and `components/<name>.css`, dark mode selector from the configuration; check with a snapshot test and byte-for-byte repeatability of two runs
- [x] 6.3 Generate and commit the CSS for Aura; check by comparing `--p-*` values with the PrimeNG tokens for a sample (primary, surface, border-radius) and by building without Node
- [x] 6.4 Implement in `prime-theme` including the base CSS separately from component CSS and document it in `docs/theme.md`; check with a test that for Button only the base CSS and the Button CSS end up on the page

## 7. Showcase

- [x] 7.1 Create `showcase` on cargo-leptos (SSR + hydrate), a stub page with the theme included; check `cargo leptos build` and opening the page
- [x] 7.2 Add a light/dark scheme switch to the showcase; check in the browser (save the screenshot to a temporary directory, not the repository)

## 8. Benchmarks

- [x] 8.1 Create the `benches` crate on `divan` with a shared SSR render function and a stub benchmark for 1 and 1000 instances; check `cargo bench -p benches`
- [x] 8.2 Add allocation measurement via `dhat` with JSON output; check that the report contains the allocation count and peak size
- [x] 8.3 Implement `xtask wasm-size` and `benches/budgets.toml`; check on a minimal app and an artificial budget excess (non-zero exit code)
- [x] 8.4 Implement `xtask bench-compare` (10 % threshold, report only); check with unit tests on 5 % and 15 % regressions
- [x] 8.5 Document running the benchmarks in `docs/benchmarks.md`; check that the described commands run as written

## 9. CI

- [x] 9.1 Implement `xtask affected` (changed and dependent crates, full run when the core, dependencies, `xtask`, `.github/**` change); check with unit tests on the "one component" and "core" scenarios
- [x] 9.2 Create `.github/workflows/ci.yml` (fmt, clippy -D warnings, nextest, wasm-bindgen-test, deny, bench, ssr and wasm builds), cache with `cargo-sweep` and a limit; check by running on a test PR (without the e2e job) and failing on unformatted code
- [x] 9.3 Add to `ci.yml` a PR comment with the `bench-compare` results and the wasm budget check; check on a test PR with an artificial regression (merge is not blocked)
- [x] 9.4 Create `.github/workflows/weekly.yml` (schedule, workflow_dispatch, `run-e2e` label): clone PrimeNG at the pinned commit, build both showcases, Playwright (e2e, screenshots, memory measurements), artifacts for 7 days, the result does not block PRs; check with a manual run and by viewing the artifact
- [x] 9.5 Add the `cargo-machete` schedule and a description of the CI policy in `docs/ci.md`; check that the document matches the contents of the workflow files

## 10. Integration check

- [x] 10.1 Run from a clean clone `cargo build --workspace`, `cargo nextest run --workspace`, `cargo leptos build` and record the size of `target/` in `docs/build.md`
- [x] 10.2 Run `openspec validate bootstrap-workspace` and make sure all spec scenarios are covered by the tasks above

## Workflow follow-up

- Archive the change after review and passing the CI checks.
- Next change — the Button port (the first wave 1 component), using `xtask gen-spec button`.
