# Proposal

## Why

Toolbar is a simple grouping container for buttons and other content with three areas (start, center, end). It requires no new shared logic, has the role `toolbar`, and suits as the next small component after the composite Panel, Fieldset and Accordion.

## What Changes

- A new crate `prime-toolbar` with the `Toolbar` component: the slots `start_template`, `center_template`, `end_template` (`ViewFn`), `aria_labelledby`, `style`, `style_class`, `children` (projected content before the areas), `pt`.
- Markup as in PrimeNG 21.1.10 (`toolbar.ts`, `toolbarstyle.ts`): the root `<div class="p-toolbar p-component" role="toolbar">`, the areas `p-toolbar-start`, `p-toolbar-center`, `p-toolbar-end` are rendered only when a slot is set and in this order.
- The `/toolbar` page (`basic`, `custom`), e2e, benchmarks, budgets, `docs/toolbar.md`, the CSS of the theme `toolbar`.
- **Not included:** `BlockableUI`; the old template names `left` and `right` (in PrimeNG they are synonyms of `start` and `end`); the demos with SplitButton, IconField and InputIcon, which are not yet in the port (`basic` is built from Button and InputText).

## Capabilities

### New Capabilities
- `toolbar`: the Toolbar component — structure, classes, role and `aria-labelledby`, three areas with slots, `style`, `pt`, performance.

### Modified Capabilities
<!-- None. -->

## Impact

- A new crate `crates/prime-toolbar` (`publish = false`); dependencies `prime-core`.
- Changes to the workspace `Cargo.toml`, `upstream.toml` (`toolbar` in `theme.components`), `showcase`, `benches/*`, `e2e/tests`.
