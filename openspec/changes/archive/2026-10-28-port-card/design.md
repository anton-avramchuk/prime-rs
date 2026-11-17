# Design

## Context

Requirements are in `specs/card`. PrimeNG 21.1.10 sources: `packages/primeng/src/card/card.ts`, `card/style/cardstyle.ts` (classes `root`, `header`, `body`, `title`, `subtitle`, `content`, `footer`; also `caption`, which is not used in the template), demos `apps/showcase/doc/card` (`basic`, `advanced`). Template — `docs/button.md`.

## Goals / Non-Goals

**Goals:** Card with the same blocks, order and slot priority; branches without `RenderEffect` for static; budgets.
**Non-Goals:** `BlockableUI`, `unstyled`, `p-header` and `p-footer` as facets (in Leptos these are `ViewFn` slots).

## Decisions

### D1. Slots — `Option<ViewFn>`
Four slots (`header_template`, `title_template`, `subtitle_template`, `footer_template`) — `Option<ViewFn>`, content — `children`. The names repeat the PrimeNG templates (`#header`, `#title`, `#subtitle`, `#footer`).

### D2. Slot staticness
`ViewFn` and `children` are static by nature (render closures), so the branch is chosen by `all_static!(header, subheader, style, style_class)`; the blocks are computed by the functions `header_block`, `title_block`, `subtitle_block`, `footer_block` with a shared result `Option<AnyView>`.

### D3. Root — `<div>`
The `<p-card>` host is an Angular detail; the extra `display: block` from `cardstyle.ts` is not needed for `<div>`.

## Risks / Trade-offs

- [Slots are computed once, not by signals] → slots accept a `ViewFn`, which itself may be reactive inside; only the text properties are reactive.
- [Branch divergence] → a parameterized parity test.

## Migration Plan

No migration: a new crate. Rollback — `git revert`.

## Open Questions

None.
