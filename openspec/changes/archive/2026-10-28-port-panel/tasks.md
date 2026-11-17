# Tasks

## 1. Shared collapsing logic

- [x] 1.1 The module `prime-core::collapsible`: `CollapseState` (an external `RwSignal<bool>` or an internal one), `CollapseIds`, the ARIA attribute functions; check with unit tests (a state table, identifiers)
- [x] 1.2 The client `animate` (the classes `p-collapsible-*-active`, `animationend` + a fallback timer, completion) and setting `tabindex` on the descendants; check with `wasm-bindgen-test`

## 2. Crate and component

- [x] 2.1 Create the crate `crates/prime-panel` (`publish = false`, features `ssr`/`hydrate`, dependencies `prime-core`, `prime-icons` with `minus`, `plus`), add it to the workspace; check `cargo check -p prime-panel --all-features`
- [x] 2.2 Implement `PanelPt`, the classes per `panelstyle.ts`, the markup (`macro_rules!`), two branches, slots; check with SSR tests (structure, classes, ARIA, `show_header`, `icon_pos`, `pt`) and branch parity
- [x] 2.3 Toggling (`toggler = icon|header`), `on_before_toggle`, `on_after_toggle`, the icon change; check with `wasm-bindgen-test` (click, keyboard, no double toggle, `aria-expanded`, `tabindex`)
- [x] 2.4 rustdoc with an example; `cargo test -p prime-panel --all-features`

## 3. Theme, showcase, e2e

- [x] 3.1 Add `panel` to `theme.components` (`upstream.toml`), `cargo xtask gen-theme`, connect the CSS
- [x] 3.2 The `/panel` page (`basic`, `toggleable`, `header-toggler`, `template`), the menu, the route, `stylesheet(&[..])`
- [x] 3.3 `e2e/tests/pages.json` and `e2e/tests/panel.spec.ts` (expanding/collapsing, `aria-expanded`, the animation class); `npx playwright test panel`

## 4. Benchmarks and budgets

- [x] 4.1 Scenarios `panel-1`, `panel-1000`, `case-panel` in `benches/render`, `benches/wasm-size`, budgets by measurement

## 5. Integration

- [x] 5.1 `docs/panel.md` (conformance with PrimeNG, differences, performance); clippy, tests, deny, machete, `openspec validate port-panel`; `prime-panel` in the browser job of `ci.yml` (3 places); CI on `dev` is green

## Workflow follow-up

- Archive after CI; the next change is `port-fieldset` on top of `prime-core::collapsible`.
