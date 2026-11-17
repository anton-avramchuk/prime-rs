# Design

## Context

Requirements are in `specs/stepper`. PrimeNG 21.1.10 sources: `stepper/stepper.ts` (all parts in one file), `stepper/style/*.ts`; styles `@primeuix/styles/stepper`; demos `apps/showcase/doc/stepper/*-doc.ts`. Models in the project: `prime-tabs` (a compound component: `Tabs` + context, string values, identifiers `<id>_tab_<value>`), `prime-accordion` (vertical expansion via `prime_core::collapsible::bind_region_with`, `NodeRef` only on the client).

Observations about the source:
- `Stepper.value` is a number (model); `Step.value`/`StepPanel.value` are numbers; inside a `StepItem` the value is handed to the nested `Step` and `StepPanel` by effects.
- `Step.isStepDisabled = !active && (stepper.linear || step.disabled)`; click -> `stepper.updateValue(value)`; the `content` template's `activateCallback` is `onStepClick` (for a step) and `updateValue(index)` (for a panel).
- The separator after a step exists if the step is not last in `StepList.steps()`; the panel separator in the vertical layout exists if the item is not last among `[data-pc-name=steppanel]` (the number of `StepItem`s).
- `StepPanel` wraps its content in `p-motion` (`visible = active`, animation only in vertical mode): an inactive horizontal panel has no content at all.
- Root class `p-readonly` with `linear` (the theme CSS has `p-stepper-readonly`; we follow the PrimeNG class).
- There is no keyboard navigation between steps: steps are plain buttons.

## Goals / Non-Goals

**Goals:** the same DOM, classes, roles and `aria-*`; the value is a two-way signal; free and linear modes; vertical layout with animation; `content` as a counterpart of the templates.

**Non-Goals:** `transitionOptions`/`motionOptions`/`@primeuix/motion`, `Bind`, `pTemplate`, keyboard navigation beyond native buttons.

## Decisions

### D1. Crate and context
`prime-stepper` with `Stepper`, `StepList`, `Step`, `StepPanels`, `StepPanel`, `StepItem`, `StepContext`, `StepperPt`. `Stepper` puts `StepperCtx { id, value: RwSignal<Option<u32>>, linear: Prop<bool>, steps: RwSignal<Vec<u32>>, items: RwSignal<Vec<u32>> }` into the context; `StepItem` puts `ItemCtx { value }`. Step/panel value: its own `value` or the value of the nearest `StepItem`; with neither - a panic with a clear message (like `NO_TABS` in Tabs).

### D2. Pure logic (`state.rs`)
`step_id(id, value)`, `panel_id(id, value)`, `is_active(value, current)`, `step_disabled(active, linear, disabled)`, `is_last(values, value)`, `root_classes(linear, extra)`, `step_classes(active, disabled)`, `item_classes(active)`, `panel_classes(vertical, active)`. Table-driven tests.

### D3. Step registry for the separator
On creation a `Step` writes its value into `steps` (order = creation order = DOM order), `on_cleanup` removes it. A step's separator is the reactive expression `!is_last(steps, value)`: components are called while the tree is built, before serialization and mounting, so the registry is filled by the time of output; dynamically added steps recompute the separators. Likewise `StepItem` writes its value into `items` for the panel separator.

### D4. Vertical layout
A panel knows it is vertical by the presence of `ItemCtx` in the context. Vertical panel: root `div.p-steppanel` with `p-steppanel-active`; inside it `div.p-motion` with `bind_region_with(region, move || !active)` (only `cfg(wasm32)`, like Accordion), in it `div.p-steppanel-content-wrapper` > separator + `div.p-steppanel-content`; the initial style of an inactive one is `display: none` (`hidden_style`). Horizontal panel: `Show`-like output of the content only when `active`.

### D5. `content`
`StepContext { value: u32, active: Signal<bool>, activate: Callback<u32> }` (`Copy`/`Clone`). `Step.content: Option<Callback<StepContext, AnyView>>` replaces the button; `StepPanel.content` is the content of `p-steppanel-content`. Child elements (`children`) are regular content; if both are set, `content` wins.

### D6. Identifiers
`Stepper.id` is a property or `pn_id_<N>` via `prime_core::ids::next_unique`. Step: `<id>_step_<value>`, panel: `<id>_steppanel_<value>`.

### D7. CSS
`stepper` is added to `theme.components`. Check whether the preset has `p-steppanel`, `p-stepitem`, `p-stepper-separator`; `stepperstyle.ts` has a separate `/* For PrimeNG */` block - `.p-steppanel .p-motion { display: grid; grid-template-rows: 1fr }`, so `stepper` is added to `primeng_extra` too (`xtask gen-theme` reads the block); check that the rule made it into the CSS.

### D8. Tests
SSR: root, `p-readonly`, steps and their states, separators (for two and three steps, vertical), panels (horizontal: content only on the active one; vertical: all in the markup), `StepItem`, `content`, identifiers and `aria-controls`, `pt`. Browser `tests/web.rs`: click on a step, linear mode, `activate` from a panel, external `value`, dynamically adding a step (separators), vertical mode (hide and show), a disabled step. e2e: real mouse and Tab/Enter over steps, screenshot comparisons. The crate is added in three places of `ci.yml`.

### D9. Benchmarks
Scenarios `stepper-1`/`-1000` (three steps, three panels, first active) and `case-stepper`; first `cargo bench -p render-bench --bench alloc`; for wasm - a separate budgets file `--budgets`.

## Risks / Trade-offs

- [Registry and order] The separator depends on the creation order of components; dynamically inserting a step in the middle breaks the registry order -> insertion in the middle is not supported (like the `contentChildren` order in Angular depending on the DOM); recorded in docs.
- [Vertical panel animation] Height is animated via `grid-template-rows`; correctness is checked in the browser (e2e) and visually, like Accordion.
- [Snapshot of `active` in `content`] In PrimeNG `active` is the value at that moment; here it is a signal.

## Open Questions

- Is a `StepperHandle` needed (programmatic control without one's own signal): no, `value` and `StepContext::activate` are enough.

## Migration Plan

No migration. Rollback is `git revert`.
