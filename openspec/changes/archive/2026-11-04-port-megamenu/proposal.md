# Proposal

## Why

MegaMenu is the fourth menu of wave 5: a root bar (horizontal or vertical) where each item expands a wide overlay with submenu columns holding items. Path state, outside click, `resize`, the mobile button and auto-hide are already worked out in `prime-menubar`; what is new is the two-level model "item → columns → submenus → items", orientation, column layout and its own keyboard navigation (across columns).

## What Changes

- New crate `prime-megamenu` (port of `p-megamenu` from PrimeNG 21.1.10).
- Model: `MegaMenuItem` is a root item (fields as in `prime_menu::MenuItem`: `label`, `icon`, `url`, `target`, `title`, `disabled`, `visible`, `separator`, `command`, classes) with the field `columns: Vec<Vec<MegaMenuSubmenu>>`; `MegaMenuSubmenu` is a column header (`label`, `visible`, classes) with `items: Vec<MenuItem>` (flat items without nested submenus, as in PrimeNG).
- `MegaMenu`: root `div.p-megamenu.p-component` (`p-megamenu-horizontal|vertical`, `p-megamenu-mobile`, `p-megamenu-mobile-active`; `id`, `style`, `style_class`) > `div.p-megamenu-start`, `a.p-megamenu-button` (with a non-empty model), `ul.p-megamenu-root-list[role=menubar]` (`aria-orientation`) > `li.p-megamenu-item` > `div.p-megamenu-item-content` > `a.p-megamenu-item-link` (icon, label, down/right arrow) + `div.p-megamenu-overlay` > `div.p-megamenu-grid` > `div.p-megamenu-col-N` (N by the number of columns: 12/6/4/3/2, 12 on a narrow window) > `ul.p-megamenu-submenu[role=menu]` > `li.p-megamenu-submenu-label[role=presentation]` and `li.p-megamenu-item`; separators `p-megamenu-separator`; `div.p-megamenu-end`.
- Behavior: a click on a root item with columns expands the overlay, a repeated click closes it; a leaf calls `command` and closes; hover expands when `dirty` (after a click), on a narrow window the path does not change; an outside click and `resize` close; the mobile button and `Menu` layer as in Menubar. MegaMenu has no `auto_hide`.
- Keyboard per the source: Down/Up/Right/Left across roots and inside the overlay (depending on orientation), `Home` `End` `Enter` `Space` `Escape` `Tab`, search by label prefix.
- Properties: `model`, `orientation` (`Horizontal` by default, `Vertical`), `style`, `style_class`, `breakpoint` (`960px`), `scroll_height`, `disabled`, `tabindex`, `id`, `aria_label`, `aria_labelledby`, `auto_z_index`, `base_z_index`; views `start`, `end`, `item`, `submenu_icon`, `menu_icon`.
- Page `/megamenu` (`basic`, `vertical`, `command`, `template`), e2e, benchmarks and budgets, `docs/megamenu.md`, `megamenu` in `theme.components` (the style source has no `For PrimeNG` block).
- **Out of scope:** `routerLink` and the `router` demo, `Tooltip`/`Badge`, HTML labels, `Ripple`, `Bind`, `pTemplate`, `button` as a full button replacement (`#button`), the `accessibility` demo.

## Capabilities

### New Capabilities
- `megamenu`: a menu with a wide overlay of submenu columns, two orientations, keyboard and mobile mode.

### Modified Capabilities
<!-- None. -->

## Impact

- New crate `crates/prime-megamenu` (`publish = false`); dependencies: `prime-rs-core`, `prime-dom`, `prime-menu`, `prime-icons` (angledown, angleright, bars), possibly `prime-tieredmenu` after D1 (shared helper functions only).
- Changed: workspace `Cargo.toml`, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (browser job, three places). The behavior of existing crates does not change.
