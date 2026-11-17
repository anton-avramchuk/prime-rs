# Tasks

## 1. Foundation

- [x] 1.1 Add `contextmenu` to `theme.components` (`upstream.toml`), `cargo xtask gen-theme`; check that the CSS of the other components did not change and the `p-contextmenu*` classes are present
- [x] 1.2 Extend `pub use model::…` in `prime-tieredmenu` with the needed functions (no behaviour change)

## 2. The `prime-contextmenu` crate

- [x] 2.1 `place` (position at the cursor) with table tests
- [x] 2.2 The `ContextMenu`, `ContextMenuPt`, `ContextMenuRef` component: root, list, levels, items, separators, groups, `item`/`submenu_icon`, SSR tests, rustdoc
- [x] 2.3 Active item path, focus, hover (`dirty`, mobile), keyboard, search, submenu position
- [x] 2.4 Trigger: `target`, `global`, `trigger_event`, long press; showing, repositioning, `on_show`/`on_hide`
- [x] 2.5 Closing: outside click (without the right button and Ctrl), `resize`, leaf, `Escape`, `Tab`; the `Menu` layer; listener removal
- [x] 2.6 Browser `tests/web.rs` (one sequential test)

## 3. Showcase and e2e

- [x] 3.1 The `/contextmenu` page (`basic`, `command`, `template`), menu, route, `stylesheet(&[..])`, the crate in the workspace and showcase
- [x] 3.2 `e2e/tests/pages.json` and `contextmenu.spec.ts` (styles, position at the cursor, flip at the edge, submenu, keyboard, command); `networkidle`, restart the showcase server; `PW_CHANNEL=chrome npx playwright test contextmenu`

## 4. Benchmarks and budgets

- [x] 4.1 Scenarios `-1` and `-1000`, `case-contextmenu` (with `dep:prime-menu`, `dep:prime-tieredmenu`), budgets (first `cargo bench -p render-bench --bench alloc`; wasm in the background, a separate budgets file)

## 5. Integration

- [x] 5.1 `docs/contextmenu.md` (differences from PrimeNG, what was not verified); `prime-contextmenu` in the browser job of `ci.yml` (three places)
- [x] 5.2 `cargo fmt --all -- --check`, clippy with the CI exceptions and `--all-features`, tests (SSR one by one: `--exact`; effects only under `cfg(wasm32)`; check the wasm test result by `test result`), deny, machete, `openspec validate port-contextmenu`; CI `dev` green

## Workflow follow-up

- Archive after CI.
- Next in the wave 5 plan: Menubar.
