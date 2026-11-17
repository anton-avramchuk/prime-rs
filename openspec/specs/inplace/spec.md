# inplace Specification

## Purpose
Inplace switches an area between a "view" and "content" according to the user's action.

## Requirements

### Requirement: Markup of Inplace
The `Inplace` component of the `prime-inplace` crate SHALL render the root `div.p-inplace.p-component` with `aria-live="polite"`. In the inactive state the root SHALL contain `div.p-inplace-display` with `tabindex="0"`, `role="button"` and the content `display`; with `disabled` — the class `p-disabled` and the attribute `data-p-disabled`. In the active state the root SHALL contain `div.p-inplace-content` with the content `content`. `style`, `style_class` and `pt` SHALL be applied to the root.

#### Scenario: Default view
- **WHEN** the component is rendered without properties
- **THEN** `.p-inplace-display[role=button][tabindex="0"]` is present, and `.p-inplace-content` is absent

#### Scenario: Active state
- **WHEN** `active` equals `true`
- **THEN** `.p-inplace-content` is present, and `.p-inplace-display` is absent

#### Scenario: Unavailable
- **WHEN** `disabled`
- **THEN** the view has the class `p-disabled` and `data-p-disabled`

### Requirement: Activation
A click on the view and the `Enter` key on it SHALL make the component active and call `on_activate`; `Enter` SHALL cancel the default action. A click SHALL be ignored with `prevent_click`. With `disabled` activation SHALL not take place. The two-way `active` signal SHALL reflect the state and allow changing it from outside.

#### Scenario: Click
- **WHEN** the user clicks the view
- **THEN** the content is shown, `on_activate` is called, and the `active` signal equals `true`

#### Scenario: Key
- **WHEN** `Enter` is pressed on the view
- **THEN** the content is shown

#### Scenario: prevent_click
- **WHEN** `prevent_click` is set and the view is clicked
- **THEN** the state does not change

### Requirement: Closing
With `closable` a `Button` (`type=button`, the `TimesIcon` or `close_icon` icon, `aria-label` from `close_aria_label`) SHALL be rendered in the content; a click on it SHALL return the component to the view and call `on_deactivate`. The close function passed to `content` SHALL do the same. With `prevent_click` the close button SHALL do nothing. With `disabled` closing SHALL not take place.

#### Scenario: Close button
- **WHEN** `closable` and the active state are set, and the button is pressed
- **THEN** the view is shown, and `on_deactivate` is called

#### Scenario: Callback in the content
- **WHEN** the content calls the passed close function
- **THEN** the view is shown
