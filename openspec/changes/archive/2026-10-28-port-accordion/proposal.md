# Proposal

## Why

Accordion groups content into tabs and uses the same collapse animation as Panel and Fieldset, but its state is shared per group (`value`, the `multiple` mode) and it has keyboard navigation between headers. It is the third consumer of `prime-rs-core::collapsible` and the first composite component (Accordion, AccordionPanel, AccordionHeader, AccordionContent) with a context between the parts.

## What Changes

- New crate `prime-accordion` with the components `Accordion`, `AccordionPanel`, `AccordionHeader`, `AccordionContent`.
- `Accordion`: two-way `value` (`RwSignal<Vec<String>>`, the list of active panel values; in single mode 0 or 1 value) and `default_value`, `multiple`, `select_on_focus`, `id`, `style_class`, events `on_open` and `on_close` (the original event and the panel value), `pt`.
- `AccordionPanel`: `value` (required, a string), `disabled`, `style_class`. `AccordionHeader`: the `toggle_icon_template` slot (receives `active`), `style_class`. `AccordionContent`: `style_class`.
- Classes and attributes as in PrimeNG 21.1.10 (`accordionstyle.ts`, `accordion.ts`): `p-accordion p-component`, `p-accordionpanel` (+ `p-accordionpanel-active`, `p-disabled`), `p-accordionheader`, `p-accordionheader-toggle-icon`, `p-accordioncontent`, `p-accordioncontent-wrapper`, `p-accordioncontent-content`, `data-p-active`, `data-p-disabled`, `data-p`.
- Accessibility: header `role="button"`, `aria-expanded`, `aria-controls`, `aria-disabled`, `tabindex`; content `role="region"`, `aria-labelledby`. Keyboard: Enter/Space toggle, ArrowDown/ArrowUp/Home/End move focus between headers, skipping disabled ones.
- The expand animation uses the `p-collapsible-*` classes via the shared logic; for this `prime-rs-core::collapsible` gets binding a region to a derived state (`bind_region_with`), existing behaviour does not change.
- `cargo xtask gen-theme` is extended with the `/* For PrimeNG */` block from `<name>style.ts` (needed by Accordion: `.p-accordioncontent .p-motion { display: grid; grid-template-rows: 1fr }`, icon placement); `accordion` theme CSS.
- The `/accordion` page (`basic`, `multiple`, `disabled`, `controlled`, `dynamic`, `template`), e2e, benchmarks, budgets, `docs/accordion.md`.
- **Out of scope:** `expandIcon`/`collapseIcon` as CSS icon classes (replaced by the `toggle_icon_template` slot), ripple, `BlockableUI`, `motionOptions`/`transitionOptions`, numeric values (values are strings), the `accessibility` demo.

## Capabilities

### New Capabilities
- `accordion`: Accordion components — structure, classes, value and the `multiple` mode, events, ARIA and keyboard, animation, slots, `pt`, performance.

### Modified Capabilities
- `collapsible`: binding a collapsible region to a derived (read-only) state is added; current requirements do not change.

## Impact

- New crate `crates/prime-accordion` (`publish = false`); dependencies `prime-rs-core`, `prime-icons` (`chevrondown`, `chevronup`), `web-sys`.
- `prime-rs-core::collapsible`: a new function; `xtask`: reading the `For PrimeNG` block.
- Changed: workspace `Cargo.toml`, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, the CI browser job.
