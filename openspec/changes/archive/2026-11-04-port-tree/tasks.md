# Tasks

## 1. Foundation

- [x] 1.1 Add `tree` to `theme.components` (`upstream.toml`), `cargo xtask gen-theme`; check that the CSS of other components did not change and the `p-tree*` classes are present
- [x] 1.2 Cross-check against the source's `onNodeClick`, `propagateUp/Down`, `_filter`, `onArrow*`; `prime-icons` icons: chevronright, chevrondown, spinner, search, check, minus

## 2. Crate `prime-tree`

- [x] 2.1 `TreeNode` model, pure logic in `state.rs` (visible nodes, selection of all modes, propagation, partial selection, filter, keys) with table-driven tests
- [x] 2.2 `Tree`, `TreePt`, `TreeNodeContext` component: root, mask, filter, list, node, button, checkbox, icon, label, children, empty message, SSR tests, rustdoc
- [x] 2.3 Expansion, selection (`Single`/`Multiple`/`Checkbox`, `meta`, `selectable`), events, external `expanded_keys`/`selection` signals
- [x] 2.4 Filter: field, modes, expansion of matching ancestors, `on_filter`
- [x] 2.5 Keyboard and roving tabindex, `on_node_double_click`
- [x] 2.6 Browser `tests/web.rs` (one sequential test)

## 3. Showcase and e2e

- [x] 3.1 `/tree` page (`basic`, `single`, `multiple`, `checkbox`, `controlled`, `event`, `filter`, `lazy`, `template`), menu, route, `stylesheet(&[..])`, the crate in the workspace and showcase
- [x] 3.2 `e2e/tests/pages.json` and `tree.spec.ts`; `networkidle`, restart of the showcase server; `PW_CHANNEL=chrome npx playwright test tree`

## 4. Benchmarks and budgets

- [x] 4.1 Scenarios `-1` and `-1000`, `case-tree`, budgets (first `cargo bench -p render-bench --bench alloc`; wasm — in the background, a separate budgets file)

## 5. Integration

- [x] 5.1 `docs/tree.md`; `prime-tree` in the browser job of `ci.yml` (three places)
- [x] 5.2 `cargo fmt --all -- --check`, clippy with CI exclusions and `--all-features` (and for wasm), tests, deny, machete, `openspec validate port-tree`; CI — once at the end of wave 5 (not run)

## Workflow follow-up

- Archive; no push and no waiting for CI until the end of wave 5.
- Next in the wave 5 plan: TreeSelect, CascadeSelect, AutoComplete, InputMask.
