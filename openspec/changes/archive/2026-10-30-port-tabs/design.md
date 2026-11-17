# Design

## Context

Requirements are in `specs/tabs`. PrimeNG 21.1.10 sources: `tabs/{tabs,tablist,tab,tabpanels,tabpanel}.ts` and `tabs/style/*`; demos `apps/showcase/doc/tabs/*-doc.ts`. Reference implementations in the project: `prime-accordion` (compound component, context, `Nav`), `prime-scrolltop` (subscribing to elements, `Closure`), `prime-select` (two-way `Option<String>`).

Observations about the source:
- Focus on `ArrowRight`/`ArrowLeft` finds a neighbour via `nextElementSibling`/`previousElementSibling`, skipping disabled tabs (`data-p-disabled`) and the bar (`data-pc-section="activebar"`); with no neighbour it goes to the first/last tab. We repeat the lookup over the DOM: the tab list is formed by adjacent elements, not by a collection known to the root.
- The bar is a `span`, `width`/`left` are written straight into the DOM after `setTimeout(0)` on value change and from the tab's `MutationObserver`; the buttons are updated on content `scroll` and the list root's `ResizeObserver`.
- The content of an inactive panel is hidden with the `hidden` attribute; `lazy` defers rendering until the first activation.
- The default button icon is `ChevronLeftIcon`/`ChevronRightIcon`, the labels are `aria.previous`/`aria.next` from the locale.

## Decisions

### D1. Crate and dependencies
`prime-tabs` with `Tabs`, `TabList`, `Tab`, `TabPanels`, `TabPanel`, `TabListPt`. Depends on `prime-rs-core`, `prime-icons` (`chevronleft`, `chevronright`), `web-sys`.

### D2. Context
`TabsCtx { id, value: RwSignal<Option<String>>, lazy, select_on_focus, show_navigators, tabindex, ink: RwSignal<Ink>, tabs_ref }` is introduced in `Tabs` via `provide_context` (as in `Accordion`); the parts take it with `use_context` and panic with a clear message outside `Tabs`. `TabList` puts a reference to the tab list element into the context via `RwSignal<Option<...>>`/`NodeRef`, so that `Tab` can find the first and last tabs.

### D3. Pure logic (`state.rs`)
- `tab_id(tabs_id, value)`, `panel_id(tabs_id, value)`.
- `is_active(current, value)`; `tabindex_for(disabled, active, tabs_tabindex)`.
- `root_classes(scrollable)`, `tab_classes(active, disabled)`, `panel_classes(active)`.
- `Key` and `key_action(code) -> Option<KeyAction>` (`Next`, `Prev`, `First`, `Last`, `ScrollFirst`, `ScrollLast`, `Activate`).
- `wrap_target(index_of_current, enabled: &[bool], action) -> Option<usize>`: wrap-around target lookup among enabled tabs; the DOM traversal uses it on an array of "enabled" flags.
- `should_render(lazy, has_rendered, active)`.
- `prev_enabled(scroll_left) / next_enabled(list_width, content_width, scroll_left, scroll_width, width)`, `prev_scroll(scroll_left_abs, width) -> f64`, `next_scroll(scroll_left, width, buttons, scroll_width) -> f64`.
Table-driven tests for each.

### D4. DOM navigation
In the tab's `keydown` handler, the parent (the tab list) is taken via the tab's `NodeRef`, its child elements with `role=tab` are collected, an "enabled" vector is formed and `wrap_target` is applied; focus is `HtmlElement::focus`, then `scrollIntoView({block: nearest})`. This avoids the recursive `nextElementSibling` traversal and gives a testable pure function.

### D5. Indicator
`ink: RwSignal<(f64, f64)>` (width, offset). Update: an `Effect` (`wasm32` only) on `value` via `set_timeout(0)` and a `ResizeObserver` on the tab list; measurement is `getBoundingClientRect` of the active tab (`[role=tab][data-p-active=true]`) and the list. The bar style is `width: Xpx; left: Ypx`. No style on SSR.

### D6. Scroll buttons
`prev_visible`/`next_visible: RwSignal<bool>`; `update_buttons` is called on mount (if `show_navigators`), on content `scroll` and `ResizeObserver`. A click changes `scrollLeft` (accounting for `rtl`).

### D7. Lazy panels
A `Memo` on `should_render`; the "has been rendered" flag is kept in a `StoredValue<bool>` and set on first activation. Panel content is a `ChildrenFn`, rendered by a closure over the `Memo`.

### D8. CSS
`@primeuix/styles` has only the `tabs` module, which holds the styles of all five parts; it is added to `theme.components`. `cargo xtask gen-theme`, a check that the CSS of the others has not changed.

### D9. Tests
SSR: markup of the parts, activity, `hidden`, `id` linkage, `disabled`, `lazy`, `scrollable`. Browser `tests/web.rs`: click, disabled, `select_on_focus`, keys (wrap-around, skipping disabled, `Home`/`End`, `Enter`/`Space`), roving `tabindex`, `lazy` with retention, indicator, scroll buttons. e2e: real keyboard, button scrolling, indicator. The crate is added in three places of `ci.yml`.

### D10. Benchmarks
Scenarios `tabs-1`/`-1000` (three tabs and three panels), `case-tabs`; first `cargo bench -p render-bench --bench alloc`, for wasm a temporary large budget (replace by line, not with a global `sed`).

## Risks / Trade-offs

- [No `MutationObserver`] → the indicator is not updated if tab content changes its width without changing the list size; recorded in docs.
- [Context via `provide_context` without a child owner] → as in `Accordion`; the parts are created synchronously inside `Tabs`.
- [Values are strings only] → the difference is recorded in docs.

## Migration Plan

No migration. Rollback is `git revert`.

## Open Questions

None.
