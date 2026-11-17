## ADDED Requirements

### Requirement: Collapse state
The shared collapsing logic in `prime-core` SHALL keep a `collapsed` flag, accepting an optional external `RwSignal<bool>` (a two-way binding) or an initial value for an internal state. Toggling SHALL write the new value to the external signal, if it is set.

#### Scenario: External signal
- **WHEN** an `RwSignal<bool>` is passed and a toggle is called
- **THEN** the value of the signal is inverted

#### Scenario: Internal state
- **WHEN** no signal is passed and the initial value is `true`
- **THEN** the state is collapsed, and a toggle expands it

### Requirement: ARIA attributes of a collapsible region
The logic SHALL derive, from the base `id`, the derived identifiers (of the header, of the content region) and the attribute values: `aria-expanded` and `aria-controls` for the toggle; `role="region"`, `aria-labelledby`, `aria-hidden` and `tabindex="-1"` (only when collapsed) for the region. The values SHALL be the same during SSR and on the client.

#### Scenario: Expanded
- **WHEN** `collapsed = false`
- **THEN** `aria-expanded="true"`, `aria-hidden="false"`, and the region has no `tabindex`

#### Scenario: Collapsed
- **WHEN** `collapsed = true`
- **THEN** `aria-expanded="false"`, `aria-hidden="true"`, `tabindex="-1"`

### Requirement: Animation and events of collapsing
On the client, when the state changes, the logic SHALL set on the region the class `p-collapsible-enter-active` or `p-collapsible-leave-active` until the `animationend` event, then remove it and call the completion callback. If no animation starts (no `animationend`, for example with `prefers-reduced-motion` or a hidden element), the completion SHALL still happen, so that `on_after_toggle` is not lost.

#### Scenario: Completion without animation
- **WHEN** the animation is disabled or lasts 0
- **THEN** the completion callback is called, and the class does not remain on the element

### Requirement: Unavailability of hidden content
When the state changes, the logic SHALL set `tabindex="-1"` on the focusable descendants of the region when collapsing, and remove it when expanding.

#### Scenario: Expanding
- **WHEN** the region expands
- **THEN** the attribute set by the logic is removed from the descendants
