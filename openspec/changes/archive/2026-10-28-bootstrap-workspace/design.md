# Design

## Context

The motivation and contents of the change are described in `proposal.md`, requirements are in `specs/`. What matters for the implementation:

- The repository is empty apart from `openspec/`. Development on Windows 11 (MSVC), CI on ubuntu-latest. rustc/cargo 1.98, `trunk`, `cargo-leptos` and the `wasm32-unknown-unknown` target are installed.
- The PrimeNG sources are in `C:\Projects\primeng` (v21.1.10): `packages/primeng/src/<component>` (≈117 directories, of which ≈80–90 are components), `packages/primeng/src/icons` (55 directories, 53 icons, each a single `<svg:path d=… fill="currentColor"/>`), `apps/showcase`.
- **The Aura tokens and component CSS are not in the PrimeNG repository**: `packages/themes/src/presets/aura/*` only re-exports `@primeuix/themes`, and `button/style/buttonstyle.ts` imports CSS from `@primeuix/styles/button`. `node_modules` is not installed in `C:\Projects\primeng`. The source for `gen-theme` is the npm packages `@primeuix/themes` and `@primeuix/styles` at the needed versions.
- Component classes are defined in TS (`classes = { root: ({instance}) => [...] }`), so they can be parsed with a script but not imported as data.

## Goals / Non-Goals

**Goals:**
- Wave 0: after this change the first component (Button) can be added in one following change without touching the infrastructure.
- A regular build and tests do not require Node or PrimeNG; they are needed only by the `xtask` generators and the weekly CI.
- Any generation is deterministic and repeatable.

**Non-Goals:**
- Porting components (including Button) and presets other than Aura.
- Form integration, publishing to crates.io, SSR streaming.
- Runtime theme generation (`updatePreset`, switching the theme on the fly other than the light/dark scheme).

## Decisions

### D1. Workspace layout
```
crates/prime-core  prime-dom  prime-theme  prime-icons  prime-<component>…
showcase/   xtask/   benches/   .github/workflows/
upstream.toml  rust-toolchain.toml  .cargo/config.toml  deny.toml  NOTICE
```
`resolver = "3"`, `members = ["crates/*", "benches/*", "showcase", "xtask"]` (`benches/` has two crates: `render` and `wasm-size`). The PrimeNG base classes (`basecomponent`, `baseinput`, `baseeditableholder`) become `prime-core` modules rather than crates: in Rust these are traits and helpers without their own runtime. Utility directories (`dom`, `utils`, `classnames`, `focustrap`, `ripple`) are split between `prime-core` and `prime-dom`.
*Alternative*: a crate per PrimeNG directory — gives ~117 crates, a third of which are empty wrappers, and extra minutes on every check.

### D2. A single set of features
Every crate with Leptos: `default = []`, features `ssr = ["leptos/ssr"]`, `hydrate = ["leptos/hydrate"]`, CSR by default. Code that touches `web_sys::window()` is hidden behind `#[cfg(feature = "hydrate")]`/`cfg(target_arch = "wasm32")` or runs inside an `Effect`, which does not run on the server.
*Alternative*: CSR only — simpler, but the SSR benchmarks (D8) and the cargo-leptos showcase would lose their meaning, and adding SSR later means rewriting the DOM code of all components.

### D3. CSS classes
`ClassNames` in `prime-core` is a builder that writes into a single `String` with pre-allocated capacity (`with_capacity`), without intermediate `Vec<String>`; in the reactive part the result is wrapped in a `Memo<String>` so that the `class` attribute updates only on change. Class names are copied from PrimeNG's `classes` verbatim (the `core-runtime` spec).

### D4. Pass-through
`pt` is a `Pt { class: Option<…>, attrs: … }` struct for the root and named parts (for example, `PtRoot`); applied via `.attr()` and class merging. The full set of PrimeNG `pt:*` hooks and the unstyled mode are not part of wave 0 (see Non-Goals), but the `Pt` type is designed to be extensible.

### D5. Theme generation
`xtask gen-theme`:
1. Installs `@primeuix/themes`, `@primeuix/styles`, `@primeuix/styled`, `@primeuix/utils` at pinned versions (`npm install --ignore-scripts`) into a cache **outside the repository** (`~/.cache/prime-rs`, overridable with `PRIME_RS_CACHE`) and runs `xtask/theme/extract.mjs`.
2. The script builds CSS with the same code as PrimeNG (`Theme.getCommon`/`getComponent` from `@primeuix/styled`, `dt()` resolution) and prints JSON `{base, components}`. The Rust part writes `css/base.css` and `css/components/<name>.css`; the component list and the dark mode selector come from `upstream.toml` (`[theme]`).
3. The result is **committed** to `crates/prime-theme/css/`. A regular build and CI do not require Node; regeneration is needed only when upstream is updated.
Size: the Aura tokens and component CSS are tens to hundreds of KB of text; that is acceptable in the repository, and screenshots and binaries are not committed.
*Alternatives*: (a) generation in `build.rs` — pulls Node into every build and breaks in offline CI; (b) a Rust port of `@primeuix/styled` with runtime generation — matches PrimeNG but contradicts the goal of zero runtime overhead (see the memory and performance requirements).

