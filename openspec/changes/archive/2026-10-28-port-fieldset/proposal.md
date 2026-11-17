# Proposal

## Why

Fieldset is a grouping container with a `<legend>` and optional collapsing, the counterpart of Panel. The common collapsing logic (`prime-rs-core::collapsible`) already exists, so the port comes down to the markup of `<fieldset>`/`<legend>` and a button in the legend.

## What Changes

- A new crate `prime-fieldset` with the `Fieldset` component: `legend`, `toggleable`, a two-way `collapsed` (`RwSignal<bool>`) and `default_collapsed`, `id`, `style`, `style_class`, the slots `header_template`, `expand_icon_template`, `collapse_icon_template`, the events `on_before_toggle`, `on_after_toggle`, `pt`.
- Classes as in PrimeNG 21.1.10 (`fieldsetstyle.ts`): `p-fieldset p-component`, `p-fieldset-toggleable`, `p-fieldset-collapsed`, `p-fieldset-legend`, `p-fieldset-legend-label`, `p-fieldset-toggle-button`, `p-fieldset-toggle-icon`, `p-fieldset-content-container`, `p-fieldset-content-wrapper`, `p-fieldset-content`; `data-p="toggleable"`.
- With `toggleable` the legend contains a `<button>` with `aria-controls`, `aria-expanded`, `aria-label` (the text of the legend); the content region is `role="region"`, `aria-labelledby`, `aria-hidden`; the animation and the `tabindex` of the descendants — through `prime-rs-core::collapsible`.
- The `/fieldset` page (`basic`, `toggleable`, `template`), e2e, benchmarks, budgets, `docs/fieldset.md`, the CSS of the theme `fieldset`.
- **Not included:** `BlockableUI`, `motionOptions`/`transitionOptions`, the `<p-header>` facet; changes to `prime-rs-core::collapsible` (it is used as is).

## Capabilities

### New Capabilities
- `fieldset`: the Fieldset component — structure and classes, collapsing and the two-way `collapsed`, ARIA, animation and `on_after_toggle`, slots, `pt`, performance.

### Modified Capabilities
<!-- None: collapsible is reused without changing the requirements. -->

## Impact

- A new crate `crates/prime-fieldset` (`publish = false`); dependencies `prime-rs-core`, `prime-icons` (`minus`, `plus`), `web-sys`.
- Changes to the workspace `Cargo.toml`, `upstream.toml` (`fieldset` in `theme.components`), `showcase`, `benches/*`, `e2e/tests`, the CI browser job.
