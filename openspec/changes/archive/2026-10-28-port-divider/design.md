# Design

## Context

Requirements are in `specs/divider`. PrimeNG 21.1.10 sources: `packages/primeng/src/divider/divider.ts`, `divider/style/dividerstyle.ts` (classes and `inlineStyles`), demos `apps/showcase/doc/divider`. The `<p-divider>` host carries the classes, `role=separator`, `aria-orientation`, the inline style and `data-p`; inside there is always `<div class="p-divider-content">`. `data-p` is built from `align`, `layout`, `type` (exactly in this order, `align` only if set). Template - `docs/button.md`.

## Goals / Non-Goals

**Goals:** Divider with the same classes, style and attributes; a static branch without `RenderEffect`; budgets.
**Non-Goals:** `unstyled`.

## Decisions

### D1. The root is a `<div>`
The `<p-divider>` host is an Angular detail.

### D2. The `type` property name
In Rust the property is called `divider_type` (like `button_type` on Button): `type` is a keyword. There is no such attribute in the DOM, so markup is not affected.

### D3. Enums
`DividerLayout` (`Horizontal`, `Vertical`), `DividerType` (`Solid`, `Dashed`, `Dotted`), `DividerAlign` (`Left`, `Center`, `Right`, `Top`, `Bottom`); the first two have `Default` (`Horizontal`, `Solid`), `align` is an `OptProp`.

### D4. Classes and style are pure functions
`write_classes(cn, &state)` and `align_style(&state) -> Option<(&str, &str)>` (property name and value) are covered by unit test tables following `dividerstyle.ts` (all 2 x 5 combinations); `data_p` is a separate function.

### D5. Branches
`all_static!(layout, divider_type, align, style, style_class)`; the content block is a single `Option<AnyView>` value (`children`) shared by the branches.

## Risks / Trade-offs

- [The alignment class table is easy to get wrong] -> a test table over all combinations and a screenshot comparison with PrimeNG.

## Migration Plan

No migration: a new crate. Rollback is `git revert`.

## Open Questions

None.
