# AvatarGroup

Port of `p-avatar-group` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-avatargroup::AvatarGroup` | `packages/primeng/src/avatargroup/avatargroup.ts`, `avatargroup/style/avatargroupstyle.ts` |
| CSS | `.p-avatar-group` is part of `@primeuix/styles/avatar` (feature `avatar` in `prime-theme`) |
| Demos | `apps/showcase/doc/avatar/avatargroup-doc.ts` (on the `/avatar` page, the `avatargroup` demo) |

Spec: `openspec/specs/avatargroup`.

## What matches PrimeNG

- Root classes `p-avatar-group p-component`, child content without wrappers, `style` and `style_class`.
- Overlap and alignment of the avatars are entirely in the theme CSS.

## Differences

| Difference | Reason |
|---|---|
| The root is a `<div>`, not the host `<p-avatar-group>` | The Angular host is a framework detail. |
| No dependency on `prime-avatar` | The container accepts any child elements; in PrimeNG the link is also only through CSS. |

## Performance

SSR: 6 allocations per instance (budget 8); contribution to raw wasm +64 KB (budget 80 KB).
Template — as for Divider (`docs/divider.md`).
