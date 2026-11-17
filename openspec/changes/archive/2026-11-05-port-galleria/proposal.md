# Proposal

## Why

Galleria — an image gallery: a large frame, scrollable thumbnails, indicators, captions, automatic change and a full-screen mode. The second component of wave 6. The logic of the thumbnail window (the offset, the median element, page navigation, the adaptive number of visible items) is a self-contained pure model; the shell is built on the ready `Portal`/`FocusTrap`/`block_body_scroll`, as in Drawer.

## What Changes

- A new crate `prime-galleria` (a port of `p-galleria` from PrimeNG 21.1.10).
- A pure model `model.rs`: the thumbnail window (`median_index`, `total_pages`, the offset by the active index, the availability of the buttons, the visibility of thumbnails), the next/previous index of the frame, the choice of `num_visible` by the window width (`responsive_options`), the position classes.
- `Galleria`: `div.p-galleria.p-component` (`role=region`; in the full-screen mode — inside `div.p-galleria-mask.p-overlay-mask` in `Portal`, `role=dialog`, `aria-modal`) > the close button, `header`, `.p-galleria-content` > `.p-galleria-items-container` (`.p-galleria-items` > navigation buttons, `.p-galleria-item`, `.p-galleria-caption`; the list of indicators) and `.p-galleria-thumbnails` (`.p-galleria-thumbnails-content` > buttons, `-viewport` > `-thumbnail-items` > `-thumbnail-item` > `.p-galleria-thumbnail`), `footer`.
- The data — `Vec<GalleriaImage>` (`item_image_src`, `thumbnail_image_src`, `alt`, `title`); custom templates — `item`, `thumbnail`, `caption`, `indicator`, `header`, `footer` as `Callback`/`ViewFn`.
- Behaviour: the active index — `RwSignal<usize>`; the frame buttons (`show_item_navigators`, `circular`), the indicators (click, `Enter`/`Space`, hover with `change_item_on_indicator_hover`), the thumbnails (click, `Enter`/`Space`, arrows/`Home`/`End`, page buttons, touch), auto-change (`auto_play`, `transition_interval`, stopping by click with `should_stop_autoplay_by_click`), the positions of thumbnails and indicators, `show_indicators_on_item`, `show_item_navigators_on_hover`, vertical thumbnails, adaptive `num_visible`, the full-screen mode (`visible`, the mask, closing by the mask and the button, scroll blocking, focus trap).
- The page `/galleria` (`basic`, `autoplay`, `caption`, `controlled`, `thumbnail`, `responsive`, `advanced`), e2e, benchmarks and budgets, `docs/galleria.md`, `galleria` in `theme.components`.
- **Not included:** the document's `fullscreenchange` and the system full screen (`requestFullscreen`), `Bind`, Angular forms, `maskMotionOptions`/`motionOptions`/`showTransitionOptions`; `numVisibleLimit` is replaced by the limit `min(num_visible, len)`.

## Capabilities

### New Capabilities
- `galleria`: an image gallery with thumbnails, indicators, auto-change and a full-screen mode.

### Modified Capabilities
<!-- None. -->

## Impact

- A new crate `crates/prime-galleria` (`publish = false`); dependencies: `prime-rs-core`, `prime-dom`, `prime-icons`.
- Changes to `Cargo.toml` of the workspace, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml`.
