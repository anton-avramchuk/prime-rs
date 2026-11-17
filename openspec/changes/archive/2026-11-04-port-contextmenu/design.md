# Design

## Context

Requirements are in `specs/contextmenu`. PrimeNG 21.1.10 sources: `contextmenu/contextmenu.ts` (`ContextMenuSub` is a level, `ContextMenu` is state, trigger, position), `contextmenu/style/contextmenustyle.ts`; the demos `apps/showcase/doc/contextmenu/{basic,command,template}-doc.ts` (`router-doc`, `table-doc` are not ported). Model: `prime-tieredmenu` (model, path, focus, keyboard, search, submenu position, listener scheme).

Differences from `TieredMenu` per the source:
- No `popup`/`show_at`; showing is by event only (`show(event)` takes `pageX/pageY`).
- Position: `position()` is the cursor coordinates + flip/clamp by `getViewport()` and `scrollingElement`; a submenu uses `position(sublist)` in pixels (`left = item_width` or `-sub_width`).
- Trigger: a listener on `target` or `document` (`global`); on iOS/Android `touchstart` with a `pressDelay` timer and `touchend`.
- Closing by click ignores `button == 2` and `ctrlKey`; `resize` always closes.
- The root list is focused after showing (`onAfterEnter`).

## Goals / Non-Goals

**Goals:** the same DOM, classes, `aria-*`, keys and behaviour; exact position at the cursor.

**Non-Goals:** `Table`, `appendTo`, animation options, `routerLink`, tooltips, HTML labels, `Ripple`, `Bind`, `pTemplate`.

## Decisions

### D1. Logic reuse
`prime-contextmenu` depends on `prime-tieredmenu` and takes the pure model from it (`build`, `is_valid`, `item_id`, search, path transitions, `submenu_side`, `is_printable`), extending `pub use model::…` as needed. The level markup and state are written in the new crate on the model of `TieredMenu` (the classes differ: `p-contextmenu-*`); a shared level component is not extracted, since classes and `pt` differ. The markup duplication is deliberate: what to extract will be decided by a third consumer (Menubar/MegaMenu).

### D2. Position at the cursor
The pure function `place(page_x, page_y, width, height, viewport, scroll) -> (left, top)`: `+1`, flip, clamp, with table tests. The panel size is taken after insertion into the DOM (an effect under `cfg(wasm32)`); until computed the panel is hidden (`visibility: hidden` inline style) so it does not flash.

### D3. Trigger
`target: OptProp<String>` (a CSS selector resolved on mount; for an arbitrary element, `ContextMenuRef::show` from your own handler) and `global: bool`. Listeners are attached by an effect under `cfg(wasm32)` and removed in `on_cleanup`. A touch device is detected by `Navigator.userAgent` (iOS/Android), as in PrimeNG. `ContextMenuRef` (Copy) holds the `visible` signal and the coordinates.

### D4. Closing listeners
Click and `resize` on `window` for the duration of the show (the click via `set_timeout(0)`); a click closes if it is outside the root, not `button == 2` and without Ctrl. `watch_scroll` is not needed: PrimeNG's ContextMenu does not listen to scrolling.

### D5. State and keyboard
As in `TieredMenu`: `active_path`, `focus`, `dirty`, `search`, `mobile`; one `keydown` on the root list. `Escape`/`Tab` close the menu; there is no focus return to the target (PrimeNG has none either).

### D6. Tests
SSR: hidden by default; markup of a shown menu, groups, separators, `pt`, `item`. Pure `place` tests. Browser `tests/web.rs` as one sequential test (shared window listeners): `contextmenu` on a target → show at the cursor and `preventDefault`, `global`, outside click and right button, `resize`, a leaf, submenus and keys, search, long press, `item`/`submenu_icon`. e2e: theme styles, position at the cursor and flip at the edge, submenu right/left, keyboard, `command` with Toast.

### D7. Demos and benchmarks
`basic` (a picture and a Copy/Rename menu), `command` (Toast), `template`; no `router`/`table`. The bench `contextmenu-1`/`-1000` and `case-contextmenu` (with `dep:prime-menu`, `dep:prime-tieredmenu`).

## Risks / Trade-offs

- [Sizes before insertion] The position depends on browser sizes → a hidden first frame and e2e at the edge.
- [Markup duplication] Deliberate; removed by extraction after Menubar.
- [Touch input] Long press is not verified on a real device; recorded in docs.

## Open Questions

- None.

## Migration Plan

No migration. Rollback is `git revert`.
