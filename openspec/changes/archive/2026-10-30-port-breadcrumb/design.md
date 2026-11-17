# Design

## Context

Requirements are in `specs/breadcrumb`. PrimeNG 21.1.10 sources: `breadcrumb/breadcrumb.ts`, `breadcrumb/style/breadcrumbstyle.ts`; demos `apps/showcase/doc/breadcrumb/{basic,template,router}-doc.ts`. Models in the project: `crates/prime-chip` (click, `ViewFn` icon, `web-sys` events), `crates/prime-badge` (nested `Badge`), `crates/prime-timeline` (callback templates, a list from `Prop<Vec<_>>`), `prime_core::Pt`.

Observations about the source:
- The template has two nearly identical link blocks - with `href` and with `routerLink`; without Angular Router only the first remains.
- `onClick`: disabled - `preventDefault` and exit; without `url` and `routerLink` - `preventDefault`; then `command` and `onItemClick`.
- The separator after the "home" item is rendered when `model && home`; after a model item - `!end && visible`.
- The style has no `/* For PrimeNG */` block - `primeng_extra` does not change.
- `Steps` is marked deprecated in 21.1.10 and has no showcase page; it is not ported to the project.

## Decisions

### D1. Crate and contents
`prime-breadcrumb` with `Breadcrumb`, `BreadcrumbItem`, `BreadcrumbPt`. Dependencies: `prime-core`, `prime-badge`, `prime-icons` (`chevronright`, `home`), `web-sys` (`Event`, `MouseEvent`). A shared `MenuItem` is not introduced until a second consumer.

### D2. Model
`BreadcrumbItem { id, label, icon: Option<ViewFn>, url, target, title, disabled, visible (true by default), tabindex, style_class, style, link_class, badge, command: Option<Callback<BreadcrumbClick>> }` with a builder (`BreadcrumbItem::new().label("...")`), `Clone`. `BreadcrumbClick { event: MouseEvent, item: BreadcrumbItem }` is also the argument of `on_item_click`. `model: Prop<Vec<BreadcrumbItem>>`, `home: OptProp<BreadcrumbItem>`.

### D3. Markup and clicks
The item list is built in a single closure over `model`/`home` (a signal rebuilds the whole list). Click: the link's `on:click` calls the pure function `click_plan(disabled, has_url) -> ClickPlan { prevent_default, run_commands }` (table-driven test), and by its result - `prevent_default`, `command`, `on_item_click`. Calling it during SSR is impossible, so the logic is checked by a table-driven test and the behavior by e2e.

### D4. Templates
`item: Option<Callback<BreadcrumbItem, AnyView>>` and `separator: Option<ViewFn>`. The `item` template replaces the `li` content, the wrapper and classes remain.

### D5. CSS
`breadcrumb` is added to `theme.components`; `cargo xtask gen-theme`, check that the CSS of the others did not change.

### D6. Demos and tests
`/breadcrumb`: `basic` (home + five items), `template` (icons instead of labels, separator `/`; the available `prime-icons` icons instead of `pi-sitemap`, `pi-book`, etc.), `router` is not ported. SSR tests for each scenario; table-driven test of `click_plan`; e2e: markup, the number of separators, a click on an item with a command (a counter in the demo is not needed - it is checked via `url` and `preventDefault`), a disabled item. Benchmarks `breadcrumb-1` and `-1000`, `case-breadcrumb`.

## Risks / Trade-offs

- [Own model instead of a shared `MenuItem`] -> when `Menu`/`Tabs` appear, extract a shared type; `BreadcrumbItem` will remain an alias or wrapper.
- [Clicks are not checked in SSR] -> a table-driven test of the decision and e2e.
- [No `routerLink`] -> the user drives SPA navigation via `command` (or `url` and their own router); the difference is recorded in `docs/breadcrumb.md`.

## Migration Plan

No migration. Rollback is `git revert`.

## Open Questions

None.
