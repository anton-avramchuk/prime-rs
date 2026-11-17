# Design

## Context

The requirements are in `specs/menubar`. PrimeNG 21.1.10 sources: `menubar/menubar.ts` (`MenubarSub` — a level, `Menubar` — the state, the mobile button, auto-hide), `menubar/style/menubarstyle.ts`; demos `apps/showcase/doc/menubar/{basic,command,template}-doc.ts` (`router-doc` is not ported). Templates — `prime-tieredmenu` and `prime-contextmenu` (the model, the path, focus, search, the side of the submenu, listeners).

Differences from `TieredMenu`/`ContextMenu` according to the source:
- The root is horizontal (`role=menubar`): ← → move along the root, ↓ opens the group (in `TieredMenu` ↓ is the next item).
- `mobileActive`, the button with `aria-expanded`, the layer on the root list when opened, return of the focus to the button; `queryMatches` by `breakpoint` gives `p-menubar-mobile`.
- `autoHide`/`autoHideDelay` — closing on `mouseleave` with a delay (`Subject` with `debounce`).
- The click-outside and `resize` listeners hang while the path is not empty; a click outside while the mobile panel is open closes it.
- The slots `start`/`end`.

## Goals / Non-Goals

**Goals:** the same DOM, classes, `aria-*`, keys and behaviour.

**Non-Goals:** `routerLink`, `Tooltip`/`Badge`, HTML labels, `Ripple`, `Bind`, `pTemplate`, `appendTo`, animations.

## Decisions

### D1. Model
We reuse `Tree`/`State` from `prime-tieredmenu`. The differences of the root (← → and ↓) are separate model methods (for example `root_arrow_left/right/down`), chosen after reading the source's `onArrowDownKey/onArrowRightKey/onArrowLeftKey`, so as not to change the behaviour of `TieredMenu`. The new functions get table tests in `prime-tieredmenu`.

### D2. Markup and state
Following the sample of `ContextMenu`: `Core` with `state`, `focus`, `sides`, `mobile`, `mobile_active`, the search; the levels by a recursive function; the classes `p-menubar-*`. The root list on a narrow window is hidden by the theme styles, shown by the class `p-menubar-mobile-active`.

### D3. Submenus
The side of the nested submenus — `submenu_side`, as in `TieredMenu`; the submenus of the root open downwards by the theme CSS.

### D4. Auto-hide
`mouseleave` of the root starts the timer `auto_hide_delay`, `mouseenter` cancels it; the timer is in `StoredValue`, reset on unmount (wasm only).

### D5. Mobile button
`mobile_active` — a signal; opening issues the layer `Menu` (`zindex::generate`) to the root list and on the next tick makes the first showing; closing — `hide` and the focus on the button (`set_timeout(0)`). The click-outside listener distinguishes the root list and the button.

### D6. Tests
SSR: markup, slots, the button, groups, separators, `pt`, templates. Browser `tests/web.rs` as one sequential test: click/repeated click, a leaf and a command, hovering, `auto_hide`, a click outside, the keys of the root and the submenu, search, the mobile button (`breakpoint` larger than the window), templates, unmounting. e2e: the horizontal layout of the theme, a submenu under the item, the keyboard, the mobile button on a narrow window.

### D7. Demos and benchmarks
`basic`, `command` (Toast), `template` (`start`/`end`, a custom item). Bench `menubar-1`/`-1000`, `case-menubar`.

## Risks / Trade-offs

- [Horizontal navigation] The divergence from `TieredMenu` in the keys of the root → separate model functions with tests, `TieredMenu` is not touched.
- [Mobile mode] Depends on the theme styles and `matchMedia`; checked by e2e on a narrow window.
- [Duplicated markup] Deliberate (as in ContextMenu); extraction — after MegaMenu/PanelMenu.

## Open Questions

- Whether the `Menu` layer applies to submenus on desktop (`autoZIndex` in `MenubarSub`) — to be clarified against the source during apply.

## Migration Plan

No migration. Rollback — `git revert`.
