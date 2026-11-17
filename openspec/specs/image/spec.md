# image Specification

## Purpose
Image shows a picture and, if configured, opens it in a modal layer with rotation and zoom.

## Requirements

### Requirement: Markup
`Image` of the `prime-image` crate SHALL render `div.p-image.p-component` (`p-image-preview` when `preview`; `style_class`, `style`, `pt`) with an `img` whose set `src`, `srcset`, `sizes`, `alt`, `width`, `height`, `loading` (`lazy` or `eager`), the `image_class` class and the `image_style` style are rendered as attributes, and unset ones are omitted. The `image` view SHALL replace the `img` and receive an error callback. With `preview` a `button.p-image-preview-mask[type=button]` SHALL be rendered with an `aria-label` from the translation `aria.zoomImage` and the size `width × height`, containing the eye icon `p-image-preview-icon`; the `indicator` view SHALL replace the icon.

#### Scenario: Without preview
- **WHEN** `Image` with `src` and `alt` is rendered and `preview` is not set
- **THEN** the `img` with these attributes is rendered, and there is no preview button

#### Scenario: With preview
- **WHEN** `preview = true` is set
- **THEN** the root has the class `p-image-preview`, and after the `img` the button `p-image-preview-mask` with the icon is rendered

### Requirement: Opening the preview
A click on the preview button SHALL open, in a `Portal`, the mask `div.p-image-mask.p-overlay-mask[role=dialog][aria-modal=true]` with the `modal` layer from the shared `z-index` stack, block the page scroll, set a focus trap, and after showing move the focus to the close button, then call `on_show`. Inside the mask there SHALL be the toolbar `div.p-image-toolbar` and the picture `img.p-image-original`, whose `src` is `preview_image_src` or, if it is not set, `src` (likewise `srcset` and `sizes` from `preview_image_srcset`/`preview_image_sizes`). The `preview_content` view SHALL replace the picture and receive the class `p-image-original`, a style with `transform` and a click callback on the picture.

#### Scenario: Opening
- **WHEN** the user clicks the preview button
- **THEN** a mask with the toolbar and the enlarged picture is rendered in `body`, the page scroll is blocked, and the focus is on the close button

#### Scenario: Another source
- **WHEN** `preview_image_src` is set
- **THEN** the enlarged picture takes it, not `src`

### Requirement: Action toolbar
The toolbar SHALL contain the buttons in this order: rotate right (`p-image-rotate-right-button`), rotate left (`p-image-rotate-left-button`), zoom out (`p-image-zoom-out-button`), zoom in (`p-image-zoom-in-button`), close (`p-image-close-button`); each has the class `p-image-action`, `type=button` and an `aria-label` from the translations `aria.rotateRight`, `aria.rotateLeft`, `aria.zoomOut`, `aria.zoomIn`, `aria.close`; the icons are replaced by the properties `rotate_right_icon`, `rotate_left_icon`, `zoom_out_icon`, `zoom_in_icon`, `close_icon`. The rotations SHALL change the angle by ±90°, the zoom — by ±0.1 within the range from 0.5 to 1.5; the picture SHALL get `transform: rotate(<angle>deg) scale(<zoom>)`. The zoom-out (zoom-in) button SHALL get `disabled` and the class `p-disabled` when the next step would leave the zoom outside the limit. Clicks on the toolbar SHALL not close the preview.

#### Scenario: Rotation and zoom
- **WHEN** "rotate right" and "zoom in" are pressed
- **THEN** the picture has `transform: rotate(90deg) scale(1.1)`, and the preview stays open

#### Scenario: Zoom limit
- **WHEN** the zoom is brought to the upper limit with the "zoom in" button
- **THEN** the "zoom in" button is disabled and has the class `p-disabled`

### Requirement: Closing
The preview SHALL close with the close button, with a click on the mask (but not on the panel and not on the enlarged picture) and with the `Escape` key; on `Escape` the focus SHALL return to the preview button. After the exit animation ends, the layer, the focus trap and the scroll block SHALL be released, the rotation and zoom — reset (0° and 1), the mask — removed from the DOM, and `on_hide` SHALL be called. A repeated close and the closing of an unfinished show SHALL not unblock the scroll twice.

#### Scenario: Click on the mask
- **WHEN** the preview is open and the user clicks the mask outside the picture and the panel
- **THEN** the preview closes, the scroll is unblocked, and `on_hide` is called

#### Scenario: Click on the picture
- **WHEN** the user clicks the enlarged picture
- **THEN** the preview stays open

#### Scenario: Reset
- **WHEN** a preview rotated by 90° is closed and opened again
- **THEN** the picture is again without rotation and at zoom 1

### Requirement: Loading error and styles
On a loading error of the main picture `on_image_error` SHALL be called with the event. The theme styles SHALL be connected through the `image` set (`cargo xtask gen-theme`).

#### Scenario: Error
- **WHEN** `img` reports a loading error
- **THEN** `on_image_error` is called
