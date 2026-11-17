# imagecompare Specification

## Purpose
ImageCompare shows two images on top of each other and lets the user, with a slider, reveal the upper one up to the desired fraction of the width.

## Requirements

### Requirement: Markup
`ImageCompare` of the `prime-imagecompare` crate SHALL render `div.p-imagecompare` with `tabindex`, `aria-label`, `aria-labelledby` (if set), `style`, `style_class` and `pt`, inside which — the view `left`, the view `right` and `input[type=range].p-imagecompare-slider` with `min="0"`, `max="100"` and `value="50"`, exactly in this order. The views `left` and `right` SHALL be rendered as passed, without wrappers.

#### Scenario: Composition
- **WHEN** `ImageCompare` with two `<img>` is rendered
- **THEN** the root contains `img`, `img` and `input[type=range]` in order, and the slider has `min=0`, `max=100`, `value=50`

#### Scenario: Accessibility
- **WHEN** `tabindex=0` and `aria_label="Compare"` are set
- **THEN** the root has `tabindex="0"` and `aria-label="Compare"`

### Requirement: Slider
Input on the slider SHALL set, on the element directly in front of it, `clip-path: polygon(0 0, V% 0, V% 100%, 0 100%)`, where `V` is the slider value; if the slider or its ancestor has `dir="rtl"`, `polygon((100−V)% 0, 100% 0, 100% 100%, (100−V)% 100%)` SHALL be set. Input without a previous sibling SHALL not cause errors.

#### Scenario: Entering a value
- **WHEN** the slider value changes to 30
- **THEN** the element preceding it has `clip-path` equal to `polygon(0 0, 30% 0, 30% 100%, 0 100%)`

#### Scenario: Right to left
- **WHEN** the root is inside `dir="rtl"` and the value is 30
- **THEN** `clip-path` equals `polygon(70% 0, 100% 0, 100% 100%, 70% 100%)`

### Requirement: Rendering without a browser
Outside the browser (SSR) `ImageCompare` SHALL be rendered in full; `window` and `document` SHALL not be used.

#### Scenario: SSR
- **WHEN** a page with `ImageCompare` is rendered on the server
- **THEN** the markup contains both images and the slider
