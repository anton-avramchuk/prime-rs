# Menubar

Port of `p-menubar` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-menubar::Menubar`, `MenubarPt`, `MenubarItemContext` | `packages/primeng/src/menubar/menubar.ts` (`Menubar` and `MenubarSub`), `menubar/style/menubarstyle.ts` |
| Transition model (path, focus, search) | shared with `TieredMenu`: `prime_tieredmenu::{Tree, State, Node, …}`; horizontal root keys — `Tree::bar_down/bar_up/bar_left/bar_right` |
| CSS | `@primeuix/styles/menubar` and the Aura preset — `cargo xtask gen-theme` (no `/* For PrimeNG */` block) |
| Demos | `apps/showcase/doc/menubar/*-doc.ts` (`basic`, `command`, `template`) |

Spec: `openspec/specs/menubar`. The item model is `prime_menu::MenuItem` (the `items` field defines a submenu), commands are `MenuCommand`.

```rust
view! {
    <Menubar
        model=items
        start=ViewFn::from(|| view! { <b>"Logo"</b> })
        end=ViewFn::from(|| view! { <input placeholder="Search" /> })
    />
}
```

## What matches PrimeNG

- Markup and classes: `div.p-menubar.p-component` (`p-menubar-mobile`, `p-menubar-mobile-active`) > `div.p-menubar-start`, `a.p-menubar-button[role=button]`, `ul.p-menubar-root-list[role=menubar]` > `li.p-menubar-item` (`-item-active`, `p-focus`, `p-disabled`, `data-p-*`, `aria-*`) > `div.p-menubar-item-content` > `a.p-menubar-item-link`; nested `ul.p-menubar-submenu` (inline `display: flex` while the group is expanded); separators; `div.p-menubar-end`. Group arrow: down at the root, right for nested ones.
- Keyboard: ← → along the root, ↓ on a root group expands it and enters, ↑ on a root group — to the last item, ↑ on the first item of a submenu — to the parent; inside a submenu ↓ ↑ → ←; `Home` `End` `Enter` `Space` `Escape` `Tab`; search by label prefix with a 500 ms buffer. Focus from outside immediately sets the first item.
- Hover expands groups only after a mouse click/press on the menu (`dirty`) and with `auto_display`; in a narrow window the path does not change. A repeated click on an expanded item collapses it (after a hover that already expanded the item, a click, as in PrimeNG, will close it).
- A click outside and `resize` (not on touch devices) close the submenu; a click outside with the mobile panel open closes it.
- Mobile mode by `matchMedia` with `breakpoint` (default `960px`): button, `aria-expanded`, `Menu` group layer on the open list, focus returns to the button; resizing the window across the threshold closes the panel.

## Differences

| Difference | Reason |
|---|---|
| `auto_hide`: the cursor returning to the menu before `auto_hide_delay` elapses cancels the close | In PrimeNG the close goes through `debounce` without cancellation and fires even if the cursor returned. |
| Submenus are not mirrored at the right edge of the window | PrimeNG's menubar does not do this either: the position is set by the theme CSS (`left: 100%`). |
| ↑ ↓ → ← always cancel the default action | In PrimeNG ↓ on a root leaf and → on a submenu leaf leave it; page scrolling from arrows in the menu is not needed. |
| The button's `aria-label` comes from `locale` (`aria.navigation`) | Same in PrimeNG (`config.translation.aria.navigation`). |
| No `routerLink`, item `Tooltip`/`Badge`, `escape = false`, `Ripple`, `Bind`, `pTemplate`; icons are `ViewFn` | No router, `innerHTML` and PrimeIcons. |
| No `appendTo` and animations | The submenu is positioned by the theme CSS. |
| The `router` demo is not ported; icons without an analog in `prime-icons` are replaced by similar ones | No router and PrimeIcons. |

## What was not verified

- Right-to-left layout and touch input (on touch, hover expands groups immediately, as in PrimeNG, but there are no real touches in tests).
- Screen reading: `aria-*` were checked against PrimeNG, screen reader behavior was not.
- A submenu at the very right edge of the window (it overflows the window, as in PrimeNG).

## Performance

See `benches/` (`menubar-1`, `menubar-1000`, `case-menubar`) and `benches/*budgets.toml`.
