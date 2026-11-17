# Design

## Context

Requirements are in `specs/cascadeselect`. PrimeNG 21.1.10 sources: `cascadeselect/cascadeselect.ts` (`CascadeSelectSub` — level list, `CascadeSelect` — state, path, keyboard), `style/cascadeselectstyle.ts`; demos `apps/showcase/doc/cascadeselect/*-doc.ts`. Reference implementations — `prime-treeselect` (shell, panel inside the root, window listeners) and `prime-menubar` (path of active items, focus, search, `matchMedia`).

## Goals / Non-Goals

**Goals:** the same DOM, classes, `aria-*`, path and keys.

**Non-Goals:** arbitrary option objects, Angular forms, `appendTo`, mirroring the list at the window edge, `header`/`footer`/icon templates.

## Decisions

### D1. Model
`CascadeOption { label, value: Option<String>, disabled, children }`; the leaf value is `value` or else `label`. `build(options) -> Vec<Node>` with keys `0`, `0_1`, `0_1_2` (like `createProcessedOptions`), the node holds: `parent_key`, `level`, `index`, `group`, `valid`, `label`, `value`.

### D2. Pure logic (`state.rs`)
`State { active: Vec<String>, focus: Option<String>, dirty: bool }` and `Menu { nodes }` with functions following the source: `show` (path to the value and initial focus), `option_change`, `option_click` (result `Closed`/`Group`/`Select(value)`), `hover`, `arrow_down/up/left/right`, `home`/`end`, `enter`, `escape`/`tab`, `search`, `visible(focus)` — the list of the focus level. Focus is stored as a key; the index among visible ones is computed. All functions are pure, with table-driven tests.

### D3. Shell
Like `TreeSelect`: `open`, `focused`, references to the root, panel and input; open effects (`z-index` of the `Overlay` group, `align_overlay(Relative)`, outside-click and `resize` listeners) — under `cfg(wasm32)`. The panel is rendered only when open. After the path changes the panel is realigned (`set_timeout(0)`). `matchMedia(breakpoint)` provides `mobile`.

### D4. Nested lists
A recursive level function; a nested list is rendered only for the active group. The position of nested lists is set by the theme CSS (`left: 100%`). Signals are read during construction only via `*_untracked`.

### D5. Tests
Tables for the model; SSR: closed field, label, placeholder, states, clear, loading, `pt`; browser `tests/web.rs` as one sequential test: opening, cascade, leaf selection, clearing, outside click, hover, keyboard, mobile class; `sleep` after a key.

## Risks / Trade-offs

- [Focus and path] In PrimeNG focus stores the level index — here a key, so level transitions (`←`) are computed from the parent key; behavior is checked by a table.
- [Nested list position] Without `position()` a list may go beyond the right window edge.

## Open Questions

- What exactly `Tab` does on a leaf: `onOptionChange` without selection (verified: selects the path, the value does not change) — keep as is.

## Migration Plan

No migration. Rollback — `git revert`.
