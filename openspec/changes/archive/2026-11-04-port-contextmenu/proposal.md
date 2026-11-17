# Proposal

## Why

ContextMenu is the first of the remaining menus of wave 5 (after `Steps`). It shows the same item tree as `TieredMenu`, but opens with the right mouse button (or a long press) at the cursor. All the path, keyboard and search logic already exists in `prime-tieredmenu`; what is new is the binding to a target/document, positioning by cursor coordinates and the absence of a `popup` mode.

## What Changes

- A new crate `prime-contextmenu` (a port of PrimeNG 21.1.10's `p-contextmenu`). The item model is `prime_menu::MenuItem`; the pure logic (`model`) is taken from `prime-tieredmenu`.
- `ContextMenu`: root `div.p-contextmenu.p-component` (`p-contextmenu-mobile` in a narrow window; `id`, `style`, `style_class`) > `ul.p-contextmenu-root-list[role=menu]` > `li.p-contextmenu-item` > `div.p-contextmenu-item-content` > `a.p-contextmenu-item-link` (icon `p-contextmenu-item-icon`, label `p-contextmenu-item-label`, arrow `p-contextmenu-submenu-icon`) + nested `ul.p-contextmenu-submenu`; separators `li.p-contextmenu-separator`. The `aria-*` attributes, `data-p-*` and the keyboard are as in `TieredMenu`.
- Opening: the `contextmenu` event (the `trigger_event` property) on `target` (a CSS selector) or on the whole document (`global`); on touch devices a long press (`press_delay`, 500 ms). The menu is shown at the cursor (`pageX + 1`, `pageY + 1`), flipping at the right/bottom window edge and clamped to the left/top; the event is cancelled (`preventDefault`, `stopPropagation`).
- `ContextMenuRef` (`show(event)`, `hide`, `toggle(event)`, `visible`); `on_show`, `on_hide`.
- Closing: a click outside the menu (except the right button and Ctrl-click), `resize`, selecting a leaf, `Escape`, `Tab`. The `Menu` group layer (`auto_z_index`, `base_z_index`), root list focus after showing.
- Submenus expand to the right, and to the left when there is no room (as in `TieredMenu`).
- Properties: `model`, `target`, `global`, `trigger_event`, `style`, `style_class`, `auto_z_index`, `base_z_index`, `id`, `breakpoint` (`960px`), `aria_label`, `aria_labelledby`, `press_delay`, `pt`; the views `item`, `submenu_icon`; the events `on_show`, `on_hide`.
- The showcase page `/contextmenu` with the `basic`, `command`, `template` demos (integration with `Table` and `router` is not ported); e2e; benchmarks and budgets; `docs/contextmenu.md`.
- Styles: `contextmenu` is added to `theme.components`.
- **Out of scope:** integration with `Table`, `appendTo`, animation options (`motionOptions`), `routerLink`, `tooltip` and HTML item labels, `Ripple`, `Bind`, `pTemplate`.

## Capabilities

### New Capabilities
- `contextmenu`: a context menu with nested submenus opened at the cursor.

### Modified Capabilities
<!-- None. -->

## Impact

- A new crate `crates/prime-contextmenu` (`publish = false`); dependencies: `prime-rs-core`, `prime-dom`, `prime-menu`, `prime-tieredmenu`, `prime-icons` (angleright), `web-sys`.
- The public export of `prime-tieredmenu` may be extended (`pub use model::…`) without a behaviour change.
- Changed: workspace `Cargo.toml`, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (browser job, three places).
