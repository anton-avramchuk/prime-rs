# Tasks

## 1. Foundation

- [x] 1.1 Add `image` to `theme.components` (`upstream.toml`), `cargo xtask gen-theme`; check that the CSS of the other components did not change and that the classes `p-image*` are present
- [x] 1.2 Check the `aria.*` keys in `translation_defaults.rs` (`rotateLeft`, `zoomOut`), add the missing ones

## 2. Crate

- [x] 2.1 `crates/prime-image`: `state.rs` (`root_classes`, the zoom limits and steps, `transform`, the button classes, `mask_should_close`) with table tests
- [x] 2.2 The `Image`, `ImageLoading`, `ImagePt` component: the root, `img`, the preview button, the views `image`/`indicator`, `on_image_error`, SSR tests, rustdoc
- [x] 2.3 The preview: `Portal`, the mask, the panel, rotation and zoom, the `preview` template, closing by the button/mask/`Escape`, the layer, scroll blocking, the trap and the return of focus, `on_show`/`on_hide`, reset on closing
- [x] 2.4 Browser `tests/web.rs` (one sequential preview test)

## 3. Showcase and e2e

- [x] 3.1 The `/image` page (`basic`, `preview`, `previewimagesource`, `template`), local pictures, the menu, the route, `stylesheet(&[..])`, the crate in the workspace and the showcase
- [x] 3.2 `e2e/tests/pages.json` and `image.spec.ts` (styles, opening and closing the preview, rotation and zoom, the `Escape` key); wait for `networkidle`, restart the showcase server; `PW_CHANNEL=chrome npx playwright test image`

## 4. Benchmarks and budgets

- [x] 4.1 Scenarios `-1` and `-1000`, `case-image` and the budgets (first `cargo bench -p render-bench --bench alloc`; the wasm measurement — in the background, with a separate budgets file)

## 5. Integration

- [x] 5.1 `docs/image.md` (differences from PrimeNG, what was not verified); `prime-image` in the browser job of `ci.yml` (three places)
- [x] 5.2 `cargo fmt --all -- --check`, clippy with the CI exceptions and `--all-features` (and for the wasm target), tests, deny, machete, `openspec validate port-image`; CI `dev` is green

## Workflow follow-up

- Archive after CI.
- Next: ConfirmPopup.
