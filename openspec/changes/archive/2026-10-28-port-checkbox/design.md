# Design

## Context

The requirements are in `specs/checkbox`. PrimeNG 21.1.10 sources: `packages/primeng/src/checkbox/checkbox.ts`, `checkbox/style/checkboxstyle.ts`, demos `apps/showcase/doc/checkbox`. The `<p-checkbox>` root contains a hidden `<input type="checkbox">` and a `<div class="p-checkbox-box">` with an icon. The value model and the component template — `docs/inputtext.md`, `docs/button.md`.

## Goals / Non-Goals

**Goals:** classes and `data-p` as in PrimeNG; two value modes; a static branch for the case without value signals.
**Non-Goals:** forms (validators, dirty/touched), arbitrary `trueValue`/`falseValue`, `checkboxIcon` as a CSS class.

## Decisions

### D1. Root — `<div>`
Instead of the `<p-checkbox>` host (as for the other components). The `data-p*` attributes are put on the root.

### D2. Two value modes
- `binary=true`: `checked: Option<RwSignal<bool>>`.
- otherwise: `value: String` and `model: Option<RwSignal<Vec<String>>>`; checked if `model` contains `value`; toggling adds or removes `value`.
Without a signal the state is uncontrolled: the initial value comes from `default_checked`, after that the state lives in the DOM, and the root classes are not tracked.
*Alternative*: a generic value type (`T: PartialEq`) — rejected for the sake of simplicity and wasm size.

### D3. Computing the state
`checked = !indeterminate && (binary ? checked_signal : model.contains(value))`. `indeterminate` is a two-way `RwSignal<bool>` (with it the static branch is not selected); when the user changes it, it is reset to `false`, `on_change` receives `CheckboxChange { checked, original_event }`; with `readonly` the change is ignored (the native state of `<input>` returns to the signal).

### D4. Icon
`check` when `checked`, `minus` when `indeterminate`; the `icon_template` slot (`ViewFn`) replaces the icon; the icons are from `prime-icons`.

### D5. Branches
Static — when `all_static!` holds and there are no value signals; otherwise reactive. One markup, in a `macro_rules!`.

### D6. Tests
SSR: structure, classes, `data-p*`, attributes, modes, `pt`, parity. Browser: a click changes the signal, a set of values, `indeterminate`, `readonly`, `disabled`, events.

## Risks / Trade-offs

- [`<input>` is hidden and the click goes to it over the box] → the theme CSS `p-checkbox-input` covers the box; checked by e2e.

## Migration Plan

No migration: a new crate. Rollback — `git revert`.

## Open Questions

None.
