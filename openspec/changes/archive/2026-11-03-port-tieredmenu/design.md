# Design

## Context

Requirements are in `specs/tieredmenu`. PrimeNG 21.1.10 sources: `tieredmenu/tieredmenu.ts` (about 1400 lines: the `TieredMenuSub` component for one level and `TieredMenu` with state and keyboard), `tieredmenu/style/tieredmenustyle.ts`; styles `@primeuix/styles/tieredmenu`; demos `apps/showcase/doc/tieredmenu/{basic,popup,command,template}-doc.ts` (`router-doc` is a router demo, not ported). Reference implementations in the project: `prime-menu` (the `MenuItem` model, `MenuRef`, popup: alignment, window listeners, focus), `prime-dock` (`matchMedia`), `prime-dialog`/`prime-speeddial` (keyboard and pure logic in `state.rs`).

Observations about the source:
- The "processed items" tree (`createProcessedItems`): each node is `{ item, index, level, key, parent, parentKey, items }`, the key is `"0"`, `"0_1"`, `"0_1_2"`.
- State: `activeItemPath` (a chain of processed items from the root to the deepest expanded group), `focusedItemInfo { index, level, parentKey, item }`, flags `dirty` (something was expanded by click at least once), `focused`, `searchValue`, `visible` (popup).
- The visible list (`visibleItems`) is the children of the active group whose key equals the focus `parentKey`, otherwise the root items. An item is valid (`isValidItem`) if it is not disabled, not a separator and visible.
- Click (`onItemClick`): if the item is already in the path, the path is trimmed up to it (collapsing the branch), focus stays on the item, `dirty = true`; otherwise the group is expanded (`onItemChange`), and a leaf closes everything (`hide`) and moves focus to the root item of the branch.
- `onItemChange`: items with the item's `parentKey` and the item's key are removed from the path, the group is added to the path, focus moves to the item; on hover in a narrow window (`queryMatches`) the path does not change.
- Hover (`onItemMouseEnter`): without touch input it fires only if `dirty`; a sublevel passes the event up only with `autoDisplay`.
- Keys: ↓ is the next valid item (or the first/selected one when focus is empty), ↑ is the previous (or the last), `Alt+↑` closes in popup; → expands the group, goes inside and takes the first item; ← moves up to the parent and collapses the submenu; `Home`/`End`; `Enter`/`Space` click the item link; `Escape` closes and returns focus; `Tab` activates the focused leaf and closes; a printable character is a search by label prefix among the visible items (the search wraps around from the current one), reset after 500 ms.
- Closing: `hide` resets the path and focus, and in popup also hides the panel; the click listener on the document stays attached while the path is not empty.
- Submenu position (`nestedPosition`): to the right of the item (`left: 100%`); if the right edge would go beyond the window, to the left (`left: -100%`).
- `TieredMenuSub` shows the submenu with an inline style `display: flex|none` depending on the item's activity.

## Goals / Non-Goals

**Goals:** the same DOM, classes, `aria-*`, keys, path behavior and popup.

**Non-Goals:** `appendTo`, `routerLink`, item tooltips, HTML labels, `Ripple`, `Bind`, `pTemplate`, changing `SplitButton`.

## Decisions

### D1. Model and crate
Items are `prime_menu::MenuItem` (it already has `items`, `separator`, `visible`, `disabled`, `command`, `url`, `target`, `title`, `icon`, `style_class`, `link_class`, `icon_class`). `TieredMenu` does not clone the tree on every render: a flat description keyed by key is built.

