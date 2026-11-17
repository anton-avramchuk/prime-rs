# Proposal

## Why

PanelMenu is the third menu of wave 5: a hybrid of Accordion and Tree, a vertical list of panels, each expanding in place (no popup submenus). The expansion state, the collapse animation and the header markup are close to `Accordion` (`prime-rs-core::collapsible`), while the item model and tree keyboard (↑ ↓ ← → `Home` `End` `Enter`) are close to `prime-tieredmenu`, but with a different notion of a "visible item" (expanded branches rather than popup levels).

## What Changes

- New crate `prime-panelmenu` (port of PrimeNG 21.1.10 `p-panelmenu`). Item model — `prime_menu::MenuItem` (`key`, `expanded`, `disabled`, `visible`, `separator`, `items`, `command`).
- `PanelMenu`: root `div.p-panelmenu.p-component` (`id`, `style`, `style_class`) > for each root item `div.p-panelmenu-panel` > `div.p-panelmenu-header` (`p-panelmenu-header-active`, `p-disabled`; `aria-expanded`, `aria-controls`, `aria-disabled`, `data-p-highlight`) > `div.p-panelmenu-header-content` > `a.p-panelmenu-header-link` (`p-panelmenu-header-icon`, `p-panelmenu-header-label`, `p-panelmenu-submenu-icon` — chevron right/down) + `div.p-panelmenu-content-container` (`p-panelmenu-expanded`) > `div.p-panelmenu-content-wrapper` > `div.p-panelmenu-content` > `ul.p-panelmenu-root-list[role=tree]` > `li.p-panelmenu-item[role=treeitem]` (`aria-level`, `aria-setsize`, `aria-posinset`, `aria-expanded`, `p-focus`, `p-disabled`) > `div.p-panelmenu-item-content` > `a.p-panelmenu-item-link` (`-item-icon`, `-item-label`, `-submenu-icon`) + nested `ul.p-panelmenu-submenu`; separators `p-menuitem-separator`.
- Behavior: a click on a header/group toggles expansion; with `multiple=false` one root panel is expanded (expanding another collapses the previous one), with `multiple=true` — any number; a leaf invokes `command`; the initial state is taken from the items' `expanded`; controlled expansion — via `PanelMenuRef` (`expand_all`/`collapse_all`/`toggle_all`) and/or a signal of the set of expanded keys (by `key`, as in the `controlled` demo). Expanding/collapsing is the `prime-rs-core::collapsible` animation.
- Keyboard (focus on the root, `aria-activedescendant`): ↓ ↑ over visible items and headers, → expands a branch (or goes down), ← collapses (or goes to the parent), `Home`/`End`, `Enter`/`Space` — activation (toggle/command). `Tab` leaves the component; unavailable items are skipped.
- Props: `model`, `style`, `style_class`, `multiple` (default `false`), `id`, `tabindex` (0); views `item` (replaces the `-item-content` content) and `submenu_icon`; `on_focus`/`on_blur` events are not included — they are absent in the source.
- Page `/panelmenu` (`basic`, `multiple`, `controlled`, `command`, `template`), e2e, benchmarks and budgets, `docs/panelmenu.md`, `panelmenu` in `theme.components` (and in `primeng_extra`: the styles source has a `/*For PrimeNG*/` block).
- **Out of scope:** `routerLink` and the `router` demo, item `Tooltip`/`Badge`, HTML labels (`escape: false`), `Ripple`, `Bind`, `pTemplate`, `transitionOptions` (a single animation), the `accessibility` demo as separate code.

## Capabilities

### New Capabilities
- `panelmenu`: a vertical accordion menu with nested branches, keyboard and controlled expansion.

### Modified Capabilities
<!-- None. -->

## Impact

- New crate `crates/prime-panelmenu` (`publish = false`); dependencies: `prime-rs-core` (`collapsible`, `ids`), `prime-dom`, `prime-menu`, `prime-icons` (chevronright, chevrondown; several icons for the demo), if needed the pure model from `prime-tieredmenu` (only what fits; otherwise our own flat model in the crate).
- Changed: workspace `Cargo.toml`, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (browser job, three places). The behavior of existing crates does not change.
