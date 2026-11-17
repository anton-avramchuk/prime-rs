# galleria Specification

## Purpose
Galleria is an image gallery with thumbnails, indicators, auto-advance and fullscreen mode.

## ADDED Requirements

### Requirement: Thumbnail window
The model SHALL compute the thumbnail strip offset from the active index using the PrimeNG formulas: median index `floor(n/2)` (one less for even `n`), offset `0` while `active <= median`; `n_visible - len` at the end; otherwise `-active + median` (`+1` for even `n` at the end of the list). Page buttons SHALL be disabled if there are no more thumbnails than visible ones or (without `circular`) an edge one is active.

#### Scenario: Start
- **WHEN** `len = 6`, `n_visible = 3`, `active = 1`
- **THEN** offset `0`

#### Scenario: Middle
- **WHEN** `len = 6`, `n_visible = 3`, `active = 3`
- **THEN** offset `-2`

#### Scenario: End
- **WHEN** `len = 6`, `n_visible = 3`, `active = 5`
- **THEN** offset `-3`

### Requirement: Markup
`Galleria` SHALL render `div.p-galleria.p-component` (`role=region`, position classes `p-galleria-thumbnails-<pos>` and `p-galleria-indicators-<pos>` when shown, `p-galleria-inset-indicators`, `p-galleria-hover-navigators`, `p-galleria-fullscreen`) > `.p-galleria-content` > `.p-galleria-items-container` (`.p-galleria-items` > navigation buttons with `show_item_navigators`, `.p-galleria-item` with `role=group`, `aria-label` "N", `.p-galleria-caption` with a caption template; `ul.p-galleria-indicator-list` with `show_indicators`) and `.p-galleria-thumbnails` with `show_thumbnails`. With an empty `value` the content SHALL be absent.

#### Scenario: Basics
- **WHEN** three images, default values
- **THEN** there is a `.p-galleria-item` with the first frame, three `.p-galleria-thumbnail-item`, no indicators and no frame buttons

#### Scenario: Indicators
- **WHEN** `show_indicators = true`
- **THEN** three `li.p-galleria-indicator`, the first with `p-galleria-indicator-active`

### Requirement: Frame navigation
Frame buttons, indicators and thumbnails SHALL change the active index and trigger an `active_index` update. Next frame: `circular` and last → `0`, otherwise `+1`; previous: `circular` and first → last, otherwise `-1` (not below 0). A button is disabled (`p-disabled`) at an edge without `circular`. Indicator: click, `Enter`/`Space`; with `change_item_on_indicator_hover`, hover. Thumbnail: click, `Enter`/`Space`; `ArrowLeft`/`ArrowRight`/`Home`/`End` move focus across thumbnails.

#### Scenario: Forward button
- **WHEN** `show_item_navigators`, 0 is active, click on next
- **THEN** the active index is `1`, the second frame is shown

#### Scenario: Loop
- **WHEN** `circular`, the last is active, click on next
- **THEN** the active index is `0`

### Requirement: Auto-advance
With `auto_play` the frame SHALL change every `transition_interval` ms (after the last one, to the first with `circular`, otherwise it SHALL NOT move further); a user action (buttons, indicators, thumbnails) SHALL stop the advance if `should_stop_autoplay_by_click`.

#### Scenario: Stop
- **WHEN** auto-advance is running, the user clicks a thumbnail
- **THEN** auto-advance is stopped

### Requirement: Responsiveness
`responsive_options` (`breakpoint` in pixels, `num_visible`) SHALL choose the number of visible thumbnails by window width: the smallest `breakpoint` not less than the window width; otherwise the component `num_visible`. The visible count SHALL NOT exceed the number of images.

#### Scenario: Narrow window
- **WHEN** `[{960, 4}, {640, 2}]`, width 600
- **THEN** visible is `2`

### Requirement: Fullscreen mode
With `full_screen` the gallery SHALL be rendered in a `Portal` inside the `p-galleria-mask p-overlay-mask` mask when `visible`, with the `p-galleria-close-button` close button, page scroll lock and a focus trap; a click on the mask itself and the close button SHALL reset `visible`; leaving SHALL play the `p-galleria-leave-active` animation before removal from the DOM.

#### Scenario: Open and close
- **WHEN** `visible = true`, then a click on the close button
- **THEN** first the mask and content are in the DOM, then `visible = false` and removal after the animation

#### Scenario: Click inside
- **WHEN** a click on the frame inside the mask
- **THEN** the gallery stays open
