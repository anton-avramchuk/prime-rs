# Proposal

## Why

`Breadcrumb` is the next component of wave 2: a path through the page hierarchy, which any application with nested navigation needs. It builds on the already finished `Badge`, `ChevronRightIcon`, `HomeIcon` and the theme CSS. The original plan also included `Steps`, but in PrimeNG 21 that component is marked deprecated (replaced by `Stepper`), has no showcase page and no demos to compare against, so it is not ported to the project; `Stepper` is a separate, larger piece of work.

## What Changes

- New crate `prime-breadcrumb`: `Breadcrumb` and the item model `BreadcrumbItem`.
  - Root `<nav class="p-breadcrumb p-component">` with `<ol class="p-breadcrumb-list">`; a home item (`li.p-breadcrumb-home-item`, with `HomeIcon` by default), model items (`li.p-breadcrumb-item`, `p-disabled` for disabled ones), separators `li.p-breadcrumb-separator` with `ChevronRightIcon` or a custom template.
  - The item link is `a.p-breadcrumb-item-link` with `href` (`url`), `target`, `title`, `tabindex`, an icon, a `span.p-breadcrumb-item-label` label and an optional `Badge`; a click on a disabled item is cancelled, an item without `url` does not follow the link, for the others `command` and `on_item_click` are called.
  - Properties: `model`, `home`, `home_aria_label`, `style`, `style_class`, `pt`; the `item` and `separator` templates are callbacks.
- The `breadcrumb` theme CSS is added to `theme.components`.
- Showcase page `/breadcrumb` with demos `basic` and `template` (the `router` demo relies on Angular Router and is not ported), e2e, benchmarks and budgets, `docs/breadcrumb.md`.
- **Not included:** `Steps` (deprecated, no demos); `routerLink` and its parameters (`queryParams`, `fragment`, etc.) - `url` instead; the `MenuItem` type shared by all menus (will appear with the second consumer - `Menu` or `Tabs`); `tooltipOptions`; HTML labels (`escape = false`); per-item `pt` with the `{item, index}` context.

## Capabilities

### New Capabilities
- `breadcrumb`: a path through the page hierarchy with a home item, separators and templates.

### Modified Capabilities
<!-- None. -->

## Impact

- New crate `crates/prime-breadcrumb` (`publish = false`); dependencies: `prime-rs-core`, `prime-badge`, `prime-icons` (`chevronright`, `home`), `web-sys`.
- Changed: workspace `Cargo.toml`, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`.
