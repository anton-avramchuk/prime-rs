# scrolltop Specification

## Purpose
ScrollTop shows a button that returns to the top of the page or of a scrollable parent once the scroll has exceeded a threshold.

## Requirements

### Requirement: ScrollTop markup
The `ScrollTop` component of the `prime-scrolltop` crate SHALL, while it is visible, render a `<button type="button">` of the `Button` component with the class `p-scrolltop`, rounded corners (`rounded`, enabled by default), an icon in `.p-scrolltop-icon` (default `ChevronUpIcon`) and an `aria-label` from the locale `aria.scrollTop`, if `aria_label` is not set. With `target = Parent` the button SHALL get the class `p-scrolltop-sticky`. `style_class` SHALL be added to the classes, `style` — sets the style, `pt` — adds classes, style and attributes to the button. While the scroll has not exceeded the threshold, with `target = Window` the component SHALL render nothing.

#### Scenario: Hidden by default
- **WHEN** a component with `target = Window` is rendered on the server
- **THEN** the markup of the button is absent

#### Scenario: Parent
- **WHEN** `target = Parent`
- **THEN** the button has the class `p-scrolltop-sticky`

#### Scenario: Icon and label
- **WHEN** the button is shown without `icon` and `aria_label`
- **THEN** `.p-scrolltop-icon` contains `ChevronUpIcon`, and `aria-label` equals "Scroll Top"

### Requirement: Visibility of ScrollTop
With `target = Window` the component SHALL subscribe to the `scroll` of the window, with `Parent` — to the `scroll` of the parent element. If the scroll (`scrollY` of the window or `scrollTop` of the parent) is strictly greater than `threshold` (default 400), the button SHALL be shown; otherwise hidden. The scroll SHALL be checked right after the subscription as well. The subscription SHALL be removed when the component is destroyed.

#### Scenario: Below the threshold
- **WHEN** the window is scrolled by 400 or less
- **THEN** there is no button

#### Scenario: Above the threshold
- **WHEN** the window is scrolled by 401 or more
- **THEN** the button is shown

#### Scenario: Return
- **WHEN** the button is shown and the scroll has returned to the threshold or below
- **THEN** the button is hidden

#### Scenario: Parent
- **WHEN** `target = Parent`, `threshold = 100`, and the parent is scrolled by 150
- **THEN** the button is shown

#### Scenario: Page already scrolled
- **WHEN** the component is mounted while the window is scrolled by 800
- **THEN** the button is shown without a new `scroll` event

### Requirement: Action of ScrollTop
A click on the button SHALL scroll the target (the window or the parent) to the top (`top: 0`) with the `behavior` (`Smooth` by default).

#### Scenario: Window scroll
- **WHEN** the window is scrolled by 800 and the user presses the button
- **THEN** the window scrolls to the top, after which the button is hidden

#### Scenario: Keyboard
- **WHEN** the button has focus and the user presses `Enter` or `Space`
- **THEN** the same action is performed as on a click

### Requirement: Position and animation of ScrollTop
With `target = Window` the button SHALL have `position: fixed`, with `Parent` — `position: sticky`; the z-index SHALL be issued from the `Overlay` group when shown and returned when hidden. Appearance and disappearance SHALL be animated with the classes `p-scrolltop-enter-from`/`-active`/`-to` and `p-scrolltop-leave-from`/`-active`/`-to`; after the disappearance animation ends, the button SHALL be hidden.

#### Scenario: Window
- **WHEN** the button is shown with `target = Window`
- **THEN** its `position` equals `fixed`, and the z-index is greater than the base of the `Overlay` group

#### Scenario: Releasing the z-index
- **WHEN** the button is hidden
- **THEN** the issued z-index is returned to the manager

### Requirement: Rendering cost of ScrollTop
The number of allocations per instance during SSR SHALL fit the budget `scrolltop-1000` in `benches/alloc-budgets.toml`, and the wasm contribution — the budget `scrolltop` in `benches/budgets.toml`.

#### Scenario: Budgets
- **WHEN** `cargo xtask alloc-budget` and `cargo xtask wasm-size` are run
- **THEN** the values of ScrollTop do not exceed the budgets
