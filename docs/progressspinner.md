# ProgressSpinner

Port of `p-progressspinner` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-progressspinner::ProgressSpinner` | `packages/primeng/src/progressspinner/progressspinner.ts`, `progressspinner/style/progressspinnerstyle.ts` |
| CSS | `@primeuix/styles/progressspinner` and the Aura preset (`cargo xtask gen-theme`) |
| Demos | `apps/showcase/doc/progressspinner/*-doc.ts` (`basic`, `custom`) |

Spec: `openspec/specs/progressspinner`.

## What matches PrimeNG

- Root `div.p-progressspinner` with `role="progressbar"`, `aria-busy="true"` and `aria-label`; inside it `svg.p-progressspinner-spin` (`viewBox="25 25 50 50"`, `animation-duration` in `style`) and `circle.p-progressspinner-circle` (`cx="50" cy="50" r="20"`, `fill`, `stroke-width`, `stroke-miterlimit="10"`).
- Default values: `stroke_width="2"`, `fill="none"`, `animation_duration="2s"`.

## Differences

| Difference | Reason |
|---|---|
| The root is a `<div>`, not the host `<p-progressspinner>` | The Angular host is a framework detail. |
| No selector synonyms and no `BlockableUI` | Outdated names and the Angular interface. |
| `style_class` instead of `styleClass` | Outdated name; PrimeNG replaced it with `class`. |

## Performance

SSR: 14 allocations per instance (budget 16); contribution to raw wasm +95 KB (budget 105 KB).
Template — as for Toolbar (`docs/toolbar.md`).
