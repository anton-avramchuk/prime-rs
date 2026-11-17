# Tasks

## 1. Crate

- [x] 1.1 `crates/prime-popover`: `state.rs` (`root_classes`, `arrow_left`, `is_flipped`, `outside_click`, `retarget`) with table tests
- [x] 1.2 `PopoverRef` (`show`, `show_at`, `toggle`, `toggle_at`, `hide`, `visible`), the `Popover` component and `PopoverPt`: markup, SSR tests, rustdoc
- [x] 1.3 Behaviour: alignment, arrow and `flipped`, `z-index`, click outside/inside/on the target, Escape, `resize`, ancestor scroll, target change, `autofocus`, `on_show`/`on_hide`; browser `tests/web.rs`

## 2. Theme, showcase, e2e

- [x] 2.1 Add the `popover` module to `theme.components` (`upstream.toml`), `cargo xtask gen-theme`, make sure the CSS of other components did not change
- [x] 2.2 Page `/popover` (`basic`, `target`, `selectdata`, `template`), menu, route, `stylesheet(&[..])`, the crate in the workspace and showcase
- [x] 2.3 `e2e/tests/pages.json` and `popover.spec.ts` (click, click outside, Escape, another target, placement above the target); wait for `networkidle`, restart the showcase server; `PW_CHANNEL=chrome npx playwright test popover`

## 3. Benchmarks and budgets

- [x] 3.1 Scenarios `-1` and `-1000`, `case-popover` and budgets (first `cargo bench -p render-bench --bench alloc`)

## 4. Integration

- [x] 4.1 `docs/popover.md` (differences from PrimeNG, what was not verified); `prime-popover` in the browser job of `ci.yml` (three places)
- [x] 4.2 `cargo fmt --all -- --check`, clippy with `--all-features` (and for the wasm target), tests, deny, machete, `openspec validate port-popover`; CI `dev` green

## Workflow follow-up

- Archive after CI.
- Next: Drawer.
