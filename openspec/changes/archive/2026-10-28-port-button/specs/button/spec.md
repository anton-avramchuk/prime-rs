## ADDED Requirements

### Requirement: Button properties
The `Button` component of the `prime-button` crate SHALL accept the same properties as `p-button` in PrimeNG 21.1.10: `label`, `icon`, `icon_pos` (`left`, `right`, `top`, `bottom`, default `left`), `loading`, `loading_icon`, `severity` (`secondary`, `success`, `info`, `warn`, `help`, `danger`, `contrast`), `variant` (`outlined`, `text`), `outlined`, `text`, `plain`, `link`, `raised`, `rounded`, `size` (`small`, `large`), `fluid`, `badge`, `badge_severity` (default `secondary`), `disabled`, `button_type` (the `type` attribute, default `button`), `aria_label`, `tabindex`, `autofocus`, `style`, `style_class`. The content is passed through `children`. All properties are optional; those that can change at runtime SHALL be reactive.

#### Scenario: Default button
- **WHEN** `Button` is rendered without properties
- **THEN** an element `<button type="button">` is created with the classes `p-button p-component p-ripple`

#### Scenario: Label
- **WHEN** `label = "Save"` is set
- **THEN** the button contains `<span class="p-button-label">Save</span>`

### Requirement: Root classes and attributes of Button
The root `<button>` SHALL have the class `p-button p-component` and the modifiers as in PrimeNG: `p-button-icon-only` (there is an icon, no label and no badge), `p-button-vertical` (`icon_pos` is `top` or `bottom` and there is a label), `p-button-loading`, `p-button-link`, `p-button-<severity>`, `p-button-raised`, `p-button-rounded`, `p-button-text` (`text` or `variant = text`), `p-button-outlined` (`outlined` or `variant = outlined`), `p-button-sm`, `p-button-lg`, `p-button-plain`, `p-button-fluid`. The attributes `data-p` (flags separated by spaces), `data-p-disabled` and `data-p-severity` SHALL be computed in the same way as in PrimeNG. Additional `style_class` and `style` SHALL be added without losing the own classes.

#### Scenario: Severity and variant
- **WHEN** `severity = danger`, `variant = outlined`, `size = small` are set
- **THEN** the root has the classes `p-button-danger`, `p-button-outlined`, `p-button-sm`, and `data-p-severity` equals `danger`

#### Scenario: Icon only
- **WHEN** an icon is set but there is no label and no badge
- **THEN** the root has `p-button-icon-only`, and `data-p` contains `icon-only`

#### Scenario: Vertical layout
- **WHEN** `icon_pos = top`, and an icon and a label are set
- **THEN** the root has `p-button-vertical`

#### Scenario: Custom class and style
- **WHEN** `style_class = "custom"` and `style = "margin: 4px"` are set
- **THEN** the root has the class `custom`, the own classes remain, and the `style` attribute equals `margin: 4px`

### Requirement: Disabled and loading states
The button SHALL be unavailable (`disabled`) if `disabled` or `loading` is set, and then `data-p-disabled` equals `true`. When `loading` is set, a loading indicator SHALL be shown instead of the icon: the value of `loading_icon` if it is set, otherwise `SpinnerIcon` with rotation and the classes `p-button-loading-icon`; the indicator has `aria-hidden="true"`.

#### Scenario: Loading
- **WHEN** `loading = true`
- **THEN** the button is unavailable, `p-button-loading` is present, the `p-button-loading-icon` indicator is visible, and there is no regular icon

#### Scenario: Custom loading indicator
- **WHEN** `loading = true` and `loading_icon` is set
- **THEN** the passed content is rendered instead of `SpinnerIcon`, inside a wrapper with the class `p-button-loading-icon`

#### Scenario: Unavailable button
- **WHEN** `disabled = true`
- **THEN** the button has the `disabled` attribute, `data-p-disabled="true"`, and it does not call `on_click`

