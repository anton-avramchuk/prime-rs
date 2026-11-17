# Design

## Context

Requirements are in `specs/menu`. PrimeNG 21.1.10 sources: `menu/menu.ts`, `menu/style/menustyle.ts`; demos `apps/showcase/doc/menu/*-doc.ts`. Models in the project: `prime-popover` (`PopoverRef`, panel inside the root, `align_overlay(Relative)`, window listeners in `set_timeout(0)`), `prime-breadcrumb` (item model without a router), `prime-select` (keys over a list, focus).

Observations about the source:
- One template: root → `start` → `ul[role=menu]` → items (flat or by groups: `submenu.items`) → `end`. A group is `li.p-menu-submenu-label` + its items; a separator is `li.p-menu-separator`.
- Item identifier: `item.id ?? "<menuId>_<i>[_<j>]"`; the focused item is stored by this id (`focusedOptionIndex`), `p-focus` and `aria-activedescendant` are taken from it.
- Keys work on the DOM: "available" are `li[data-pc-section=item][data-p-disabled=false]`; Down/Up — the current index ±1 (first/zero by default), Home/End — first/last; Enter/Space click the item's `a`.
- Focus on the list in static mode makes the first item focused; losing it resets.
- Popup: `toggle(event)` remembers `currentTarget`, shows; after showing — `position: absolute; top: 0`, `z-index`, document (click), window (resize, not on touch devices), target ancestor scroll listeners, `absolutePosition`, focus on the list. `preventDocumentDefault` — a click inside the panel does not close it as "outside".
- Root inline style: `position: absolute` (popup) or `relative`; it is not in the theme CSS.
- The `p-anchored-overlay` animation goes through `@primeuix/motion`; we do not port it (as in Popover).

## Goals / Non-Goals

**Goals:** the same DOM, the same classes and roles, key and closing behavior, `MenuRef` for popup mode.

**Non-Goals:** `routerLink`, `appendTo`, `@primeuix/motion`, `tooltipOptions`, `badge`, HTML labels (`escape`), `Ripple`, `Bind`, `pTemplate`.

## Decisions

### D1. Crate and model
`prime-menu` with `Menu`, `MenuRef`, `MenuItem`, `MenuCommand { event, item }`, `MenuPt`. `MenuItem` is a builder (`MenuItem::new(label)`, `.icon(..)`, `.url(..)`, `.command(..)`, `.items(..)`, `MenuItem::separator()`). `model` is `Prop<Vec<MenuItem>>` (a value or a signal).

### D2. Pure logic (`state.rs`)
- `root_classes(popup, style_class)`, `item_classes(item, focused)`, `root_style(popup)`.
- `item_id(item, menu_id, i, j)`.
- `flatten(model) -> Vec<Row>`: a flat list of rows (`Separator`, `Header(label)`, `Item{ id, item }`) taking `visible` of items and groups into account — a single source for markup and keys.
- `enabled_ids(rows) -> Vec<String>`; `next_id(ids, current)`, `prev_id(ids, current)` (like `findNext/PrevOptionIndex`), `first`, `last`.
- `should_prevent_default(item)` (no `url`) and `can_run_command(item)`.
Table-driven tests for each. Keys work on this data, not on the DOM.

### D3. State and markup
Inside `Menu`: `focused_id: RwSignal<Option<String>>`, `list_focused: RwSignal<bool>`, `visible: RwSignal<bool>` (popup). Rows come from `flatten` via a `Memo`; the list is drawn by `For` over row `id`s. In the list closure read the focus signal only with `*_untracked` — otherwise all items are rebuilt and focus is lost (lesson from SelectButton); `p-focus`/`aria-*` — via reactive attributes of each item.

### D4. Popup
`MenuRef` (`Copy`) like `PopoverRef`: `visible`, `target` in `StoredValue<_, LocalStorage>`; `toggle/show/hide`. After the panel is inserted (the effect watches the `NodeRef`, lesson from Popover): `align_overlay(Relative)`, `z-index` (`zindex::generate(ZGroup::Menu, ..)`, `revert` on hide), focus on the list, `on_show`. Document (click) and window (resize, target ancestor scroll) listeners are attached in `set_timeout(0)` and removed in `on_cleanup` and on hide. A click inside the panel does not close it (`contains` check). On a touch device — resize does not close.

### D5. Keys
List `keydown`: ArrowDown/Up (Alt+Up in popup — close and focus the target), Home, End, Enter/NumpadEnter/Space (click on the `a` of the focused item; in popup first focus the target), Escape/Tab (in popup — focus the target and `hide`). `preventDefault` in the same places as in PrimeNG.

### D6. Views
`item: Option<Callback<MenuItem, AnyView>>`, `submenu_header: Option<Callback<MenuItem, AnyView>>`, `start`/`end: Option<ViewFn>`.

### D7. CSS
`menu` is added to `theme.components`. The `For PrimeNG` block is not needed (position is an inline style, D2). Check that `p-menu-*` and `p-menu-overlay` exist.

### D8. Tests
SSR: markup, groups, separators, invisible, `data-p`, popup is not rendered, `style_class`, `pt`. Browser `tests/web.rs` (one sequential run — window listeners are shared): click and `command`, disabled item, keys, focus and `aria-activedescendant`, popup `toggle/show/hide`, outside click, Escape, resize, layer, `on_show/on_hide`, templates, unsubscribing on unmount. e2e: a real mouse and keys. The crate is added in three places of `ci.yml`.

### D9. Benchmarks
Scenarios `menu-1`/`-1000` (three items) and `case-menu`; first `cargo bench -p render-bench --bench alloc`, for wasm — a temporary large budget (replace by a unique line/line number).

## Risks / Trade-offs

- [Focus] Rebuilding the list drops focus → only `*_untracked` in list closures.
- [Shared window listeners] Several menus in one test → one sequential run.
- [Alignment] The parent is `position: relative` without `overflow: hidden` (as in Popover); recorded in docs.

## Open Questions

- Whether a show animation is needed: check whether the theme has `@keyframes` for `p-anchored-overlay`; if not — no animation.

## Migration Plan

No migration. Rollback — `git revert`.
