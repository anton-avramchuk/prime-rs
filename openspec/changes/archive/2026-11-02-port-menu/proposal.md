# Proposal

## Why

Menu is the ninth component of wave 3: a list of commands and links, static or popping up at a button. It is needed as a foundation for the next menus (ContextMenu, TieredMenu, Menubar) and covers the typical scenario "an ellipsis button → a list of actions". The popup mode repeats Popover (a panel inside the root, a control object); the list with groups and keyboard navigation is the new part.

## What Changes

- New crate `prime-menu` (port of `p-menu` from PrimeNG 21.1.10).
- `MenuItem` (instead of PrimeNG `MenuItem`, without Angular router fields): `id`, `label`, `icon` (`ViewFn`), `url`, `target`, `title`, `disabled`, `visible` (yes), `separator`, `items` (subitems — a group), `command` (`Callback<MenuCommand>`), `style_class`, `style`, `link_class`, `icon_class`.
- `Menu`: root `div.p-menu.p-component` (with `p-menu-overlay` and `position: absolute` in popup mode) > optional `start`/`end` > `ul.p-menu-list[role=menu]` with items `li.p-menu-item[role=menuitem]` > `div.p-menu-item-content` > `a.p-menu-item-link` (icon and `span.p-menu-item-label`), group headers `li.p-menu-submenu-label` and separators `li.p-menu-separator`.
- Properties: `model`, `popup`, `style`, `style_class`, `auto_z_index`, `base_z_index`, `aria_label`, `aria_labelledby`, `id`, `tabindex` (0), `pt`; views `item` (instead of the item content), `submenu_header`, `start`, `end`; events `on_show`, `on_hide`, `on_focus`, `on_blur`.
- `MenuRef` (`Copy`): `toggle(event)`, `show(event)`, `hide()` — like `menu.toggle($event)` in PrimeNG; the target is the event's `currentTarget`.
- Behavior: a click on an item calls `command` and closes the popup menu; a disabled item and an item without `url` cancel navigation; keys Down Up Home End Enter Space Escape Tab (in popup Escape/Tab/Alt+Up return focus to the target); focus (`p-focus`, `aria-activedescendant`); an outside click, ancestor scroll and resize close the popup menu; `z-index` of the `Menu` group.
- Showcase page `/menu` with the demos `basic`, `group`, `command` (via `MessageService` and `Toast`), `popup`, `template`, an `accessibility` description; e2e; benchmarks and budgets; `docs/menu.md`.
- Styles: `menu` is added to `theme.components`.
- **Out of scope:** `routerLink` and related fields, `appendTo` (the panel is always inside the root, as in Popover), `@primeuix/motion`/`motionOptions`/`show/hideTransitionOptions` (animation is theme CSS classes), item `tooltipOptions`, item `badge`, `escape=false` (HTML in the label), `Ripple`, `pt` with context, `Bind`, `pTemplate`.

## Capabilities

### New Capabilities
- `menu`: a list of commands and links, static or popup, with groups, separators and keyboard navigation.

### Modified Capabilities
<!-- None. -->

## Impact

- New crate `crates/prime-menu` (`publish = false`); dependencies: `prime-core`, `prime-dom`, `web-sys`.
- Changed: workspace `Cargo.toml`, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (browser job, three places).
