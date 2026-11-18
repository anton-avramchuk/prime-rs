# prime-rs

A port of [PrimeNG](https://github.com/primefaces/primeng) components to Rust and [Leptos](https://leptos.dev) 0.8.
One component, one crate (`crates/prime-<name>`). The port is synced with PrimeNG 21.1.10 (see `upstream.toml`, `NOTICE`).

**Live showcase: https://anton-avramchuk.github.io/prime-rs/**

## Showcase

The showcase (`showcase/`) has a page for each component: description, import, live examples with Rust code, API reference and notes about the port.
It is published as a client-only wasm build to GitHub Pages by `.github/workflows/pages.yml`, which runs on the `pages` tag and on every `crates_*` release tag.

To run it locally:

```sh
cd showcase
cargo leptos serve        # http://127.0.0.1:3000
```

## Structure

```
crates/prime-rs-core    config, ClassNames, z-index, pass-through
crates/prime-dom        positioning, focus trap, portal, ripple
crates/prime-theme      static theme CSS (Aura)
crates/prime-icons      PrimeNG SVG icons
crates/prime-<name>     components
showcase/               documentation site (Leptos SSR + hydration, GitHub Pages build)
benches/                benchmarks
e2e/                    end-to-end and visual comparison tests
xtask/                  generators and utility commands (cargo xtask)
openspec/               specs and changes
docs/                   build, CI, showcase and per-component notes
```

## Documentation

- [docs/build.md](docs/build.md) — toolchain, build, disk usage and `cargo xtask` commands
- [docs/showcase.md](docs/showcase.md) — the showcase and the documentation site
- [docs/ci.md](docs/ci.md) — CI workflows
- [docs/benchmarks.md](docs/benchmarks.md) — benchmarks

## Process

Work is done through [OpenSpec](https://github.com/Fission-AI/OpenSpec): one change per component, a spec skeleton is created with `cargo xtask gen-spec <name>`.

## License

MIT, see `LICENSE` and `NOTICE`.
