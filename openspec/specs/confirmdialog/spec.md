# confirmdialog Specification

## Purpose
ConfirmDialog shows a modal window with a question and "Yes" and "No" buttons on a confirmation from the shared service.

## Requirements

### Requirement: Showing on confirmation
The `ConfirmDialog` of the `prime-confirmdialog` crate SHALL subscribe to `ConfirmationService` and show the window when a `Confirm` arrives with a `key` equal to its `key` (both unset are also equal); a confirmation with a different `key` SHALL be ignored. The subscription SHALL be dropped on unmount. The window SHALL be a `Dialog` with the `alertdialog` role, the `p-confirmdialog` class on the window root and the `mask_style_class` class on the mask. While there is no confirmation, there is no window in the markup; on the server there never is. A confirmation that arrives while the window is open SHALL replace the shown one in place.

#### Scenario: Own key
- **WHEN** a confirmation with `key = "a"` is passed to the service, and the `ConfirmDialog` has `key = "a"`
- **THEN** the window is shown with the confirmation text

#### Scenario: Foreign key
- **WHEN** a confirmation arrives with a different `key`
- **THEN** this `ConfirmDialog`'s window is not shown

### Requirement: Content and buttons
The window body SHALL contain the `p-confirmdialog-icon` icon (if `icon` is set) and the `p-confirmdialog-message` text. The header SHALL be `header`, the footer — the reject button `p-confirmdialog-reject-button` and the accept button `p-confirmdialog-accept-button` (in that order). Values SHALL be taken from the confirmation, and if it does not set them — from the same-named component props. Button labels SHALL be chosen in the order: `*_label`, `*_button_props.label`, the `reject`/`accept` translation; `reject_visible = false` and `accept_visible = false` SHALL remove the buttons; `*_button_props` and `*_button_style_class` SHALL be applied to the buttons. The views `header_content`, `footer`, `message_content`, `icon_content`, `headless` SHALL replace the header, the footer, the icon with the text, the icon and the whole window content respectively; `accept_icon_content` and `reject_icon_content` — the button icons.

#### Scenario: Composition
- **WHEN** a confirmation with `header` and `message` is shown
- **THEN** there are a header, text and the "No" and "Yes" buttons

#### Scenario: Label from button props
- **WHEN** the confirmation has `accept_button_props.label = "Delete"` and no `accept_label`
- **THEN** the accept button is labeled "Delete"

### Requirement: Response and closing
The accept button SHALL call the confirmation's `accept`, the reject button — `reject`, and hide the window. Closing with the close icon, the `Escape` key (if `close_on_escape` is not `false`) or a click on the mask (if `dismissable_mask` is true) SHALL call `reject` and hide the window. Service `Close` SHALL hide the window without calling `accept` and `reject`. On every hide `on_hide` SHALL be called with the `ConfirmEventType` type (`Accept`, `Reject`, `Cancel` respectively, and for `Close` — without a type). After the exit ends the confirmation SHALL be forgotten; a repeated response to an already hidden window SHALL NOT call callbacks.

#### Scenario: Accept
- **WHEN** the accept button is pressed
- **THEN** `accept` is called, the window is hidden, `on_hide` received `Accept`

#### Scenario: Close icon
- **WHEN** the window is closed with the close icon
- **THEN** `reject` is called, `on_hide` received `Cancel`

#### Scenario: Service Close
- **WHEN** `ConfirmationService::close` is called
- **THEN** the window is hidden, `accept` and `reject` are not called

### Requirement: Window parameters and focus
The props `modal`, `position`, `closable`, `block_scroll`, `draggable`, `focus_trap`, `auto_z_index`, `base_z_index`, `breakpoints`, `style`, `style_class`, `close_aria_label` SHALL be passed to `Dialog`, and the confirmation's `modal`, `position`, `closable`, `block_scroll`, `close_on_escape`, `dismissable_mask` values SHALL take priority over the component props. The position SHALL be parsed from a string (`center`, `top`, `bottom`, `left`, `right`, `topleft`, `topright`, `bottomleft`, `bottomright`); an unknown string SHALL give `center`. The window SHALL be of fixed size and not maximizable. On show focus SHALL move to the button according to `default_focus` (`Accept` by default, `Reject`, `Close` — the window close button, `None` — focus does not change).

#### Scenario: Position
- **WHEN** the confirmation has `position = "bottomright"`
- **THEN** the window is shown in the bottom right corner

#### Scenario: Default focus
- **WHEN** the window is shown without `default_focus`
- **THEN** focus is on the accept button
