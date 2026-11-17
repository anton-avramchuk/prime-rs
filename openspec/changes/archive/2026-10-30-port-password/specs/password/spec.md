## Purpose
Password — a password input field with a strength indicator, a show toggle and clearing.

## ADDED Requirements

### Requirement: Password markup
The `Password` component of the `prime-password` crate SHALL render the root `span.p-password.p-component.p-inputwrapper` with an `InputText` field (class `p-password-input`, `type=password`). The root SHALL get `p-inputwrapper-filled` for a non-empty value, `p-inputwrapper-focus` on focus, `p-variant-filled` with `variant = Filled`, `p-password-fluid` with `fluid` (or inside `Fluid`) and `data-p`. The properties `placeholder`, `autocomplete`, `input_id`, `max_length`, `disabled`, `readonly`, `invalid`, `required`, `size`, `aria_label`, `input_style`, `input_style_class` SHALL be passed to the field; `style`, `style_class` and `pt` — to the root.

#### Scenario: Default value
- **WHEN** the component is rendered without properties
- **THEN** inside the root there is `input.p-password-input.p-inputtext[type=password]`, no panel

#### Scenario: Filled
- **WHEN** `value` is non-empty
- **THEN** the root has the class `p-inputwrapper-filled`

### Requirement: Two-way value
`value: RwSignal<String>` SHALL be reflected in the field, and user input SHALL be written to the signal. Without `value` the value SHALL be stored inside the component.

#### Scenario: Input
- **WHEN** the user types "abc"
- **THEN** the `value` signal equals "abc"

### Requirement: Show password
With `toggle_mask` the component SHALL render the icon `svg.p-password-toggle-mask-icon.p-password-unmask-icon` (`EyeIcon`), on click of which the field type SHALL become `text` and the icon `svg.p-password-mask-icon` (`EyeSlashIcon`); a repeated click SHALL return `password`. The templates `show_icon` and `hide_icon` SHALL replace the icons. Without `toggle_mask` there are no icons.

#### Scenario: Toggle
- **WHEN** `toggle_mask` is on and the "eye" icon is pressed
- **THEN** the field `type` equals `text`, the icon `p-password-mask-icon` is visible

### Requirement: Clearing
With `show_clear` and a non-empty value the component SHALL render `svg.p-password-clear-icon`; a click SHALL clear the value and call `on_clear`. The template `clear_icon` SHALL replace the icon. With an empty value there is no icon.

#### Scenario: Click on the icon
- **WHEN** `show_clear` is on, the value is "abc", the clear icon is pressed
- **THEN** the value is empty, the icon disappeared, `on_clear` is called

### Requirement: Strength estimate
The estimate SHALL be: empty — a prompt; non-empty — weak; at least 6 characters and two kinds from {lowercase Latin, uppercase Latin, digits} — medium; at least 8 characters and all three kinds — strong. `strength` SHALL replace the estimate (a function `String → PasswordStrength`). The logic SHALL be a pure function with a table-driven test.

#### Scenario: Boundaries
- **WHEN** "a", "abcDEF", "abcDEF1", "abcDEF12" are evaluated
- **THEN** the result is weak, medium, medium, strong

#### Scenario: Empty value
- **WHEN** the value is empty
- **THEN** the estimate is a prompt (no level)

### Requirement: Strength panel
With `feedback` (default `true`) the panel `div.p-password-overlay.p-component` SHALL open on focus and on input if it is closed, and close on focus loss and on `Escape`. Inside — `div.p-password-content` > `div.p-password-meter` > `div.p-password-meter-label` and `div.p-password-meter-text`. The label SHALL get the class `p-password-meter-<weak|medium|strong>` and width 33.33 % / 66.66 % / 100 %; the text — the level label (from `weak_label`, `medium_label`, `strong_label`, `prompt_label` or the locale). The panel SHALL be aligned below the field, have the z-index of the `Overlay` group, and `header`, `footer`, `content` SHALL replace the corresponding parts. With `feedback = false` there is no panel.

#### Scenario: Opening
- **WHEN** the field receives focus with `feedback`
- **THEN** the panel is shown with the text "Enter a password"

#### Scenario: Level
- **WHEN** "abcDEF12" is entered
- **THEN** the label has the class `p-password-meter-strong`, width 100 %, text "Strong"

#### Scenario: Closing
- **WHEN** `Escape` or the field loses focus
- **THEN** the panel is removed

### Requirement: Callbacks and locale
`on_focus` and `on_blur` SHALL be called on focus and focus loss of the field. Labels SHALL by default come from the locale (`passwordPrompt`, `weak`, `medium`, `strong`) and change reactively with it.

#### Scenario: Label from the locale
- **WHEN** the locale overrides `weak`
- **THEN** the panel shows the overridden text for a weak password
