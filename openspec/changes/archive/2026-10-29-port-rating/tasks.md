# Tasks

## 1. Core, crate and component

- [x] 1.1 `prime-core`: extract `next_unique` from `collapsible` into `ids` (`pub fn next_unique`), use it in `collapsible`; `cargo test -p prime-core --all-features` and the Panel, Fieldset, Accordion tests without edits
- [x] 1.2 Create the crate `crates/prime-rating` (`publish = false`, features `ssr`/`hydrate`, dependencies `prime-core` and `prime-icons` with `star`, `starfill`, wasm dev-dependencies as in `prime-selectbutton`), add it to the workspace; verify `cargo check -p prime-rating --all-features`
- [x] 1.3 `state.rs`: `next_value`, classes of the root, option and icons, `data-p` per `ratingstyle.ts` and `Rating.dataP`; table tests
- [x] 1.4 Implement `Rating`, `RatingPt`, `RatingRate`, `RatingIcon` (value, stars, radio buttons and labels, focus and `p-focus-visible`, icons and slots, attributes, events); SSR tests (structure, star count, classes, `data-p`, `checked`, labels, attributes, icons and slots, `invalid`, `pt`)
- [x] 1.5 Browser tests `tests/web.rs`: select and reset by click, `readonly`, `disabled`, one `on_rate` per action, radio button `change` without a reset, external signal, reactive `stars`, `p-focus-visible` and focus, keeping the stars and focus after a selection
- [x] 1.6 rustdoc with an example; `cargo test -p prime-rating --all-features`

## 2. Theme, showcase, e2e

- [x] 2.1 Add `rating` to `theme.components` (`upstream.toml`), `cargo xtask gen-theme`, make sure the CSS of other components did not change, link the CSS
- [x] 2.2 The `/rating` page (`basic`, `disabled`, `numberofstars`, `readonly`, `template`), menu, route, `stylesheet(&[..])`
- [x] 2.3 `e2e/tests/pages.json` and `e2e/tests/rating.spec.ts` (including arrows); restart the showcase server before the run; `npx playwright test rating`

## 3. Benchmarks and budgets

- [x] 3.1 Scenarios `rating-1`, `rating-1000`, `case-rating`, budgets by measurement (first `cargo bench -p render-bench --bench alloc`)

## 4. Integration

- [x] 4.1 Add `prime-rating` to the browser job of `ci.yml` (3 places); `docs/rating.md`; clippy, tests, deny, machete, `openspec validate port-rating`; CI `dev` green

## Workflow follow-up

- Archive after CI.
