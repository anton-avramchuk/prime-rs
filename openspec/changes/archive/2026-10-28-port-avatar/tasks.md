# Tasks

## 1. Crate and logic

- [x] 1.1 Create the crate `crates/prime-avatar` (`publish = false`, features `ssr`/`hydrate`, dependencies `prime-core`, `web-sys`) and add it to the workspace; check `cargo check -p prime-avatar --all-features`
- [x] 1.2 Implement `AvatarSize`, `AvatarShape` (with `Default`), `AvatarState`, `write_classes`, `data_p` per `avatarstyle.ts` and `Avatar.dataP`; check with a table of unit tests following the spec scenarios (sizes, shapes, `p-avatar-image`, default `data-p`)

## 2. Component

- [x] 2.1 Implement `Avatar` with two render branches, the label/icon/image blocks with priority, `aria-*` on the root, `pt` (`AvatarPt`) and `on_image_error`; check all the spec scenarios with SSR tests
- [x] 2.2 Check `on_image_error` and the behaviour in the browser (`wasm-bindgen-test`: a loading error calls the callback once)
- [x] 2.3 Add a branch parity table and rustdoc with an example; check `cargo test -p prime-avatar --all-features` and the doc tests

## 3. Showcase and e2e

- [x] 3.1 Add the `/avatar` page (`label`, `icon`, `image`, `shape`, `size`, `template`), a menu item and the CSS `avatar` in the wrapper; check `cargo leptos build` and the page in Chrome without console errors
- [x] 3.2 Add entries to `e2e/tests/pages.json` and `e2e/tests/avatar.spec.ts` (classes, `data-p`, block priority); check `npx playwright test avatar` locally

## 4. Benchmarks and budgets

- [x] 4.1 Add the scenarios `avatar-1`, `avatar-1000` in `benches/render`, `case-avatar` in `benches/wasm-size`, budgets by measurement; check `cargo bench -p render-bench --bench alloc`, `cargo xtask alloc-budget` and `cargo xtask wasm-size`

## 5. Integration

- [x] 5.1 Write `docs/avatar.md` (conformance with PrimeNG, differences); run `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace --all-features` (except the applications), `cargo deny check`, `openspec validate port-avatar`; make sure CI on `dev` is green

## Workflow follow-up

- Archive the change after CI passes; the next wave-1 components: Divider, Card.
