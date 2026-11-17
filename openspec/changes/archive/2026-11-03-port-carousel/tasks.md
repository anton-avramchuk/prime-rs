# Tasks

## 1. Foundation

- [x] 1.1 Add `carousel` to `theme.components` (`upstream.toml`), `cargo xtask gen-theme`; check that the CSS of other components did not change and that the `p-carousel*` classes exist
- [x] 1.2 Check the `aria.slide`, `slideNumber`, `pageLabel`, `prevPageLabel`, `nextPageLabel` translations

## 2. Crate `prime-carousel`

- [x] 2.1 `model.rs`: `Track` and pure functions (`total_dots`, `is_circular`, `recalc`, `step`, `settle`, `after_transition_shift`, `first_index`/`last_index`, `nav_disabled`, `responsive`, `swipe`, `autoplay_next`, `translate`, substitution into `aria` strings) with table tests
- [x] 2.2 `Carousel<T>`, `CarouselResponsive`, `CarouselOrientation`, `CarouselPt` component: root, navigators, viewport, items, clones, indicators, header/footer, SSR tests, rustdoc
- [x] 2.3 Paging: buttons, dots, `page` (two-way), `on_page`, `transition`/`transitionend`, `p-items-hidden`, the jump from a clone in infinite mode
- [x] 2.4 Responsiveness on `resize`, item width by inline style; autoplay and its stop; swipe; indicator keyboard
- [x] 2.5 Browser `tests/web.rs` (one sequential test)

## 3. Showcase and e2e

- [x] 3.1 Page `/carousel` (`basic`, `responsive`, `circular`, `vertical`), local product data, menu, route, `stylesheet(&[..])`, the crate in the workspace and showcase
- [x] 3.2 `e2e/tests/pages.json` and `carousel.spec.ts` (styles, paging, infinite mode, autoplay, responsiveness, vertical mode, swipe); wait for `networkidle`, restart the showcase server; `PW_CHANNEL=chrome npx playwright test carousel`

## 4. Benchmarks and budgets

- [x] 4.1 Scenarios `-1` and `-1000`, `case-carousel` and budgets (first `cargo bench -p render-bench --bench alloc`; wasm measurement — in the background, with a separate budgets file)

## 5. Integration

- [x] 5.1 `docs/carousel.md` (differences from PrimeNG, what was not verified); `prime-carousel` in the browser job of `ci.yml` (three places)
- [x] 5.2 `cargo fmt --all -- --check`, clippy with CI exclusions and `--all-features` (also for the wasm target), tests (SSR one by one: `--exact`; effects only under `cfg(wasm32)`), deny, machete, `openspec validate port-carousel`; CI `dev` green

## Workflow follow-up

- Archive after CI.
- Next: TieredMenu.
