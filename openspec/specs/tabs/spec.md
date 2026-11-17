# tabs Specification

## Purpose
Tabs switches views with tabs, with keyboard navigation, an active tab indicator and scrolling.

## Requirements

### Requirement: Tabs markup
The `Tabs` of the `prime-tabs` crate SHALL render `div.p-tabs.p-component` (with `p-tabs-scrollable` when `scrollable`) and an `id` (unique by default). `TabList` SHALL render `div.p-tablist` with `div.p-tablist-content.p-tablist-viewport` > `div.p-tablist-tab-list[role=tablist]` containing the tabs and `span.p-tablist-active-bar[role=presentation][data-pc-section=activebar]`. `Tab` SHALL render `div.p-tab[role=tab]` with `id` `<tabs-id>_tab_<value>`, `aria-controls` `<tabs-id>_tabpanel_<value>`, `aria-selected`, `data-p-active`, `p-tab-active` on the active one and `p-disabled`, `aria-disabled`, `data-p-disabled` on a disabled one. `TabPanels` SHALL render `div.p-tabpanels`, `TabPanel` SHALL render `div.p-tabpanel[role=tabpanel]` with `id` `<tabs-id>_tabpanel_<value>`, `aria-labelledby`, `data-p-active`, `p-tabpanel-active` on the active one and the `hidden` attribute on an inactive one. `style`, `style_class` and `pt` SHALL apply to the root.

#### Scenario: Active tab
- **WHEN** `value` equals "1"
- **THEN** tab "1" has `p-tab-active`, `aria-selected="true"`, panel "1" has no `hidden`, the other panels have `hidden`

#### Scenario: Tab and panel linkage
- **WHEN** a tab and a panel with the value "a" are rendered
- **THEN** the tab's `aria-controls` equals the panel's `id`, the panel's `aria-labelledby` equals the tab's `id`

### Requirement: Value
`value` SHALL be a two-way `Option<String>` signal; without it the state is internal, starting from `default_value`. A click on an enabled tab SHALL make its value active. With `select_on_focus`, focus on an enabled tab SHALL make it active.

#### Scenario: Click
- **WHEN** the user presses tab "2"
- **THEN** `value` equals "2", panel "2" is visible

#### Scenario: Disabled tab
- **WHEN** a disabled tab is pressed
- **THEN** `value` does not change

### Requirement: Keyboard
A tab's `tabindex` SHALL be the group's `tabindex` for the active enabled tab and -1 for the others. On a tab, `ArrowRight`/`ArrowLeft` SHALL move focus to the next/previous enabled tab (wrapping around), `Home`/`End` to the first/last enabled one, `PageUp`/`PageDown` SHALL scroll the list to the first/last tab, `Enter`, `NumpadEnter` and `Space` SHALL activate the tab; handled keys SHALL cancel the default action, and the event SHALL NOT bubble beyond the tab. Focus SHALL NOT activate a tab without `select_on_focus`.

#### Scenario: Wrap-around arrow
- **WHEN** focus is on the last enabled tab and `ArrowRight` is pressed
- **THEN** focus is on the first enabled tab

#### Scenario: Skipping a disabled tab
- **WHEN** the next tab is disabled
- **THEN** focus moves past it

### Requirement: Indicator and scrolling
The `p-tablist-active-bar` bar SHALL get the `width` and `left` of the active tab relative to the list; recomputation SHALL happen on value change and list resize. With `show_navigators` (default), `TabList` SHALL render a `p-tablist-prev-button` button (`aria-label` "Previous") when the content is scrolled from the start, and a `p-tablist-next-button` ("Next") when there is a hidden part on the right; a click SHALL scroll the content by the width of the visible area (for next, minus the buttons). `prev_icon` and `next_icon` SHALL replace the icons.

#### Scenario: Indicator
- **WHEN** the second tab is active
- **THEN** the bar width equals the width of that tab, `left` equals its offset from the start of the list

#### Scenario: Scroll buttons
- **WHEN** the tabs do not fit and the content is scrolled to the right
- **THEN** both buttons are rendered

### Requirement: Lazy panels
With `lazy` on `Tabs` or `TabPanel`, the panel content SHALL NOT be rendered until first activation, and after it SHALL stay (hidden on deactivation). Without `lazy` the content is always rendered.

#### Scenario: Before activation
- **WHEN** `lazy` is set and the panel is inactive
- **THEN** there is nothing inside `div.p-tabpanel`

#### Scenario: After activation
- **WHEN** a tab became active and then inactive
- **THEN** the content stays in the DOM, the panel has `hidden`
