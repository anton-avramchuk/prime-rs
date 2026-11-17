# dialog Specification

## Purpose
Dialog shows a window over the page with a header, content and footer; the window can be dragged, resized and maximized to full screen, and it is closed by a button, the Escape key and (optionally) a click on the mask.

## ADDED Requirements

### Requirement: Markup
While `visible` is false and the leave animation has not finished, `Dialog` of the `prime-dialog` crate SHALL render nothing. A shown dialog SHALL consist of a `div.p-dialog-mask` mask and a `div.p-dialog.p-component` window nested in it with `role` (default `dialog`), `aria-modal="true"` and `aria-labelledby` pointing at the title. The window SHALL contain `div.p-dialog-header` (with `show_header`), `div.p-dialog-content` with `children` and `div.p-dialog-footer` only when `footer` is set. The mask and window SHALL be located in `body`, not inside the component's declaration place. `style`, `style_class`, `content_style`, `content_style_class`, `mask_style`, `mask_style_class` and `pt` SHALL apply to the corresponding parts.

#### Scenario: Hidden by default
- **WHEN** `visible` is false and the dialog has never been shown
- **THEN** there is no `.p-dialog` or `.p-dialog-mask` in the document

#### Scenario: Shown
- **WHEN** `visible` becomes true with `header="Header"`
- **THEN** `body` contains `.p-dialog-mask > .p-dialog[role=dialog][aria-modal=true]` with the title "Header"

### Requirement: Header and buttons
The header SHALL contain the `span.p-dialog-title` title (with `header` and without `header_content`) or `header_content`, and a `div.p-dialog-header-actions` block with the `p-dialog-maximize-button` maximize button when `maximizable` and the `p-dialog-close-button` close button when `closable`. The buttons SHALL have an `aria-label` from the locale (`aria.close`, `aria.maximizeLabel` / `aria.minimizeLabel`). With `headless`, only the supplied view SHALL be rendered, without header, content and footer.

#### Scenario: Buttons by settings
- **WHEN** `maximizable` is on and `closable` is off
- **THEN** the header has a maximize button and no close button

### Requirement: Visibility value
`visible` SHALL be a two-way `bool` signal: true shows the dialog, false closes it. A user-initiated close (button, Escape, mask) SHALL write false to `visible`.

#### Scenario: Closing via button
- **WHEN** the dialog is shown and the user presses the close button
- **THEN** `visible` is false, and the window is removed after the leave animation

### Requirement: Position
`position` SHALL select the window's position on the screen: center, edge or corner. The mask SHALL get the class `p-dialog-<position>` for positions other than center and align the window accordingly.

#### Scenario: Corner
- **WHEN** `position=BottomRight`
- **THEN** the window is pressed to the bottom right corner of the browser window, the mask has `p-dialog-bottomright`

### Requirement: Modality
With `modal` the mask SHALL get `p-overlay-mask`, intercept clicks and block page scrolling while shown. Without `modal` the mask SHALL stay in the markup but not intercept page clicks and not block scrolling. With `dismissable_mask` and `closable`, a mouse button press on the mask itself (not on the window) SHALL close the dialog.

#### Scenario: Non-modal dialog
- **WHEN** `modal` is off and the user clicks the page behind the window
- **THEN** the click reaches the page, the dialog stays visible

#### Scenario: Closing by mask
- **WHEN** `modal` and `dismissable_mask` are on and the user presses the mask outside the window
- **THEN** the dialog closes

#### Scenario: Press inside the window
- **WHEN** `dismissable_mask` is on and the user presses inside the window
- **THEN** the dialog stays visible

### Requirement: Escape
With `close_on_escape` and `closable`, the `Escape` key SHALL close the dialog; with several open, only the topmost by `z-index`. After closing, the listener SHALL be removed.

#### Scenario: Nested dialogs
- **WHEN** two dialogs are open and Escape is pressed
- **THEN** only the topmost closes

### Requirement: Dragging
With `draggable`, a mouse button press on the header (but not on the buttons block) and pointer movement SHALL move the window (`position: fixed`, `left`, `top`, `margin: 0`). With `keep_in_viewport`, the window SHALL NOT go beyond the screen or `min_x` / `min_y`. After the button is released, `on_drag_end` SHALL be called. Without `draggable` the window does not move.

#### Scenario: Dragging the header
- **WHEN** the user presses the header and moves the pointer 50 px to the right
- **THEN** the window moved 50 px to the right

#### Scenario: Screen boundary
- **WHEN** the window is dragged toward the left screen edge with `keep_in_viewport`
- **THEN** the window's left edge does not go left of `min_x`

### Requirement: Resizing
With `resizable`, the window SHALL contain a `div.p-resizable-handle`; pressing it and moving the pointer SHALL change the window width and height and the content height, as long as the window fits the screen and is not smaller than `min-width` / `min-height`. The start SHALL call `on_resize_init`, the end — `on_resize_end`.

#### Scenario: Widening
- **WHEN** the user drags the handle 40 px to the right
- **THEN** the window became wider

### Requirement: Maximizing
With `maximizable`, the maximize button SHALL toggle the `p-dialog-maximized` class on the window and the button icon (`windowmaximize` / `windowminimize`), call `on_maximize` and block page scrolling if the dialog is non-modal and without `block_scroll`. On closing, maximization SHALL be reset.

#### Scenario: Expanding
- **WHEN** the user presses the maximize button
- **THEN** the window has `p-dialog-maximized`, `on_maximize` is called

### Requirement: Responsive width
With `breakpoints`, a `style` element with `@media screen and (max-width: <width>)` rules setting the window width without `p-dialog-maximized` SHALL be added to `head`; after closing and unmounting the element SHALL be removed.

#### Scenario: Width rule
- **WHEN** `breakpoints` contains `("575px", "90vw")`
- **THEN** `head` has a rule with `max-width: 575px` and `width: 90vw`

### Requirement: Animation
On show the window SHALL get the class `p-dialog-enter-active`, on closing — `p-dialog-leave-active`; the modal mask — `p-overlay-mask-enter-active` and `p-overlay-mask-leave-active`. The window SHALL stay in the DOM until `animationend` (or until the fallback time elapses if there is no animation).

#### Scenario: Leave
- **WHEN** the dialog is closed
- **THEN** first the window has `p-dialog-leave-active`, after the animation ends the window and mask are gone from the document

### Requirement: Focus
While the window is shown, with `focus_trap` `Tab` and `Shift+Tab` SHALL cycle focus within the window. With `focus_on_show`, focus SHALL go to the first focusable element of the content, or, if there is none, of the footer, then the header.

#### Scenario: Focus on show
- **WHEN** the content has an input field and the dialog is shown
- **THEN** focus is on that field

### Requirement: Layers
With `auto_z_index`, the window SHALL get the `z-index` of the modal window group offset by `base_z_index`, and the mask one below; the index SHALL be released after closing.

#### Scenario: Two dialogs
- **WHEN** two dialogs are open
- **THEN** the `z-index` of the later opened one is higher

### Requirement: Events
`on_show` SHALL be called once after showing, `on_hide` — once on closing by any means.

#### Scenario: Event order
- **WHEN** the dialog is shown and closed
- **THEN** `on_show` is called once before `on_hide`

### Requirement: Rendering without a browser
Outside the browser (SSR), `Dialog` SHALL render hidden and not access `window` and `document`; hydration SHALL NOT diverge from the server markup.

#### Scenario: SSR
- **WHEN** a page with `Dialog` is rendered on the server
- **THEN** there are no mask and window in the markup
