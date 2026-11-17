# Design

## Context

Requirements are in `specs/avatargroup`. PrimeNG 21.1.10 sources: `packages/primeng/src/avatargroup/avatargroup.ts`, `avatargroup/style/avatargroupstyle.ts`, demo `apps/showcase/doc/avatar/avatargroup-doc.ts`. The component is a container: the `<ng-content>` template, the class `p-avatar-group p-component`, `styleClass`, `style`. The crate's templates are `docs/divider.md` and `docs/card.md`.

## Goals / Non-Goals

**Goals:** markup and classes as in PrimeNG; `pt.root`; a static branch.
**Non-Goals:** collapsing of extra avatars, a dependency on `prime-avatar` (the container accepts any child elements).

## Decisions

### D1. Root — `<div>`
Instead of the `<p-avatar-group>` host; the theme CSS targets `.p-avatar-group`.

### D2. Branches
Static — when `all_static!(style, style_class)`: ready values; otherwise closures. One markup, in a `macro_rules!`.

### D3. Tests
SSR: structure, classes, children, `style`/`style_class`, `pt`, branch parity. There are no browser tests: there is no behaviour.

## Risks / Trade-offs

- [The overlap of the avatars is entirely in the theme CSS] → checked by e2e (a negative offset of the second avatar).

## Migration Plan

No migration: a new crate. Rollback — `git revert`.

## Open Questions

None.
