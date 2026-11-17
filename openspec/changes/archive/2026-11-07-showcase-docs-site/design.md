# Design

## Context

The look reference is primeng.dev/components/<name>. In PrimeNG the page is assembled by hand from `apps/showcase/doc/<name>/*-doc.ts`; we have 80+ pages with live demos, which need the code and the reference without manual work.

## Decisions

### D1. The `DocPage` shell and the page data
`showcase/src/doc.rs`: `DocPage { name, title, description, import }` + `children` (the list of `<Demo>`). The page data (the code of the demos, the properties, the CSS classes, the notes) is taken from `generated::PAGES: &[PageDoc]` by name, so the pages barely change: the `<h2>` heading is replaced by `<DocPage …>`.

### D2. Demo code from the sources
`gen-docs` reads `showcase/src/pages/<name>.rs`, finds `<Demo name="x" …><Component /></Demo>` and takes the text of `#[component] fn Component` in full (it is not linked to constants: auxiliary `const`/`fn` are not part of the code, their names are shown in a comment "uses: …"). The text is stored as `&'static str` in `generated`.

### D3. Reference from the crate
For `crates/prime-<name>/src/lib.rs` all `#[component] pub fn X(…)` are taken: each parameter has a `///` description, `#[prop(…)]` (optional/into/default) and the type. The events are the parameters with the prefix `on_`; the Pass Through parts are the fields of the `*Pt` struct; the CSS classes are from `crates/prime-theme` (`p-<name>*`), present in the styles of the component. Non-standard cases (crates with several components) are collected by the name prefix.

### D4. Markdown
`docs/<name>.md` is turned into HTML at generation time (`pulldown-cmark`) and shown in the "Notes" section.

### D5. Styling
New shell styles — a small block of CSS in `shell` (the example card, the header with the Code button, the API tables, the table of contents on the right); the theme — `--p-*` tokens. The code expansion is client-side (`RwSignal`), without JS libraries; the Rust highlighting is simple keyword markup at generation time.

### D6. Menu
The list of components — one array `(section, name, path)` in `doc.rs`; the menu markup and the search take it from there.

### D7. Tests
Unit: parsing of properties and demos on fixtures. SSR: `DocPage` renders all sections. e2e: `docs.spec.ts` (Code expands the source, the table of contents, the search in the menu, the transition to the next component).

## Risks / Trade-offs

- [Parsing by text, not `syn`] The format of the signatures is uniform; for non-standard cases the API section is empty, and `--check` in CI will show the discrepancies.
- [The size of showcase] The code and HTML strings end up in the server binary, but not in the wasm of the page (the generated data is read only by SSR; the client receives only the needed code when it is expanded).

## Migration Plan

The pages are moved to `DocPage` in packages by section; the old pages keep working, the e2e does not change. Rollback — `git revert`.
