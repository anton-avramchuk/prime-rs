# Menu

Port of `p-menu` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-menu::Menu`, `MenuPt`, `MenuRef`, `MenuItem`, `MenuCommand` | `menu/menu.ts`, `menu/style/*` |
| CSS | `@primeuix/styles/menu` — `cargo xtask gen-theme` (no `For PrimeNG` block; the root `position` is an inline style written by the component itself) |
| Demos | `apps/showcase/doc/menu/{basic,group,command,popup,template}-doc.ts` |

Spec: `openspec/specs/menu`.

## Usage

A static menu — just `model`:

```rust
view! { <Menu model=vec![MenuItem::new().label("New"), MenuItem::new().label("Search")] /> }
```

A popup — `popup=true` and a `MenuRef` created in advance (the analog of `menu.toggle($event)`):

```rust
let menu = MenuRef::new();
view! {
    <div style="position: relative">
        <Menu popup=true menu_ref=menu model=model />
        <Button label="Options" on_click=Callback::new(move |e: web_sys::MouseEvent| menu.toggle(&e)) />
    </div>
}
```

An item with `items` becomes a group header; `MenuItem::separator()` is a separator; `command` receives `MenuCommand { event, item }`.

## What matches PrimeNG

- Markup and classes: `div.p-menu.p-component` (`p-menu-overlay` and `position: absolute` for the popup) > `p-menu-start` / `ul.p-menu-list[role=menu]` / `p-menu-end`; items `li.p-menu-item[role=menuitem]` > `p-menu-item-content` > `a.p-menu-item-link` with icon and label; `li.p-menu-submenu-label`, `li.p-menu-separator`; `p-focus`, `p-disabled`, `data-p-focused`, `data-p-disabled`, `aria-disabled`, `aria-activedescendant`.
- Item identifiers (`<menu id>_<i>[_<j>]` or the item's `id`); a model with groups is detected the same way as `hasSubMenu` (at least one item has subitems).
- Click: navigation is cancelled for an item without `url` and for a disabled one, `command`, closing the popup menu.
- Keys: Down/Up (the last and first stay in place), Home, End, Enter / NumpadEnter / Space (click on the link), Escape / Tab and Alt+Up in popup — hide and return focus to the target; disabled items are skipped.
- Popup: alignment to the target (`align_overlay`), `z-index` of the `Menu` group (`auto_z_index`, `base_z_index`), focus on the list, closing by an outside click, `resize` (except on touch devices) and scrolling of a target ancestor, `on_show` / `on_hide`.

## Differences

| Difference | Reason |
|---|---|
| `MenuItem` is our own type without `routerLink`, `queryParams`, `badge`, `tooltipOptions`, `escape`, `automationId`, `labelClass`, `iconStyle`; the icon is a `ViewFn` (a component from `prime-icons`), not a font class name | No Angular router; a badge and tooltip can be put in the item template. |
| `label` and `disabled` are values, not functions | Reactivity comes from `model` as a signal. |
| The popup panel is rendered inside the root, not in `body` (no `appendTo`); the parent is `position: relative` without `overflow: hidden` | `Popover` works the same way. |
| No `@primeuix/motion`, `motionOptions`, `show/hideTransitionOptions`: the panel appears and disappears without animation | Animations are not ported. |
| Custom views are `item`, `submenu_header` (`Callback`), `start`, `end` (`ViewFn`), not `pTemplate` | Analog of templates. |
| An invisible group is hidden together with its items | In PrimeNG the items of a hidden group with their own `visible` remain. |
| `pt` has no context (`item`, `index`, `focused`, `disabled`), no `Bind` and `Ripple` | `pt` are simple additions of classes, style and attributes. |
| Keys work on the model, not on the DOM | Same order; disabled items are skipped. |
| The `router` demo is not ported; `command` shows a `Toast`; the icons `cog`, `sign-out`, `ellipsis-v` are replaced with the closest ones from `prime-icons`, `template` has no logo and avatar | No router and no such icons. |

## Cost

158 allocations per instance in SSR (three items); contribution to wasm ≈ 383 KB (budget 400 KB in `benches/budgets.toml`).

## Not verified

Touch, `rtl`, screen readers, scrolling of a target ancestor with real input, closing on `resize` on a touch device. Verified in Chrome (browser tests and e2e with a real mouse and keys): markup, groups and separators, invisible and disabled items, click and `command`, focus and all keys, showing the popup at a button, closing by an outside click, Escape, Alt+Up, item selection and `resize`, `z-index`, `on_show` / `on_hide`, templates. Screenshot comparison with PrimeNG (`pages.json`) was not run.
