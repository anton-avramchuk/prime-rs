# Divider

Port of `p-divider` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-divider::Divider` | `packages/primeng/src/divider/divider.ts`, `divider/style/dividerstyle.ts` |
| CSS | `@primeuix/styles/divider` and the Aura preset (`cargo xtask gen-theme`) |
| Demos | `apps/showcase/doc/divider/*-doc.ts` (`basic`, `content`, `type`, `vertical`) |

Spec: `openspec/specs/divider`.

## What matches PrimeNG

- Classes `p-divider p-component`, `p-divider-<layout>`, `p-divider-<type>` and the alignment class by the same table as in `dividerstyle.ts`
  (all combinations are verified by a unit test table, including invalid ones: they have no alignment class).
- `role="separator"`, `aria-orientation`, inline style `justify-content` or `align-items`, `data-p` (`align`, `layout`, type).
- The `div.p-divider-content` block is always present.

## Differences

| Difference | Reason |
|---|---|
| The root is a `<div>`, not the `<p-divider>` host | The host in Angular is a framework detail. |
| The `type` property is called `divider_type` | `type` is a Rust keyword. |
| The `login` demo has no Tailwind responsive classes and no button icons | A vertical divider always, `pi pi-*` are not shipped. |
| Placeholder texts in the demos are shortened | Page size. |

## Performance

SSR: 22 allocations per instance (budget 27); contribution to raw wasm +95 KB (budget 130 KB). The template is like Button's (`docs/button.md`).
