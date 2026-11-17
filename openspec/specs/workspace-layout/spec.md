# workspace-layout Specification

## Purpose
Structure of the prime-rs Cargo workspace: one crate per PrimeNG component, CSR/SSR/hydrate modes, toolchain, license and attribution.

## Requirements

### Requirement: One component, one crate
Each PrimeNG component SHALL be ported into a separate crate `crates/prime-<name>`, where `<name>` matches the component's directory name in PrimeNG. Shared code SHALL live in `prime-core`, `prime-dom`, `prime-theme` and `prime-icons`, not in component crates.

#### Scenario: Component crate naming
- **WHEN** a port of the `button` component is added
- **THEN** the crate `crates/prime-button` is created and listed in the workspace as a member

#### Scenario: A component does not duplicate shared code
- **WHEN** a component needs ClassNames, z-index or positioning
- **THEN** it depends on `prime-core` or `prime-dom` and does not contain its own copy

### Requirement: A single workspace and dependency versions
The repository SHALL be a single Cargo workspace in which external dependency versions are declared once in `[workspace.dependencies]` and crates refer to them.

#### Scenario: Workspace build
- **WHEN** `cargo check --workspace` is run
- **THEN** the build finishes without errors and `Cargo.lock` has no multiple versions of `leptos`

### Requirement: CSR and SSR/hydrate modes
Each crate that depends on Leptos SHALL build in CSR, `ssr` and `hydrate` modes via same-named feature flags. Code that accesses `window` or `document` SHALL run only on the client.

#### Scenario: SSR build
- **WHEN** a crate is built with `--features ssr` for a native target
- **THEN** the build succeeds and rendering the component to a string does not touch the DOM API

#### Scenario: Client build
- **WHEN** a crate is built for `wasm32-unknown-unknown` with `--features hydrate`
- **THEN** the build succeeds without errors

### Requirement: Pinned toolchain
The repository SHALL pin the Rust version in `rust-toolchain.toml` (stable) together with the `wasm32-unknown-unknown` target and the `rustfmt` and `clippy` components.

#### Scenario: Fresh clone
- **WHEN** a developer clones the repository and runs `cargo build`
- **THEN** the pinned toolchain is used and the required target is installed automatically

### Requirement: License and attribution
The repository SHALL be distributed under MIT, and the `NOTICE` file SHALL contain the PrimeNG copyright and the pinned version the port was made from.

#### Scenario: NOTICE check
- **WHEN** the repository root is viewed
- **THEN** `LICENSE` and `NOTICE` mentioning PrimeNG are present

### Requirement: Windows service files
`.gitignore` SHALL exclude `nul` files so they do not end up in commits.

#### Scenario: Stray nul file
- **WHEN** a `nul` file appears in the working copy
- **THEN** `git status` does not show it
