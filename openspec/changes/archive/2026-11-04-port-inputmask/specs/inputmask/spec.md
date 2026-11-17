# inputmask Specification

## Purpose
InputMask — an input field that accepts only the characters that match the mask.

## ADDED Requirements

### Requirement: Mask
The mask SHALL consist of the template characters `9` (a digit), `a` (a letter per `character_pattern`, ASCII by default), `*` (a letter or a digit), the sign `?` (everything after it is optional) and constant characters. The engine SHALL build a buffer (the constant characters and the placeholder `slot_char` in the places of the templates) and check each typed character against the template of its position.

#### Scenario: Buffer
- **WHEN** the mask is `(999) 999-9999`, the placeholder is `_`
- **THEN** the empty buffer is `(___) ___-____`

#### Scenario: Optional part
- **WHEN** the mask is `99-9?99`
- **THEN** the value with three digits `12-3` is considered complete

### Requirement: Markup
`InputMask` SHALL render `span.p-inputmask.p-component.p-inputwrapper` (`p-variant-filled` with the variant `filled`) with `input.p-inputtext.p-component` (the classes `p-filled`, `p-invalid`, the size, `p-inputtext-fluid`; the attributes `type`, `placeholder`, `title`, `autocomplete`, `tabindex`, `aria-label`, `aria-labelledby`, `aria-required`, `readonly`, `disabled`) and, with `show_clear`, a non-empty value and an available field, the icon `p-inputmask-clear-icon`.

#### Scenario: Initial value
- **WHEN** the value is `5551234567` with the mask `(999) 999-9999`
- **THEN** the field contains `(555) 123-4567`

### Requirement: Input
On focus the field SHALL show the mask buffer and put the caret on the first free position (for a completely filled mask — select everything). A printable character matching the template SHALL be inserted with a shift of the already entered characters to the right and move the caret to the next position of the template; a non-matching character SHALL be ignored. `Backspace` and `Delete` SHALL delete a character or the selection with a shift to the left, `Escape` — return the text as of the moment of focus. Paste SHALL parse the text against the mask. Filling the required part SHALL call `on_complete`.

#### Scenario: Typing digits
- **WHEN** the mask is `99-99`, the user types `1`, `2`, `3`, `4`
- **THEN** the field is `12-34`, and `on_complete` is called

#### Scenario: Non-matching character
- **WHEN** the user types a letter in the position of a digit
- **THEN** the text does not change

### Requirement: Losing focus and the value
On losing focus an incomplete value SHALL be cleared (`auto_clear`), otherwise it SHALL remain; with `keep_buffer` the buffer SHALL be kept. The value SHALL be updated after changes: the masked text or the entered characters when `unmask`. The clear icon SHALL reset the value and call `on_clear`.

#### Scenario: Incomplete value
- **WHEN** `12` is entered with the mask `99-99` and the focus is lost
- **THEN** the field is empty, and the value is empty

### Requirement: Unavailability
With `readonly` the input SHALL be ignored; with `disabled` the field SHALL be inactive.

#### Scenario: Read-only
- **WHEN** `readonly` is set and a digit is pressed
- **THEN** the text does not change
