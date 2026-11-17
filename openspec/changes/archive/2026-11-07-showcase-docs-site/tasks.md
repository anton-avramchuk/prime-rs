# Tasks

## 1. Generation

- [x] 1.1 `xtask gen-docs`: parsing of the demos (`<Demo>` → `fn Component`) and of the properties tables from `crates/prime-*/src/lib.rs`, unit tests on fixtures
- [x] 1.2 Markdown `docs/*.md` → HTML (`pulldown-cmark`), the CSS classes of the component from the theme
- [x] 1.3 The output `showcase/src/generated/*`, `--check`

## 2. Shell

- [x] 2.1 `showcase/src/doc.rs`: `DocPage`, the example card with the Code button, the API tables, the table of contents, the neighbouring pages; styles in `shell`
- [x] 2.2 The menu by sections with search; the list of components in one place
- [x] 2.3 SSR test of `DocPage`

## 3. Pages

- [x] 3.1 Moving the pages to `DocPage` (the description and the import line per page); a selective check of AutoComplete, Button, Table
- [x] 3.2 The remaining pages by the menu sections

## 4. Checks

- [x] 4.1 `e2e/tests/docs.spec.ts`; the existing e2e pass
- [x] 4.2 `ci.yml`: `cargo xtask gen-docs --check`; fmt, clippy, deny, machete, `openspec validate showcase-docs-site`
- [x] 4.3 `docs/showcase.md`: how to run and update the documentation
