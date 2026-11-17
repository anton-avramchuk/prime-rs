# Tasks

## 1. Foundation

- [x] 1.1 Add `speeddial` to `theme.components` (`upstream.toml`), `cargo xtask gen-theme`; check that the CSS of other components did not change and that the `p-speeddial*` classes exist
- [x] 1.2 Check that the needed icons exist in `prime-icons` for the demos (`plus`, and analogs of pencil/refresh/trash/upload/external-link); replace missing ones with close ones

## 2. Crate `prime-speeddial`

- [x] 2.1 `state.rs` (classes, flex styles, `point_style` for the four types, `transition_delay`, `arrow_step`, `step_index`, button icon) with table-driven tests
- [x] 2.2 Component `SpeedDial`, `SpeedDialItem`, `SpeedDialCommand`, `SpeedDialPt`: root, button, list, items, mask, SSR tests, rustdoc
- [x] 2.3 Showing and hiding: button, `command`, `on_*`, controlled `visible`, `disabled`, outside click, `Escape`, mask, `--item-diff-x/y`
- [x] 2.4 Keyboard: button and list, eight directions, `Home`/`End`/`Enter`/`Space`, focus via `aria-activedescendant`, skipping disabled items
- [x] 2.5 `tooltip_options`, `button_severity`, `button_content`/`item_content`/`icon_content`
- [x] 2.6 Browser `tests/web.rs` (one sequential test)

## 3. Showcase and e2e

- [x] 3.1 Page `/speeddial` (`linear`, `circle`, `semicircle`, `quartercircle`, `mask`, `tooltip`, `template`), menu, route, `stylesheet(&[..])`, crate in the workspace and showcase
- [x] 3.2 `e2e/tests/pages.json` and `speeddial.spec.ts` (styles, expansion by direction, layouts, mask, tooltip, keyboard, templates); wait for `networkidle`, restart the showcase server; `PW_CHANNEL=chrome npx playwright test speeddial`

## 4. Benchmarks and budgets

- [x] 4.1 Scenarios `-1` and `-1000`, `case-speeddial` and budgets (first `cargo bench -p render-bench --bench alloc`; wasm measurement — in the background, with a separate budgets file)

## 5. Integration

- [x] 5.1 `docs/speeddial.md` (differences from PrimeNG, what was not verified); `prime-speeddial` in the browser job of `ci.yml` (three places)
- [x] 5.2 `cargo fmt --all -- --check`, clippy with CI exclusions and `--all-features` (and for the wasm target), tests (SSR one by one: `--exact`), deny, machete, `openspec validate port-speeddial`; CI `dev` green

## Workflow follow-up

- Archive after CI.
- Next: Carousel.
