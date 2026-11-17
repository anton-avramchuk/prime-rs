# Image

Port of `p-image` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-image::Image`, `ImagePt`, `ImageLoading`, `ImagePreviewContext` | `packages/primeng/src/image/image.ts`, `image/style/imagestyle.ts` |
| CSS | `@primeuix/styles/image` and the Aura preset: `cargo xtask gen-theme` (no `/* For PrimeNG */` block) |
| Demos | `apps/showcase/doc/image/*-doc.ts` (`basic`, `preview`, `previewimagesource`, `template`) |

Spec: `openspec/specs/image`.

## What matches PrimeNG

- Markup and classes: `div.p-image.p-component` (`p-image-preview`) > `img` and `button.p-image-preview-mask` with the eye icon `p-image-preview-icon` (`aria-label` is the `aria.zoomImage` translation, size is `width × height`). Preview: `div.p-image-mask.p-overlay-mask[role=dialog][aria-modal]` > `div.p-image-toolbar` with five `p-image-action ...` buttons (rotate right and left, zoom out, zoom in, close) and `img.p-image-original`.
- Rotation by ±90°, zoom in steps of 0.1; the image gets `transform: rotate(..deg) scale(..)`. The zoom out and zoom in buttons are disabled (`disabled` and `p-disabled`) by the same condition as in PrimeNG (`scale ± 0.1` does not leave the zoom strictly within 0.5-1.5), so zooms from 0.6 to 1.4 are reachable.
- Closing by the button, a click on the mask and `Escape` (focus returns to the preview button). A click on the enlarged image or the toolbar does not close the preview. Page scrolling is locked, the mask gets a layer of the `Modal` group, there is a focus trap inside, and after showing, focus is on the close button. On closing, rotation and zoom are reset.
- Custom views: `indicator`, `image` (receives the error callback), `preview_content` (receives the class, style and the image click callback) and the toolbar icons; events `on_show`, `on_hide`, `on_image_error`.

## Differences

| Difference | Reason |
|---|---|
| After a toolbar action the first click on the mask closes the preview. In PrimeNG the `previewClick` flag set by a toolbar button stays (the click is swallowed in `handleToolbarClick` and does not reach the mask), and the first click on the mask after a rotation or zoom does nothing | An obvious source bug: the toolbar already stops propagation, and the flag is needed only for a click on the image. |
| `Escape` is handled by a single listener on `window` and only for the topmost layer (a preview over a dialog will not close the dialog) | PrimeNG has two handlers (on the mask and on `document`); here there is a shared layer order, as in `Dialog`. |
| No `appendTo`: the preview is always in a `Portal` at `body` | The project's other overlays have no such option either. |
| No `showTransitionOptions`, `hideTransitionOptions`, `modalEnterAnimation`, `modalLeaveAnimation`, `motionOptions`, `maskMotionOptions` | Animation is set by theme classes (`p-overlay-mask-*-active`, `p-image-original-*-active`); the exit waits for the mask's `animationend` with a 400 ms fallback timer. |
| Icons are `ViewFn`, not `ng-template`; views instead of `pTemplate` | There is no `ng-template`. |
| `width` and `height` as numbers get `px` on the preview button; percentages and `rem` are taken as is | PrimeNG always uses `height + 'px'` (the style would be wrong for `50%`). |
| No `SafeUrl` | Addresses are strings. |
| The preview is not served by the server and appears after hydration | The mask is needed only after a click. |
| In the demos CDN photos are replaced with built-in SVG images | Independence from the network. |

## Not verified

- Right-to-left layout and touch input (PrimeNG has no pinch zoom either).
- Screen reading: `aria-*` were checked against PrimeNG, screen reader behavior was not.
- A preview over another modal layer was verified only at the `z-index` order level.

## Performance

See `benches/` (`image-1`, `image-1000`, `case-image`) and `benches/*budgets.toml`.
