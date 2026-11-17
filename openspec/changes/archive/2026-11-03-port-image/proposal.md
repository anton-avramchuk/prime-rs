# Proposal

## Why

Image is the fifth component of wave 4: a picture with an optional preview in a modal layer (rotation, zoom, closing). It relies on the ready `Portal`, scroll blocking, focus trap, the `z-index` stack and icons, so it adds little that is new.

## What Changes

- A new crate `prime-image` (a port of `p-image` from PrimeNG 21.1.10).
- `Image`: the root `div.p-image.p-component` (`p-image-preview` with `preview`) > `img` (`src`, `srcset`, `sizes`, `alt`, `width`, `height`, `loading`, the class and the style) > with `preview` a button `button.p-image-preview-mask` with an eye icon (`aria-label` from the translation `aria.zoomImage`).
- Preview: a click on the button renders in `Portal` a `div.p-image-mask.p-overlay-mask[role=dialog]` with the panel `div.p-image-toolbar` (rotate right and left, zoom out, zoom in, close; the buttons `p-image-action …`, at the zoom limits `p-disabled` and `disabled`) and the enlarged picture `img.p-image-original` with `transform: rotate(..deg) scale(..)`. Zoom from 0.5 to 1.5 with a step of 0.1.
- Closing: the button, a click on the mask (but not after an action on the panel or a click on the picture), `Escape` (the focus returns to the preview button). Blocking of the page scrolling, the layer by the `modal` stack, the focus trap, the focus on the close button; on closing the rotation and zoom are reset.
- Properties: `src`, `srcset`, `sizes`, `preview_image_src`, `preview_image_srcset`, `preview_image_sizes`, `alt`, `width`, `height`, `loading` (`ImageLoading`), `preview`, `image_class`, `image_style`, `style_class`, `style`, `pt`; the views `image`, `indicator`, `preview_content`, the panel icons (`rotate_right_icon`, `rotate_left_icon`, `zoom_out_icon`, `zoom_in_icon`, `close_icon`) as `ViewFn`; the events `on_show`, `on_hide`, `on_image_error`.
- The showcase page `/image` with the demos `basic`, `preview`, `previewimagesource`, `template`; e2e; benchmarks and budgets; `docs/image.md`.
- Styles: `image` is added to `theme.components`.
- **Not included:** `appendTo` (the preview is always in `Portal` at `body`), `showTransitionOptions`/`hideTransitionOptions`/`modalEnterAnimation`/`motionOptions`/`maskMotionOptions` (the animation is the theme classes), `Bind`, `pTemplate`, `SafeUrl`.

## Capabilities

### New Capabilities
- `image`: a picture with a preview in a modal layer, with rotation and zoom.

### Modified Capabilities
<!-- None. -->

## Impact

- A new crate `crates/prime-image` (`publish = false`); dependencies: `prime-core`, `prime-dom`, `prime-icons` (eye, refresh, undo, searchminus, searchplus, times), `web-sys`.
- Changes to `Cargo.toml` of the workspace, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (the browser job, three places).
