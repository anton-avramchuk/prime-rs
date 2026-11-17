# imagecompare Specification

## Purpose
ImageCompare shows two images one above the other and lets the user reveal the upper one, by a slider, to the required share of the width.

## ADDED Requirements

### Requirement: Markup
The `ImageCompare` of the `prime-imagecompare` crate SHALL render `div.p-imagecompare` with `tabindex`, `aria-label`, `aria-labelledby` (if set), `style`, `style_class` and `pt`; inside it — the view `left`, the view `right` and `input[type=range].p-imagecompare-slider` with `min="0"`, `max="100"` and `value="50"`, in exactly this order. The views `left` and `right` SHALL be rendered as passed, without wrappers.

#### Scenario: Composition
- **WHEN** `ImageCompare` with two `<img>` is rendered
- **THEN** the root contains `img`, `img` and `input[type=range]` in order; the slider has `min=0`, `max=100`, `value=50`

#### Scenario: Accessibility
- **WHEN** `tabindex=0` and `aria_label="Compare"` are set
- **THEN** the root has `tabindex="0"` and `aria-label="Compare"`

### Requirement: Slider
Input into the slider SHALL set, on the element directly before it, `clip-path: polygon(0 0, V% 0, V% 100%, 0 100%)`, where `V` is the value of the slider; if the slider or its ancestor has `dir="rtl"`, `polygon((100−V)% 0, 100% 0, 100% 100%, (100−V)% 100%)` SHALL be set. Input without a previous sibling SHALL not cause errors.

#### Scenario: Input of a value
- **WHEN** the value of the slider changes to 30
- **THEN** the `clip-path` of the element preceding it equals `polygon(0 0, 30% 0, 30% 100%, 0 100%)`

#### Scenario: Right to left
- **WHEN** the root is inside `dir="rtl"` and the value is 30
- **THEN** `clip-path` equals `polygon(70% 0, 100% 0, 100% 100%, 70% 100%)`

### Requirement: Rendering without a browser
Outside the browser (SSR) `ImageCompare` SHALL render fully; `window` and `document` SHALL not be used.

#### Scenario: SSR
- **WHEN** a page with `ImageCompare` is rendered on the server
- **THEN** the markup contains both images and the slider
