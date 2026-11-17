# Tasks

## 1. Foundation

- [x] 1.1 Add `panelmenu` to `theme.components` and to `primeng_extra` (`upstream.toml`; `panelmenustyle.ts` has a `/*For PrimeNG*/` block), `cargo xtask gen-theme`; check that the CSS of other components did not change and the `p-panelmenu*` classes exist
- [x] 1.2 Check against the source's `onArrow*Key`, `flatItems`, `onItemToggle`, `changeFocusedItem`; decide (D2) whether to take `prime-tieredmenu::model::{Node, build, item_at}`
- [x] 1.3 `prime-icons` icons: chevronright, chevrondown (and for the demo, if needed)

## 2. Crate `prime-panelmenu`

- [x] 2.1 Pure model: `toggle_key`, visible items, navigation (↓ ↑ ← → `Home` `End`), `aria` positions; table tests
- [x] 2.2 `PanelMenu`, `PanelMenuPt`, `PanelMenuItemContext` component: root, panels, headers, areas, levels, items, separators, `item`/`submenu_icon`, SSR tests, rustdoc
- [x] 2.3 Expansion: single and multiple mode, `expanded`, the `collapsible` animation, `PanelMenuRef` (`expand_all`/`collapse_all`/`toggle_all`)
- [x] 2.4 Focus and keyboard: `aria-activedescendant`, `p-focus`, moving between panels, `Enter`/`Space`, `Tab`
- [x] 2.5 Browser `tests/web.rs` (one sequential test)

## 3. Showcase and e2e

- [x] 3.1 Page `/panelmenu` (`basic`, `multiple`, `controlled`, `command`, `template`), menu, route, `stylesheet(&[..])`, the crate in the workspace and showcase
- [x] 3.2 `e2e/tests/pages.json` and `panelmenu.spec.ts` (styles, expansion, single/multiple mode, keyboard, command); `networkidle`, restart the showcase server; `PW_CHANNEL=chrome npx playwright test panelmenu`

## 4. Benchmarks and budgets

- [x] 4.1 Scenarios `-1` and `-1000`, `case-panelmenu` (with the needed `dep:`), budgets (first `cargo bench -p render-bench --bench alloc`; wasm — in the background, a separate budgets file, not `xtask wasm-size --help`; change the budget in `budgets.toml` with `sed` by line number)

## 5. Integration

- [x] 5.1 `docs/panelmenu.md` (differences from PrimeNG, what was not verified); `prime-panelmenu` in the browser job of `ci.yml` (three places)
- [x] 5.2 `cargo fmt --all -- --check`, clippy with CI exclusions and `--all-features` (also for wasm), tests (SSR one by one; wasm — by the `test result` line), deny, machete, `openspec validate port-panelmenu`; CI — once at the end of wave 5

## Workflow follow-up

- Archive; CI runs once at the end of wave 5.
- Next in the wave 5 plan: MegaMenu, Tree, TreeSelect, CascadeSelect, AutoComplete, InputMask.
