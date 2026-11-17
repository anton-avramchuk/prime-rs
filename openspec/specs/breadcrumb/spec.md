# breadcrumb Specification

## Purpose
Breadcrumb shows the path through the page hierarchy: an optional "home" item, model items and separators between them.

## Requirements

### Requirement: Breadcrumb markup
The `Breadcrumb` component of the `prime-breadcrumb` crate SHALL render `<nav class="p-breadcrumb p-component">` with a single `<ol class="p-breadcrumb-list">`. `style_class` SHALL be appended after the own classes, `style` SHALL set the inline style, `pt.root` and `pt.list` SHALL add classes, style and attributes to the corresponding elements without replacing the own ones.

#### Scenario: Root
- **WHEN** `Breadcrumb` is rendered without properties
- **THEN** `nav.p-breadcrumb.p-component` is rendered with an empty `ol.p-breadcrumb-list`

#### Scenario: Classes and style
- **WHEN** `style_class = "mine"` and `style = "width: 20rem"` are set
- **THEN** the root class is `p-breadcrumb p-component mine`, the style is `width: 20rem;`

#### Scenario: pt
- **WHEN** `pt.root` contains the attribute `data-x="1"` and `pt.list` the class `custom`
- **THEN** `nav` has `data-x="1"` and `ol` has `p-breadcrumb-list custom`

### Requirement: Breadcrumb items
For each model item with `visible` not equal to `false`, an `li.p-breadcrumb-item` SHALL be rendered (with `p-disabled` on disabled ones, the item's `id`, `style_class` and `style`) with an `a.p-breadcrumb-item-link` link. Items SHALL follow the model order.

#### Scenario: Labels
- **WHEN** the model contains the items `Electronics` and `Computer`
- **THEN** two `li.p-breadcrumb-item` are rendered with the labels in that order

#### Scenario: Hidden item
- **WHEN** an item has `visible = false`
- **THEN** it is not rendered, and neither is the separator after it

#### Scenario: Disabled item
- **WHEN** an item has `disabled = true`
- **THEN** the `li` has the class `p-disabled` and the link has no `tabindex`

### Requirement: Breadcrumb item link
The link SHALL have `href` from `url` (no `href` without `url`), `target`, `title`, the item's `tabindex` (`0` by default, no `tabindex` on disabled ones) and the class `link_class`. Inside it SHALL come the `span.p-breadcrumb-item-icon` icon if present, the `span.p-breadcrumb-item-label` label if `label` is non-empty and a `Badge` if `badge` is set.

#### Scenario: Link
- **WHEN** an item has `url = "/a"`, `target = "_blank"`, `title = "T"`
- **THEN** the link has `href="/a"`, `target="_blank"`, `title="T"`, `tabindex="0"`

#### Scenario: Item without url
- **WHEN** an item has no `url`
- **THEN** the link is rendered without the `href` attribute

#### Scenario: Icon and badge
- **WHEN** an item has an icon and `badge = "3"`
- **THEN** inside the link first comes the icon, then the label, then `span.p-badge` with the value `3`

### Requirement: Breadcrumb "home" item
When `home` is set with `visible` not equal to `false`, an `li.p-breadcrumb-home-item` SHALL be rendered first, with a link having the same properties as a regular item and the class `p-breadcrumb-item-link`. Without its own icon a `HomeIcon` with the class `p-breadcrumb-item-icon` SHALL be rendered. The link SHALL have `aria-label` from `home_aria_label` if it is set.

#### Scenario: Default icon
- **WHEN** `home` is set without an icon and a label
- **THEN** the first `li` is `p-breadcrumb-home-item` with the `home` icon

#### Scenario: aria-label
- **WHEN** `home_aria_label = "Home"`
- **THEN** the "home" item link has `aria-label="Home"`

### Requirement: Breadcrumb separators
Between adjacent items a `li.p-breadcrumb-separator` with a `chevron-right` icon (`pt.separator_icon`) SHALL be rendered; after the "home" item the separator SHALL be rendered if there is at least one model item; not after the last item. The `separator` template SHALL replace the standard icon.

#### Scenario: Count
- **WHEN** there are `home` and three model items
- **THEN** three separators are rendered

#### Scenario: Without home
- **WHEN** `home` is not set and the model has three items
- **THEN** two separators are rendered

#### Scenario: Custom separator
- **WHEN** a `separator` template rendering `/` is set
- **THEN** each separator contains `/`, there is no `chevron-right` icon

### Requirement: Breadcrumb item template
The `item` template SHALL replace the content of the item's `li` (including "home"), receiving the item; the standard link is not rendered, while the `li` wrapper with its classes remains.

#### Scenario: Custom item
- **WHEN** an `item` template rendering `<b>` with the label is set
- **THEN** each `li.p-breadcrumb-item` contains `<b>`, there is no `p-breadcrumb-item-link` link

### Requirement: Pressing a Breadcrumb item
When an item's link is pressed the default action SHALL be cancelled if the item is disabled or has no `url`. For a non-disabled item its `command` SHALL be called, then the component's `on_item_click`, both with the event and the item; for a disabled one - neither.

#### Scenario: Command
- **WHEN** the user presses an item without `url` with a `command`
- **THEN** `command` and `on_item_click` are called, no navigation happens

#### Scenario: Disabled
- **WHEN** the user presses a disabled item
- **THEN** neither `command` nor `on_item_click` is called

#### Scenario: Link with url
- **WHEN** the user presses an item with `url`
- **THEN** the default action is not cancelled

### Requirement: Breadcrumb rendering cost
The number of allocations per instance in SSR SHALL fit the `breadcrumb-1000` budget in `benches/alloc-budgets.toml`, the contribution to wasm - the `breadcrumb` budget in `benches/budgets.toml`.

#### Scenario: Budgets
- **WHEN** `cargo xtask alloc-budget` and `cargo xtask wasm-size` are applied
- **THEN** the Breadcrumb values do not exceed the budgets