### D6. Icons
`xtask gen-icons` reads `packages/primeng/src/icons/*/<name>.ts`, extracts `d` and `viewBox` (by regex parsing of the `<svg:path … d="…"` template; on a mismatch — an error, not a skip) and writes `crates/prime-icons/src/<name>.rs` + features in `Cargo.toml`. The shared `Icon` component (size, `spin`, a11y) lives in `prime-icons`, so that icon data are `&'static str` constants.
*Alternatives*: a third-party set (Lucide) and a font were rejected by the user; PrimeNG has only 53 icons.

### D7. Overlay and DOM
`prime-dom` uses `web-sys` directly (a minimal feature set `Window`, `Document`, `Element`, `DomRect`, `HtmlElement`, `EventTarget`, `KeyboardEvent`), without `leptos-use`. Positioning is a port of `absolutePosition`/`relativePosition` from `DomHandler` with the same flip rules. The portal is `leptos::portal::Portal`. Animations are CSS classes and `transition`, like `p-motion`.
*Alternative*: `leptos-use` — faster to write, but pulls a large dependency graph into wasm and `target/`.

### D8. Benchmarks
- `benches/render` — a crate with `divan` (`render`), `dhat` (`alloc`) and the median time in JSON (`time`); a component is rendered to a string via `leptos::ssr` (the same function for 1 and 1000 instances).
- Allocations: `dhat` in test mode (`dhat::Profiler::builder().testing()`), the result is written to JSON.
- wasm size: `xtask wasm-size` builds a minimal app with one component, computes the contribution as the difference in raw `.wasm` size (release, without `wasm-opt`) against the empty `benches/wasm-size` app and compares with `benches/budgets.toml`.
- All reports are `target/bench/*.json` of the form `{"metrics": {name: number}}`. The baseline is the `bench-baseline` artifact of the last successful `dev` run; `xtask bench-compare` compares the directories (10 % threshold, PR comment only, `--fail-over` enables blocking).
- Browser memory measurements — Playwright in the weekly job (CDP `Performance.getMetrics`: `JSHeapUsedSize`, `Nodes`): time to create 1000 components and heap and DOM growth after 20 create-and-remove cycles.

### D9. CI
`.github/workflows/` files:
- `ci.yml` (PR and push): `fmt` → `clippy -D warnings` → `nextest` → `wasm-bindgen-test` (headless Chrome) → `cargo-deny` → `bench`. Affected crates are determined by `xtask affected` (`cargo metadata` + `git diff`); changes in `prime-core`, `prime-dom`, `Cargo.toml`, `Cargo.lock`, `xtask`, `.github/**` and others trigger a full run. Applications (`showcase`, `wasm-size`, `render-bench`) are checked by a separate `apps` job.
- `weekly.yml` (`schedule`, `workflow_dispatch`, PR with the `run-e2e` label): clone PrimeNG at the commit from `upstream.toml`, build its showcase, build our showcase, Playwright (e2e + screenshot comparison + browser benchmarks), artifacts `retention-days: 7`.
- Cache — `Swatinem/rust-cache` with `cache-on-failure: false`, `cargo-sweep` before saving; `cargo-machete` on a schedule.
The cache strategy is chosen so that the cache size is bounded; `sccache` is not used.

### D10. Build profiles
```toml
[profile.dev]            debug = "line-tables-only"  incremental = true
[profile.dev.package."*"] debug = false opt-level = 1
[profile.release]        lto = "thin"  codegen-units = 1  panic = "abort"  strip = true  incremental = false
```
In `.cargo/config.toml`: `rust-lld` for `x86_64-pc-windows-msvc` and `x86_64-unknown-linux-gnu`; `CARGO_INCREMENTAL=0` in CI. This reduces the size of `target/` without losing line-level debugging.

### D11. upstream.toml and spec stubs
`upstream.toml` stores `version`, `commit`, `path` (local clone; overridden by `PRIMENG_PATH`) and the `@primeuix/*` versions. `xtask upstream-diff` runs `git diff --name-only <pinned>..<target> -- packages/primeng/src/<component>` for the ported components (the list is the `crates/prime-<name>` crates excluding infrastructure ones). `xtask gen-spec <name>` parses `<name>.ts` and `style/<name>style.ts` and fills in the spec template.

## Risks / Trade-offs

- [Parsing TS with regular expressions is brittle] → generators fail with an error on an unexpected source shape, key files have snapshot tests (`insta`), the upstream version is pinned.
- [`@primeuix/*` are external npm packages, not part of the PrimeNG repository] → versions are pinned in `upstream.toml`, the result is committed, Node is needed only for regeneration.
- [Repository growth from committed CSS and generated icons] → text only, regeneration is deterministic, git deltas are small; binaries are not committed.
- [SSR tests do not catch DOM bugs] → `wasm-bindgen-test` on PRs and Playwright in the weekly job.
- [The weekly job depends on building the Angular showcase and may become flaky] → its result does not block PRs, artifacts are kept for 7 days.
- [The wasm budgets and the 10 % threshold were chosen without measurements] → the first version of the budgets is recorded after the first component appears and is revisited.

## Migration Plan

No migration: the repository is created from scratch. Rollback — `git revert` of the corresponding commit. The `dev` branch is protected by the `ci.yml` check after the first green run.

## Open Questions

- Whether to hook `cargo-leptos` into PR CI (showcase build) or whether a build in the weekly job is enough? Decided after measuring build time; does not affect the specs.
- The exact wasm size budget values and the `cargo-sweep` expiry period — determined from the first measurements.
