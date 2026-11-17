# TieredMenu

Port of `p-tieredmenu` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-tieredmenu::TieredMenu`, `TieredMenuRef`, `TieredMenuPt`, `TieredMenuItemContext`, pure model (`Tree`, `State`, `Node`) | `packages/primeng/src/tieredmenu/tieredmenu.ts` (`TieredMenu` and `TieredMenuSub`), `tieredmenu/style/tieredmenustyle.ts` |
| CSS | `@primeuix/styles/tieredmenu` and the Aura preset: `cargo xtask gen-theme` (no `/* For PrimeNG */` block) |
| Demos | `apps/showcase/doc/tieredmenu/*-doc.ts` (`basic`, `popup`, `command`, `template`) |

Spec: `openspec/specs/tieredmenu`. The item model is `prime_menu::MenuItem` (the `items` field defines the submenu), commands are `MenuCommand`.

```rust
let menu = TieredMenuRef::new();
view! {
    <button on:click=move |e| menu.toggle(&e)>"Toggle"</button>
    <TieredMenu model=items popup=true popup_ref=menu />
}
```

## What matches PrimeNG

- Markup and classes: `div.p-tieredmenu.p-component` (`p-tieredmenu-overlay`, `p-tieredmenu-mobile`) > `ul.p-tieredmenu-root-list[role=menu]` > `li.p-tieredmenu-item` (`p-tieredmenu-item-active`, `p-focus`, `p-disabled`, `data-p-highlight|focused|disabled`, `aria-haspopup`, `aria-expanded`, `aria-setsize`, `aria-posinset`) > `div.p-tieredmenu-item-content` > `a.p-tieredmenu-item-link` with an icon, a label and a group arrow; nested `ul.p-tieredmenu-submenu`; separators `p-tieredmenu-separator`. A submenu is rendered only while its group is expanded.
- The transition state is a copy of the source, extracted into a pure model: the key tree (`0`, `0_1`, ...), the path of expanded groups, focus per level, the "expanded by click" flag (`dirty`), click (expand / collapse the branch / a leaf closes everything and moves focus to the root item of the branch), path change, hover (after a click; immediately on touch; in a narrow window the path does not change), `auto_display`.
- Keyboard: ↓ ↑ → ← `Home` `End` `Enter` `Space` `Escape` `Tab`, `Alt+↑` in `popup`, case-insensitive search by label prefix that wraps around, with a buffer reset after 500 ms; `aria-activedescendant`.
- Submenu position (`nestedPosition`): to the right of the item (`left: 100%`, `top: 0`), to the left (`-100%`) when there is not enough room on the right, taking the scrollbar and very narrow windows into account.
- `popup` mode: alignment to the target (`AlignMode::Relative`, minimum width from the target), the `Menu` group layer, an outside click / `resize` (not on touch devices) / scrolling an ancestor closes it, `Escape` returns focus to the target, `on_show`/`on_hide`.
- Mobile mode via `matchMedia` with `breakpoint` (default `960px`).

## Differences

| Difference | Reason |
|---|---|
| A click on a branch collapses it by key boundaries: key `"1"` is not considered an ancestor of `"10"` | PrimeNG checks `key.startsWith(p.key)` on the string, so with ten or more siblings collapsing affected unrelated branches. |
| Submenus have no `tabindex`, and `id`s are not repeated across the page: `<menu id>_<key>_list` | In PrimeNG all submenus share the same `id` `<menu id>_list` and `tabindex="0"`; this is a markup bug. |
| No `appendTo`: the popup menu is rendered in its own place in the markup (an ancestor with `position: relative` is required) | `Menu` and `Popover` work the same way. |
| No `showTransitionOptions`, `hideTransitionOptions`, `motionOptions`: the menu and submenus appear without an enter animation | The theme does not attach animation classes; PrimeNG sets `p-anchored-overlay`. |
| No `routerLink`, item tooltips, `escape = false` (HTML in the label), `Ripple`, `Bind`, `pTemplate`; icons are `ViewFn` | There is no router, PrimeIcons or `innerHTML`; the rest is not ported. |
| The `router` demo is not ported; PrimeIcons icons with no `prime-icons` equivalent are omitted in the demos | There is no router or PrimeIcons. |
| `SplitButton` still shows a flat `Menu`: wiring up `TieredMenu` is a separate change | It changes the `SplitButton` API and tests. |

## Not verified

- Right-to-left layout and touch input (on touch, hover expands groups immediately, as in PrimeNG, but there are no real touches in the tests).
- Screen reading: `aria-*` were checked against PrimeNG, screen reader behavior was not.
- Submenus deeper than three levels at the very edge of the window.

## Performance

See `benches/` (`tieredmenu-1`, `tieredmenu-1000`, `case-tieredmenu`) and `benches/*budgets.toml`.
