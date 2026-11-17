## Purpose
MeterGroup shows a composite scale: several coloured segments whose shares are measured from the range `min`–`max`, and labels for them.

## ADDED Requirements

### Requirement: Root of MeterGroup
The `MeterGroup` component of the `prime-metergroup` crate SHALL render the root with `role="meter"`, the classes `p-metergroup p-component p-metergroup-<orientation>` (`horizontal` by default or `vertical`), `aria-valuemin` and `aria-valuemax` (`min`, default 0, and `max`, default 100), `aria-valuenow` — the total percentage, and `data-p` with the orientation. `style_class` SHALL be added after the own classes, `style` — sets the inline style, `pt.root` — adds classes, style and attributes.

#### Scenario: Default values
- **WHEN** `MeterGroup` is rendered without properties, except `value`
- **THEN** the root has `role="meter"`, `aria-valuemin="0"`, `aria-valuemax="100"`, the class `p-metergroup-horizontal`

#### Scenario: Total value
- **WHEN** the segments have the values 20 and 30 with `min = 0`, `max = 100`
- **THEN** `aria-valuenow="50"`

#### Scenario: Vertical orientation
- **WHEN** `orientation = Vertical`
- **THEN** the root class is `p-metergroup-vertical`, `data-p="vertical"`

### Requirement: Segments of MeterGroup
Inside `div.p-metergroup-meters` a `span.p-metergroup-meter` SHALL be rendered for each item with a value greater than zero, in the original order; items with a zero value are skipped. The percentage of a segment SHALL be computed as `(value - min) / (max - min) * 100`, limited to the range 0–100 and rounded to an integer; with `min == max` it equals 100. Horizontally the percentage sets the width, vertically — the height; the colour of the segment is its background.

#### Scenario: Width
- **WHEN** `min = 0`, `max = 200`, the item value is 50, the colour is `#f00`
- **THEN** the segment has `width: 25%` and `background-color: #f00`

#### Scenario: Limit
- **WHEN** the value of an item is 150 with `max = 100`, and another is −10
- **THEN** the first segment has the width 100%, the second is not rendered

#### Scenario: Vertically
- **WHEN** `orientation = Vertical`, the value is 40
- **THEN** the segment has `height: 40%` and has no percentage width

#### Scenario: Custom segment
- **WHEN** the `meter` template is set
- **THEN** instead of the standard segment, the result of the template is rendered for each item

### Requirement: Labels of MeterGroup
By default `ol.p-metergroup-label-list` SHALL be rendered (with the modifier `-vertical` or `-horizontal` according to `label_orientation`, horizontal by default) with `li.p-metergroup-label` for each item: a marker `p-metergroup-label-marker` with the background colour (or the icon `p-metergroup-label-icon`, if the item has an `icon`) and the text `p-metergroup-label-text` of the form `Label (percentage%)`. With `label_position = End` (the default) the labels SHALL come after the segments, with `Start` — before them.

#### Scenario: Label
- **WHEN** the item `{label: "Apps", value: 16, color: "#34d399"}` with the range 0–100
- **THEN** the label contains a marker with the background `#34d399` and the text `Apps (16%)`

#### Scenario: Position
- **WHEN** `label_position = Start`
- **THEN** the list of labels is before `p-metergroup-meters`

#### Scenario: Orientation of the labels
- **WHEN** `label_orientation = Vertical`
- **THEN** the list of labels has the class `p-metergroup-label-list-vertical`

#### Scenario: Icon of an item
- **WHEN** the item has an `icon` set
- **THEN** instead of the marker an icon with the colour of the item is rendered

### Requirement: Templates of MeterGroup
The `label` template SHALL replace the whole list of labels, `start` and `end` — rendered before and after the segments, `icon` — replaces the marker of a label. The templates SHALL receive the items, the total percentage and the cumulative percentages.

#### Scenario: Custom labels
- **WHEN** the `label` template is set
- **THEN** the standard list of labels is not rendered

### Requirement: Rendering cost of MeterGroup
The number of allocations per instance during SSR SHALL fit the budget `metergroup-1000` in `benches/alloc-budgets.toml`, and the wasm contribution — the budget `metergroup` in `benches/budgets.toml`.

#### Scenario: Budgets
- **WHEN** `cargo xtask alloc-budget` and `cargo xtask wasm-size` are run
- **THEN** the values of MeterGroup do not exceed the budgets
