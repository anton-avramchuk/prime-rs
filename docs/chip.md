# Chip

Port of `p-chip` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-chip::Chip` | `packages/primeng/src/chip/chip.ts`, `chip/style/chipstyle.ts` |
| CSS | `@primeuix/styles/chip` and the Aura preset (`cargo xtask gen-theme`) |
| Demos | `apps/showcase/doc/chip/*-doc.ts` (`basic`, `icon`, `image`, `template`) |

Spec: `openspec/specs/chip`.

## What matches PrimeNG

- Classes `p-chip p-component`, `p-disabled`, `p-chip-image`, `p-chip-icon`, `p-chip-label`, `p-chip-remove-icon`; `data-p="removable"`.
- Content order: `children`, the image (otherwise the icon), the label, the remove element.
- Removal by click and by Enter and Backspace, hiding with `display: none` on the root, `on_remove`, `on_image_error`, `tabindex` -1 when `disabled`,
  `aria-label` from the locale (`aria.removeLabel`).

## Differences

| Difference | Reason |
|---|---|
| The root is a `<div>`, not the host `<p-chip>` | The Angular host is a framework detail. |
| The remove element is a `span[role=button]` with `TimesCircleIcon` inside, not an `<svg role=button>` | A `prime-icons` icon sets its own a11y semantics; the role and the handlers must be on the same element. |
| The icon and the remove icon are `ViewFn`, not PrimeIcons class strings | There is no icon font. |
| `on_remove` receives a `web_sys::Event` (`MouseEvent` or `KeyboardEvent`) | PrimeNG types it as `MouseEvent`, but a keyboard removal delivers a `KeyboardEvent`. |
| No `chipProps` | In Rust these are ordinary properties. |
| A removable Chip always takes the reactive path | The root needs internal visibility state. |
| The brand icons of the `icon` demo are replaced by `prime-icons` icons | The 53 PrimeIcons icons have no Apple, Facebook, Google or Microsoft. |

## Performance

SSR, allocations per instance: a plain Chip 13 (budget 16), a removable one 41 (budget 50; reactive branch, the remove SVG icon, the locale); contribution to raw wasm +179 KB
(budget 230 KB; the removable path pulls in the handlers, the icon and the locale). Template — as for Button (`docs/button.md`).
