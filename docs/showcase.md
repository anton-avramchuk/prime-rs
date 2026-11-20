# Showcase and the documentation site

`showcase/` is the application (SSR + hydration), in which each component has its own page in the spirit of primeng.dev/components:
description, an "Import" block, examples with a live result and Rust code, an API reference, styling and notes about the port.

## Running

```sh
cd showcase
cargo leptos serve        # http://127.0.0.1:3000
```

On the left is a menu by sections (Form, Button, Data, Panel, Overlay, File, Menu, Messages, Media, Misc) with search by name; on the right is the table of contents of the page ("On this page"). The light and dark scheme switch is in the header.

## Static build (GitHub Pages)

The published site is pre-rendered HTML hydrated by a split wasm; `.github/workflows/pages.yml` runs the same command:

```sh
cargo xtask pages --base /prime-rs --dist target/pages   # base = the path Pages serves the site under
```

It runs two steps:

1. `cargo leptos build --release --split` with `opt-level = "z"`. Every component page is a lazy route (`lazy_page!` in
   `showcase/src/app.rs`, built on `#[lazy_route]`), so wasm-split moves each page into its own chunk
   (`pkg/split___<page>_route_view_*.wasm`) plus small shared chunks (`pkg/chunk_*.wasm`). The browser downloads the main
   module and the chunks of the open page only; other pages load when the user navigates to them.
2. `showcase --export <dist>` (the SSR binary) renders every route through the router once and writes `index.html`,
   `<route>.html` (GitHub Pages serves `/button` from `button.html`) and `404.html`. The page content is visible
   before the wasm arrives; hydration then attaches the behavior.

The styles (theme for the components in `showcase/theme-components.txt`, `showcase/src/doc.css`, `showcase/src/site.css`)
are inlined into `<head>` ahead of the hydration scripts. A linked stylesheet competes with the wasm preloads for
bandwidth and delays the first paint; inlined, the page paints as soon as its HTML arrives.

The demo code blocks are highlighted in the browser (`showcase/src/highlight.rs`) instead of shipping a pre-highlighted copy of every snippet.

## What is on a component page

| Section | Where it comes from |
|---|---|
| Description | the first `//!` paragraph in `crates/prime-<name>/src/lib.rs` |
| Import | the `use prime_…` statements of the page `showcase/src/pages/<name>.rs` |
| Examples and the **Code** button | `<Demo name=".." title="..">` of the page; the code is the very `#[component] fn …` (or the content of `<Demo>`) that runs on the page |
| Props, Events | the parameters of `#[component]` in the crate: name, type, default value (`default = …`), `///` description; `*` — required |
| Pass Through | the fields of the crate's `*Pt` structs |
| Theming | the `p-…` CSS classes from `crates/prime-theme/css/components/<name>.css` |
| Accessibility | the items of `docs/<name>.md` about `aria-*`, `role=` and keys |
| Notes | `docs/<name>.md` in full (matches PrimeNG, differences, not included, not verified) |

## How to update

All data of the section is in `showcase/src/generated.rs`, which `cargo xtask gen-docs` creates and which must not be edited by hand.
After changing a demo, component properties, `docs/*.md` or the theme CSS:

```sh
cargo xtask gen-docs          # regenerate showcase/src/generated.rs
cargo xtask gen-docs --check  # check that it is up to date (CI runs it)
```

A new component page: add `showcase/src/pages/<name>.rs` with a `<Demo>`, a lazy route in `showcase/src/app.rs`
(`<Route path=StaticSegment("<name>") view={Lazy::<NameRoute>::new()} />` plus `lazy_page!(NameRoute, "<name>", NamePage);`) and a line in `MENU` (`showcase/src/doc.rs`), then run `cargo xtask gen-docs`.

## Limitations

- Source parsing is textual (without `syn`): code formatted by `rustfmt` is expected. If a component has a non-standard
  signature, its API section will stay empty — `gen-docs --check` and the `xtask` tests do not hide this, but they do not fix it either.
- Properties are taken only from the crate's `src/lib.rs`; components in submodules do not get into the reference.
- For several components described by one document (`field-wrappers.md`, `display-batch.md`), the notes are shared.