### D2. Pure logic (`model.rs`)
`Node { key, parent_key, level, index, item_path: Vec<usize> }` and functions over the tree: `build(items) -> Vec<Node>` (nodes in depth-first order), `children(nodes, parent_key)`, `is_valid(item)`, `visible_items(nodes, active_path, focus_parent)`, `find_next/prev/first/last/selected`, `search(visible, focus_index, text)`, `on_click(path, node, is_group, is_root) -> ClickOutcome { Collapse(path), Expand(path), Leaf }`, `item_change(path, node, hover, mobile)`, `arrow_left/right(...)`, `aria_set_size/pos_inset`, `item_id(menu_id, key)`, `submenu_side(left, item_width, sub_width, viewport, scrollbar)`. Table-driven tests for every function (including nested trees and disabled/hidden items).

### D3. Component state
`active_path: RwSignal<Vec<String>>` (keys), `focus: RwSignal<Focus { key: Option<String>, parent_key: String }>`, `dirty: StoredValue<bool>`, `focused: RwSignal<bool>`, `search: StoredValue<String>` with a reset timer, `mobile: RwSignal<bool>` (`matchMedia`, browser only). Tree nodes are a `Memo` over `model`.

### D4. Markup
A recursive function component `level_view(parent_key)` renders the `ul` and child `li`s with nested `ul`s for groups; the submenu's `display` is a reactive inline style driven by `active_path`. The label and icon are the item's `ViewFn`; the `item` view replaces the content of `div.p-tieredmenu-item-content`.

### D5. Submenu position
After the submenu is shown (an effect in the browser) the side is computed by `submenu_side` and written to the submenu's `left` (`100%` or `-100%`); the required width is taken from a hidden element by temporarily showing it (like `element_size` in `prime-dom`).

### D6. Popup
`TieredMenuRef` (like `MenuRef` from `prime-menu`: `new`, `show`, `show_at`, `toggle`, `hide`, `visible`). Showing/hiding, `AlignMode::Relative` alignment, the `ZGroup::Menu` layer, click/`resize`/scroll listeners: we copy the `prime-menu` scheme (subscriptions on the next tick, removal on hide and unmount). In non-popup mode the click listener stays attached while the path is not empty.

### D7. Keyboard and focus
A single `keydown` handler on the root `ul` (like `menuKeydown`), `aria-activedescendant` is a reactive attribute derived from `focus`. `Enter`/`Space` click the item's `a` (`element.click()`), as in PrimeNG. Search is a string buffer with a 500 ms timer via `set_timeout` (browser only).

### D8. Tests
SSR: root markup, items, groups (`aria-haspopup`, `aria-expanded`, `aria-setsize`, `aria-posinset`), separators, icons/labels, `popup` (no panel until shown), `pt`, `item`. Browser `tests/web.rs` as one sequential test (shared window listeners): a click on a group expands/collapses, a repeated click on a branch, a leaf invokes `command` and closes everything, hover after a click, `auto_display = false`, keys (↓/↑/→/←/`Home`/`End`/`Enter`/`Escape`/`Tab`), character search, disabled/hidden items, mobile (via a `breakpoint` larger than the window), popup (`toggle`, outside click, `Escape`, `Alt+↑`, focus return), `item`/`submenu_icon`. e2e: theme styles, expanding to the right, mirroring at the right edge of the window, keyboard, popup, `command` with Toast.

### D9. Demos and benchmarks
`basic` (the File/Edit/Users/Events/Quit tree as in PrimeNG, icons replaced with `prime-icons` and omitted where there is no equivalent), `popup`, `command` (Toast), `template`. Bench `tieredmenu-1`/`-1000` (a three-level tree) and `case-tieredmenu`.

## Risks / Trade-offs

- [State size] Many related signals; we reduce the risk by making all transition logic pure functions with tables.
- [Submenu position] Depends on browser sizes; the side is computed by a pure function and verified by e2e at the right edge.
- [Duplicated popup scheme] The listener scheme is copied from `prime-menu`; extracting the shared part into `prime-dom` is a separate change once there are more consumers (ContextMenu).

## Open Questions

- Whether to wire `TieredMenu` into `SplitButton`: left for a separate change.

## Migration Plan

No migration. Rollback is `git revert`.
