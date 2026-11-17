# Tasks

## 1. Foundation

- [x] 1.1 `xtask/theme/extract.mjs`: a component without a styles module; `inputmask` in `theme.components` and `primeng_extra`, `cargo xtask gen-theme`; the CSS of the other components did not change, `p-inputmask-clear-icon` is present
- [x] 1.2 Cross-check against the source's `initMask`, `checkVal`, `shiftL/R`, `onKeyPress`, `onInputKeydown`, `onInputFocus/Blur`, `writeValue`, `getUnmaskedValue`; the icon from `prime-icons`: times

## 2. Crate `prime-inputmask`

- [x] 2.1 The mask engine `mask.rs` with table tests
- [x] 2.2 The `InputMask`, `InputMaskPt` component: the shell, the field, the clear icon, SSR tests, rustdoc
- [x] 2.3 Focus and the caret, character input, deletion, `Enter`/`Escape`, paste, `input`, losing focus, `on_complete`, `unmask`, the external value, clearing
- [x] 2.4 Browser `tests/web.rs` (one sequential test)

## 3. Showcase and e2e

- [x] 3.1 The `/inputmask` page, the menu, the route, `stylesheet(&[..])`, the crate in the workspace and the showcase
- [x] 3.2 `e2e/tests/pages.json` and `inputmask.spec.ts`; `networkidle`, restart the showcase server; `PW_CHANNEL=chrome npx playwright test inputmask`

## 4. Benchmarks and budgets

- [x] 4.1 Scenarios `-1` and `-1000`, `case-inputmask`, budgets

## 5. Integration

- [x] 5.1 `docs/inputmask.md`; `prime-inputmask` in the browser job of `ci.yml` (three places)
- [x] 5.2 `cargo fmt --all -- --check`, clippy with the CI exceptions and `--all-features` (and for wasm), tests, deny, machete, `openspec validate port-inputmask`; push and one CI run of the whole wave 5 — after the archive

## Workflow follow-up

- Archive; wave 5 is complete: push and `gh run watch`, and if it fails — fix it.
