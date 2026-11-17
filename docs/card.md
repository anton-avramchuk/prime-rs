# Card

Port of `p-card` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-card::Card` | `packages/primeng/src/card/card.ts`, `card/style/cardstyle.ts` |
| CSS | `@primeuix/styles/card` and the Aura preset (`cargo xtask gen-theme`) |
| Demos | `apps/showcase/doc/card/*-doc.ts` (`basic`, `advanced`) |

Spec: `openspec/specs/card`.

## What matches PrimeNG

- Classes `p-card p-component`, `p-card-header`, `p-card-body`, `p-card-title`, `p-card-subtitle`, `p-card-content`, `p-card-footer`.
- Structure: the header (only with a slot) outside the body; in the body the title, subtitle (only with text or a slot), content (always), footer (only with a slot).
- A slot takes priority over the text property (`header_template` over `header`, etc.).

## Differences

| Difference | Reason |
|---|---|
| The root is a `<div>`, not the `<p-card>` host | The host in Angular is a framework detail (`display: block` is not needed for `<div>`). |
| Slots are `ViewFn` (`header_template`, `title_template`, `subtitle_template`, `footer_template`), content is `children` | Instead of the `#header`, `#title`, … templates and the `p-header`/`p-footer` facets. |
| No `BlockableUI` | An Angular blocking interface. |
| The `p-card-caption` class is not used | It is not applied in the PrimeNG template either. |

## Performance

SSR: 29 allocations per instance with a title and a subtitle (budget 35); contribution to raw wasm +96 KB (budget 130 KB).
The template is as for Button (`docs/button.md`).
