# Proposal

## Why

Right now the showcase is a set of pages with live demos without explanations, and the documentation lies separately in `docs/*.md`. We need a result that can be shown and learned from: a "demo + documentation" site in the spirit of primeng.dev/components (a component page: title and description, import, examples with a live result and code, the API reference, styling, accessibility).

## What Changes

- The component page gets a single `DocPage` shell: title, a short description, an "Import" block, examples (a live demo + a "Code" button with the Rust code), API sections (Props, Events, Pass Through, Theming/CSS classes), "Accessibility", a side table of contents of the page ("On this page") and navigation to the neighbouring components.
- The left menu is grouped by sections, as in PrimeNG (Form, Button, Data, Panel, Overlay, File, Menu, Messages, Media, Misc), with search by name.
- The code of the examples is not written by hand: `cargo xtask gen-docs` extracts the source of each demo from `showcase/src/pages/*.rs` (the `#[component] fn …` referenced by `<Demo>`), so that the code always matches the live result.
- The reference of properties, events and Pass Through parts is built from the crates' sources (`#[component]` with `///` documentation and the property type), without a manual table.
- The texts of `docs/<name>.md` (matches, differences, not included, not verified) are shown on the "Notes" section of the page (Markdown rendered at generation time).
- The result — the generated files `showcase/src/generated/*` (in the repository) and a freshness check `cargo xtask gen-docs --check`.
- No publishing: run locally (`cargo leptos serve` in `showcase/`).
- **Not included:** deployment (GitHub Pages), search over the content, documentation versions, guide pages (Installation, Theming) — after agreement.

## Capabilities

### New Capabilities
- `docs-site`: the shell of the component page, the generation of the example code, the API reference and the notes from the sources.

### Modified Capabilities
<!-- None. -->

## Impact

- `xtask` (a new command `gen-docs`, the dependency `pulldown-cmark`), `showcase/src` (`doc.rs`, `generated/`, `app.rs`, `pages/mod.rs`: `Demo` gets a link to the code), `showcase/Cargo.toml`, e2e (`docs.spec.ts`), `ci.yml` (the `gen-docs --check` check), `docs/`.
- The component pages remain; only the shell around the demos changes; the `[data-demo]` markup and the identifiers are kept so as not to break the e2e.
