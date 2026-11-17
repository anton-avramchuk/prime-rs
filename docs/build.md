# Build

The toolchain is pinned in `rust-toolchain.toml` (Rust 1.98.0, target `wasm32-unknown-unknown`, rustfmt and clippy): rustup installs everything itself on the first build.

```sh
cargo check --workspace
cargo check -p prime-rs-core --target wasm32-unknown-unknown --features hydrate   # client
cargo check -p prime-rs-core --features ssr                                       # server
cargo deny check
cargo xtask --help
```

Leptos modes are enabled by the `ssr` and `hydrate` features in each crate; the default is CSR.

## Linker

`.cargo/config.toml` uses `rust-lld` for `x86_64-pc-windows-msvc` and `lld` for Linux (the `lld` package must be installed).

## Size of `target/`

Profiles (see the root `Cargo.toml`): in dev `debug = "line-tables-only"`, for dependencies `debug = false`; in release `lto = "thin"`, `strip`, no incremental build. In CI set `CARGO_INCREMENTAL=0`.

So that several clones and projects do not each create their own `target/`, move the build directory to a shared location:

PowerShell (permanently for the user):
```powershell
[Environment]::SetEnvironmentVariable("CARGO_TARGET_DIR", "D:\cargo-target", "User")
```

Linux and macOS (`~/.profile`):
```sh
export CARGO_TARGET_DIR="$HOME/.cache/cargo-target"
```

Size of `target/` from a clean clone (Windows, dev profile, measured at the end of `bootstrap-workspace`):

| Step | `target/` |
|---|---:|
| `cargo build --workspace` (2 min) | 664 MB |
| + `cargo test --workspace --all-features` | 1 002 MB |
| + `cargo leptos build` (showcase) | 2 489 MB |

Different feature sets of the same crates (`ssr`, `hydrate`, CSR, wasm: `tachys` has up to five copies) and the release builds
of the benchmarks grow `target/` noticeably. A working copy in which all commands were run one after another reached 4.5 GB: run `cargo xtask clean`
periodically and use a shared `CARGO_TARGET_DIR`. Cargo has no size limit; to keep `target/` within bounds (for example 5 GB) you can use
`cargo xtask clean --max-size 5` (cargo-sweep does not remove artifacts needed by the current build, so it gives no guarantee when close to the limit).
Debug information is already trimmed (`debug = "line-tables-only"` for our crates, `debug = false` for dependencies, see `[profile.dev]`).

## xtask commands

```sh
cargo xtask upstream-diff [--to <rev>]   # which ported components changed in PrimeNG
cargo xtask gen-spec <name> [--out file] # a spec skeleton from the PrimeNG sources
cargo xtask clean [--days N]             # cargo-sweep: remove target/ artifacts older than N days (default 7)
cargo xtask clean --max-size 5           # remove the oldest artifacts while target/ is larger than 5 GB (can be combined with --days)
```

`upstream-diff` and `gen-spec` read `upstream.toml`; the path to the PrimeNG clone can be overridden with `PRIMENG_PATH`.
`clean` requires `cargo install cargo-sweep`. The list of "ported" components is `crates/prime-<name>` without `core`, `dom`, `theme`, `icons`.

## Browser tests (wasm-bindgen-test)

```sh
cargo install wasm-bindgen-cli --version 0.2.129 --locked   # the version must match wasm-bindgen in Cargo.lock
# chromedriver matching the installed Chrome (https://googlechromelabs.github.io/chrome-for-testing/)
export CHROMEDRIVER=/path/to/chromedriver
cargo test -p prime-dom --target wasm32-unknown-unknown --test web
```

The runner `wasm-bindgen-test-runner` is set in `.cargo/config.toml`. Without `CHROMEDRIVER` the runner looks for `chromedriver` in `PATH`.
The tests are marked `#![cfg(target_arch = "wasm32")]`, so a plain `cargo test` on the host skips them.

## Showcase

```sh
cd showcase
cargo leptos serve    # http://127.0.0.1:3000, SSR + hydration; cargo leptos build — build only
```

The page includes the theme with an inline `<style>` (`prime_theme::stylesheet`), shows all icons and the light and dark scheme switch (the class `p-dark` on `<html>`).
`cargo-leptos` builds the client part into a separate directory `target/front`, so `target/` is about twice the size of a regular build: clean it with `cargo xtask clean`.
