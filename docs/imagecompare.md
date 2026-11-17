# ImageCompare

Port of `p-imagecompare` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-imagecompare::ImageCompare`, `ImageComparePt` | `packages/primeng/src/imagecompare/imagecompare.ts`, `imagecompare/style/imagecomparestyle.ts` |
| CSS | `@primeuix/styles/imagecompare` and the Aura preset — `cargo xtask gen-theme` (there is no `/* For PrimeNG */` block) |
| Demos | `apps/showcase/doc/imagecompare/{basic,responsive}-doc.ts` |

Spec: `openspec/specs/imagecompare`.

## What matches PrimeNG

- Markup: root `div.p-imagecompare` (`tabindex`, `aria-label`, `aria-labelledby` on it) with the `left` and `right` views without wrappers and `input[type=range].p-imagecompare-slider` (`min=0`, `max=100`, `value=50`).
- Input on the slider sets `clip-path` on the element directly in front of it (the right image): `polygon(0 0, V% 0, V% 100%, 0 100%)`; with `dir="rtl"` on the root or an ancestor — `polygon((100−V)% 0, 100% 0, 100% 100%, (100−V)% 100%)`.
- Before the first input the clipping is set only by the theme rule `.p-imagecompare img + img` (50 %), so `left` and `right` must be `<img>`: with other elements the theme rule does not apply — as in PrimeNG.

## Differences

| Difference | Reason |
|---|---|
| The root is a `<div>`, not the host `<p-imagecompare>` | The Angular host is a framework detail. |
| `left`/`right` are `ViewFn` instead of the `#left`/`#right` templates | There is no `ng-template`. |
| No `MutationObserver` on the `dir` attribute of `<html>`: the direction is computed at the moment of input (`closest('[dir=rtl]')`) | PrimeNG applies `isRTL` only in `onSlide`, so the result is the same; no observer is needed. |
| `tabindex` is not set by default | The PrimeNG documentation mentions `0`, but in the code the value is `undefined` — we follow the code. |
| The demos use two built-in SVG images (`data:` links) instead of the photos from the CDN | No network dependency. |
| No `Bind`, `pTemplate`, `pt` with context | `pt` is simple additions of classes, style and attributes (`root`, `slider`). |

## Not verified

- A real touch: dragging the handle was verified with the mouse and the keyboard.
- Screen readers: the slider has no `aria-label` of its own (as in PrimeNG — labels are only on the root).

## Performance

See `benches/` (`imagecompare-1`, `imagecompare-1000`, `case-imagecompare`) and `benches/*budgets.toml`: 12 allocations per instance with SSR.
