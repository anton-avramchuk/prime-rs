# Tasks

## 1. Crate and component

- [x] 1.1 Create the crate `crates/prime-avatargroup` (`publish = false`, features `ssr`/`hydrate`, dependency `prime-rs-core`), add it to the workspace; check `cargo check -p prime-avatargroup --all-features`
- [x] 1.2 Implement `AvatarGroup` (root, classes, `style`, `style_class`, `pt`, two branches) and rustdoc with an example; check with SSR tests, parity and `cargo test -p prime-avatargroup --all-features`

## 2. Showcase and e2e

- [x] 2.1 Add the `avatargroup` demo to the `/avatar` page; the theme CSS is already connected; check `cargo leptos build` and the page in Chrome
- [x] 2.2 Add an entry to `e2e/tests/pages.json` and checks in `e2e/tests/avatar.spec.ts` (overlap of the avatars); check `npx playwright test avatar`

## 3. Benchmarks and budgets

- [x] 3.1 Scenarios `avatargroup-1`, `avatargroup-1000`, `case-avatargroup`, budgets by measurement

## 4. Integration

- [x] 4.1 `docs/avatargroup.md`, an edit of `docs/avatar.md`; clippy, tests, deny, machete, `openspec validate port-avatargroup`; CI on `dev` is green

## Workflow follow-up

- Archive after CI.
