# MegaMenu

Port of `p-megamenu` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-megamenu::MegaMenu`, `MegaMenuPt`, `MegaMenuItemContext`, `MegaMenuItem`, `MegaMenuSubmenu`, `MegaMenuOrientation` | `packages/primeng/src/megamenu/megamenu.ts` (`MegaMenu` and `MegaMenuSub`), `megamenu/style/megamenustyle.ts` |
| Transition model (path, focus, search) | `prime-megamenu::state` — our own: `build`, `Menu`, `State` |
| CSS | `@primeuix/styles/megamenu` and the Aura preset — `cargo xtask gen-theme` (no `/* For PrimeNG */` block) |
| Demos | `apps/showcase/doc/megamenu/*-doc.ts` (`basic`, `vertical`, `command`, `template`) |

Spec: `openspec/specs/megamenu`. A root item is `MegaMenuItem` (a plain `prime_menu::MenuItem` plus `columns`), a column is a list of `MegaMenuSubmenu` (a header and flat `MenuItem`s). Node keys: root `"0"`, submenu `"0_<column>_<index>"`, item `"0_1_0_2"`; they are also in the `id` of the `li` elements (`<menu id>_<key>`).

```rust
let model = vec![MegaMenuItem::new().label("Furniture").columns(vec![
    vec![MegaMenuSubmenu::new("Living Room").items(vec![MenuItem::new().label("Armchair")])],
    vec![MegaMenuSubmenu::new("Kitchen").items(vec![MenuItem::new().label("Chair")])],
])];
view! { <MegaMenu model=model orientation=MegaMenuOrientation::Vertical /> }
```

## What matches PrimeNG

- Markup and classes: `div.p-megamenu.p-component` (`-horizontal|-vertical`, `-mobile`, `-mobile-active`) > `div.p-megamenu-start`, `a.p-megamenu-button`, `ul.p-megamenu-root-list[role=menubar]` (`aria-orientation`) > `li.p-megamenu-item` (`-item-active`, `p-focus`, `p-disabled`, `data-p-*`, `aria-*`) > `div.p-megamenu-item-content` > `a.p-megamenu-item-link` and `div.p-megamenu-overlay` > `div.p-megamenu-grid` > `div.p-megamenu-col-N` > `ul.p-megamenu-submenu[role=menu]` > `li.p-megamenu-submenu-label` and items; separators; `div.p-megamenu-end`. Column class: 2 → `col-6`, 3 → `col-4`, 4 → `col-3`, 6 → `col-2`, otherwise and on a narrow window — `col-12`. Arrow: down in horizontal orientation and on mobile, right in vertical.
- A click on a root expands/closes the overlay, a leaf calls the command and closes everything; hover works only after a click/mouse press on the menu (`dirty`) and not on a narrow window; an outside click and `resize` (not on touch devices) close it.
- Keyboard per `onArrow*Key`: in horizontal orientation Left/Right walk the roots, Down expands a group and enters the first column, Down on a plain root goes to the next one; inside the overlay Down/Up walk the flat list of items, Right/Left move between columns (the first focusable item of the column), `Alt+Up` returns to the root; in vertical orientation Down/Up walk the roots, Right expands and enters, Left from the first column returns to the root; `Home` `End` `Enter` `Space` `Escape` `Tab`; search by label prefix with a 500 ms buffer. Focus from outside immediately sets the first item.
- Mobile mode via `matchMedia` with `breakpoint` (default `960px`): button, `aria-expanded`, the `Menu` group layer on the open list, focus returns to the button; resizing the window across the threshold closes the panel.

## Differences

| Difference | Reason |
|---|---|
| Root transitions with an open overlay (Left/Right/Down/Up) walk the root items, and the overlay follows focus (it closes on a plain root) | In PrimeNG, with an open overlay `visibleItems` is the list of submenu items, and the root index points at an unrelated item. |
| Hover over a plain root item closes the open overlay | In PrimeNG it stays open. |
| Invisible and disabled items, as well as hidden submenus, are skipped in navigation and search | In PrimeNG `isValidItem` does not check `visible`. |
| The overlay is rendered only for the expanded item | Display is set by the theme CSS (`.p-megamenu-item-active > .p-megamenu-overlay`); cheaper for the DOM. |
| The inline style of the root list (`max-height`, `overflow: auto`) is set only with `scroll_height` | In PrimeNG `overflow: auto` is always set; for a horizontal menu that would clip the overlay. |
| The button `aria-label` comes from `locale` (`aria.navigation`) | Same in PrimeNG (`config.translation.aria.navigation`). |
| No `routerLink`, item `Tooltip`/`Badge`, `escape = false`, `Ripple`, `Bind`, `pTemplate`, replacing the whole button (`#button`); icons are `ViewFn` | No router, `innerHTML` or PrimeIcons. |
| The `router` and `accessibility` demos are not ported; icons with no counterpart in `prime-icons` are replaced with similar ones | No router or PrimeIcons. |

## Not verified

- Right-to-left layout and touch input (on touch, hover expands immediately, as in PrimeNG; there are no real touches in the tests).
- Screen reading: `aria-*` was checked against PrimeNG, screen reader behavior was not.
- Overlay at the very right edge of the window (it overflows the window, position is set by the theme CSS).

## Performance

See `benches/` (`megamenu-1`, `megamenu-1000`, `case-megamenu`) and `benches/*budgets.toml`.
