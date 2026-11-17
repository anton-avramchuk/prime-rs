# Tasks

## 1. Crate and component

- [x] 1.1 Create the crate `crates/prime-togglebutton` (`publish = false`, features `ssr`/`hydrate`, dependency `prime-rs-core`, the dev-dependencies for wasm as in `prime-toggleswitch`), add it to the workspace; check `cargo check -p prime-togglebutton --all-features`
- [x] 1.2 `state.rs`: the root classes and `data-p` per `togglebuttonstyle.ts` and `ToggleButton.dataP`, a table test
- [x] 1.3 Implement `ToggleButton`, `ToggleButtonPt`, `ToggleButtonChange`, `IconPos` (state, labels and icons, slots, attributes, click and keys, `allow_empty`); SSR tests (structure, classes, `data-p`, attributes, `tabindex`, slots, `pt`)
- [x] 1.4 Browser tests `tests/web.rs`: click, Enter and Space, `disabled`, `allow_empty`, one `on_change` per press, `on_focus` and `on_blur`
- [x] 1.5 rustdoc with an example; `cargo test -p prime-togglebutton --all-features`

## 2. Theme, showcase, e2e

- [x] 2.1 Extend `primeng_extra_block` in `xtask/src/gen_theme.rs` (the marker `/* For PrimeNG (…) */`), add `togglebutton` to `theme.components` and `primeng_extra` (`upstream.toml`), `cargo xtask gen-theme`, check that the CSS of the other components did not change, connect the CSS
- [x] 2.2 The `/togglebutton` page (`basic`, `customized`, `disabled`, `fluid`, `invalid`, `sizes`), the menu, the route, `stylesheet(&[..])`
- [x] 2.3 `e2e/tests/pages.json` and `e2e/tests/togglebutton.spec.ts`; `npx playwright test togglebutton`

## 3. Benchmarks and budgets

- [x] 3.1 Scenarios `togglebutton-1`, `togglebutton-1000`, `case-togglebutton`, budgets by measurement

## 4. Integration

- [x] 4.1 Add `prime-togglebutton` to the browser job of `ci.yml` (3 places); `docs/togglebutton.md`; clippy, tests, deny, machete, `openspec validate port-togglebutton`; CI `dev` is green

## Workflow follow-up

- Archive after CI.
