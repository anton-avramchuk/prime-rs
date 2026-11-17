# carousel Specification

## Purpose
Carousel shows a row of items in pages of `num_visible` items and pages through them with buttons, dots, autoplay and swipe.

## Requirements

### Requirement: Markup
The `Carousel` of the `prime-carousel` crate SHALL render `div.p-carousel.p-component.p-carousel-horizontal|vertical[role=region]` (`id`, `style`, `style_class`, `pt`) with the content: `div.p-carousel-header` (if `header` is set), `div.p-carousel-content-container` (+ `content_class`) > `div.p-carousel-content` (`aria-live="polite"` while autoplay is allowed, otherwise `off`) with a `p-carousel-prev-button` button, a `div.p-carousel-viewport` viewport (in vertical orientation the height is `vertical_view_port_height`, otherwise `auto`) and a `p-carousel-next-button` button (both with `show_navigators`; text and round, `p-disabled` when paging is impossible, `aria-label` from the `aria.prevPageLabel`/`aria.nextPageLabel` translations; icons are chevrons by orientation or `previous_icon`/`next_icon`). The viewport SHALL contain `div.p-carousel-item-list` with `div.p-carousel-item[role=group]` items (`p-carousel-item-active|start|end`, `data-p-carousel-item-active|start|end`, `aria-hidden`, `aria-label` from `aria.slideNumber`, `aria-roledescription` from `aria.slide`), and in infinite mode also `p-carousel-item-clone` clones before and after. With `show_indicators` a `ul.p-carousel-indicator-list` SHALL be rendered (+ `indicators_content_class`, `indicators_content_style`) with `li.p-carousel-indicator` (`-active`) and `button.p-carousel-indicator-button` (`aria-label` from `aria.pageLabel`, `aria-current="page"` on the active one, `tabindex`), then `div.p-carousel-footer` (if `footer` is set).

#### Scenario: Composition
- **WHEN** a `Carousel` of five items is rendered, `num_visible = 2`, `num_scroll = 1`
- **THEN** five items, four dots and both buttons are rendered, the root has `p-carousel-horizontal`

#### Scenario: Without navigators and dots
- **WHEN** `show_navigators = false`, `show_indicators = false`
- **THEN** there are no buttons and no dot list

### Requirement: Pages and offset
The page count SHALL be `⌈(n − num_visible)/num_scroll⌉ + 1` (0 for an empty value). The "next" button SHALL be disabled on the last page (unless in infinite mode) and for an empty value, "previous" — on page zero. Paging to a page SHALL shift the list by `−num_scroll·page` items (`translate3d` along the orientation axis in percent `100/num_visible` per item) with a `transform 500ms ease` transition. If the items split into pages with a remainder, the last page SHALL show the last `num_visible` items. The active items SHALL be those from `first_index` to `first_index + num_visible − 1`. When the page count shrinks, the page SHALL be clamped to the last one with an `on_page` event.

#### Scenario: Remainder
- **WHEN** nine items, `num_visible = 3`, `num_scroll = 2`, the user pages forward to the end
- **THEN** on the last (fourth) page items 7–9 are visible

#### Scenario: Boundaries
- **WHEN** the last page is open without infinite mode
- **THEN** the "next" button has `p-disabled`

### Requirement: Infinite mode
With `circular` and `n ≥ num_visible`, `num_visible` clones of the start and of the end SHALL be rendered. Paging forward from the last page SHALL go to the clones and, after the transition ends (`transitionend`), return without animation to the start of the list; on page zero "previous" does the opposite; the buttons are not disabled. With `n < num_visible` infinite mode SHALL NOT be enabled.

#### Scenario: Crossing the boundary
- **WHEN** the last page of infinite mode is open and "next" is pressed
- **THEN** the page becomes zero, and after the transition ends the list snaps to the start without animation

### Requirement: Autoplay
With `autoplay_interval > 0` a timer SHALL start: every `autoplay_interval` ms the next page SHALL open, and from the last one — page zero. Any manual paging (button, dot, external change of `page`) SHALL stop autoplay for good for this instance. The timer SHALL be cleared on unmount.

#### Scenario: Stopping
- **WHEN** autoplay is running and the user presses "next"
- **THEN** autoplay stops, `aria-live` becomes `off`

### Requirement: Responsiveness
With `responsive_options`, on mount and on window `resize` the last option with `breakpoint ≥ window width` SHALL be selected (otherwise the defaults), the page SHALL be recomputed by the formula `⌊page·old_step/new_step⌋`, clones SHALL be rebuilt, and `on_page` SHALL be called when the page changes. The width of each item SHALL be set by the inline style `flex: 1 0 <100/num_visible>%`.

#### Scenario: Narrow window
- **WHEN** the window is narrower than the `breakpoint` of an option with `num_visible = 1`
- **THEN** items take the full viewport width, `num_visible` equals 1

### Requirement: Dots, keyboard and swipe
A click on a dot SHALL open its page. The focused dot SHALL have `tabindex` `0`, the others `-1`; ←/→ on the dot list SHALL move focus to the adjacent dot without leaving the bounds. A swipe over the viewport (touch events) of more than 20 px SHALL page forward (left/up) or backward (right/down); `touchmove` SHALL NOT scroll the page.

#### Scenario: Dot
- **WHEN** the third dot is pressed
- **THEN** the third page is open, `on_page(2)`

#### Scenario: Swipe
- **WHEN** the user swipes left by 50 px
- **THEN** the next page opens
