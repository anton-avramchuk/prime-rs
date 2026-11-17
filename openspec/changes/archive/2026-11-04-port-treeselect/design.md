# Design

## Context

Requirements are in `specs/treeselect`. PrimeNG 21.1.10 sources: `treeselect/treeselect.ts`, `treeselect/style/treeselectstyle.ts`; demos `apps/showcase/doc/treeselect/*-doc.ts`. Models: `prime-select` (shell, panel inside the root, `align_overlay`, window listeners, `ZGroup::Overlay`) and `prime-tree` (inside the panel).

## Goals / Non-Goals

**Goals:** the same DOM, classes, `aria-*`, keys and selection through a shared `Tree`.

**Non-Goals:** virtual scrolling, `appendTo`, Angular forms, `header`/`footer`/`empty`/icon templates, hidden panel focus traps.

## Decisions

### D1. Value and tree
The value is `RwSignal<Vec<String>>` of keys (in `Single` — 0 or 1). The same signal is passed to `Tree` as `selection`, the expanded keys are our own `RwSignal<HashSet<String>>` in `expanded_keys`. The label takes node captions by keys from `options`. Selection mode `Single` by default (unlike `Tree`).

### D2. Shell
`Props`-like state `Core`: `open`, `focused`, `value`, `expanded`, references to the root, panel and hidden field. Opening side effects (`Overlay` group z-index, `align_overlay(Relative)`, outside-click and `resize` listeners, focus) — under `cfg(wasm32)` in an `Effect`, as in `Select`. The panel is built only in the open state, so the tree filter resets by itself.

### D3. Opening
`show()` expands the ancestors of the selected keys (the pure function `ancestors_of(options, keys)` with a test) and opens; `hide()` closes. After a node is expanded/collapsed the panel is re-aligned in `set_timeout(0)`. A click on the root toggles the panel, except clicks inside the panel and on the clear icon.

### D4. Keyboard
Handler on the hidden field: Down — open and, on the next tick, focus the first `li[role=treeitem]` of the panel; `Space`/`Enter` — open; `Escape` — close and return focus; `Tab` — close.

### D5. Tests
The pure ancestors function — a table-driven test. SSR: markup of the closed field, label (comma, chips, placeholder), state classes, clear, `pt`. Browser `tests/web.rs` as one sequential test: open/close, single and multiple selection, checkboxes, clear, keyboard, outside click, expanding ancestors, filter. e2e: theme layout, panel below the field, selection.

## Risks / Trade-offs

- [Shared selection signal] `Tree` and the shell write one signal → one source of truth; in `Single` `Tree` itself replaces the selection.
- [Focus in the panel] Without focus traps `Tab` inside the panel leaves the component → `Tab` on the field closes the panel, inside the panel we do not close on `focusout` (an outside click closes).

## Open Questions

- Whether the panel must close on `Tab` from the tree — check against `onTabKey` (in PrimeNG focus goes to hidden traps).

## Migration Plan

No migration. Rollback — `git revert`.
