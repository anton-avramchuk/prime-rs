# Tasks

## 1. Foundation

- [x] 1.1 Add `menubar` to `theme.components` (`upstream.toml`; in `primeng_extra` — if `menubarstyle.ts` has a `For PrimeNG` block), `cargo xtask gen-theme`; check that the CSS of the other components did not change and the classes `p-menubar*` are present
- [x] 1.2 Cross-check against the source's `onArrow*Key`/`onItemChange`; add to `prime-tieredmenu::model` the functions of the horizontal root (← → ↓) with table tests, without changing the behaviour of `TieredMenu`
- [x] 1.3 The `prime-icons` icons: angleright, angledown, bars (and for the demo)

## 2. Crate `prime-menubar`

- [x] 2.1 The `Menubar`, `MenubarPt`, `MenubarItemContext` component: the root, `start`/`end`, the button, the root list, levels, items, separators, groups, `item`/`submenu_icon`/`menu_icon`, SSR tests, rustdoc
- [x] 2.2 The path of active items, focus, hovering (`auto_display`, mobile), `auto_hide`, a click outside, `resize`
- [x] 2.3 The keyboard of the root and the submenus, search, `aria-activedescendant`, `on_focus`/`on_blur`
- [x] 2.4 Mobile mode: `matchMedia`, the button, the `Menu` layer, return of focus, the button's keys
- [x] 2.5 The position of nested submenus: there is no separate logic, the position is set by the theme CSS (as in PrimeNG)
- [x] 2.6 Browser `tests/web.rs` (one sequential test)

## 3. Showcase and e2e

- [x] 3.1 The `/menubar` page (`basic`, `command`, `template`), the menu, the route, `stylesheet(&[..])`, the crate in the workspace and the showcase
- [x] 3.2 `e2e/tests/pages.json` and `menubar.spec.ts` (styles, a submenu under the item, the keyboard, the command, the mobile button); `networkidle`, restart the showcase server; `PW_CHANNEL=chrome npx playwright test menubar`

## 4. Benchmarks and budgets

- [x] 4.1 Scenarios `-1` and `-1000`, `case-menubar` (with `dep:prime-menu`, `dep:prime-tieredmenu`), budgets (first `cargo bench -p render-bench --bench alloc`; wasm — in the background, a separate budgets file, not `xtask wasm-size --help`)

## 5. Integration

- [x] 5.1 `docs/menubar.md` (differences from PrimeNG, what was not verified); `prime-menubar` in the browser job of `ci.yml` (three places)
- [x] 5.2 `cargo fmt --all -- --check`, clippy with the CI exceptions and `--all-features` (and for wasm), tests (SSR one by one; wasm — by the `test result` line), deny, machete, `openspec validate port-menubar`; CI `dev` is green

## Workflow follow-up

- Archive after CI.
- Next in the wave 5 plan: PanelMenu, MegaMenu, Tree, TreeSelect, CascadeSelect, AutoComplete, InputMask.
