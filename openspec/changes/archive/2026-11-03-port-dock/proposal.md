# Proposal

## Why

Dock is the fourth component of wave 4: a quick-launch panel of icons, pinned to the edge of the container (like the dock in macOS). It is simple in logic (a list of items, keyboard focus, adaptation to the screen width) and does not depend on heavy components.

## What Changes

- A new crate `prime-dock` (a port of `p-dock` from PrimeNG 21.1.10).
- `DockItem` (instead of PrimeNG's `MenuItem`, without router fields): `id`, `label`, `icon` (`ViewFn`), `url`, `target`, `title`, `disabled`, `badge`, `command` (`Callback<DockCommand>`), `style_class`, `style`, `link_class`, `icon_class`.
- `Dock`: the root `div.p-dock.p-component.p-dock-{position}` (`p-dock-mobile` when the window width is not greater than `breakpoint`) > `div.p-dock-list-container` > `ul.p-dock-list[role=menu]` (`aria-orientation`, `tabindex`, `aria-label`, `aria-labelledby`, `aria-activedescendant`) > items `li.p-dock-item[role=menuitem]` (`p-focus`, `p-disabled`, `data-p-focused`, `data-p-disabled`) > `div.p-dock-item-content` > `a.p-dock-item-link` (`aria-hidden`, `href`, `target`, `title`) with the icon `span.p-dock-item-icon`, the view `item` and the badge.
- Properties: `model`, `position` (`DockPosition`: `Bottom` by default, `Top`, `Left`, `Right`), `breakpoint` (`960px`), `id`, `aria_label`, `aria_labelledby`, `style_class`, `style`, `pt`; the view `item` (instead of the item icon); the events `on_focus`, `on_blur`.
- Behaviour: a click on an item calls `command` (not for a disabled one); focus on the list puts the focus on the first available item; arrows depending on the orientation (→/← for the top and bottom dock, ↓/↑ for the side dock), `Home`, `End`, `Enter`/`Space` click the item's link; disabled items are skipped; the `media query` with `breakpoint` enables the `p-dock-mobile` mode.
- The showcase page `/dock` with the demo `basic` (choosing the position); e2e; benchmarks and budgets; `docs/dock.md`.
- Styles: `dock` is added to `theme.components`.
- **Not included:** `routerLink` and the related fields, `tooltipOptions` (a hint can be added with the `use:tooltip` directive in the `item` template), the `advanced` demo (it needs Terminal, Galleria and Dialog inside the dock), `Ripple`, `Bind`, `pTemplate`.

## Capabilities

### New Capabilities
- `dock`: a panel of icons at the edge of the container with keyboard navigation and adaptation to the window width.

### Modified Capabilities
<!-- None. -->

## Impact

- A new crate `crates/prime-dock` (`publish = false`); dependencies: `prime-core`, `prime-badge`, `web-sys`.
- Changes to `Cargo.toml` of the workspace, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (the browser job, three places).
