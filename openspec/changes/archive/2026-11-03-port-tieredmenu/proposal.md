# Proposal

## Why

TieredMenu is the last component of wave 4: a menu with nested submenus that expand in adjacent panels (as in a context menu). It closes what was deferred in `SplitButton` (the `nested` demo) and paves the way for the other menus (ContextMenu, Menubar, MegaMenu, PanelMenu), which are built on the same ideas: a tree of processed items, a path of active items, keyboard navigation across levels.

## What Changes

- New crate `prime-tieredmenu` (a port of `p-tieredmenu` from PrimeNG 21.1.10). The item model is the existing `prime_menu::MenuItem` and `MenuCommand` (the `items` field of `MenuItem` defines the nested submenu).
- `TieredMenu`: root `div.p-tieredmenu.p-component` (`p-tieredmenu-overlay` in `popup` mode, `p-tieredmenu-mobile` in a narrow window; `id`, `style`, `style_class`) > `ul.p-tieredmenu-root-list[role=menu][aria-orientation=vertical]` (`tabindex`, `aria-label`, `aria-labelledby`, `aria-activedescendant`) > `li.p-tieredmenu-item[role=menuitem]` (`p-tieredmenu-item-active`, `p-focus`, `p-disabled`, `data-p-highlight|focused|disabled`, `aria-label`, `aria-disabled`, `aria-haspopup=menu`, `aria-expanded`, `aria-setsize`, `aria-posinset`) > `div.p-tieredmenu-item-content` > `a.p-tieredmenu-item-link` with the `p-tieredmenu-item-icon` icon, the `p-tieredmenu-item-label` label and the `p-tieredmenu-submenu-icon` arrow (on groups) + a nested `ul.p-tieredmenu-submenu[role=menu]` (shown for the active item); separators `li.p-tieredmenu-separator[role=separator]`.
- Behavior: a click on a group expands it, a second click collapses it; a click on a leaf invokes `command`, closes all submenus (and in `popup` the menu too) and returns focus to the root list; hover expands groups after the first expansion by click (`auto_display`, on by default; in a narrow window it does not expand); keyboard: ↓/↑ across the level's items, → expands the group and goes inside, ← closes the submenu and returns to the parent, `Home`/`End`, `Enter`/`Space` activate, `Escape` closes, `Tab` closes, `Alt+↑` closes the popup menu, printable characters search for an item by label prefix (reset after 500 ms); `aria-activedescendant` points to the focused item.
- `popup` mode: `TieredMenuRef` (`show`/`show_at`/`toggle`/`hide`/`visible`) is a popup panel at the target; the `Menu` group layer; alignment to the target; an outside click, `resize` (not on touch devices) and scrolling an ancestor close it; `on_show`, `on_hide`.
- Submenu position per `nestedPosition`: to the right of the item, and to the left if there is no room on the right.
- Properties: `model`, `popup`, `style`, `style_class`, `breakpoint` (`960px`), `auto_z_index`, `base_z_index`, `auto_display`, `id`, `aria_label`, `aria_labelledby`, `disabled`, `tabindex`, `pt`; views `item`, `submenu_icon`; events `on_show`, `on_hide`.
- Showcase page `/tieredmenu` with the `basic`, `popup`, `command`, `template` demos; the `nested` demo of `SplitButton` is unchanged (see "Out of scope"); e2e; benchmarks and budgets; `docs/tieredmenu.md`.
- Styles: `tieredmenu` is added to `theme.components`.
- **Out of scope:** wiring `TieredMenu` into `SplitButton` (a separate change: it changes the `SplitButton` API and tests), `appendTo` (the panel is in its own place, like `Menu`), `showTransitionOptions`/`hideTransitionOptions`/`motionOptions` (animation is theme classes), item `routerLink`, item `tooltip`, HTML labels (`escape = false`), `Ripple`, `Bind`, `pTemplate`.

## Capabilities

### New Capabilities
- `tieredmenu`: a menu with nested submenus, keyboard navigation and a popup panel mode.

### Modified Capabilities
<!-- None. -->

## Impact

- New crate `crates/prime-tieredmenu` (`publish = false`); dependencies: `prime-core`, `prime-dom`, `prime-menu`, `prime-icons` (angleright), `web-sys`.
- Changed: workspace `Cargo.toml`, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (browser job, three places: `prime-tieredmenu`).
