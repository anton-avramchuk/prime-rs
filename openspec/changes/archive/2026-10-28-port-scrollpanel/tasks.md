# Tasks

## 1. Crate and component

- [x] 1.1 Create the crate `crates/prime-scrollpanel` (`publish = false`, features `ssr`/`hydrate`, dependencies `prime-core`, `web-sys`), add it to the workspace; check `cargo check -p prime-scrollpanel --all-features`
- [x] 1.2 Implement `ScrollPanel` and `ScrollPanelPt` (markup, bar ARIA, content `id`, the `content_template` slot, two branches); SSR tests: structure, order, attributes, `style`, `pt`, branch parity
- [x] 1.3 Pure bar arithmetic (`bar_size`, `bar_offset`, visibility) with native unit tests
- [x] 1.4 Client part: `moveBar` via `requestAnimationFrame`, `scroll`/`mouseenter`/`resize` listeners, height calculation with `max-height`, bar dragging, keyboard, `ScrollPanelRef` (`refresh`, `scroll_top`), listener removal
- [x] 1.5 Browser tests `tests/web.rs` (bar hiding, size and position, dragging, arrows and `keyup`, `scroll_top`, unmount)
- [x] 1.6 rustdoc with an example; `cargo test -p prime-scrollpanel --all-features`

## 2. Theme, showcase, e2e

- [x] 2.1 Add `scrollpanel` to `theme.components` (`upstream.toml`), `cargo xtask gen-theme`; check `.p-scrollpanel { display: block }` and add it to `primeng_extra` if missing; wire up the CSS
- [x] 2.2 Page `/scrollpanel` (`basic`, `custom`), menu, route, `stylesheet(&[..])`
- [x] 2.3 `e2e/tests/pages.json` and `e2e/tests/scrollpanel.spec.ts` (bar visibility, dragging, arrows); `npx playwright test scrollpanel`

## 3. Benchmarks and budgets

- [x] 3.1 Scenarios `scrollpanel-1`, `scrollpanel-1000`, `case-scrollpanel`, budgets by measurement; add the crate to `benches/render`, `benches/wasm-size`, the browser job `ci.yml` (3 places)

## 4. Integration

- [x] 4.1 `docs/scrollpanel.md` (matches, differences, performance); clippy, tests, deny, machete, `openspec validate port-scrollpanel`; CI `dev` green

## Workflow follow-up

- Archive after CI.
