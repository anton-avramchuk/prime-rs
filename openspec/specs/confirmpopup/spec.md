# confirmpopup Specification

## Purpose
ConfirmPopup shows a question with "Yes" and "No" buttons next to the element that triggered the action.

## Requirements

### Requirement: Show on confirmation
`ConfirmPopup` of the `prime-confirmpopup` crate SHALL subscribe to `ConfirmationService` and show the panel when a `Confirm` arrives with a `key` equal to its own `key` (both unset also counts as equal); a confirmation with a different `key` SHALL be ignored. `Close` SHALL hide the panel. While the panel is not shown it is absent from the markup; on the server there is never a panel. The subscription SHALL be dropped on unmount. If the `visible` property is set, it SHALL determine the show instead of the internal state.

#### Scenario: Own key
- **WHEN** the service is given a confirmation with `key = "a"` and the `ConfirmPopup` has `key = "a"`
- **THEN** the panel is shown at the confirmation `target`

#### Scenario: Foreign key
- **WHEN** a confirmation arrives with `key = "b"`
- **THEN** the panel of this `ConfirmPopup` is not shown

### Requirement: Panel markup
The panel SHALL be rendered as `div.p-confirmpopup.p-component[role=alertdialog]` (`style`, `style_class`, `pt`) with content `div.p-confirmpopup-content`: an icon `p-confirmpopup-icon` (if `icon` is set) and `span.p-confirmpopup-message` with the `message` text; then `div.p-confirmpopup-footer` with the reject button `p-confirmpopup-reject-button` and the accept button `p-confirmpopup-accept-button` of size `small`. Button labels SHALL be `reject_label`/`accept_label`, and without them the `reject` and `accept` translations; `reject_visible = false` and `accept_visible = false` SHALL remove the buttons. The `reject_button_props` and `accept_button_props` properties SHALL be applied to the buttons (`label`, `severity`, `outlined`, `text`, `size`); `reject_button_style_class` and `accept_button_style_class` SHALL add a class. The `content` view SHALL replace the content of the `p-confirmpopup-content` block, `headless` the whole panel content, `accept_icon` and `reject_icon` the button icons.

#### Scenario: Composition
- **WHEN** a confirmation with `message` is shown
- **THEN** there is a content block with the text and a footer with two buttons "No" and "Yes"

#### Scenario: Hidden button
- **WHEN** `reject_visible = false`
- **THEN** there is no reject button

### Requirement: Position and arrow
The panel SHALL be aligned to the confirmation `target` with an overlay group layer (`auto_z_index`, `base_z_index`). The arrow shift SHALL be written to `--p-confirmpopup-arrow-left` (the difference of the left edges of the target and the panel if the panel is left of the target, otherwise `0`). If the panel ends up above the target, `data-p-confirmpopup-flipped="true"` and the `p-confirm-popup-flipped` class SHALL be set. A repeated confirmation with a different target while the panel is shown SHALL move the panel to the new target.

#### Scenario: Below the target
- **WHEN** the target has enough room below
- **THEN** the panel is below the target without the `p-confirm-popup-flipped` class

#### Scenario: Retargeting
- **WHEN** while the panel is shown a confirmation with a different target arrives
- **THEN** the panel is aligned to the new target

### Requirement: Accept and reject
The accept button SHALL call the confirmation `accept`, the reject button its `reject`, then hide the panel and return focus to `target`. On show, focus SHALL move to the button according to the confirmation `default_focus`, and without it the component `default_focus` property (`Accept` by default; `Reject` is the reject button; `None`/`Close` leave focus unchanged).

#### Scenario: Accept
- **WHEN** the accept button is pressed
- **THEN** `accept` is called, the panel is hidden, focus is on the target

#### Scenario: Default focus
- **WHEN** the panel is shown without `default_focus`
- **THEN** focus is on the accept button

### Requirement: Closing
The `Escape` key SHALL call `reject` and hide the panel unless `close_on_escape` is `false`. A click outside the panel and the target SHALL hide the panel (without `accept`/`reject`) unless `dismissable_mask` is `false`. A window resize (except on touch devices) and scrolling of a target ancestor SHALL hide the panel. Listeners SHALL be attached while shown (the click on the next tick, so as not to catch the click that triggered the confirmation) and removed on hide and unmount. `on_show` SHALL be called on show and `on_hide` after hide.

#### Scenario: Escape
- **WHEN** the panel is shown and `Escape` is pressed
- **THEN** `reject` is called, the panel is hidden

#### Scenario: Outside click
- **WHEN** the user clicks outside the panel and the target
- **THEN** the panel is hidden, `accept` and `reject` are not called

#### Scenario: Closing disabled
- **WHEN** the confirmation has `dismissable_mask = false`
- **THEN** an outside click does not hide the panel
