# Tasks

## 1. Foundation

- [x] 1.1 Add `tieredmenu` to `theme.components` (`upstream.toml`), `cargo xtask gen-theme`; check that the CSS of the other components did not change and that the `p-tieredmenu*` classes are present
- [x] 1.2 Check against `nestedPosition` from `@primeuix/utils` and the `prime-icons` icon set (angleright and the icons for the demo)

## 2. Crate `prime-tieredmenu`

- [x] 2.1 `model.rs`: tree nodes, validity, next/previous/first/last lookup, character search, click, path change, ←/→ arrows, `aria-setsize`/`aria-posinset`, submenu side, identifiers, with table-driven tests
- [x] 2.2 `TieredMenu` component, `TieredMenuPt`: root, root list, levels and items, separators, groups with an arrow, `item`/`submenu_icon`, SSR tests, rustdoc
- [x] 2.3 Path of active items and focus: click, repeated click, leaf, hover (`auto_display`, `dirty`, mobile), closing
- [x] 2.4 Keyboard: ↓/↑/→/←/`Home`/`End`/`Enter`/`Space`/`Escape`/`Tab`/`Alt+↑`, character search; `aria-activedescendant`
- [x] 2.5 Submenu position (side by available room), `matchMedia` for `p-tieredmenu-mobile`
- [x] 2.6 `popup` mode: `TieredMenuRef`, alignment, layer, outside click, `resize`, scrolling, focus return, `on_show`/`on_hide`
- [x] 2.7 Browser `tests/web.rs` (one sequential test)

## 3. Showcase and e2e

- [x] 3.1 Page `/tieredmenu` (`basic`, `popup`, `command`, `template`), menu, route, `stylesheet(&[..])`, the crate in the workspace and showcase
- [x] 3.2 `e2e/tests/pages.json` and `tieredmenu.spec.ts` (styles, expanding to the right, mirroring at the edge, keyboard, popup, command); wait for `networkidle`, restart the showcase server; `PW_CHANNEL=chrome npx playwright test tieredmenu`

## 4. Benchmarks and budgets

- [x] 4.1 Scenarios `-1` and `-1000`, `case-tieredmenu` (with `dep:prime-menu`) and budgets (first `cargo bench -p render-bench --bench alloc`; measure wasm in the background, with a separate budgets file)

## 5. Integration

- [x] 5.1 `docs/tieredmenu.md` (differences from PrimeNG, what was not verified); `prime-tieredmenu` in the browser job of `ci.yml` (three places)
- [x] 5.2 `cargo fmt --all -- --check`, clippy with the CI exclusions and `--all-features` (and for the wasm target), tests (SSR one by one: `--exact`; effects only under `cfg(wasm32)`), deny, machete, `openspec validate port-tieredmenu`; CI `dev` green

## Workflow follow-up

- Archive after CI.
- Wave 4 is done; next is the plan for the next wave (the remaining menus: ContextMenu, Menubar, MegaMenu, PanelMenu) and, as a separate change, `TieredMenu` inside `SplitButton`.
