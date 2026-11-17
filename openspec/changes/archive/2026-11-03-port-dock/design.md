# Design

## Context

The requirements are in `specs/dock`. PrimeNG 21.1.10 sources: `dock/dock.ts` (about 350 lines), `dock/style/dockstyle.ts`; styles `@primeuix/styles/dock`; demos `apps/showcase/doc/dock/{basic,advanced}-doc.ts`. Templates in the project: `prime-menu` (the list of items, `MenuItem`, focus and keys via pure logic), `prime-breadcrumb` (its own `BreadcrumbItem` without router fields), `prime-badge`.

Observations on the source:
- In version 21 the "magnification" of items (`currentIndex`, the classes `second-prev` and the like) is absent from the styles: `onItemMouseEnter` only remembers the index. We carry it over as it is — without enlarging.
- The item identifier: `item.id` or the number `"0"`, `"1"`…; the focused item is stored by identifier (`focusedOptionIndex`). Selecting the next/previous one searches the list `li[data-pc-section=item][data-p-disabled=false]`, and `changeFocusedOptionIndex` clamps the index within the list of available items.
- The list is focused as a whole (`tabindex=0`); `onListFocus` puts the focus on index 0 of the available items, `onListBlur` resets it.
- Keys: depending on the orientation; `Home`/`End`; `Enter`/`Space` — a click on `a, button` inside the item.
- `matchMedia("(max-width: <breakpoint>)")` → `queryMatches` → the class `p-dock-mobile`; a `change` listener.
- The root is `position: absolute` (the theme), so the demo container must be `position: relative`.

## Goals / Non-Goals

**Goals:** the same DOM, classes, roles, keys and mobile mode.

**Non-Goals:** `routerLink`, `tooltipOptions`, `Ripple`, `Bind`, `pTemplate`, the `advanced` demo.

## Decisions

### D1. Crate and model
`prime-dock` with `Dock`, `DockItem`, `DockCommand { event, item }`, `DockPosition`, `DockPt`. `DockItem` is a builder (`DockItem::new().label(..).icon(..).command(..)`), following the sample of `MenuItem`. Its own model rather than `MenuItem`: dock must not depend on the `prime-menu` crate.

### D2. Pure logic (`state.rs`)
`root_classes(position, mobile, extra)`, `orientation(position)`, `item_classes(focused, disabled, extra)`, `item_id(list_id, item, index)`, `enabled_ids`, `key_action(position, code) -> Option<Action>` (`Next`, `Prev`, `First`, `Last`, `Activate`, `Ignore`), `step(ids, current, action) -> Option<usize>` with clamping within the bounds. Table tests.

### D3. Identifiers
An item without `id` gets `<list id>_<index>`, not a bare number: bare numbers collide with other `id`s on the page; the same identifier is used in `aria-activedescendant`. The difference is recorded in the docs.

### D4. State and markup
`focused_index: RwSignal<Option<usize>>` (the index among the available items). The list of items is built once from `model`; the state of an item (`p-focus`, `data-p-focused`) — reactive attributes. A click on an item — a direct handler; `Enter`/`Space` — `click()` on the `a` of the focused item (as in PrimeNG).

### D5. Mobile mode
`cfg(wasm32)`: `window.matchMedia("(max-width: {breakpoint})")`, the initial value `matches`, a `change` listener (a `wasm_bindgen` closure), removal in `on_cleanup`. The `web-sys` features are needed: `MediaQueryList`, `MediaQueryListEvent`.

### D6. Badge
`Badge` from `prime-badge` with the item's `value` and the badge's `style_class` (`badge_style_class`); without a value it is not rendered.

### D7. CSS
`dock` is added to `theme.components`. Check `p-dock-*` in the preset.

### D8. Tests
SSR: markup, positions, the item classes, the icon, the badge, `item`, `href`/`target`/`title`, `aria-*`, `pt`, `style_class`. Browser `tests/web.rs`: click and `command`, a disabled item, focus and keys (all four positions), `Home`/`End`/`Enter`/`Space`, `on_focus`/`on_blur`, the mobile mode (via `matchMedia` with a `breakpoint` of a very small and a very large value). e2e: the position, keys, the switcher of positions, a screenshot pair.

### D9. Demos and benchmarks
`basic`: radio buttons of the position and a `position: relative` container with a background; the icons — coloured tiles with emoji (PrimeNG's pictures from the CDN are not used), a hint `use:tooltip`. Bench `dock-1`/`-1000` and `case-dock`.

## Risks / Trade-offs

- [Identifier difference] The item identifier does not match PrimeNG (there it is a bare number); the behaviour is the same.
- [matchMedia] The listener is removed on unmount; in SSR the value is always "no".

## Open Questions

- Whether item magnification is needed: PrimeNG 21 does not have it.

## Migration Plan

No migration. Rollback — `git revert`.
