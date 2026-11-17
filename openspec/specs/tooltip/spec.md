# tooltip Specification

## Purpose
Tooltip shows a hint next to an element on mouse hover or focus without changing the page markup; the tooltip picks a side and moves to another one when there is not enough room.

## Requirements

### Requirement: Attaching to an element
The `tooltip` directive of the `prime-tooltip` crate SHALL attach to any element with the `use:` attribute and SHALL NOT change its markup or set of attributes. Outside the browser (SSR) the directive SHALL do nothing and SHALL NOT access `window` and `document`.

#### Scenario: Markup is unchanged
- **WHEN** the `tooltip` directive is added to an element
- **THEN** the server markup of the element matches the markup without the directive

### Requirement: Tooltip markup
A shown tooltip SHALL be a `div.p-tooltip.p-component[role=tooltip]` in `body` with a `p-tooltip-<position>` class (`right`, `left`, `top`, `bottom`), with child `div.p-tooltip-arrow` and `div.p-tooltip-text`; `style_class` SHALL be added to the root classes and `id` SHALL be set on the root. While the tooltip is not shown, the document contains no `.p-tooltip` from this element.

#### Scenario: Shown
- **WHEN** the user hovers the mouse over an element with the tooltip text "Enter your username"
- **THEN** `body` contains `.p-tooltip.p-tooltip-right[role=tooltip]` with the text in `.p-tooltip-text`

### Requirement: Show events
With `Hover` the tooltip SHALL be shown on `mouseenter` and touch and hidden on `mouseleave`, `click` and touch end. With `Focus` it SHALL be shown on `focus` and hidden on `blur` of the element itself (or of the input inside a `p-inputwrapper` wrapper, or of the `.p-component` component inside the element). With `Both` both sets of events SHALL work.

#### Scenario: Focus only
- **WHEN** `event=Focus` and the user hovers the mouse over the element
- **THEN** the tooltip is not shown; on keyboard focus it is shown

### Requirement: Delays and lifetime
`show_delay` SHALL postpone the show, `hide_delay` the hide, and `life` SHALL hide a shown tooltip after the given time (plus `show_delay`). The cursor leaving before `show_delay` ends SHALL cancel the show.

#### Scenario: Show delay
- **WHEN** `show_delay` is 300 ms and the cursor leaves after 100 ms
- **THEN** the tooltip is not shown

### Requirement: Auto-hide
With `auto_hide` the tooltip SHALL NOT capture the pointer. Without `auto_hide` the tooltip SHALL be hoverable with the cursor: the cursor moving from the element onto the tooltip does not hide it, moving off the tooltip hides it.

#### Scenario: Hovering the tooltip
- **WHEN** `auto_hide` is off and the cursor moves from the element onto the tooltip
- **THEN** the tooltip stays visible

### Requirement: Position
The tooltip SHALL be placed on the chosen side of the element, centered on the perpendicular axis, with the `position_top` and `position_left` offsets; the arrow SHALL point at the element. If the tooltip goes beyond the window bounds on the chosen side, the other sides SHALL be tried in priority order (`top`: top → bottom → right → left; `bottom`: bottom → top → right → left; `left`: left → right → top → bottom; `right`: right → left → top → bottom) until one fits.

#### Scenario: No room on the right
- **WHEN** the element is at the right edge of the window and `position=Right`
- **THEN** the tooltip is placed on the left, the root has `p-tooltip-left`

### Requirement: Content
Text SHALL be rendered as text (markup is not parsed); `Html` SHALL be rendered as markup; `View` SHALL render the passed view. Empty content does not show a tooltip.

#### Scenario: Text is not parsed as markup
- **WHEN** the content is `<b>x</b>` as `Text`
- **THEN** the tooltip shows the literal `<b>x</b>`, there is no `b` element

### Requirement: Disabling and clipped text
With `disabled` the tooltip SHALL NOT be shown, and an already shown one SHALL be hidden. With `show_on_ellipsis` the tooltip SHALL be shown only if the element content does not fit in it by width or height.

#### Scenario: Text fits
- **WHEN** `show_on_ellipsis` is on and the text is fully visible
- **THEN** the tooltip is not shown

### Requirement: Closing
A shown tooltip SHALL be hidden by the `Escape` key (with `hide_on_escape`), by a window resize and by scrolling of a nested scrollable ancestor of the element; scrolling the page itself does not hide the tooltip, since it is positioned in page coordinates. After hiding, the subscriptions SHALL be removed and the `z-index` released.

#### Scenario: Escape
- **WHEN** the tooltip is shown and Escape is pressed
- **THEN** the tooltip is hidden

#### Scenario: Page scroll
- **WHEN** the tooltip is shown and the page is scrolled
- **THEN** the tooltip stays visible next to the element

### Requirement: Layers
Without `z_index` the tooltip SHALL get the `z-index` of the tooltip group; with `z_index`, the given value.

#### Scenario: Above a modal window
- **WHEN** the tooltip is shown for an element inside an open `Dialog`
- **THEN** its `z-index` is higher than the `z-index` of the dialog window

### Requirement: Content change
When the text changes while shown, the tooltip SHALL update the content and recompute the position; when it changes to empty, it SHALL hide.

#### Scenario: Text change
- **WHEN** the tooltip is shown and the text signal changes
- **THEN** the tooltip shows the new text

### Requirement: Cleanup
When the element is removed from the document, the listeners and the shown tooltip SHALL be removed.

#### Scenario: Unmount
- **WHEN** an element with a shown tooltip is unmounted
- **THEN** the tooltip is removed from `body`
