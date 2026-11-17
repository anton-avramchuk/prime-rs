# Tasks

## 1. Crate and component

- [x] 1.1 Create the crate `crates/prime-selectbutton` (`publish = false`, features `ssr`/`hydrate`, dependencies `prime-rs-core` and `prime-togglebutton`, dev dependencies for wasm as in `prime-togglebutton`), add it to the workspace; check `cargo check -p prime-selectbutton --all-features`
- [x] 1.2 `state.rs`: `SelectButtonOption`, `SelectButtonValue`, pure selection logic (single and multiple modes, `allow_empty`, `getAllowEmpty`), root classes and `data-p` per `selectbuttonstyle.ts` and `SelectButton.dataP`; table-driven tests
- [x] 1.3 Implement `SelectButton`, `SelectButtonPt`, `SelectButtonChange`, `SelectButtonOptionClick` (options, value, button signal synchronization, the `item_template` slot, attributes, events); SSR tests (structure, labels, selected option, `disabled`, classes, `data-p`, `size`/`fluid`/`style_class`, slot, `pt`)
- [x] 1.4 Browser tests `tests/web.rs`: select and deselect, `allow_empty`, multiple selection, `disabled`, Enter and Space, one `on_change` per press, `aria-pressed` synchronization after a rejected click, external signal and `options` change
- [x] 1.5 rustdoc with an example; `cargo test -p prime-selectbutton --all-features`

## 2. Theme, showcase, e2e

- [x] 2.1 Add `selectbutton` to `theme.components` (`upstream.toml`), `cargo xtask gen-theme`, make sure the CSS of other components did not change, include the CSS (`selectbutton` together with `togglebutton`)
- [x] 2.2 Page `/selectbutton` (`basic`, `disabled`, `fluid`, `invalid`, `multiple`, `sizes`, `template`), menu, route, `stylesheet(&[..])`
- [x] 2.3 `e2e/tests/pages.json` and `e2e/tests/selectbutton.spec.ts`; `npx playwright test selectbutton`

## 3. Benchmarks and budgets

- [x] 3.1 Scenarios `selectbutton-1`, `selectbutton-1000`, `case-selectbutton`, budgets by measurement (first `cargo bench -p render-bench --bench alloc`)

## 4. Integration

- [x] 4.1 Add `prime-selectbutton` to the browser job of `ci.yml` (3 places); `docs/selectbutton.md`; clippy, tests, deny, machete, `openspec validate port-selectbutton`; CI `dev` green

## Workflow follow-up

- Archive after CI.
