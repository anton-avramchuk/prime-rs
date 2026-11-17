# Design

## Context

The requirements are in `specs/image`. PrimeNG 21.1.10 sources: `image/image.ts` (about 550 lines), `image/style/imagestyle.ts`; styles `@primeuix/styles/image`; demos `apps/showcase/doc/image/{basic,preview,previewimagesource,template}-doc.ts`. Templates in the project: `prime-dialog` (`Portal`, scroll blocking, `z-index`, `FocusTrap`, `Escape`, the stages of showing and hiding), `prime-blockui` (the mask and the shared stack), `prime-imagecompare` (pictures, `Pt`).

Observations on the source:
- The root class is `p-image p-component` + `p-image-preview`; the `img` without a wrapper; the preview button has the size `width × height` (strings with `px`, as in PrimeNG).
- `onImageClick` enables `maskVisible`/`previewVisible`, builds the mask and the preview, blocks the scrolling. `onMaskClick` closes, if there was no `previewClick`; the panel actions and the click on the picture set `previewClick = true`, so a click on them does not close the mask; `handleToolbarClick` stops the propagation.
- `Escape` on the mask (and the global `document:keydown.escape`): close; on the mask the focus then returns to the preview button (after 25 ms).
- After showing (`onAnimationStart`): the layer `modal` by `zIndex`, `onShow`, the focus on the close button after 25 ms. After hiding: release the layer, `rotate = 0`, `scale = 1`, unblock the scrolling, `onHide`.
- Zoom: `step 0.1`, `min 0.5`, `max 1.5`; the buttons are blocked when the next step goes beyond the limit (`scale ± step <= min`/`>= max`; with floating-point arithmetic the comparison is done with a tolerance).
- The mask animations are set by the classes `p-overlay-mask-enter-active`/`-leave-active`; for the picture in theme 21 — through `p-motion` with the name `p-image-original`.

## Goals / Non-Goals

**Goals:** the same DOM, classes, roles and `aria-*`; the same preview with rotation, zoom and closing; scroll blocking and layers as in the other modal components.

**Non-Goals:** `appendTo`, transitions through `motionOptions`, `Bind`, `pTemplate`, `SafeUrl`.

## Decisions

### D1. Crate and API
`prime-image` with `Image`, `ImageLoading {Lazy, Eager}`, `ImagePt`. Properties — `Prop`/`OptProp`, as in the other crates; the views — `ViewFn`; `preview` (a template) receives `ImagePreviewContext { class, style, preview_callback }` — like the PrimeNG context; `image` receives `error_callback`.

### D2. Pure logic (`state.rs`)
`root_classes(preview, extra)`, `zoom_in_disabled(scale)`, `zoom_out_disabled(scale)`, `zoom_in(scale)`/`zoom_out(scale)` (rounded to hundredths), `transform(rotate, scale)`, `action_classes(kind, disabled)`, `mask_should_close(preview_click)`. Table tests, including the zoom limits.

### D3. Preview state
`RwSignal` for `open` (showing requested), `rendered` (the layer is in the DOM while the exit is in progress), `rotate`, `scale`, the flag `preview_click` (not a signal). The core structure follows the sample of `prime-dialog`: `StoredValue` for the layer `z-index`, the focus trap and the blocking flag; repeated closing and double scroll blocking are guarded by flags (a lesson from BlockUI).

### D4. Showing
`Portal` is built anew on each showing (the ready views are not `Send + Sync`). The mask gets the classes `p-overlay-mask-enter-active`/`-leave-active`; the exit waits for `animationend` with a fallback timer, as in `Dialog`. The layer — `prime_rs_core::zindex::generate(ZGroup::Modal, ..)`.

### D5. Keys and focus
`Escape` — a `keydown` listener on the mask and on `document` (while open): close and return the focus to the preview button via `set_timeout(25)`. `FocusTrap` on the mask; the focus on the close button via `set_timeout(25)`.

### D6. Labels
The `aria-label` of the buttons — from `use_prime_config()` (`aria.zoomImage`, `aria.rotateRight`, `aria.rotateLeft`, `aria.zoomIn`, `aria.zoomOut`, `aria.close`); check that the keys are in `translation_defaults.rs`, and add the missing ones.

### D7. Loading error
`on_error` of the `img` calls `on_image_error`; in SSR the handler is not attached.

### D8. CSS
`image` is added to `theme.components`; check `p-image*` in the preset.

### D9. Tests
SSR: markup, `preview`/without, the attributes of `img`, the `image` template, `indicator`, `pt`, `style_class`, `aria-label`. Browser `tests/web.rs` (one sequential test because of the shared `z-index` stack and the scroll-blocking counter): opening, the composition of the panel, rotation and zoom (`transform`, `disabled` at the limits), closing by the button/mask/`Escape`, a click on the picture and the panel does not close, resetting the rotation and zoom, focus, scroll blocking, `on_show`/`on_hide`. e2e: the theme styles (the preview button over the picture, the eye on hover, the mask over the page), the full preview scenario.

### D10. Demos and benchmarks
PrimeNG's pictures from the CDN are not used in the demos: a local SVG (a gradient) in `showcase/assets`; `template` with custom `indicator`, `image`, `preview`. Bench `image-1`/`-1000` and `case-image`.

## Risks / Trade-offs

- [Exit animation] If the theme does not set `animationend`, a fallback timer is needed, otherwise the layer will not be removed.
- [Fractional zoom] Accumulated floating-point error; we round to hundredths.
- [Shared listeners] `Escape` on `document` must not fire for a closed preview; it is removed on closing and in `on_cleanup`.

## Open Questions

- Whether `appendTo` is needed: all the other overlays of the project manage without it.

## Migration Plan

No migration. Rollback — `git revert`.
