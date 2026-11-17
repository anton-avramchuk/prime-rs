# Proposal

## Why

Tabs is the third component of wave 3: switching between views with tabs, with keyboard navigation, an active tab indicator bar and scrolling of a long tab list. A compound component of the same kind as `Accordion` (the parts communicate through context), so it builds on the project's existing solutions.

## What Changes

- New crate `prime-tabs` with five parts (port of PrimeNG 21.1.10 `p-tabs`, `p-tablist`, `p-tab`, `p-tabpanels`, `p-tabpanel`): `Tabs` is the root `div.p-tabs.p-component` (with `p-tabs-scrollable`), `TabList` is `div.p-tablist` with scroll buttons, `div.p-tablist-content` > `div.p-tablist-tab-list[role=tablist]` and the bar `span.p-tablist-active-bar`, `Tab` is `div.p-tab[role=tab]` (with `p-tab-active`, `p-disabled`), `TabPanels` is `div.p-tabpanels`, `TabPanel` is `div.p-tabpanel[role=tabpanel]`.
- `Tabs`: `value` (two-way `RwSignal<Option<String>>`), `default_value`, `scrollable`, `lazy`, `select_on_focus`, `show_navigators` (on by default), `tabindex` (0), `id`, `style`, `style_class`, `pt`. `Tab`: `value`, `disabled`. `TabPanel`: `value`, `lazy`. `TabList`: `prev_icon`, `next_icon` (`ViewFn`), `pt`.
- Behaviour: a click (and focus with `select_on_focus`) makes a tab active; left/right arrows move focus to the adjacent enabled tab wrapping around, `Home`/`End` to the first/last, `PageUp`/`PageDown` scroll to the first/last, `Enter`/`Space` activate; the active tab's `tabindex` equals the group's `tabindex`, the others' is -1. The indicator bar follows the active tab; "back"/"forward" buttons appear on scrolling and scroll the list by the width of the visible area. `lazy` defers panel content rendering until first activation (after that it stays).
- Showcase page `/tabs` with demos `basic`, `controlled`, `disabled`, `scrollable`, `dynamic`, `lazy`, `customtemplate`; e2e with a real keyboard; benchmarks and budgets; `docs/tabs.md`.
- Styles: `tabs` (which holds the styles of all five parts) is added to `theme.components`.
- **Out of scope:** `Ripple`, a `MutationObserver` on tab content (the indicator is recomputed on value change and list resize, `ResizeObserver`), numeric `value` (values are strings), the panel `content` template via `ng-template` (the content of a `lazy` panel is ordinary `children`, rendering is deferred), the `tabmenu` demo (needs Router/Menu), `pt` with context.

## Capabilities

### New Capabilities
- `tabs`: tabs with keyboard navigation, an indicator and scrolling.

### Modified Capabilities
<!-- None. -->

## Impact

- New crate `crates/prime-tabs` (`publish = false`); dependencies: `prime-core`, `prime-icons` (`chevronleft`, `chevronright`), `web-sys` (`ResizeObserver`, `Element`, `HtmlElement`, `KeyboardEvent`, `ScrollIntoViewOptions`).
- Changed: workspace `Cargo.toml`, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (browser job, three places).
