# popover Specification

## Purpose
Popover shows a floating panel with arbitrary content next to a trigger element; the panel opens and closes imperatively, closes on a click outside it, the Escape key, scrolling and window resize.

## Requirements

### Requirement: Markup
While the panel is hidden, `Popover` of the `prime-popover` crate SHALL render nothing. A visible panel SHALL be a `div.p-popover.p-component[role=dialog]` with `aria-modal="true"` and a single child `div.p-popover-content` containing `children`. `id`, `style`, `style_class`, `aria_label` (`aria-label`), `aria_labelledby` (`aria-labelledby`) and `pt` SHALL be applied to the panel root.

#### Scenario: Hidden by default
- **WHEN** a `Popover` that was never shown is rendered
- **THEN** there is no `.p-popover` in the markup

#### Scenario: Shown
- **WHEN** `show` is called on a click event
- **THEN** a `.p-popover[role=dialog]` is rendered with the content inside `.p-popover-content`

### Requirement: Show and hide
The control object `PopoverRef` SHALL provide `show`, `hide`, `toggle` and `visible`. `toggle` SHALL show a hidden panel and hide a visible one. The alignment target SHALL be the element on which the event fired (`currentTarget`), or an explicitly passed element. If a visible panel is toggled with a different target, it SHALL hide and immediately show at the new target.

#### Scenario: Toggle
- **WHEN** `toggle` is called twice in a row with the same target
- **THEN** after the first call the panel is visible, after the second it is hidden

#### Scenario: Different target
- **WHEN** the panel is visible at button A and `toggle` is called with button B
- **THEN** the panel stays visible and is aligned to button B

#### Scenario: Explicit target
- **WHEN** `show` is called on a button click with an explicit target — a different element
- **THEN** the panel is aligned to the explicit target, not to the button

### Requirement: Positioning
A visible panel SHALL be aligned to the target in the same way as the `Select` overlay: below the target, and above it when there is not enough room below. When placed above the target the root SHALL get `p-popover-flipped` and `data-p-popover-flipped="true"`. The horizontal arrow shift SHALL be written to the root's `--p-popover-arrow-left` custom property. The layer SHALL be given a `z-index` from the overlay group if `auto_z_index` is on, offset by `base_z_index`.

#### Scenario: Room below
- **WHEN** the target is at the top edge of the window
- **THEN** the panel is below the target and without `p-popover-flipped`

#### Scenario: No room below
- **WHEN** the target is at the bottom edge of the window
- **THEN** the panel is above the target, the root has `p-popover-flipped`

### Requirement: Closing
With `dismissable` a click outside the panel and the target SHALL hide the panel; a click inside the panel — no. The `Escape` key SHALL hide the panel regardless of `dismissable`. A window resize SHALL hide the panel on non-touch devices. A scroll of any scrollable ancestor of the target SHALL hide the panel. After hiding, the window and document listeners SHALL be removed.

#### Scenario: Click outside the panel
- **WHEN** the panel is visible and the user clicks an empty place on the page
- **THEN** the panel is hidden

#### Scenario: Click inside the panel
- **WHEN** the panel is visible and the user clicks its content
- **THEN** the panel stays visible

#### Scenario: Click on the target
- **WHEN** the panel is visible and the user clicks the target
- **THEN** the panel closes once and does not reopen from the same click

#### Scenario: Non-dismissable
- **WHEN** `dismissable` is off and the user clicks outside the panel
- **THEN** the panel stays visible, and hides on Escape

### Requirement: Focus
On show, if `focus_on_show` is on, the first panel element with the `autofocus` attribute SHALL get focus; without such an element focus does not move.

#### Scenario: Element with autofocus
- **WHEN** the content has an `input[autofocus]` and the panel is shown
- **THEN** focus is on that field

### Requirement: Events
`on_show` SHALL be called after the panel is shown and aligned; `on_hide` — after the panel is hidden and removed.

#### Scenario: Event order
- **WHEN** the panel is shown and hidden
- **THEN** `on_show` is called once before `on_hide`, `on_hide` — once

### Requirement: Rendering without a browser
Outside the browser (SSR) `Popover` SHALL render hidden and not access `window` and `document`; with hydration it SHALL NOT diverge from the server markup.

#### Scenario: SSR
- **WHEN** a page with `Popover` is rendered on the server
- **THEN** there is no panel in the markup and hydration passes without errors