### Requirement: Icon, label, badge and content
The icon (`icon`: Leptos content) SHALL be rendered in a `<span>` with the class `p-button-icon` and a position modifier (`p-button-icon-left`, `-right`, `-top`, `-bottom`; a button without a label has no position modifier). If `badge` is set and there are no `children`, a `Badge` SHALL be rendered with `value = badge` and `severity = badge_severity`. `children`, if set, SHALL be rendered first, as projected content in PrimeNG; in that case the label and the badge are not shown.

#### Scenario: Icon position
- **WHEN** `icon_pos = right`, and an icon and a label are set
- **THEN** the icon has `p-button-icon-right`

#### Scenario: Badge
- **WHEN** `badge = "3"`, `badge_severity = danger`
- **THEN** the button contains a `Badge` with the value `3` and the class `p-badge-danger`

#### Scenario: Content instead of the label
- **WHEN** `children` and `label` are set
- **THEN** `children` are rendered, and `p-button-label` and the badge are absent

### Requirement: Events and focus
The button SHALL call `on_click`, `on_focus`, `on_blur` with the original events. With `autofocus = true` the button SHALL receive focus after being mounted in the browser; on the server no focus is set.

#### Scenario: Click
- **WHEN** the user clicks an available button
- **THEN** `on_click` is called exactly once with a `MouseEvent`

#### Scenario: Autofocus
- **WHEN** a button with `autofocus = true` is mounted in the browser
- **THEN** `document.activeElement` is this button

### Requirement: Button accessibility
The button SHALL be a native `<button>` with keyboard support (Enter and Space), and SHALL pass `aria_label` as `aria-label` and `tabindex` as `tabindex`. Loading indicators and an icon without a label must not be duplicated in the accessible name.

#### Scenario: aria-label and tabindex
- **WHEN** `aria_label = "Close"` and `tabindex = 2` are set
- **THEN** the `<button>` has the attributes `aria-label="Close"` and `tabindex="2"`

### Requirement: Ripple
The root SHALL have the class `p-ripple`. If `ripple` is enabled in `PrimeConfig`, after mounting in the browser an element `p-ink` SHALL be created, and it is activated on `mousedown`. If `ripple` is disabled, there is no `p-ink` element.

#### Scenario: Ripple enabled
- **WHEN** `PrimeConfig.ripple = true` and the user presses the button with the mouse
- **THEN** the button contains `span.p-ink` with `data-p-ink-active="true"`

#### Scenario: Ripple disabled
- **WHEN** `PrimeConfig.ripple = false`
- **THEN** there is no `p-ink` element

### Requirement: Pass-through Button
`Button` SHALL accept `pt` for the root and for the parts `label`, `icon`, `loading_icon` and `badge`; the classes and attributes from `pt` are added to the own ones, without replacing them.

#### Scenario: pt on a part
- **WHEN** `pt` with the class `extra` is passed for the label
- **THEN** `p-button-label` has both `p-button-label` and `extra`

### Requirement: Conformance with PrimeNG
The DOM and classes of `Button` SHALL match PrimeNG: inside the Angular host `<p-button>` there is a `<button>`; in Leptos only the `<button>` is rendered, because the host element in PrimeNG carries no classes and styles. The Button page in the showcase SHALL repeat the PrimeNG demos, and the weekly screenshot comparison SHALL include it.

#### Scenario: DOM structure
- **WHEN** a button with an icon, a label and a badge is rendered
- **THEN** the order of the children of `<button>` is: icon, label, badge

### Requirement: Button performance
The rendering of `Button` SHALL have the scenarios `button-1` and `button-1000` in `render-bench`, and the wasm size contribution SHALL have the budget `case-button` in `benches/budgets.toml`. The root classes SHALL be recalculated only when the incoming properties change.

#### Scenario: Budget
- **WHEN** `cargo xtask wasm-size` is run
- **THEN** the contribution of `case-button` does not exceed the budget

#### Scenario: Reactivity without extra updates
- **WHEN** a property that does not affect the classes changes
- **THEN** the `class` attribute of the root is not rewritten
