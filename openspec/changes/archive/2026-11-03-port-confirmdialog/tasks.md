# Tasks

## 1. Foundation

- [x] 1.1 Add `confirmdialog` to `theme.components` (`upstream.toml`), `cargo xtask gen-theme`; check that the CSS of other components did not change and that the `p-confirmdialog*` classes exist
- [x] 1.2 `prime-confirmation`: `ConfirmEventType` (`Accept`, `Reject`, `Cancel`, `as_str`), test

## 2. Crate `prime-confirmdialog`

- [x] 2.1 `state.rs` (`pick`, `position_of`, button label, defaults) with table tests
- [x] 2.2 `ConfirmDialog`, `ConfirmDialogPt`, `ConfirmDialogHeadless` component: service subscription, wrapper over `Dialog`, body, footer with buttons, SSR tests, rustdoc
- [x] 2.3 Closing: buttons, close icon/`Escape`/mask as a rejection with `Cancel`, service `Close`, `on_hide` with the type, forgetting the confirmation after exit, replacing the confirmation while the window is open
- [x] 2.4 Focus by `default_focus`; templates `header_content`, `footer`, `message_content`, `icon_content`, `headless`, button icons
- [x] 2.5 Browser `tests/web.rs` (one sequential test)

## 3. Showcase and e2e

- [x] 3.1 Page `/confirmdialog` (`basic`, `position`, `template`, `headless`), menu, route, `stylesheet(&[..])`, the crate in the workspace and showcase
- [x] 3.2 `e2e/tests/pages.json` and `confirmdialog.spec.ts` (styles, accept/reject, close icon, `Escape`, positions, templates); wait for `networkidle`, restart the showcase server; `PW_CHANNEL=chrome npx playwright test confirmdialog`

## 4. Benchmarks and budgets

- [x] 4.1 Scenarios `-1` and `-1000`, `case-confirmdialog` and budgets (first `cargo bench -p render-bench --bench alloc`; wasm measurement — in the background, with a separate budgets file)

## 5. Integration

- [x] 5.1 `docs/confirmdialog.md` (differences from PrimeNG, what was not verified); `prime-confirmdialog` in the browser job of `ci.yml` (three places); `docs/confirmation.md` — `ConfirmEventType`
- [x] 5.2 `cargo fmt --all -- --check`, clippy with CI exclusions and `--all-features` (also for the wasm target), tests, deny, machete, `openspec validate port-confirmdialog`; CI `dev` green

## Workflow follow-up

- Archive after CI.
- Next: SpeedDial.
