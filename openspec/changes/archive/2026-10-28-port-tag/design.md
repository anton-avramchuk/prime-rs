# Design

## Context

Requirements are in `specs/tag`. PrimeNG 21.1.10 sources: `packages/primeng/src/tag/tag.ts`, `tag/style/tagstyle.ts`; demos in `apps/showcase/doc/tag`. Tag is a `<p-tag>` host with classes on it, inside it `ng-content`, an icon (`span.p-tag-icon`, if set) and a label (`span.p-tag-label`, always, even when empty). Root classes: `p-tag p-component`, severity, `p-tag-rounded`; `data-p`: `rounded`, severity.

The template from `docs/button.md`: `Prop`/`OptProp` properties, no `Memo`, properties in one `Arc<Props>`, `all_static!` chooses the static or reactive branch, the markup once in `macro_rules!`, parity tests.

## Goals / Non-Goals

**Goals:** a Tag matching PrimeNG in classes, `data-p` and content order; branches without `RenderEffect` for static properties; budgets.
**Non-Goals:** font icons, `unstyled` mode.

## Decisions

### D1. The root is a `<span>`
As in Badge: the `<p-tag>` host is an Angular detail, CSS is bound to classes.

### D2. Icon and content
`icon: Option<ViewFn>` in `<span class="p-tag-icon">` (always a wrapper, as in PrimeNG with a template icon); `children: Option<Children>` come first. The label is always rendered (the same in PrimeNG).

### D3. Severity
Its own enum `TagSeverity` (`Secondary`, `Info`, `Success`, `Warn`, `Danger`, `Contrast`), like `BadgeSeverity` but a separate type of the crate, so that components do not depend on each other.

### D4. State and branches
Pure functions `write_classes`/`data_p` over a borrowed `TagState`; the branch is chosen by `all_static!(value, severity, rounded, style, style_class)`; the icon and label blocks are computed by the functions `icon_of`/`label_of`.

## Risks / Trade-offs

- [Duplicating the severity enum with Badge] → deliberate: components are independent; a shared type may appear in `prime-core` when there are more than three of them.
- [Branch divergence] → a parameterized parity test.

## Migration Plan

No migration: a new crate. Rollback is `git revert`.

## Open Questions

None.
