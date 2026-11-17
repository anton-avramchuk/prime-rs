# Tasks

## 1. Foundation

- [x] 1.1 Add `menu` to `theme.components` (`upstream.toml`), `cargo xtask gen-theme`; check that the CSS of other components did not change and that `p-menu-*` and `p-menu-overlay` exist (otherwise — resolve the question from design.md)

## 2. Crate

- [x] 2.1 `crates/prime-menu`: `state.rs` (`MenuItem`, `flatten`, `item_id`, `root_classes`, `item_classes`, `root_style`, `enabled_ids`, `next_id`, `prev_id`, `should_prevent_default`, `can_run_command`) with table-driven tests
- [x] 2.2 Component `Menu` and `MenuPt`: root, `start`/`end`, list, items, groups, separators, `item`/`submenu_header`, SSR tests, rustdoc
- [x] 2.3 Static mode behavior: click and `command`, disabled items, focus and keys, `on_focus`/`on_blur`
- [x] 2.4 `MenuRef` and popup mode: show/hide, alignment, layer, outside click, resize, scroll, Escape/Tab/Alt+Up, `on_show`/`on_hide`; browser `tests/web.rs`

## 3. Showcase and e2e

- [x] 3.1 Page `/menu` (`basic`, `group`, `command`, `popup`, `template`), menu, route, `stylesheet(&[..])`, crate in workspace and showcase
- [x] 3.2 `e2e/tests/pages.json` and `menu.spec.ts` (markup, groups, click and command, keys, popup: show, outside click, Escape); wait for `networkidle`, restart the showcase server; `PW_CHANNEL=chrome npx playwright test menu`

## 4. Benchmarks and budgets

- [x] 4.1 Scenarios `-1` and `-1000`, `case-menu` and budgets (first `cargo bench -p render-bench --bench alloc`; wasm measurement — in the background)

## 5. Integration

- [x] 5.1 `docs/menu.md` (differences from PrimeNG, what was not verified); `prime-menu` in the browser job of `ci.yml` (three places)
- [x] 5.2 `cargo fmt --all -- --check`, clippy with `--all-features` (and for the wasm target), tests, deny, machete, `openspec validate port-menu`; CI `dev` green

## Workflow follow-up

- Archive after CI.
- Next: Listbox.
