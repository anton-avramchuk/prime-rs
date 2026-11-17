# stepper Specification

## Purpose
Stepper guides the user through numbered steps: it shows a list of steps with separators, the active content panel, supports free and linear modes and a vertical layout.

## Requirements

### Requirement: Root and value
The `Stepper` of the `prime-stepper` crate SHALL render `div.p-stepper.p-component[role=tablist]` with `id` (the `id` property or `pn_id_<N>`), `style_class`, `style` and `pt`; with `linear` the root SHALL get the class `p-readonly`. The active step SHALL be determined by the two-way signal `value` (`RwSignal<Option<u32>>`); without it the component keeps its own state starting at `default_value`. An external change of `value` SHALL switch the active step and panel.

#### Scenario: Initial value
- **WHEN** `default_value = 2` is set
- **THEN** the step and panel with value 2 are active

#### Scenario: Linear mode
- **WHEN** `linear` is set
- **THEN** the root has the class `p-readonly`

### Requirement: Steps
`StepList` SHALL render `div.p-steplist`. A `Step` with value `value` SHALL render `div.p-step[role=presentation]` with the classes `p-step-active` (active) and `p-disabled` (unavailable), `aria-current="step"` on the active one, `data-p-active`, `data-p-disabled` and a `button.p-step-header[role=tab][type=button]` button with `id` `<id>_step_<value>`, `aria-controls` `<id>_steppanel_<value>`, a `span.p-step-number` number (the step value) and a `span.p-step-title` title (the step content). A step SHALL be considered unavailable if it is not active and (`Stepper.linear` or `Step.disabled`); on an unavailable step the button SHALL have the `disabled` attribute and `tabindex="-1"`.

#### Scenario: Step markup
- **WHEN** `Step value=1` with the title "Header I" is rendered in the active state
- **THEN** there is `.p-step.p-step-active`, a `button[role=tab]` with `.p-step-number` "1" and `.p-step-title` "Header I", the step root has `aria-current="step"`

#### Scenario: Unavailable step
- **WHEN** an inactive `Step disabled=true`
- **THEN** the button has `disabled` and `tabindex="-1"`, the step root has the class `p-disabled`

### Requirement: Separators
After each list step except the last, a `div.p-stepper-separator` SHALL be rendered. For the vertical layout the separator SHALL be rendered in the `StepPanel` (in `div.p-steppanel-content-wrapper` before `div.p-steppanel-content`) for each `StepItem` except the last; a `Step` inside a `StepItem` renders no separator.

#### Scenario: Three steps
- **WHEN** `StepList` has three steps
- **THEN** two separators are rendered

### Requirement: Step switching
A click on an available step SHALL write its value to `value`. A click on an unavailable step (button `disabled`) SHALL change nothing. `StepContext::activate(value)` SHALL write the given value to `value` regardless of `linear` (linear mode blocks only clicks on steps).

#### Scenario: Step click
- **WHEN** the user clicks step 3 in free mode
- **THEN** `value` equals 3, step 3 is active

#### Scenario: Linear mode
- **WHEN** `linear`, step 1 is active and the user clicks step 3
- **THEN** `value` stays 1

#### Scenario: Programmatic transition
- **WHEN** `activate(2)` is called in linear mode
- **THEN** `value` equals 2

### Requirement: Panels
`StepPanels` SHALL render `div.p-steppanels`. A `StepPanel` with value `value` SHALL render `div.p-steppanel[role=tabpanel]` with `id` `<id>_steppanel_<value>`, `aria-controls` `<id>_step_<value>`, `data-p-active`, `data-pc-name="steppanel"`. In the horizontal layout the content (`div.p-steppanel-content-wrapper` > `div.p-steppanel-content`) SHALL be rendered only for the active panel. The content SHALL be set by child elements or by the `content` property.

#### Scenario: One panel
- **WHEN** step 2 is active in the horizontal layout
- **THEN** only panel 2 has content

### Requirement: Vertical layout
A `StepItem` with value `value` SHALL render `div.p-stepitem` (`p-stepitem-active` on the active one, `data-p-active`) and pass its value to the nested `Step` and `StepPanel` that have no value of their own. In the vertical layout the panel SHALL get the class `p-steppanel-active` when active; all panels SHALL stay in the markup, inactive ones hidden, and expansion and collapse SHALL be animated by the theme classes `p-collapsible-enter-active` and `p-collapsible-leave-active`.

#### Scenario: Vertical list
- **WHEN** three `StepItem`s, the second is active
- **THEN** `.p-stepitem-active` only on the second, its panel has `p-steppanel-active`

### Requirement: Custom views
`content` of a `Step` SHALL replace the header button (and receive `StepContext { value, active, activate }`), and `content` of a `StepPanel` SHALL replace the content of `div.p-steppanel-content`. `activate` SHALL be callable without tying it to a step click, for example from "Next"/"Back" buttons.

#### Scenario: Custom step
- **WHEN** `content` is set on a `Step`
- **THEN** inside `.p-step` instead of `button.p-step-header` is the passed view

### Requirement: Rendering without a browser
Outside the browser (SSR) `Stepper` SHALL render fully (steps, separators, the active panel); `window` and `document` SHALL NOT be used.

#### Scenario: SSR
- **WHEN** a page with a `Stepper` is rendered on the server
- **THEN** the markup has the step list, separators and the content of the active panel
