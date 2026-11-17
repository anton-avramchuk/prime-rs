# drawer Specification

## Purpose
Drawer shows a panel with content at the screen edge over the page: with a mask, enter and leave animation, closing by the mask, a button and the Escape key, page scroll lock and a focus trap.

## Requirements

### Requirement: Markup
While `visible` is false and the leave animation has not finished, the `Drawer` of the `prime-drawer` crate SHALL render nothing. A visible panel SHALL be `div.p-drawer.p-component[role=complementary]` with the class `p-drawer-<position>` (`left`, `right`, `top`, `bottom`, `full`), `p-drawer-open` when `visible` and `p-drawer-full` with `position=Full` or `full_screen`, and children `div.p-drawer-header` (title `div.p-drawer-title` with `header`, close button `button.p-drawer-close-button` with `closable`), `div.p-drawer-content` with `children` and `div.p-drawer-footer` only when `footer` is given. With `headless` only the supplied view SHALL be rendered, without the header, content and footer. `style`, `style_class` and `pt` SHALL apply to the panel root. The panel and mask SHALL be in `body`, not inside the place where the component is declared.

#### Scenario: Hidden by default
- **WHEN** `visible` is false and the panel has never been shown
- **THEN** the document has no `.p-drawer` and no `.p-drawer-mask`

#### Scenario: Shown
- **WHEN** `visible` becomes true with `header="Left Drawer"`
- **THEN** `body` contains `.p-drawer.p-drawer-left.p-drawer-open[role=complementary]` with the title "Left Drawer" and a close button

#### Scenario: Headless
- **WHEN** `headless` is set
- **THEN** inside `.p-drawer` there is only the supplied view

### Requirement: Position and size
`position` SHALL choose the screen edge: `Left`, `Right`, `Top`, `Bottom`; `Full` and `full_screen` SHALL stretch the panel to the full screen. Default sizes (width 20rem for the side panels, height 10rem for top and bottom) SHALL be set by theme styles and overridable with `style`.

#### Scenario: Side position
- **WHEN** `position=Right`
- **THEN** the panel is pinned to the right edge and takes the full window height

#### Scenario: Custom size
- **WHEN** `style="height: auto"` on a top panel
- **THEN** the panel height is determined by its content

### Requirement: Visibility value
`visible` SHALL be a two-way `bool` signal: setting it to true shows the panel, to false closes it. Closing by the user (mask, button, Escape) SHALL write false to `visible`.

#### Scenario: Closing with the button
- **WHEN** the panel is visible and the user presses the close button
- **THEN** `visible` becomes false and the panel is removed after the leave animation

### Requirement: Animation
On show the root SHALL get the enter animation class `p-drawer-enter-<position>` (`p-drawer-enter-full` for full screen), on close `p-drawer-leave-<position>`; the panel SHALL stay in the DOM until `animationend` (or until the fallback time elapses if there is no animation) and only then be removed, and the mask SHALL get `p-overlay-mask-enter-active` on show and `p-overlay-mask-leave-active` on close.

#### Scenario: Leave
- **WHEN** the panel is closed
- **THEN** first the root has `p-drawer-leave-left`, after the root and mask animation finish `.p-drawer` is absent from the document

### Requirement: Mask and closing by it
With `modal` a mask `div.p-drawer-mask.p-overlay-mask` SHALL be rendered behind the panel with a `z-index` one below the panel and `mask_style`; with `dismissible` a click on the mask SHALL close the panel, a click on the panel itself SHALL NOT. Without `modal` there is no mask.

#### Scenario: Click on the mask
- **WHEN** the panel is modal, `dismissible` is on and the user clicks the mask
- **THEN** the panel closes

#### Scenario: Not closable by the mask
- **WHEN** `dismissible` is off and the user clicks the mask
- **THEN** the panel stays visible

#### Scenario: No mask
- **WHEN** `modal` is off
- **THEN** the document has no `.p-drawer-mask`

### Requirement: Escape
With `close_on_escape` the `Escape` key SHALL close the panel; with several open panels only the topmost by `z-index`. The listener SHALL be removed after closing.

#### Scenario: Two panels
- **WHEN** two panels are open and Escape is pressed
- **THEN** only the topmost closes

#### Scenario: Escape off
- **WHEN** `close_on_escape` is off and Escape is pressed
- **THEN** the panel stays visible

### Requirement: Scroll lock
With `block_scroll`, while shown, `body` SHALL get the class `p-overflow-hidden`; after closing the class SHALL be removed, and with several panels - when the last of the locking ones is closed.

#### Scenario: Lock and release
- **WHEN** a panel with `block_scroll` is shown and then closed
- **THEN** the class is on `body` while shown and removed after closing

### Requirement: Focus
While the panel is shown, `Tab` and `Shift+Tab` SHALL cycle focus within the panel; after closing the trap SHALL be removed.

#### Scenario: Tab cycle
- **WHEN** focus is on the last element of the panel and Tab is pressed
- **THEN** focus moves to the first focusable element of the panel

### Requirement: Layers
With `auto_z_index` the panel SHALL get a `z-index` of the modal group offset by `base_z_index`; the index SHALL be released after closing.

#### Scenario: Two panels
- **WHEN** two panels are open
- **THEN** the `z-index` of the later opened one is higher

### Requirement: Events
`on_show` SHALL be called once on show, `on_hide` once on closing by any means.

#### Scenario: Event order
- **WHEN** the panel is shown and closed
- **THEN** `on_show` is called once before `on_hide`

### Requirement: Rendering without a browser
Outside the browser (SSR) `Drawer` SHALL render hidden and not access `window` and `document`; hydration SHALL NOT diverge from the server markup.

#### Scenario: SSR
- **WHEN** a page with `Drawer` is rendered on the server
- **THEN** there are no panels and masks in the markup
