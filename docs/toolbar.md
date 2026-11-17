# Toolbar

Port of `p-toolbar` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-toolbar::Toolbar` | `packages/primeng/src/toolbar/toolbar.ts`, `toolbar/style/toolbarstyle.ts` |
| CSS | `@primeuix/styles/toolbar` and the Aura preset (`cargo xtask gen-theme`) |
| Demos | `apps/showcase/doc/toolbar/*-doc.ts` (`basic`, `custom`) |

Spec: `openspec/specs/toolbar`.

## What matches PrimeNG

- Root `div.p-toolbar.p-component` with `role="toolbar"` and `aria-labelledby`.
- Areas `p-toolbar-start`, `p-toolbar-center`, `p-toolbar-end` in this order and only when a slot is set; projected content comes before them.

## Differences

| Difference | Reason |
|---|---|
| The root is a `<div>`, not the host `<p-toolbar>` | The Angular host is a framework detail. |
| Slots are `ViewFn` (`start_template`, `center_template`, `end_template`), content is `children` | Instead of the `#start`, `#center`, `#end` templates and `<ng-content>`. |
| No `left` and `right` template synonyms | Outdated names `start` and `end`. |
| No `BlockableUI` | The Angular blocking interface. |
| The `basic` demo: a "Save" button instead of SplitButton, an `InputText` with a search icon instead of IconField, SVG icons from `prime-icons` (plus, trash, upload) instead of the PrimeIcons font (plus, print, upload) | SplitButton, IconField and InputIcon are not ported yet; there is no print icon in the set. |

## Performance

SSR: 23 allocations per instance with three areas (budget 26), about half of them are the three `ViewFn` slots; contribution to raw wasm +96 KB (budget 110 KB).
Template — as for Card (`docs/card.md`).
