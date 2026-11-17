# Tasks

## 1. Stubs and the shared helper

- [x] 1.1 Generate the stubs `cargo xtask gen-spec badge` and `gen-spec button` into a temporary directory and check that all their properties, events and classes are covered by the specs of this change; write the result of the check into `design.md` in the "Context" section if gaps are found
- [x] 1.2 Add the `DataP` helper (flags separated by spaces, like `cn({...})`) to `prime-rs-core` and the `style` field to `Pt`; check the order, empty values and `Pt::merge_style` with unit tests
- [x] 1.3 Document `DataP` and `Pt::style` in rustdoc with examples and check `cargo test --doc -p prime-rs-core`

## 2. Badge

- [x] 2.1 Create the crate `crates/prime-badge` (`publish = false`, features `ssr`/`hydrate`) and add it to the workspace; check `cargo check -p prime-badge --all-features`
- [x] 2.2 Implement the pure function "properties → classes and `data-p`" per `badgestyle.ts` and a table of tests following the scenarios of the `badge` spec (circle, dot, sizes, severities); check `cargo test -p prime-badge`
- [x] 2.3 Implement the `Badge` component (`value`, `severity`, `size`, `badge_disabled`, `style_class`, `pt`); check the structure, `display: none` and `pt` with SSR tests
- [x] 2.4 Add `badge` to the `crates/prime-theme` features of the showcase application and check that `stylesheet(&["badge"])` contains `.p-badge-dot`; document the API (rustdoc, an example) and check `cargo test --doc -p prime-badge`

## 3. Button: logic and markup

- [x] 3.1 Create the crate `crates/prime-button` (`publish = false`, features `ssr`/`hydrate`, dependencies per D7); check `cargo check -p prime-button --all-features`
- [x] 3.2 Implement `ButtonState` and the pure functions of the classes and `data-p*` per `buttonstyle.ts` and `button.ts`; check with a table of unit tests over all the modifiers of the spec (`icon-only`, `vertical`, `loading`, `text`/`outlined` from `variant`, sizes, `fluid`, severities)
- [x] 3.3 Implement the `Button` component: `<button>`, `children`, icon, loading indicator, label, `Badge`, `disabled` when `loading`, `aria-hidden` of the label; check the order of the nodes and all the scenarios of the section "Icon, label, badge and content" with SSR tests
- [x] 3.4 Implement `ButtonPt` and the application of `pt` to all parts, `style_class`, `style`, `aria_label`, `tabindex`, `type`; check the `pt` scenarios, the user class and style, and the a11y attributes with SSR tests
- [x] 3.5 Document the `Button` API (rustdoc with examples) and check `cargo test --doc -p prime-button`

## 4. Button: behaviour in the browser

- [x] 4.1 Implement `on_click`, `on_focus`, `on_blur` and unavailability with `disabled`/`loading`; check with `wasm-bindgen-test` (a click calls once, an unavailable button does not call)
- [x] 4.2 Implement `autofocus` and the connection of `Ripple` by `PrimeConfig.ripple` with release in `on_cleanup`; check with `wasm-bindgen-test` the scenarios of autofocus, ripple enabled and disabled, and the absence of `p-ink` after unmounting
- [x] 4.3 Check SSR safety: `cargo test -p prime-button --features ssr` does not access `window`, and the `Effect`s do not run; record the result with a test

## 5. Benchmarks and budgets

- [x] 5.1 Add the scenarios `badge-1`, `badge-1000`, `button-1`, `button-1000` in `benches/render`; check `cargo bench -p render-bench --bench alloc` and `--bench time`
- [x] 5.2 Add the cases `case-badge` and `case-button` in `benches/wasm-size` and the budgets in `benches/budgets.toml` (the measured contribution with a margin); check `cargo xtask wasm-size`
- [x] 5.3 Check that `class` is not overwritten when a property that does not affect the classes changes (an update counter in `wasm-bindgen-test`); record the result in `docs/benchmarks.md` together with the first values of Button and Badge

## 6. Showcase and comparison with PrimeNG

- [x] 6.1 Add `leptos_router` to `showcase` (the routes `/`, `/button`, `/badge`), enable the `button` and `badge` theme features; check `cargo leptos build` and opening the pages
- [x] 6.2 Move the Button demos from `apps/showcase/doc/button` (all the properties of the spec) and the Badge demos into the showcase pages; check in Chrome that every demo is displayed without console errors
- [x] 6.3 Add the pages to `e2e/tests/pages.json` and the e2e checks (`button.spec.ts`: click, loading, disabled, ripple); check `npx playwright test` locally with the showcase running
- [x] 6.4 Run `weekly.yml` manually and check that the `visual.json` report contains `visual.button.mismatch_pct`; record the differences and their causes in `docs/button.md`

## 7. Integration check

- [x] 7.1 Add links to the sources in `openspec/`: in `docs/button.md` name the PrimeNG files and the commit from which Button and Badge are ported; check `cargo xtask upstream-diff` (no changes)
- [x] 7.2 Run `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace --all-features` (except the applications), `cargo fmt --all --check`, `cargo deny check` and `openspec validate port-button`; make sure CI on the PR is green

## Workflow follow-up

- Archive the change after merging and passing CI.
- The next wave-1 components: Tag, Chip, Avatar, Divider, Card, InputText, Checkbox, RadioButton, ToggleSwitch, Message, ProgressBar.
