# metergroup Specification

## Purpose
MeterGroup shows a composite meter: several colored segments whose shares are counted from the `min`–`max` range, and labels for them.

## Requirements

### Requirement: MeterGroup root
The `MeterGroup` component of the `prime-metergroup` crate SHALL render a root with `role="meter"`, the classes `p-metergroup p-component p-metergroup-<orientation>` (`horizontal` by default or `vertical`), `aria-valuemin` and `aria-valuemax` (`min`, 0 by default, and `max`, 100 by default), `aria-valuenow` as the total percentage, and `data-p` with the orientation. `style_class` SHALL be added after the own classes, `style` SHALL set the inline style, `pt.root` SHALL add classes, style and attributes.

#### Scenario: Default values
- **WHEN** `MeterGroup` is rendered with no properties other than `value`
- **THEN** the root has `role="meter"`, `aria-valuemin="0"`, `aria-valuemax="100"`, the class `p-metergroup-horizontal`

#### Scenario: Total value
- **WHEN** the segments have values 20 and 30 with `min = 0`, `max = 100`
- **THEN** `aria-valuenow="50"`

#### Scenario: Vertical orientation
- **WHEN** `orientation = Vertical`
- **THEN** the root class is `p-metergroup-vertical`, `data-p="vertical"`

### Requirement: MeterGroup segments
Inside `div.p-metergroup-meters` a `span.p-metergroup-meter` SHALL be rendered for each item with a value greater than zero, in the original order; items with a zero value are skipped. The segment percentage SHALL be computed as `(value - min) / (max - min) * 100`, clamped to 0–100 and rounded to an integer; when `min == max` it is 100. Horizontally the percentage sets the width, vertically the height; the segment color is the background.

#### Scenario: Width
- **WHEN** `min = 0`, `max = 200`, item value 50, color `#f00`
- **THEN** the segment has `width: 25%` and `background-color: #f00`

#### Scenario: Clamping
- **WHEN** an item value is 150 with `max = 100`, and another is -10
- **THEN** the first segment has a width of 100%, the second is not rendered

#### Scenario: Vertical
- **WHEN** `orientation = Vertical`, value 40
- **THEN** the segment has `height: 40%` and has no percentage width

#### Scenario: Custom segment
- **WHEN** the `meter` template is set
- **THEN** the template result is rendered for each item instead of the standard segment

### Requirement: MeterGroup labels
By default an `ol.p-metergroup-label-list` SHALL be rendered (with the `-vertical` or `-horizontal` modifier per `label_orientation`, horizontal by default) with an `li.p-metergroup-label` for each item: a `p-metergroup-label-marker` marker with a background color (or a `p-metergroup-label-icon` icon if the item has an `icon`) and a `p-metergroup-label-text` text of the form `Label (percent%)`. With `label_position = End` (default) the labels SHALL go after the segments, with `Start` before them.

#### Scenario: Label
- **WHEN** an item `{label: "Apps", value: 16, color: "#34d399"}` with the range 0–100
- **THEN** the label contains a marker with the background `#34d399` and the text `Apps (16%)`

#### Scenario: Position
- **WHEN** `label_position = Start`
- **THEN** the label list is before `p-metergroup-meters`

#### Scenario: Label orientation
- **WHEN** `label_orientation = Vertical`
- **THEN** the label list has the class `p-metergroup-label-list-vertical`

#### Scenario: Item icon
- **WHEN** an item has an `icon`
- **THEN** an icon with the item color is rendered instead of the marker

### Requirement: MeterGroup templates
The `label` template SHALL replace the whole label list, `start` and `end` SHALL render before and after the segments, `icon` SHALL replace the label marker. Templates SHALL receive the items, the total percentage and the cumulative percentages.

#### Scenario: Custom labels
- **WHEN** the `label` template is set
- **THEN** the standard label list is not rendered

### Requirement: MeterGroup rendering cost
The number of allocations per instance in SSR SHALL fit the `metergroup-1000` budget in `benches/alloc-budgets.toml`, and the wasm contribution the `metergroup` budget in `benches/budgets.toml`.

#### Scenario: Budgets
- **WHEN** `cargo xtask alloc-budget` and `cargo xtask wasm-size` are run
- **THEN** the MeterGroup values do not exceed the budgets
