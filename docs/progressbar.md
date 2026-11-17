# ProgressBar

Port of `p-progressbar` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-progressbar::ProgressBar` | `packages/primeng/src/progressbar/progressbar.ts`, `progressbar/style/progressbarstyle.ts` |
| CSS | `@primeuix/styles/progressbar` and the Aura preset (`cargo xtask gen-theme`) |
| Demos | `apps/showcase/doc/progressbar/*-doc.ts` (`basic`, `dynamic`, `indeterminate`, `template`) |

Spec: `openspec/specs/progressbar`.

## What matches PrimeNG

- Root with `role="progressbar"`, `aria-valuemin="0"`, `aria-valuemax="100"`, `aria-valuenow`, `aria-level`, classes `p-progressbar p-component p-progressbar-determinate|indeterminate`, `data-p` on the root and parts.
- Determinate mode: `div.p-progressbar-value` (`width: <value>%; display: flex`, with `color` - `background`) with `div.p-progressbar-label`, inside it the value and unit (hidden when zero or when there is no value).
- Indeterminate mode: only `div.p-progressbar-value`.
- The `content_template` slot receives the value (PrimeNG's `content`).

## Differences

| Difference | Reason |
|---|---|
| The root is a `<div>`, not the `<p-progressbar>` host | A host in Angular is a framework detail. |
| `aria-valuenow` and `aria-level` are absent without a value | PrimeNG renders `aria-level="undefined%"`. |
| `value` is `f64`, rendered via `Display` | Integers are rendered without a fractional part, as in JavaScript. |
| `content_template: Callback<Option<f64>, AnyView>` | Leptos slots. |
| The `dynamic` demo grows by 10 every 500 ms | Deterministic e2e; PrimeNG uses a random step and `MessageService`. |
| No browser tests of the component | There is no client-side behavior, SSR tests and e2e are enough. |

## Performance

SSR: 36 allocations per instance (budget 44); contribution to raw wasm +136 KB (budget 170 KB).
The template is the same as ToggleSwitch (`docs/toggleswitch.md`).
