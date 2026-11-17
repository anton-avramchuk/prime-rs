# Proposal

## Why

Stepper is the last component of wave 3: a step-by-step wizard with numbered steps, separators and content panels, horizontal or vertical, free or linear. It is close to Tabs in structure (a compound component with a shared active value) and closes wave 3 of the plan.

## What Changes

- New crate `prime-stepper` (port of `p-stepper`, `p-step-list`, `p-step`, `p-step-panels`, `p-step-panel`, `p-step-item`, `p-stepper-separator` from PrimeNG 21.1.10).
- A compound component, like Tabs: [`Stepper`] stores the active value and communicates with the parts through the Leptos context. Parts: `StepList`, `Step`, `StepPanels`, `StepPanel`, `StepItem` (vertical layout).
- Markup and classes as in PrimeNG: `div.p-stepper.p-component[role=tablist]` (`p-readonly` with `linear`), `div.p-steplist`, `div.p-step` (`p-step-active`, `p-disabled`, `aria-current="step"`, `data-p-active`, `data-p-disabled`) with a `button.p-step-header[role=tab]` button (`span.p-step-number`, `span.p-step-title`) and a `div.p-stepper-separator` separator (except after the last step), `div.p-steppanels`, `div.p-steppanel[role=tabpanel]` (`p-steppanel-active` in vertical mode) with `div.p-steppanel-content-wrapper` > separator (vertical mode, except for the last item) + `div.p-steppanel-content`, `div.p-stepitem` (`p-stepitem-active`).
- State: `Stepper.value: RwSignal<Option<u32>>` (two-way) and `default_value`; `linear` (steps are unavailable for clicks until active; transitions are programmatic); `Step.disabled`; identifiers `<id>_step_<N>` and `<id>_steppanel_<N>`, `aria-controls` links a step and a panel.
- Custom views: `content` of `Step` and `StepPanel` receives `StepContext { value, active, activate }` (counterpart of `activateCallback`, `value`, `active` of the `content` template).
- Behavior: a click on a step activates it (if not disabled and not `linear`); one panel is active (horizontal mode - content is rendered only for the active one; vertical - all panels are in the markup, inactive ones hidden, expansion is animated by theme classes); the separator is not rendered after the last step/item, and is filled before the active one.
- Showcase page `/stepper` with demos `basic`, `linear`, `vertical`, `template`, `stepsonly`; e2e; benchmarks and budgets; `docs/stepper.md`.
- Styles: `stepper` is added to `theme.components`.
- **Not included:** `transitionOptions`/`motionOptions`/`@primeuix/motion` (the animation is theme classes), `Bind`, `pTemplate`, forced closing of other panels, keyboard navigation between steps (PrimeNG has none - steps are plain buttons), the `accessibility` demo (text).

## Capabilities

### New Capabilities
- `stepper`: a step-by-step wizard with steps, panels, separators, linear mode and vertical layout.

### Modified Capabilities
<!-- None. -->

## Impact

- New crate `crates/prime-stepper` (`publish = false`); dependencies: `prime-core`, `web-sys`.
- Changed: workspace `Cargo.toml`, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (browser job, three places).
