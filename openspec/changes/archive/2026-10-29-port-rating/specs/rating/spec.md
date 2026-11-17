## Purpose
Rating lets the user choose a rating from 1 up to a given number of stars, shows it and reports changes.

## ADDED Requirements

### Requirement: Value and events of Rating
The `Rating` component of the `prime-rating` crate SHALL accept a two-way `value` (`RwSignal<Option<u32>>`); without it, it SHALL keep an internal state starting from `default_value`. Choosing a star SHALL set its number (from 1), and a repeated choice of the star equal to the current value SHALL reset the value to `None`; each such choice SHALL call `on_rate` once with the new value and the original event. With `readonly` or `disabled` the value and `on_rate` SHALL not change. `on_focus` and `on_blur` SHALL be called when a star receives and loses focus.

#### Scenario: Choice
- **WHEN** the value is `None` and the user clicks the third star
- **THEN** the value becomes `Some(3)`, and `on_rate` is called once with the value `Some(3)`

#### Scenario: Reset
- **WHEN** the value is `Some(3)` and the user clicks the third star
- **THEN** the value becomes `None`, and `on_rate` is called with the value `None`

#### Scenario: Read-only
- **WHEN** `readonly` or `disabled` is set and the user clicks a star
- **THEN** the value and `on_rate` are not affected

### Requirement: Keyboard
Each star SHALL contain a native radio button of one group (a shared `name`, unique by default), so the arrows SHALL switch the value between the stars, and a choice from the keyboard SHALL work like a mouse choice. Only the radio button of the star equal to the value SHALL be `checked`.

#### Scenario: Arrows
- **WHEN** the focus is on the radio button of the first star and the right arrow is pressed
- **THEN** the value becomes `Some(2)`, and `on_rate` is called once

#### Scenario: Group
- **WHEN** `name` is not set
- **THEN** all radio buttons of the component have the same unique `name`, different from the `name` of another instance

### Requirement: Structure and classes of Rating
The root SHALL be `<div class="p-rating">` with the modifiers `p-readonly` and `p-disabled`, and `data-p` (`readonly`, `disabled` separated by spaces). The number of stars SHALL equal `stars` (default 5). Each star SHALL be `div.p-rating-option` with `p-rating-option-active` if its number is not greater than the value, and `p-focus-visible` if it has keyboard focus; inside it SHALL come `span.p-hidden-accessible` with `input[type=radio]` and an icon: the on icon (number not greater than the value) with the class `p-rating-icon p-rating-on-icon`, otherwise `p-rating-icon p-rating-off-icon`, and `p-invalid` on both when `invalid`. By default the icons are the filled and the outlined star; `on_icon` and `off_icon` replace them, and `on_icon_template` and `off_icon_template` SHALL receive the star number and the icon class.

#### Scenario: Value
- **WHEN** `stars = 5` and the value is `Some(3)`
- **THEN** the first three stars are active and show the on icon, the remaining two show the off icon

#### Scenario: Number of stars
- **WHEN** `stars = 10` is set
- **THEN** ten stars are rendered

#### Scenario: Invalid value
- **WHEN** `invalid` is set
- **THEN** each icon has the class `p-invalid`

### Requirement: Attributes of Rating
The radio button of the star with number `n` SHALL have `value="n"`, `name`, `required` (only with `required`), `readonly` and `disabled` (only with the corresponding properties) and an `aria-label` from the locale: "1 star" for the first star and "{n} stars" for the others (keys `aria.star` and `aria.stars`).

#### Scenario: Labels
- **WHEN** the default locale is used
- **THEN** the labels of the radio buttons are "1 star", "2 stars", "3 stars", and so on

#### Scenario: Unavailable
- **WHEN** `disabled` is set
- **THEN** the root has `p-disabled`, and each radio button has `disabled`

### Requirement: Properties and pass-through of Rating
The properties `stars`, `readonly`, `disabled`, `invalid`, `required`, `name`, `style`, `style_class` SHALL accept a value or a signal and update the markup when the signal changes. `pt` SHALL accept the parts `root`, `option`, `hidden_option_input_container`, `hidden_option_input`, `on_icon`, `off_icon`, adding classes, style and attributes without replacing the own ones.

#### Scenario: Reactive number of stars
- **WHEN** `stars` is passed as a signal and changes from 5 to 7
- **THEN** seven stars are rendered

#### Scenario: Class through pt
- **WHEN** the class `custom` is passed in `pt.option`
- **THEN** each star has both `custom` and `p-rating-option`

### Requirement: Rendering cost of Rating
The number of allocations per instance during SSR SHALL fit the budget `rating-1000` in `benches/alloc-budgets.toml`, and the wasm contribution — the budget `rating` in `benches/budgets.toml`.

#### Scenario: Budgets
- **WHEN** `cargo xtask alloc-budget` and `cargo xtask wasm-size` are run
- **THEN** the values of Rating do not exceed the budgets
