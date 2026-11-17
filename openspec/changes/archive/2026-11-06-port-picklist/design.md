# Design

## Context

Requirements are in `specs/picklist`. PrimeNG 21.1.10 sources: `picklist/picklist.ts` (`moveRight`, `moveAllRight`, `moveLeft`, `moveAllLeft`, `moveUp/Top/Down/Bottom`, `onDrop`), `style/pickliststyle.ts`, `@primeuix/styles/picklist`. Model: `prime-orderlist` (buttons + `Listbox`, `reorder`).

## Goals / Non-Goals

**Goals:** the same DOM and behaviour of the buttons and transfer.

**Non-Goals:** drag and drop, the vertical responsive view, Angular templates.

## Decisions

### D1. Transfer (`transfer.rs`)
Functions over `Vec<ListboxOption>`: `move_selected(from, to, selection) -> Vec<ListboxOption>` and `move_all(from, to, visible) -> Vec<ListboxOption>`; a duplicate is determined by value. Visibility under the search is `prime_listbox::matches_filter(label, text)`, the search text comes from each list's `on_filter`.

### D2. Reordering
`prime_orderlist::reorder` over the keys of the selected ones, then sorting the list by the resulting order.

### D3. Component
Two `Listbox`es (`multiple`, `values` = the list's selection, `header_content` for `source_header`/`target_header`, `on_dbl_click` moves the selected ones). Buttons are `Button` with `severity=secondary`. `Listbox::header_content` was added as the only change to `prime-listbox`.

### D4. Tests
Transfer — tables. SSR: five blocks, button state, headers, `show_*_controls`. Browser `tests/web.rs`: selection and transfer in both directions, "all", reordering, double click, `keep_selection`, search and "all", `disabled`. e2e: styles and clicks.

## Risks / Trade-offs

- [Selection order] transfer follows the selection order, as in PrimeNG.

## Open Questions

- None.

## Migration Plan

No migration. Rollback — `git revert`.
