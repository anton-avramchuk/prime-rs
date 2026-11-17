## Purpose
Timeline shows a sequence of events along a vertical or horizontal line, with markers, connectors and labels on both sides of the line.

## ADDED Requirements

### Requirement: Root of Timeline
The `Timeline` component of the `prime-timeline` crate SHALL render the root with the classes `p-timeline p-component p-timeline-<align> p-timeline-<layout>`, where `align` is `left` (default), `right`, `top`, `bottom` or `alternate`, and `layout` is `vertical` (default) or `horizontal`. The `data-p` attribute SHALL contain `layout` and `align` separated by a space. `style_class` SHALL be added after the own classes, `style` — sets the inline style, `pt.root` — adds classes, style and attributes.

#### Scenario: Default values
- **WHEN** `align` and `layout` are not set
- **THEN** the root classes are `p-timeline p-component p-timeline-left p-timeline-vertical`, `data-p="vertical left"`

#### Scenario: Horizontal layout
- **WHEN** `layout = Horizontal`, `align = Top`
- **THEN** the root classes contain `p-timeline-top` and `p-timeline-horizontal`

#### Scenario: Reactive properties
- **WHEN** `align` is passed as a signal and changes from `Left` to `Right`
- **THEN** the root class is replaced without recreating the events

### Requirement: Events of Timeline
For each `value` element, in the original order, a `div.p-timeline-event` SHALL be rendered with three parts: `p-timeline-event-opposite`, `p-timeline-event-separator` and `p-timeline-event-content`. The separator SHALL contain the marker `p-timeline-event-marker` and, for all events except the last, the connector `p-timeline-event-connector`. Each part SHALL have the `data-p` of the root and accept `pt`.

#### Scenario: Structure
- **WHEN** `value` contains three events
- **THEN** three `p-timeline-event` are rendered, each with three parts, three markers in total and two connectors

#### Scenario: Empty list
- **WHEN** `value` is empty
- **THEN** the root is rendered without events

#### Scenario: pt of parts
- **WHEN** `pt.event_marker` contains the class `custom`
- **THEN** each marker has the class `custom`

### Requirement: Templates of Timeline
The templates `content`, `opposite` and `marker` SHALL receive the event and be rendered in the corresponding parts. Without the `content` and `opposite` templates the parts SHALL be empty; the `marker` template replaces the standard marker entirely, while the connector is kept.

#### Scenario: Content
- **WHEN** the `content` template is set, rendering the event name
- **THEN** the `p-timeline-event-content` of each event contains its name

#### Scenario: Custom marker
- **WHEN** the `marker` template is set
- **THEN** the result of the template is rendered instead of `p-timeline-event-marker`, and the connector remains

### Requirement: Rendering cost of Timeline
The number of allocations per instance during SSR SHALL fit the budget `timeline-1000` in `benches/alloc-budgets.toml`, and the wasm contribution — the budget `timeline` in `benches/budgets.toml`.

#### Scenario: Budgets
- **WHEN** `cargo xtask alloc-budget` and `cargo xtask wasm-size` are run
- **THEN** the values of Timeline do not exceed the budgets
