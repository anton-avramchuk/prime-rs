# Breadcrumb

Port of `p-breadcrumb` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-breadcrumb::Breadcrumb`, `BreadcrumbItem` | `breadcrumb/breadcrumb.ts`, `breadcrumb/style/breadcrumbstyle.ts` |
| CSS | `@primeuix/styles/breadcrumb` - `cargo xtask gen-theme` (the component has no `/* For PrimeNG */` block) |
| Demos | `apps/showcase/doc/breadcrumb/{basic,template}-doc.ts` |

Spec: `openspec/specs/breadcrumb`.

## What matches PrimeNG

- Markup and classes: `nav.p-breadcrumb.p-component` > `ol.p-breadcrumb-list` > `li.p-breadcrumb-home-item`, `li.p-breadcrumb-item` (with `p-disabled`), `li.p-breadcrumb-separator`; inside an item `a.p-breadcrumb-item-link` with an icon, a label and a badge.
- A "home" item without its own icon gets `HomeIcon`; the default separator is `ChevronRightIcon`.
- The separator after the "home" item is rendered only if there are model items; not after the last item.
- Click: a disabled item cancels the action and calls nothing; an item without `url` cancels navigation but calls `command` and `on_item_click` (the logic is the pure function `click_plan` with a table-driven test).

## Differences

| Difference | Reason |
|---|---|
| The root is `<nav>` without the `<p-breadcrumb>` host | A host in Angular is a framework detail. |
| `BreadcrumbItem` instead of `MenuItem` | A shared menu type will appear with the second consumer (`Menu`, `Tabs`). |
| No `routerLink`, `queryParams`, `fragment`, etc. | That is the Angular Router API; SPA navigation is driven by `command` or `url` and your own router. |
| The item icon is a `ViewFn` (an icon from `prime-icons`), not a font class name | The project's icons are SVG components. |
| The `item` and `separator` templates are a callback and a `ViewFn` | Instead of `ng-template`. |
| No `tooltipOptions`, HTML labels (`escape = false`), per-item `pt` with the `{item, index}` context, `router` demo | Tied to Angular or outside the batch scope. |
| `Steps` is not ported | Deprecated in PrimeNG 21 (replaced by `Stepper`), has no page or demos. |
| In the `template` demo, `star`, `calendar`, `eye`, `info-circle`, `search` are used instead of the icons `sitemap`, `book`, `wallet`, `shopping-bag`, `calculator` | Those icons are not in `prime-icons`. |

## Cost

176 allocations per instance in SSR (home and four items, four separators with SVG), contribution to wasm is about 244 KB (most of it is the shared `Badge` and icons).

## Not verified

Keyboard and screen readers were not tested on real devices: markup, computed styles and the absence of navigation on click (e2e) were verified.
