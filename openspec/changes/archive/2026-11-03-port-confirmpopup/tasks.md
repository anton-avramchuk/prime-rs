# Tasks

## 1. Foundation

- [x] 1.1 Add `confirmpopup` to `theme.components` (`upstream.toml`), `cargo xtask gen-theme`; check that the CSS of the other components did not change and that the classes `p-confirmpopup*` are present
- [x] 1.2 Check the translations `accept` and `reject` in `translation_defaults.rs`

## 2. Crate `prime-confirmation`

- [x] 2.1 `Confirmation`, `ConfirmButtonProps`, `ConfirmFocus`, `ConfirmEvent`, `ConfirmationService`, `provide_confirmation_service`/`use_confirmation_service`, rustdoc, native tests (subscribers, `confirm`, `close`, `unsubscribe`, the context)

## 3. Crate `prime-confirmpopup`

- [x] 3.1 `state.rs` (`root_classes`, `arrow_left`, `is_flipped`, `outside_click`, `focus_target`, `accepts_key`, `button_label`) with table tests
- [x] 3.2 The `ConfirmPopup` component, `ConfirmPopupPt`: subscription to the service, the panel markup, the buttons, `content`/`headless`/icons, SSR tests, rustdoc
- [x] 3.3 Showing at the target: alignment, the arrow, flipping, the layer, focus by `default_focus`, `accept`/`reject`, returning the focus to the target, retargeting, `visible`, `on_show`/`on_hide`
- [x] 3.4 Closing: `Escape`, a click outside, `resize`, scrolling of an ancestor, `close()`; `close_on_escape`, `dismissable_mask`
- [x] 3.5 Browser `tests/web.rs` (one sequential test)

## 4. Showcase and e2e

- [x] 4.1 The `/confirmpopup` page (`basic`, `template`, `headless`), the menu, the route, `stylesheet(&[..])`, the crates in the workspace and the showcase
- [x] 4.2 `e2e/tests/pages.json` and `confirmpopup.spec.ts` (styles, position and arrow, accept/reject, Escape, a click outside, templates); wait for `networkidle`, restart the showcase server; `PW_CHANNEL=chrome npx playwright test confirmpopup`

## 5. Benchmarks and budgets

- [x] 5.1 Scenarios `-1` and `-1000`, `case-confirmpopup` and the budgets (first `cargo bench -p render-bench --bench alloc`; the wasm measurement — in the background, with a separate budgets file)

## 6. Integration

- [x] 6.1 `docs/confirmpopup.md`, `docs/confirmation.md` (differences from PrimeNG, what was not verified); `prime-confirmpopup` in the browser job of `ci.yml` (three places)
- [x] 6.2 `cargo fmt --all -- --check`, clippy with the CI exceptions and `--all-features` (and for the wasm target), tests, deny, machete, `openspec validate port-confirmpopup`; CI `dev` is green

## Workflow follow-up

- Archive after CI.
- Next: ConfirmDialog (on `prime-confirmation`).
