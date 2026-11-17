# PanelMenu

Port of PrimeNG 21.1.10 `p-panelmenu` (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-panelmenu::PanelMenu`, `PanelMenuPt`, `PanelMenuItemContext`, `PanelMenuRef` | `packages/primeng/src/panelmenu/panelmenu.ts` (`PanelMenu`, `PanelMenuList`, `PanelMenuSub`), `panelmenu/style/panelmenustyle.ts` |
| Model (expanded keys, visible items, transitions, search) | `prime-panelmenu::state` — our own, on `prime_tieredmenu::{Node, build, item_at}` |
| CSS | `@primeuix/styles/panelmenu`, the Aura preset and the `/*For PrimeNG*/` block — `cargo xtask gen-theme` (`panelmenu` in `primeng_extra`) |
| Demos | `apps/showcase/doc/panelmenu/*-doc.ts` (`basic`, `multiple`, `controlled`, `command`, `template`) |

Spec: `openspec/specs/panelmenu`. Item model — `prime_menu::MenuItem`: the `items` field defines branches, `expanded` — initial expansion, `command` — the action. An item key is the ancestors' indices joined by `_` (`"0"`, `"0_1"`), the same in `PanelMenuRef::is_expanded` and in the `id` of `li` elements (`<menu id>_<key>`).

```rust
let panel_ref = PanelMenuRef::new();
view! {
    <Button label="Toggle All" on_click=Callback::new(move |_| panel_ref.toggle_all()) />
    <PanelMenu model=items multiple=true panel_ref=panel_ref />
}
```

## What matches PrimeNG

- Markup and classes: `div.p-panelmenu` > `div.p-panelmenu-panel` > `div.p-panelmenu-header[role=button]` (`-header-active`, `p-disabled`, `aria-expanded`, `aria-controls`, `data-p-highlight`) > `-header-content` > `a.p-panelmenu-header-link` (chevron, icon, label) and `div.p-panelmenu-content-container[role=region]` > `-content-wrapper` > `-content` > `ul.p-panelmenu-root-list.p-panelmenu-submenu[role=tree]` > `li.p-panelmenu-item[role=treeitem]` (`aria-level`, `aria-setsize`, `aria-posinset`, `p-focus`, `data-p-focused`) > `-item-content` > `a.p-panelmenu-item-link`; nested branches — an expansion container with `ul.p-panelmenu-submenu`.
- Single mode: a click on a header collapses the other root panels (including via a leaf header); nested branches are independent. `multiple` lifts the restriction.
- Header keyboard: ↓ on an expanded one — into the tree, otherwise to the next header (wraps around, disabled ones are skipped); ↑ — to the previous one (into its tree if expanded); `Home` `End`; `Enter` `Space` — a click on the header link. Tree keyboard: ↓ ↑ over visible items (at the edges — to the adjacent header / its own header), → expands a branch, and on an expanded one goes down, ← collapses or goes to the parent, `Home` `End`, `Enter` `Space`, search by label prefix with a 500 ms buffer.
- Root lists get `tabindex="-1"` (like the resulting PrimeNG DOM, where a host binding overrides the `tabindex` property); item links get `tabindex="0"` while all ancestors are expanded, otherwise `-1`.
- Expanding and collapsing use the `p-collapsible-enter-active` / `-leave-active` animation from `prime_core::collapsible`.

## Differences

| Difference | Reason |
|---|---|
| ← on an expanded branch always collapses it | In PrimeNG a branch expanded by a click stays expanded because of the `expanded` flag left on the model (← removes it only from the active path). |
| Focus entering the tree from a header re-selects an item (the first or the clicked one), from outside — the last one | In PrimeNG an item is selected once, the previous one stays on return. |
| A leaf gets no expansion container and no empty `ul` | In PrimeNG they are rendered for leaves too, but are empty and hidden. |
| Expansion is stored in a key-set signal rather than in model fields; the initial state is taken from `MenuItem::expanded` once | Our `MenuItem` is an immutable model. `PanelMenuRef` replaces `collapseAll` and the manual editing of `expanded` from the `controlled` demo. |
| No item `key`: `PanelMenuRef` addresses items by position (`"0_1"`) | `MenuItem` has no `key` field. |
| No `tabindex` and `transitionOptions` props | The first does not affect the resulting DOM, the second is replaced by the shared animation. |
| No `routerLink`, `Tooltip`/`Badge`, `escape = false`, `Ripple`, `Bind`, `pTemplate`; icons are `ViewFn` | No router, `innerHTML` or PrimeIcons. |
| The `item` template applies to headers too (`PanelMenuItemContext::root`), `header_icon` is the header chevron | Like PrimeNG `#item` and `#headericon`. |
| The `router` and `accessibility` demos are not ported; icons without an analog in `prime-icons` are replaced with similar ones | No router or PrimeIcons. |

Along the way `prime_core::collapsible::animate` got a race guard: a late completion of the previous animation (the 400 ms fallback timer) no longer hides an already expanded area on a quick re-toggle; this affects Panel, Fieldset, Accordion.

## Not verified

- Right-to-left layout and touch input.
- Screen reading: `aria-*` were checked against PrimeNG, screen reader behavior was not.
- Character-by-character DOM comparison with PrimeNG (only a manual check and the `panelmenu-basic` screenshot in `e2e/tests/pages.json`).

## Performance

See `benches/` (`panelmenu-1`, `panelmenu-1000`, `case-panelmenu`) and `benches/*budgets.toml`.
