# timeline Specification

## Purpose
Timeline shows a sequence of events along a vertical or horizontal line with markers, connectors and labels on both sides of the line.

## Requirements

### Requirement: Timeline root
The `Timeline` component of the `prime-timeline` crate SHALL render a root with the classes `p-timeline p-component p-timeline-<align> p-timeline-<layout>`, where `align` is `left` (default), `right`, `top`, `bottom` or `alternate`, and `layout` is `vertical` (default) or `horizontal`. The `data-p` attribute SHALL contain `layout` and `align` separated by a space. `style_class` SHALL be added after the own classes, `style` — set the inline style, `pt.root` — add classes, style and attributes.

#### Scenario: Defaults
- **WHEN** `align` and `layout` are not set
- **THEN** root classes `p-timeline p-component p-timeline-left p-timeline-vertical`, `data-p="vertical left"`

#### Scenario: Horizontal layout
- **WHEN** `layout = Horizontal`, `align = Top`
- **THEN** the root classes contain `p-timeline-top` and `p-timeline-horizontal`

#### Scenario: Reactive properties
- **WHEN** `align` is passed as a signal and changes from `Left` to `Right`
- **THEN** the root class is replaced without recreating the events

### Requirement: Timeline events
For each `value` item in the source order a `div.p-timeline-event` SHALL be rendered with three parts: `p-timeline-event-opposite`, `p-timeline-event-separator` and `p-timeline-event-content`. The separator SHALL have the `p-timeline-event-marker` marker and, for all events except the last, the `p-timeline-event-connector` connector. Each part SHALL have the root's `data-p` and accept `pt`.

#### Scenario: Structure
- **WHEN** `value` contains three events
- **THEN** three `p-timeline-event` are rendered, each with three parts, three markers and two connectors in total

#### Scenario: Empty list
- **WHEN** `value` is empty
- **THEN** the root is rendered without events

#### Scenario: pt of the parts
- **WHEN** `pt.event_marker` contains the class `custom`
- **THEN** every marker has the class `custom`

### Requirement: Timeline templates
The `content`, `opposite` and `marker` templates SHALL receive the event and be rendered in the corresponding parts. Without a `content` and `opposite` template the parts SHALL be empty; the `marker` template replaces the standard marker entirely, the connector is kept.

#### Scenario: Content
- **WHEN** a `content` template that renders the event name is set
- **THEN** the name of each event is in its `p-timeline-event-content`

#### Scenario: Custom marker
- **WHEN** a `marker` template is set
- **THEN** the template result is rendered instead of `p-timeline-event-marker`, and the connector stays

### Requirement: Timeline rendering cost
The number of allocations per instance in SSR SHALL fit the `timeline-1000` budget in `benches/alloc-budgets.toml`, the wasm contribution — the `timeline` budget in `benches/budgets.toml`.

#### Scenario: Budgets
- **WHEN** `cargo xtask alloc-budget` and `cargo xtask wasm-size` are run
- **THEN** Timeline values do not exceed the budgets
