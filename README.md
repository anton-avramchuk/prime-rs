# prime-rs

A port of [PrimeNG](https://github.com/primefaces/primeng) components to Rust and [Leptos](https://leptos.dev) 0.8.
One component, one crate (`crates/prime-<name>`). The port is synced with PrimeNG 21.1.10 (see `upstream.toml`, `NOTICE`).

## Structure

```
crates/prime-core    config, ClassNames, z-index, pass-through
crates/prime-dom     positioning, focus trap, portal, ripple
crates/prime-theme   static theme CSS (Aura)
crates/prime-icons   PrimeNG SVG icons
crates/prime-<name>  components
xtask/               generators and utility commands (cargo xtask)
openspec/            specs and changes
```

Build, disk and commands: [docs/build.md](docs/build.md).

## Process

Work is done through [OpenSpec](https://github.com/Fission-AI/OpenSpec): one change per component, a spec skeleton is created with `cargo xtask gen-spec <name>`.

## License

MIT, see `LICENSE` and `NOTICE`.
