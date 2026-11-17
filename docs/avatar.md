# Avatar

A port of `p-avatar` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-avatar::Avatar` | `packages/primeng/src/avatar/avatar.ts`, `avatar/style/avatarstyle.ts` |
| CSS | `@primeuix/styles/avatar` and the Aura preset (`cargo xtask gen-theme`) |
| Demos | `apps/showcase/doc/avatar/*-doc.ts` (`label`, `icon`, `image`, `shape`, `size`, `template`) |

Spec: `openspec/specs/avatar`.

## What matches PrimeNG

- Classes `p-avatar p-component`, `p-avatar-image`, `p-avatar-circle`, `p-avatar-lg`, `p-avatar-xl`, `p-avatar-label`, `p-avatar-icon`.
- `data-p` always contains the shape and size, including the defaults (`square normal`), and is on the root, label, icon and image.
- Block priority: label, icon, image; `aria-label` and `aria-labelledby` on the root, `aria-label` also on the image.
- `p-avatar-image` on the root is determined by the `image` being set (like `image != null`).

## Differences

| Difference | Reason |
|---|---|
| The root is a `<div>`, not the `<p-avatar>` host | In Angular the host is a framework detail. |
| The icon is a `ViewFn`, not a PrimeIcons class string | There is no font. |
| The `pi-user` icon in the demo is replaced with `HomeIcon` | The 53 PrimeNG icons have no user icon. |

## Performance

SSR: 15 allocations per instance (budget 18); contribution to raw wasm +137 KB (budget 180 KB, almost all of it the shared Leptos reactive runtime).
The template is the same as Button (`docs/button.md`).
