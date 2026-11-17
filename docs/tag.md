# Tag

Port of `p-tag` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-tag::Tag` | `packages/primeng/src/tag/tag.ts`, `tag/style/tagstyle.ts` |
| CSS | `@primeuix/styles/tag` and the Aura preset (`cargo xtask gen-theme`) |
| Demos | `apps/showcase/doc/tag/*-doc.ts` (`basic`, `icon`, `pill`, `severity`, `template`) |

Spec: `openspec/specs/tag`.

## What matches PrimeNG

- Classes `p-tag p-component`, `p-tag-<severity>`, `p-tag-rounded`, `p-tag-icon`, `p-tag-label` and `data-p` (`rounded`, severity).
- Content order: `children`, icon (if set), label; the label is always rendered, even when empty.

## Differences

| Difference | Reason |
|---|---|
| The root is a `<span>`, not a `<p-tag>` host | The host in Angular is a framework detail; CSS is bound to classes. |
| The icon is a `ViewFn` in a `span.p-tag-icon` wrapper | There is no PrimeIcons font. |
| The `template` demo has no flag | An external image from the PrimeNG CDN. |
| Demo icons are replaced by the closest ones from `prime-icons` | `pi-user` → `HomeIcon`, `pi-cog` → `RefreshIcon`. |

## Performance

SSR: 11 allocations per instance (budget 14); contribution to raw wasm +97 KB (budget 140 KB, almost all of it the shared Leptos reactive runtime).
The template is the same as Button's and Badge's (`docs/button.md`): `Prop`, two render branches, parity tests.
