# Tasks

## 1. Foundation

- [x] 1.1 `xtask gen-theme`: recognize the `/** For PrimeNG **/` marker (a test next to the existing ones), add `drawer` to `theme.components` and `primeng_extra`, `cargo xtask gen-theme`; check that the CSS of other components did not change and that `@keyframes p-animate-drawer-*` made it into the CSS (otherwise resolve the question from design.md)
- [x] 1.2 `prime-dom`: `block_body_scroll()` / `unblock_body_scroll()` with a counter and the `p-overflow-hidden` class; table-driven counter tests, a browser DOM test

## 2. Crate

- [x] 2.1 `crates/prime-drawer`: `state.rs` (`DrawerPosition`, `root_classes`, `enter_class`, `leave_class`, `mask_classes`, `is_topmost`, `mask_z`) with table-driven tests
- [x] 2.2 `Drawer` component and `DrawerPt`: markup, `Portal`, header, close button, footer, `headless`, SSR tests, rustdoc
- [x] 2.3 Behaviour: `visible`/`rendered` and enter and leave animations, the mask and clicks on it, Escape on the topmost panel, `block_scroll`, `FocusTrap`, `z-index`, `on_show`/`on_hide`; browser `tests/web.rs`

## 3. Showcase and e2e

- [x] 3.1 Page `/drawer` (`basic`, `position`, `size`, `fullscreen`, `template`, `headless`), menu, route, `stylesheet(&[..])`, the crate in the workspace and showcase
- [x] 3.2 `e2e/tests/pages.json` and `drawer.spec.ts` (open, close by button, mask, Escape, positions, scroll lock, Tab cycle); wait for `networkidle`, restart the showcase server; `PW_CHANNEL=chrome npx playwright test drawer`

## 4. Benchmarks and budgets

- [x] 4.1 Scenarios `-1` and `-1000`, `case-drawer` and budgets (first `cargo bench -p render-bench --bench alloc`)

## 5. Integration

- [x] 5.1 `docs/drawer.md` (differences from PrimeNG, what was not verified); `prime-drawer` in the browser job of `ci.yml` (three places)
- [x] 5.2 `cargo fmt --all -- --check`, clippy with `--all-features` (and for the wasm target), tests, deny, machete, `openspec validate port-drawer`; CI `dev` green

## Workflow follow-up

- Archive after CI.
- Next: Dialog.
