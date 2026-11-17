## ADDED Requirements

### Requirement: Binding a region to a derived state
The collapse logic SHALL allow binding a region to a "collapsed" state that is computed from another source (for example, from the group's list of active values) and is not a writable signal. When such a state changes in the browser, the behaviour SHALL match binding to `CollapseState`: animation with the classes `p-collapsible-enter-active`/`p-collapsible-leave-active`, final hiding, `tabindex="-1"` on focusable descendants of the collapsed region, a call of the completion callback with the new value. The first computation animates nothing.

#### Scenario: Derived state
- **WHEN** the state from which "collapsed" is computed changes
- **THEN** the region animates and is hidden or shown, and the callback receives the new value

#### Scenario: First value
- **WHEN** a region is bound to a derived state, and it is already "collapsed"
- **THEN** there is no animation, descendants get `tabindex="-1"`
