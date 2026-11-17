# Design

## Context

The requirements are in `specs/skeleton`. PrimeNG 21.1.10 sources: `packages/primeng/src/skeleton/skeleton.ts`, `skeleton/style/skeletonstyle.ts`, demos `apps/showcase/doc/skeleton`. The crate template is `docs/progressbar.md` and `docs/avatargroup.md`.

## Decisions

### D1. Root — `<div>`
Instead of the `<p-skeleton>` host; `aria-hidden="true"` and `data-p` — on the root.

### D2. Style
As `containerStyle` in PrimeNG: `position: relative`, then `width`/`height` (when `size` is set, both equal it) and `border-radius` (only when `border_radius` is set). The string is assembled as one `String` through `write!`; the trailing `;` is added by Leptos when rendering the attribute.

### D3. Classes and `data-p`
`p-skeleton p-component`, `p-skeleton-circle` when `shape = circle`, `p-skeleton-animation-none` when `animation = none`; `data-p` — the shape name (`rectangle`/`circle`).

### D4. Branches
Static — when `all_static!` holds; otherwise reactive. The markup is in a `macro_rules!`; `Props`/`Pt` without `Arc` in the static branch.

### D5. Tests
SSR: structure, style, classes, `size`, `pt`, branch parity. There are no browser tests: there is no behaviour.

## Risks / Trade-offs

- [The `shape`/`animation` strings in PrimeNG are arbitrary] → typed enumerations with the same values.

## Migration Plan

No migration: a new crate. Rollback — `git revert`.

## Open Questions

None.
