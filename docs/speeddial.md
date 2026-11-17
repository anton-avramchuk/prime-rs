# SpeedDial

Port of `p-speeddial` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-speeddial::SpeedDial`, `SpeedDialItem`, `SpeedDialCommand`, `SpeedDialItemContext`, `SpeedDialType`, `SpeedDialDirection`, `SpeedDialPt` | `packages/primeng/src/speeddial/speeddial.ts`, `speeddial/style/speeddialstyle.ts` |
| CSS | `@primeuix/styles/speeddial` and the Aura preset — `cargo xtask gen-theme` (no `/* For PrimeNG */` block); buttons take `button` styles |
| Demos | `apps/showcase/doc/speeddial/*-doc.ts` (`linear`, `circle`, `semicircle`, `quartercircle`, `mask`, `tooltip`, `template`) |

Spec: `openspec/specs/speeddial`.

```rust
view! {
    <div style="position: relative; height: 300px">
        <SpeedDial model=items direction=SpeedDialDirection::Up style="position: absolute; right: 0; bottom: 0" />
    </div>
}
```

The root is not positioned by the theme: the container needs `position: relative`, and `SpeedDial` itself needs `position: absolute` via `style`.

## What matches PrimeNG

- Markup and classes: `div.p-speeddial.p-component.p-speeddial-<type>` (`p-speeddial-direction-<direction>` except circle, `p-speeddial-open`, `p-disabled`) > `p-speeddial-button` button (`p-speeddial-rotate`, `aria-expanded`, `aria-haspopup`, `aria-controls`) and `ul.p-speeddial-list[role=menu]` > `li.p-speeddial-item[role=menuitem]` (`p-hidden`, `p-focus`, `data-p-active`) > `p-speeddial-action` button; the `p-speeddial-mask p-overlay-mask` mask when `mask` is set and the list is shown.
- Layout: linear (`flex-direction` by direction), circle, semicircle and quarter circle with the same formulas (default radius `20 × item count`, `--item-diff-x/y` offsets from the difference between button and item sizes), the items' `transition-delay` grows toward the end when showing and toward the start when hiding.
- Clicking the button toggles display (`on_click`, `on_visible_change`, `on_show`, `on_hide`), clicking an item runs `command` and hides the list, outside click and `Escape` hide it (`Escape` returns focus to the button).
- Keyboard: on the button, ↓/← open the list and go to the first item, ↑/→ to the last; in the list the arrows follow the layout direction (the same table as in PrimeNG), `Home`/`End`, `Enter`/`Space` activate the focused item, focus returns to the button; `aria-activedescendant` points to the focused item.
- `tooltip_options` sets the item tooltip parameters (the text is the item label); `button_content`, `item_content`, `icon_content`, `show_icon`/`hide_icon`, `rotate_animation`.

## Differences

| Difference | Reason |
|---|---|
| Disabled and hidden (`visible = false`) items are skipped on arrow navigation | In PrimeNG the filter looks for an `a` inside the item, which the standard markup does not have, so focus does not skip disabled items. The port does what was intended. |
| `Enter`/`Space` on the item button itself is handled by its native click; the list reacts to them only when it has focus | In PrimeNG the item button has both `(click)` and `(keydown.enter)`, so `Enter` ran the command twice. |
| No `isItemClicked` flag: a click inside the component is not counted as "outside" thanks to a containment check | The window listener is attached while shown, on the next tick, so the click that opened the list does not catch it. |
| `SpeedDialItem` is our own type (no `routerLink`, `url`, item `tooltipOptions`, `icon` as a class) | No router and no PrimeIcons; `command` replaces routing. |
| Only `button_severity` instead of `buttonProps` (the button is always round) | The only property the demos use. |
| No `Ripple`, `Bind`, `pTemplate`, `unstyled` | Not ported in this project. |
| PrimeIcons icons in the demos are replaced by `prime-icons` icons; `pi-external-link` becomes a right arrow | No PrimeIcons. |
| `visible` is a two-way `RwSignal<bool>` instead of a `visible`/`visibleChange` pair | Leptos idiom. |

## Not verified

- Right-to-left layout and touch input.
- Screen reading: `aria-*` match PrimeNG, screen reader behavior was not checked.
- Diagonal directions for the linear and semicircle types: as in PrimeNG, no layout is defined for them.

## Performance

See `benches/` (`speeddial-1`, `speeddial-1000`, `case-speeddial`) and `benches/*budgets.toml`.
