# ContextMenu

Port of PrimeNG 21.1.10's `p-contextmenu` (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-contextmenu::ContextMenu`, `ContextMenuRef`, `ContextMenuPt`, `ContextMenuItemContext`, `place`/`Viewport` | `packages/primeng/src/contextmenu/contextmenu.ts` (`ContextMenu` and `ContextMenuSub`), `contextmenu/style/contextmenustyle.ts` |
| Transition model (path, focus, search) | shared with `TieredMenu`: `prime_tieredmenu::{Tree, State, Node, …}` |
| CSS | `@primeuix/styles/contextmenu` and the Aura preset: `cargo xtask gen-theme` (there is no `/* For PrimeNG */` block) |
| Demos | `apps/showcase/doc/contextmenu/*-doc.ts` (`basic`, `command`, `template`) and the added `nested` |

Spec: `openspec/specs/contextmenu`. The item model is `prime_menu::MenuItem` (the `items` field defines a submenu), commands are `MenuCommand`.

```rust
view! {
    <div id="area">"Right click here"</div>
    <ContextMenu target="#area" model=items />
}

// or manually
let menu = ContextMenuRef::new();
view! {
    <li on:contextmenu=move |e| menu.show(&e)>"Row"</li>
    <ContextMenu model=items popup_ref=menu />
}
```

## What matches PrimeNG

- Markup and classes: `div.p-contextmenu.p-component` (`p-contextmenu-mobile`) > `ul.p-contextmenu-root-list[role=menu]` > `li.p-contextmenu-item` (`-item-active`, `p-focus`, `p-disabled`, `data-p-*`, `aria-*`) > `div.p-contextmenu-item-content` > `a.p-contextmenu-item-link`; nested `ul.p-contextmenu-submenu`; separators. There is no panel in the DOM until the menu is shown.
- Showing: the `contextmenu` event (`trigger_event`) on a target or on the document (`global`); on iOS/Android a long press (`press_delay`, 500 ms). Showing cancels the default action and propagation, resets the path and focus, calls `on_show`.
- Position: (`pageX + 1`, `pageY + 1`), flip left/up if the panel does not fit, clamp to the left/top edge: the pure function `place`.
- Closing: a left click outside the menu (the right button and Ctrl-click do not close), `resize`, a leaf, `Escape`, `Tab`; the `Menu` group layer; `on_hide`.
- Submenus, the keyboard (↓ ↑ → ← `Home` `End` `Enter` `Space`, search by label start) and the expansion side are as in `TieredMenu`.

## Differences

| Difference | Reason |
|---|---|
| `target` is a CSS selector (resolved on mount) or `global`; for an arbitrary element, `ContextMenuRef::show` from your own `contextmenu` handler | There is no `HTMLElement` as a component property; `ContextMenuRef` covers the rest (including table and list rows). |
| No `appendTo`: the panel is output at the component's place and positioned absolutely; page coordinates are converted to the coordinates of the nearest positioned ancestor | `Menu` and `TieredMenu` work the same way. An ancestor with `overflow: hidden` will clip the panel. |
| A submenu is aligned in percent (`left: 100%` / `-100%`), as in `TieredMenu`, not in pixels | Shared model with `TieredMenu`; the result is the same. |
| No `motionOptions`, no enter and leave animations | The theme does not include animation classes. |
| No `routerLink`, item tooltips, `escape = false`, `Ripple`, `Bind`, `pTemplate`; icons are `ViewFn` | There is no router, `innerHTML` or PrimeIcons. |
| No integration with `Table` | The table is not ported. |
| The `router` and `table` demos are not ported; `basic` uses a block instead of a picture, `nested` is added | There is no router or network; `nested` shows a submenu. |

## What was not verified

- Long press (`touchstart`/`touchend` with the `press_delay` timer): neither by automated tests nor on a real device.
- Right-to-left layout.
- Screen reading: `aria-*` were checked against PrimeNG, screen reader behaviour was not.
- Positioning inside scrollable containers and ancestors with `transform`.

## Performance

See `benches/` (`contextmenu-1`, `contextmenu-1000`, `case-contextmenu`) and `benches/*budgets.toml`.
