# Proposal

## Why

Menubar — the second menu of wave 5 (after ContextMenu): a horizontal panel with dropdown submenus, which on a narrow window collapses into a "hamburger" button. The logic of paths, focus, search and the position of submenus already exists in `prime-tieredmenu`; what is new — the horizontal root level (← → move along the root, ↓ opens the group), the slots `start`/`end`, the mobile button and auto-hide.

## What Changes

- A new crate `prime-menubar` (a port of `p-menubar` from PrimeNG 21.1.10). The item model — `prime_menu::MenuItem`; the pure logic is taken from `prime-tieredmenu` and, if needed, extended with the functions of the horizontal root.
- `Menubar`: the root `div.p-menubar.p-component` (`p-menubar-mobile` on a narrow window, `p-menubar-mobile-active` when the button is open; `id`, `style`, `style_class`) > `div.p-menubar-start` (the slot `start`) + `a.p-menubar-button[role=button]` with a "bars" icon (only with a non-empty model; `aria-haspopup`, `aria-expanded`, `aria-controls`, `aria-label`) + `ul.p-menubar-root-list[role=menubar]` > `li.p-menubar-item` > `div.p-menubar-item-content` > `a.p-menubar-item-link` (`p-menubar-item-icon`, `p-menubar-item-label`, `p-menubar-submenu-icon`) + the nested `ul.p-menubar-submenu`; separators `p-menubar-separator`; the slot `end` in `div.p-menubar-end`. The `aria-*`/`data-p-*` attributes — as in `TieredMenu`.
- Behaviour: a click on a group opens/closes it, a leaf calls `command` and closes everything; hovering opens the groups with `auto_display`; `auto_hide` closes the submenus after `auto_hide_delay` (100 ms) after the cursor leaves; a click outside closes the submenus (and the mobile panel); `resize` closes them.
- Keyboard: ← → along the root, ↓ opens the root group and enters it, ↑ ↓ → ← inside the submenus as in `TieredMenu`, `Home`/`End`, `Enter`/`Space`, `Escape`, `Tab`, search by the start of the label. The menu button: `Enter`/`Space` toggle it.
- Mobile mode (`breakpoint`, by default `960px`): the button shows/hides the root list (the `Menu` layer), the submenus open by a click, hovering does not change the path.
- Properties: `model`, `style`, `style_class`, `auto_z_index`, `base_z_index`, `auto_display`, `auto_hide`, `auto_hide_delay`, `breakpoint`, `id`, `aria_label`, `aria_labelledby`; the views `start`, `end`, `item`, `submenu_icon`, `menu_icon`; the events `on_focus`, `on_blur`.
- The page `/menubar` (`basic`, `command`, `template`), e2e, benchmarks and budgets, `docs/menubar.md`, `menubar` in `theme.components` (and in `primeng_extra`, if the source styles have a `For PrimeNG` block).
- **Not included:** `routerLink`, the `Tooltip`/`Badge` of items, HTML labels, `Ripple`, `Bind`, `pTemplate`, `appendTo`, the animation options, the `router` demo.

## Capabilities

### New Capabilities
- `menubar`: a horizontal menu with submenus, keyboard control and a mobile mode.

### Modified Capabilities
<!-- None. -->

## Impact

- A new crate `crates/prime-menubar` (`publish = false`); dependencies: `prime-core`, `prime-dom`, `prime-menu`, `prime-tieredmenu`, `prime-icons` (angleright, angledown, bars).
- Possibly a change to `prime-tieredmenu::model` (the functions of the horizontal root) — without changing the behaviour of `TieredMenu` and `ContextMenu`; their tests stay green.
- Changes to `Cargo.toml` of the workspace, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (the browser job, three places).
