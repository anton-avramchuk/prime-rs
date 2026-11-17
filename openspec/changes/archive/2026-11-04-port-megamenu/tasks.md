# Tasks

## 1. Foundation

- [x] 1.1 Add `megamenu` to `theme.components` (`upstream.toml`; `megamenustyle.ts` has no `For PrimeNG` block), `cargo xtask gen-theme`; check that the CSS of other components did not change and the `p-megamenu*` classes are present
- [x] 1.2 Check `onArrow*Key`, `onItemChange`, `findNextItemIndex`/`findPrevItemIndex`, `onItemClick`, `hide`, `searchItems` of the source; decide (D1) what to reuse from `prime-tieredmenu`
- [x] 1.3 `prime-icons` icons: angledown, angleright, bars (and for the demo)

## 2. Crate `prime-megamenu`

- [x] 2.1 Model `MegaMenuItem`/`MegaMenuSubmenu`, `MegaMenuOrientation`; pure logic in `state.rs` (nodes with keys, visible, transitions for both orientations, search, `column_class`) with table-driven tests
- [x] 2.2 Component `MegaMenu`, `MegaMenuPt`, `MegaMenuItemContext`: root, `start`/`end`, button, root list, items, overlay, columns, submenus, separators, `item`/`submenu_icon`/`menu_icon`, SSR tests, rustdoc
- [x] 2.3 Path of active items, focus, hover (`dirty`, mobile), outside click, `resize`
- [x] 2.4 Keyboard for root and overlay, search, `aria-activedescendant`, `on_focus`/`on_blur`
- [x] 2.5 Mobile mode: `matchMedia`, button, `Menu` layer, focus return, button keys
- [x] 2.6 Browser `tests/web.rs` (one sequential test)

## 3. Showcase and e2e

- [x] 3.1 Page `/megamenu` (`basic`, `vertical`, `command`, `template`), menu, route, `stylesheet(&[..])`, crate in workspace and showcase
- [x] 3.2 `e2e/tests/pages.json` and `megamenu.spec.ts` (column styles, vertical orientation, keyboard, command, mobile button); `networkidle`, restart the showcase server; `PW_CHANNEL=chrome npx playwright test megamenu`

## 4. Benchmarks and budgets

- [x] 4.1 Scenarios `-1` and `-1000`, `case-megamenu` (with the needed `dep:`), budgets (first `cargo bench -p render-bench --bench alloc`; wasm — in the background, a separate budgets file, not `xtask wasm-size --help`; change a budget with `sed` by line number)

## 5. Integration

- [x] 5.1 `docs/megamenu.md` (differences from PrimeNG, what was not verified); `prime-megamenu` in the browser job of `ci.yml` (three places)
- [x] 5.2 `cargo fmt --all -- --check`, clippy with CI exclusions and `--all-features` (and for wasm), tests (SSR one at a time; wasm — by the `test result` line), deny, machete, `openspec validate port-megamenu`; CI — once at the end of wave 5 (not run)

## Workflow follow-up

- Archive; no push and no waiting for CI until the end of wave 5.
- Next in the wave 5 plan: Tree, TreeSelect, CascadeSelect, AutoComplete, InputMask.
