# Design

## Context

Requirements are in `specs/panelmenu`. PrimeNG 21.1.10 sources: `panelmenu/panelmenu.ts` (`PanelMenuSub` — the list of a level's items, `PanelMenuList` — the tree inside a panel, `PanelMenu` — header panels), `panelmenu/style/panelmenustyle.ts`; demos `apps/showcase/doc/panelmenu/{basic,multiple,controlled,command,template}-doc.ts` (`router-doc` and `accessibility-doc` are not ported). Models in the project — `prime-accordion` (panels and collapse), `prime-tieredmenu` (item model, focus, search), `prime-rs-core::collapsible`.

Differences from the earlier menus: no popup levels and `z-index` layers, no outside click and `resize`; "visible" items are the expanded branches, the active path is a set of expanded keys rather than a single chain.

## Goals / Non-Goals

**Goals:** the same DOM, classes, `aria-*`, keys, expansion and animation.

**Non-Goals:** `routerLink`, `Tooltip`/`Badge`, HTML labels, `Ripple`, `Bind`, `pTemplate`, `transitionOptions`.

## Decisions

### D1. Expansion state
The set of expanded keys is `RwSignal<HashSet<String>>` (key = the item's `key` or a position like `0_1_2`, as in the source). Toggling: with `multiple = false` it clears expansion of siblings with the same parent; with `true` it only toggles. A pure function `toggle_key(set, key, parent_key_of_siblings, multiple)` with table tests. The initial set comes from the items' `expanded`.

### D2. Tree model and focus
A small flat model of our own in the crate (`nodes`, `visible(expanded)`, `next/prev/first/last`, `arrow_left/right`) after the source's `flatItems`/`onArrow*Key`; `prime-tieredmenu::model` is not reused, since its "visibility" is tied to popup paths. If `Node`/`build`/`item_at` turn out to fit without changes — take them (check during apply), otherwise do not complicate. Pure logic without `web_sys` — with table tests.

### D3. Markup
A recursive level function; a root item is drawn as a panel with a header (and its own tree inside `content`), nested ones — as `li.p-panelmenu-item`. Collapse areas — `prime_rs_core::collapsible::bind_region_with`, `CollapseIds` for `aria-controls`. Header buttons in PrimeNG are a `div` with an `a`; `ButtonPt` is not needed.

### D4. Focus and keyboard
A single `keydown`/`focus`/`blur` handler on the root list of the active panel; the focused key is a signal; when moving to the header of another panel, focus is moved into its list (`focus_no_scroll`). `scroll_into_view` for the focused `li` is under `cfg(wasm32)`; effects only under `cfg(wasm32)` (otherwise an executor panic in nextest).

### D5. Control
`PanelMenuRef` (`Copy`) with a reference to the key set and the model: `expand_all`, `collapse_all`, `toggle_all`.

### D6. Tests
SSR: markup, ARIA, initial expansion, hidden/disabled/separators, templates, `pt`. Browser `tests/web.rs` as one sequential test: click, single and multiple mode, command, disabled, keyboard, `expand_all`/`collapse_all`. e2e: theme styles, expand/collapse, keyboard.

### D7. Demos and benchmarks
`basic`, `multiple`, `controlled`, `command` (Toast), `template`. Bench `panelmenu-1`/`-1000`, `case-panelmenu`.

## Risks / Trade-offs

- [Double expansion on click] A click on a header and on a link inside must not fire twice → the handler is on `-header-content`/`-item-content`, as in the source.
- [Similarity to Accordion] We do not pull in all of `prime-accordion`: its value is a list of strings per panel, here — nested keys.
- [Model duplication with TieredMenu] Deliberate; extraction — after Tree.

## Open Questions

- How `multiple = false` behaves for nested branches — by the source's `activeItemPath.filter(parentKey !== …)` it is "one per level"; check during apply.

## Migration Plan

No migration. Rollback — `git revert`.
