# Dock

Port of `p-dock` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-dock::Dock`, `DockItem`, `DockCommand`, `DockPosition`, `DockPt` | `packages/primeng/src/dock/dock.ts`, `dock/style/dockstyle.ts` |
| CSS | `@primeuix/styles/dock` and the Aura preset — `cargo xtask gen-theme` (there is no `/* For PrimeNG */` block) |
| Demos | `apps/showcase/doc/dock/*-doc.ts` (`basic`; `advanced` with icon magnification and `menu` are listed under differences) |

Spec: `openspec/specs/dock`.

## What matches PrimeNG

- Markup and classes: `div.p-dock.p-component.p-dock-<position>` > `div.p-dock-list-container` > `ul.p-dock-list[role=menu]` > `li.p-dock-item[role=menuitem]` > `div.p-dock-item-content` > `a.p-dock-item-link[aria-hidden]`; `p-dock-mobile` when the window width is not greater than `breakpoint` (default `960px`, via `matchMedia`).
- Positions `bottom` (default), `top`, `left`, `right`; `aria-orientation` follows them.
- Keyboard on the list: arrows along the dock, `Home`/`End`, `Enter`/`Space` — a click on the item's link. Cross-axis arrows are intercepted without an action. Disabled items are skipped, at the edge the focus stays.
- The focus is shown with the `p-focus` class, the list's `aria-activedescendant` points to the item `id`; `on_focus`/`on_blur`.
- Item: `label`, `icon`, `url`, `target`, `title`, `disabled`, `badge`, `command`, `style_class`, `style`; the identifier is its own `id` or `<id>_<index>`.
- `item` replaces the item content (icon and badge).

## Differences

| Difference | Reason |
|---|---|
| Icon magnification on hover (`advanced`) is not part of the component | In PrimeNG this is CSS of the demo, not a component style. The `item` template lets you repeat it from the outside. |
| Icons are `ViewFn`, not `pi pi-…` classes | There is no PrimeIcons font. |
| No `tooltip` wrapper, `dir`, `routerLink` | The hint is added from the outside with the `use:tooltip` directive (see the demo); routing goes through `url`/`command`. |
| The root is a `<div>`, not the host `<p-dock>` | The Angular host is a framework detail. |

## Not verified

- Right-to-left layout and touch input.
- Screen readers: `aria-*` were checked against PrimeNG, screen reader behaviour was not.

## Performance

See `benches/` (`dock-1`, `dock-1000`, `case-dock`) and `benches/*budgets.toml`.
