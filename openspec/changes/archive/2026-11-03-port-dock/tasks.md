# Tasks

## 1. Foundation

- [x] 1.1 Add `dock` to `theme.components` (`upstream.toml`), `cargo xtask gen-theme`; check that the CSS of the other components did not change and that the classes `p-dock*` are present

## 2. Crate

- [x] 2.1 `crates/prime-dock`: `state.rs` (`root_classes`, `orientation`, `item_classes`, `item_id`, `key_action`, `step`) with table tests
- [x] 2.2 The `Dock`, `DockItem`, `DockCommand`, `DockPosition`, `DockPt` components: the root, the list, the items, the icon, the badge, `item`, SSR tests, rustdoc
- [x] 2.3 Click on an item and `command`, list focus, keys depending on the orientation, `Home`/`End`/`Enter`/`Space`, `on_focus`/`on_blur`
- [x] 2.4 Mobile mode via `matchMedia` and `breakpoint`; browser `tests/web.rs`

## 3. Showcase and e2e

- [x] 3.1 The `/dock` page (`basic` with choosing the position), the menu, the route, `stylesheet(&[..])`, the crate in the workspace and the showcase
- [x] 3.2 `e2e/tests/pages.json` and `dock.spec.ts` (markup and position, click, keys, mobile mode); wait for `networkidle`, restart the showcase server; `PW_CHANNEL=chrome npx playwright test dock`

## 4. Benchmarks and budgets

- [x] 4.1 Scenarios `-1` and `-1000`, `case-dock` and the budgets (first `cargo bench -p render-bench --bench alloc`; the wasm measurement — in the background, with a separate budgets file)

## 5. Integration

- [x] 5.1 `docs/dock.md` (differences from PrimeNG, what was not verified); `prime-dock` in the browser job of `ci.yml` (three places)
- [x] 5.2 `cargo fmt --all -- --check`, clippy with the CI exceptions and `--all-features` (and for the wasm target), tests, deny, machete, `openspec validate port-dock`; CI `dev` is green

## Workflow follow-up

- Archive after CI.
- Next: Image.
