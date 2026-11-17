# Proposal

## Why

Steps is the first wave 5 component: a wizard step indicator (numbered items in a row), a simpler predecessor of `Stepper`. It remains in PrimeNG 21, its styles (`@primeuix/styles/steps`) exist, and the project needs it as the shortest port to warm up the wave and as a reference for "navigation over link items" (keyboard with a roving `tabindex`), which Menubar and PanelMenu then reuse.

## What Changes

- New crate `prime-steps` (a port of PrimeNG 21.1.10 `p-steps`).
- `StepsItem` (instead of PrimeNG's `MenuItem`, without router fields): `id`, `label`, `url`, `target`, `disabled`, `tabindex`, `style`, `command` (`Callback<StepsCommand>` with the event, the item and the index).
- `Steps`: `nav.p-steps.p-component` (`p-readonly`, `data-pc-name="steps"`) > `ul.p-steps-list` > `li.p-steps-item` (`p-steps-item-active`, `p-disabled`, `aria-current="step"` on the active one, `id`) > `a.p-steps-item-link[role=link]` (`href`, `target`, `tabindex`, `aria-expanded`, `aria-disabled`) > `span.p-steps-item-number` (number counted from one) and `span.p-steps-item-label`.
- Behaviour: by default `readonly = true` — a click does nothing (`preventDefault`), all items except the active one are disabled; in interactive mode a click on an available item calls `on_active_index_change(i)` and `command`, link navigation is cancelled if the item has no `url`; keys ←/→ over items, `Home`/`End`, `Enter`/`Space` — a click, with a roving `tabindex` (the active item `0`, the rest `-1`; `Tab` returns the focus marker to the active item).
- Properties: `model`, `active_index` (a two-way `RwSignal<usize>` or a value), `readonly` (`true` by default), `exact`, `style`, `style_class`, `pt`; events `on_active_index_change`.
- Showcase page `/steps` with demos `basic`, `controlled`, `interactive`; e2e; benchmarks and budgets; `docs/steps.md`.
- Styles: `steps` is added to `theme.components`.
- **Out of scope:** `routerLink` and router fields (`queryParams`, `fragment`, …), `RouterModule` detection of the active item by address (the active item is only `active_index`), item tooltips (`tooltipOptions`), HTML labels (`escape = false`), `Bind`, `pTemplate`. The `routing` demo is not ported: the local PrimeNG copy has no `doc/steps` directory (the demos were removed upstream during cleanup), the demo set is derived from the list in `pages/steps/index.ts` (`basic`, `controlled`, `interactive`, `routing`, `accessibility`).

## Capabilities

### New Capabilities
- `steps`: a wizard step indicator with optional interactive navigation.

### Modified Capabilities
<!-- None. -->

## Impact

- New crate `crates/prime-steps` (`publish = false`); dependencies: `prime-rs-core`, `web-sys`.
- Changed: workspace `Cargo.toml`, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (browser job, three places: `prime-steps`).
