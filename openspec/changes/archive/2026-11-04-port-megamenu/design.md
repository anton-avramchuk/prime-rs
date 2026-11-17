# Design

## Context

Requirements are in `specs/megamenu`. PrimeNG 21.1.10 sources: `megamenu/megamenu.ts` (`MegaMenuSub` — a level, `MegaMenu` — state, mobile button), `megamenu/style/megamenustyle.ts`; demos `apps/showcase/doc/megamenu/{basic,vertical,command,template}-doc.ts` (`router` and `accessibility` are not ported). Models: `prime-menubar` (path, focus, search, window listeners, mobile button) and `prime-panelmenu` (own model on `prime_tieredmenu::Node`).

Differences from Menubar per the source:
- The tree of "processed items" has three levels: root item (`level 0`) → submenu in a column (`level 1`, key `<root>_<column>_<index>`) → submenu items (`level 2`, key `<submenu>_<index>`). `visibleItems` with an active root is a flat list of items of all submenus of its columns.
- Two orientations: root arrows depend on `orientation`; the group arrow is down (horizontal, mobile) or right.
- No `auto_hide`; `scroll_height` limits the root list.
- Submenus in columns are not nested popup levels but part of one overlay, so ARIA `level` is computed as in the source (`level + 1`).

## Goals / Non-Goals

**Goals:** the same DOM, classes, `aria-*`, keys and behavior.

**Non-Goals:** `routerLink`, `Tooltip`/`Badge`, HTML labels, `Ripple`, `Bind`, `pTemplate`, replacing the whole button (`#button`), the `accessibility` demo.

## Decisions

### D1. Model and pure logic
`MegaMenuItem`/`MegaMenuSubmenu` (fields above) and our own `state.rs`: building nodes with keys per the source scheme, `visible(active)`, transitions `arrow_down/up/left/right`, `home`/`end`, `escape`, `tab`, search, `click`/`hover` with `dirty`/`mobile`. First check `onArrow*Key`, `onItemChange`, `findNextItemIndex`, `findPrevItemIndex` of the source; reuse `prime_tieredmenu::{is_printable, aria_pos_inset, aria_set_size}` and `Focus`/`State` if they fit without changes, otherwise introduce our own types. All functions are pure, with table-driven tests.

### D2. Markup and state
`Core` modeled on Menubar: `state`, `focus`, `mobile`, `mobile_active`, search, `cleanups` of window listeners. Levels are recursive functions: root list, item, overlay (`columns_view`), submenu, submenu items. The column class is computed by the pure function `column_class(columns: usize, mobile: bool)` with a test. The overlay is rendered only for the active group.

### D3. Orientation and arrows
`MegaMenuOrientation { Horizontal, Vertical }`; the root class, `aria-orientation`, the group arrow and the choice of model functions for the root depend on it.

### D4. Mobile mode and listeners
As in Menubar: `matchMedia`, button, `Menu` layer, outside-click and `resize` listeners stay attached while the path is non-empty or the panel is open; focus returns to the button. `matchMedia` is a shared helper; if it is needed a second time, we keep a copy (extraction after the next consumer).

### D5. Tests
SSR: markup, orientations, columns and their classes, submenu headers, separators, disabled, hidden, slots, templates, `pt`. Browser `tests/web.rs` as one sequential test: click/repeated click, leaf and command, hover, outside click, keyboard (both orientations, columns, Escape), search, mobile button, unmounting; after a key `dispatch` wait `sleep(10)` (effects update the DOM). e2e: column layout by the theme, vertical orientation, keyboard, mobile button.

### D6. Demos and benchmarks
`basic`, `vertical`, `command` (Toast), `template`. Bench `megamenu-1`/`-1000`, `case-megamenu`.

## Risks / Trade-offs

- [Own model] Divergence from `TieredMenu` in column navigation → a separate `state.rs` with a table of tests for each arrow and both orientations.
- [Reading signals while building the view] Reading a signal inside the closure that builds the view recreates everything and loses focus → while building use only `*_untracked` / `with_untracked`.
- [Mobile mode] Depends on theme styles and `matchMedia`; verified by e2e on a narrow window.

## Open Questions

- How `onItemChange`/`hide` behave on a click on a submenu header (`submenuLabel`) — check during apply.
- Whether `focusedItemInfo.columnIndex` is needed for Up/Down within columns or the index in the flat list suffices — decide by `findNextItemIndex`.

## Migration Plan

No migration. Rollback — `git revert`.
