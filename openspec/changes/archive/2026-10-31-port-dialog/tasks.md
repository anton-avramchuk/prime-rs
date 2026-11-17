# Tasks

## 1. Foundation

- [x] 1.1 Add `dialog` to `theme.components` (`upstream.toml`), `cargo xtask gen-theme`; check that the CSS of other components did not change and that `@keyframes` for `p-dialog-enter-active` / `p-dialog-leave-active` made it into the CSS (otherwise — resolve the question from design.md)

## 2. Crate

- [x] 2.1 `crates/prime-dialog`: `state.rs` (`DialogPosition`, `mask_classes`, `root_classes`, `mask_inline_style`, `root_inline_style`, `drag_position`, `drag_accept`, `resize_to`, `breakpoints_css`, `is_topmost`, `mask_z`) with table-driven tests
- [x] 2.2 The `Dialog` component and `DialogPt`: markup (mask > window), `Portal`, header and buttons, footer, `headless`, SSR tests, rustdoc
- [x] 2.3 Showing and closing: `visible`/`rendered`/`leaving`, animations, mask (`modal`, `dismissable_mask`), Escape on the topmost window, `block_scroll`, `FocusTrap`, focus on show, `z-index`, events; browser `tests/web.rs`
- [x] 2.4 Pointer and size: dragging (`keep_in_viewport`, `min_x`, `min_y`, `on_drag_end`), resizing (`on_resize_init`, `on_resize_end`), maximizing (`on_maximize`), `breakpoints`; browser tests

## 3. Showcase and e2e

- [x] 3.1 The `/dialog` page (`basic`, `position`, `modal`, `withoutmodal`, `maximizable`, `responsive`, `longcontent`, `template`, `headless`), menu, route, `stylesheet(&[..])`, crate in the workspace and showcase
- [x] 3.2 `e2e/tests/pages.json` and `dialog.spec.ts` (opening, closing via button, mask, Escape, modality, dragging and resizing with a real mouse, maximizing, positions, Tab cycle); wait for `networkidle`, restart the showcase server; `PW_CHANNEL=chrome npx playwright test dialog`

## 4. Benchmarks and budgets

- [x] 4.1 Scenarios `-1` and `-1000`, `case-dialog` and budgets (first `cargo bench -p render-bench --bench alloc`)

## 5. Integration

- [x] 5.1 `docs/dialog.md` (differences from PrimeNG, what was not verified); `prime-dialog` in the `ci.yml` browser job (three places)
- [x] 5.2 `cargo fmt --all -- --check`, clippy with `--all-features` (and for the wasm target), tests, deny, machete, `openspec validate port-dialog`; CI `dev` green

## Workflow follow-up

- Archive after CI.
- Next: Tooltip.
