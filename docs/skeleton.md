# Skeleton

Port of `p-skeleton` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-skeleton::Skeleton` | `packages/primeng/src/skeleton/skeleton.ts`, `skeleton/style/skeletonstyle.ts` |
| CSS | `@primeuix/styles/skeleton` and the Aura preset (`cargo xtask gen-theme`) |
| Demos | `apps/showcase/doc/skeleton/*-doc.ts` (`shapes`, `card`, `list`) |

Spec: `openspec/specs/skeleton`.

## What matches PrimeNG

- Root `aria-hidden="true"`, classes `p-skeleton p-component`, `p-skeleton-circle`, `p-skeleton-animation-none`, `data-p` — the shape name.
- Inline style `position: relative`, `width` (`100%`), `height` (`1rem`), `border-radius`; `size` sets the width and height at once.

## Differences

| Difference | Reason |
|---|---|
| The root is a `<div>`, not the `<p-skeleton>` host | The host in Angular is a framework detail. |
| `shape` and `animation` are the enums `SkeletonShape`, `SkeletonAnimation` | In PrimeNG these are arbitrary strings; the allowed values are the same. |
| The `datatable` demo is not ported, Tailwind utilities are replaced with inline styles | It needs a table; Tailwind is not shipped. |
| The user's inline style only via `pt.root` | In PrimeNG Skeleton has no `style` input. |

## Performance

SSR: 6 allocations per instance (budget 8); contribution to raw wasm +86 KB (budget 105 KB).
Template — as in AvatarGroup (`docs/avatargroup.md`).
