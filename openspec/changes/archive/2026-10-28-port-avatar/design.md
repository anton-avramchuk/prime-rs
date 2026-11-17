# Design

## Context

Requirements are in `specs/avatar`. PrimeNG 21.1.10 sources: `packages/primeng/src/avatar/avatar.ts`, `avatar/style/avatarstyle.ts`, demos `apps/showcase/doc/avatar`. The `<p-avatar>` host carries the classes, `aria-label`, `aria-labelledby`, `data-p`; inside it there are `ng-content` and one of three blocks (label, otherwise icon, otherwise image). `data-p` is built from the shape and size, which have default values (`square`, `normal`), so by default it equals `square normal`. Template — `docs/button.md`.

## Goals / Non-Goals

**Goals:** Avatar with the same classes, `data-p`, order and block priority; branches without `RenderEffect` for static values; budgets.
**Non-Goals:** `AvatarGroup` (a separate change), icon fonts.

## Decisions

### D1. Root — `<div>`
As for Chip: the `<p-avatar>` host is an Angular detail.

### D2. Enumerations with defaults
`AvatarSize` (`Normal`, `Large`, `XLarge`) and `AvatarShape` (`Square`, `Circle`) implement `Default` (`Normal`, `Square`); the properties are `Prop<AvatarSize>`, `Prop<AvatarShape>`, the default value is static. The values always go into `data-p`, as in PrimeNG.

### D3. Blocks
`label_of`, `icon_of`, `image_of` return `Option<AnyView>` with the priority label → icon → image; `p-avatar-image` on the root is determined by `image.is_some()` (like `!= null` in PrimeNG).

### D4. State and branches
Pure functions `write_classes`/`data_p` over `AvatarState`; the branch is selected by `all_static!(label, image, size, shape, style, style_class, aria_label, aria_labelledby)`.

## Risks / Trade-offs

- [The appearance depends on the CSS of the theme `avatar`] → the CSS is already generated and connected by the showcase wrapper.
- [The branches diverge] → a parameterized parity test.

## Migration Plan

No migration: a new crate. Rollback — `git revert`.

## Open Questions

None.
