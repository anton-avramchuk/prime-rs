# Stepper

Port of `p-stepper`, `p-step-list`, `p-step`, `p-step-panels`, `p-step-panel`, `p-step-item` and `p-stepper-separator` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-stepper::Stepper`, `StepList`, `Step`, `StepPanels`, `StepPanel`, `StepItem`, `StepContext`, `StepPt`, `StepPanelPt` | `stepper/stepper.ts`, `stepper/style/*` |
| CSS | `@primeuix/styles/stepper` and the `.p-steppanel .p-motion` rule from `stepperstyle.ts` - `cargo xtask gen-theme`, `primeng_extra` |
| Demos | `apps/showcase/doc/stepper/{basic,linear,vertical,template,stepsonly}-doc.ts` |

Spec: `openspec/specs/stepper`.

## What matches PrimeNG

- Markup and classes: `div.p-stepper.p-component[role=tablist]` (`p-readonly` with `linear`), `div.p-steplist`, `div.p-step[role=presentation]` (`p-step-active`, `p-disabled`, `aria-current="step"`, `data-p-active`, `data-p-disabled`) with a `button.p-step-header[role=tab]` button (`id`, `aria-controls`, `tabindex=-1` and `disabled` when unavailable) > `span.p-step-number` + `span.p-step-title`, `div.p-stepper-separator` after each list step except the last; `div.p-steppanels`, `div.p-steppanel[role=tabpanel]` (`id`, `aria-controls`, `data-p-active`, `data-pc-name="steppanel"`) > `div.p-steppanel-content-wrapper` > `div.p-steppanel-content`; `div.p-stepitem` (`p-stepitem-active`).
- Identifiers `<id>_step_<N>` and `<id>_steppanel_<N>`; the default `id` is `pn_id_<N>`.
- Unavailability: `!active && (linear || disabled)`; a click on a step writes its value to `value`.
- In the horizontal layout content is rendered only for the active panel (like `p-motion` with `visible = active`); in the vertical one (`StepItem`) the panel gets `p-steppanel-active`, the panel separator is rendered for every item except the last, a step inside `StepItem` renders no separator.
- No keyboard navigation beyond native buttons - as in PrimeNG.

## Differences

| Difference | Reason |
|---|---|
| Parts are plain `div`s, without the `<p-stepper>`, `<p-step>` etc. hosts | Hosts in Angular are a framework detail. |
| Values are `u32` (a number in PrimeNG), `value` is `RwSignal<Option<u32>>` | Two-way binding with signals; `None` means no active step. |
| `content` instead of the `#content` template: `Callback<StepContext, AnyView>`, where `StepContext { value, active: Signal<bool>, activate: Callback<u32> }` | Counterpart of the PrimeNG template's `activateCallback`, `value`, `active`; `activate(n)` sets the value `n` (for a step - any, like `updateValue`). Regular content is child elements. |
| A step's separator is determined by the step registry in the `Stepper` context (creation order = DOM order) | Instead of `contentChildren`. A step inserted dynamically into the middle of the list breaks the registry order: steps may only be added and removed at the end. |
| Vertical layout: inactive panels stay in the markup (`display: none`), the content of all panels is rendered at once; expansion is animated by the theme's `p-collapsible-*` classes via `prime_rs_core::collapsible::bind_region_with` | In PrimeNG `p-motion` removes the content of an inactive panel; the animation needs the content to still be in place. |
| No `transitionOptions`, `motionOptions` or `@primeuix/motion` | The animation is theme classes, like Accordion. |
| The `Stepper` and `StepItem` context is placed in a child owner, and the content is returned as an `OwnedView` with that owner | Otherwise the context leaks to siblings after the component (a horizontal panel after a vertical `Stepper` would consider itself vertical); `OwnedView` is needed so that steps added by a condition after mounting find the context too. |
| For components from `primeng_extra`, `xtask gen-theme` takes the CSS after `${…_style}` in the literal if there is no `/* For PrimeNG */` marker | `stepperstyle.ts` has no marker, but the `.p-steppanel .p-motion` rule is needed for the vertical animation. |
| The `template` demo: the `pi-user`/`pi-id-card` icons are replaced by `HomeIcon`/`CheckIcon`, the CDN image by "🎉"; `ngModel` is signals | `prime-icons` has no suitable icons; independence from the CDN. |
| Class `p-readonly` with `linear` instead of `p-stepper-readonly` from the theme CSS | We follow `stepperstyle.ts`: the selector `.p-stepper.p-stepper-readonly .p-step` in the styles does not fire either in PrimeNG or here. |

## Not verified

- Screen reading: `aria-*` were checked against PrimeNG, screen reader behavior was not.
- Right-to-left layout and touch input.
- The smoothness of the vertical panel's height animation was assessed only by classes and the final state (`display`), not frame by frame.

## Performance

See `benches/` (`stepper-1`, `stepper-1000`, `case-stepper`) and `benches/*budgets.toml`.
