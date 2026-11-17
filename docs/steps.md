# Steps

Port of PrimeNG 21.1.10 `p-steps` (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-steps::Steps`, `StepsItem`, `StepsCommand`, `StepsPt` | `packages/primeng/src/steps/steps.ts`, `steps/style/stepsstyle.ts` |
| CSS | `@primeuix/styles/steps` and the Aura preset — `cargo xtask gen-theme` (there is no `/* For PrimeNG */` block) |
| Demos | This PrimeNG copy has no `apps/showcase/doc/steps` directory (upstream removed the demos, the `pages/steps/index.ts` page still references them); the set `basic`, `controlled`, `interactive` is derived from the list on the page and the component code |

Spec: `openspec/specs/steps`.

```rust
let active = RwSignal::new(0usize);
view! { <Steps model=items active_index_signal=active readonly=false /> }
```

## What matches PrimeNG

- Markup and classes: `nav.p-steps.p-component` (`p-readonly`, `data-pc-name="steps"`) > `ul.p-steps-list` > `li.p-steps-item` (`p-steps-item-active`, `p-disabled`, `aria-current="step"`) > `a.p-steps-item-link[role=link]` (`aria-expanded`, `aria-disabled`, `tabindex`) > `span.p-steps-item-number` and `span.p-steps-item-label`.
- By default `readonly = true`: a click does nothing, inactive items are disabled. In interactive mode a click on an available item reports the index (`on_active_index_change`), calls the item's `command` with the event, the item and the index, and cancels link navigation if the item has no `url`.
- Keyboard: ←/→/`Home`/`End` move focus with a roving `tabindex`, `Tab` returns `tabindex="0"` to the active item, `Enter`/`Space` — a click.
- `tabindex` rules: `-1` for a disabled one, its own (or `0`) for the active one, its own (or `-1`) for the rest.

## Differences

| Difference | Reason |
|---|---|
| No `routerLink` and no detection of the active item by the router address; the active step is only `active_index` | No Angular router. |
| The current step is `active_index` (a value or a signal) and an optional two-way `active_index_signal` instead of `activeIndex`/`activeIndexChange` | A Leptos idiom. |
| Items are numbered by index; in PrimeNG `track item.label` merged items with the same label | A bug in the source. |
| No item tooltips (`tooltipOptions`), HTML labels (`escape = false`), `Bind`, `pTemplate` | Not ported. |
| `exact` is accepted but has no behaviour | In PrimeNG it affects only router links. |
| The `routing` demo is not ported | No router. |

## Not verified

- Right-to-left layout and touch input.
- Screen reading: `aria-*` were checked against PrimeNG, screen reader behaviour was not.
- Comparison with PrimeNG reference screenshots: there are no demos in the local copy.

## Performance

See `benches/` (`steps-1`, `steps-1000`, `case-steps`) and `benches/*budgets.toml`.
