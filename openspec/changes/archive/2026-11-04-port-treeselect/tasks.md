# Tasks

## 1. Foundation

- [x] 1.1 Add `treeselect` to `theme.components` and `primeng_extra` (`upstream.toml`), `cargo xtask gen-theme`; check that the CSS of other components did not change and the `p-treeselect*` classes are present
- [x] 1.2 Check `onKeyDown`, `onClick`, `show/hide`, `updateTreeState`, `label` of the source; `prime-icons` icons: chevrondown, times

## 2. Crate `prime-treeselect`

- [x] 2.1 Pure function `ancestors_of`, label of selected nodes; table-driven tests
- [x] 2.2 Component `TreeSelect`, `TreeSelectPt`: root, hidden field, label (comma/chips), clear, button, panel with `Tree`, SSR tests, rustdoc
- [x] 2.3 Opening and closing (click, outside click, `resize`, `Escape`), z-index, alignment, expanding ancestors, selection in three modes
- [x] 2.4 Hidden field keyboard, focus, `on_focus`/`on_blur`, events
- [x] 2.5 Browser `tests/web.rs` (one sequential test)

## 3. Showcase and e2e

- [x] 3.1 Page `/treeselect`, menu, route, `stylesheet(&[..])`, crate in workspace and showcase
- [x] 3.2 `e2e/tests/pages.json` and `treeselect.spec.ts`; `networkidle`, restart the showcase server; `PW_CHANNEL=chrome npx playwright test treeselect`

## 4. Benchmarks and budgets

- [x] 4.1 Scenarios `-1` and `-1000`, `case-treeselect`, budgets

## 5. Integration

- [x] 5.1 `docs/treeselect.md`; `prime-treeselect` in the browser job of `ci.yml` (three places)
- [x] 5.2 `cargo fmt --all -- --check`, clippy with CI exclusions and `--all-features` (and for wasm), tests, deny, machete, `openspec validate port-treeselect`; CI — once at the end of wave 5 (not run)

## Workflow follow-up

- Archive; no push and no waiting for CI until the end of wave 5.
- Next in the wave 5 plan: CascadeSelect, AutoComplete, InputMask.
