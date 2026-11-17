# Design

## Context

Requirements are in `specs/inputtext`. PrimeNG 21.1.10 sources: `packages/primeng/src/inputtext/inputtext.ts` (the `pInputText` directive on a native `<input>`), `inputtext/style/inputtextstyle.ts`, demos `apps/showcase/doc/inputtext`. The directive puts classes and `data-p` on the native `<input>`; the value lives in the element itself and is synced with `ngModel`. `p-filled` is determined by whether the model is filled; the variant is `variant || config.inputStyle || config.inputVariant`. The component template is `docs/button.md`; `PrimeConfig.input_variant`, `Variant` and `Size` already exist in `prime-core`.

## Goals / Non-Goals

**Goals:** a value and event model that will become the reference for Checkbox, RadioButton, ToggleSwitch; classes and `data-p` as in PrimeNG; a static branch for the non-interactive (no binding) case.
**Non-Goals:** form integration (validators, dirty/touched), `KeyFilter`, `FloatLabel`, `IconField`, `InputGroup`, the `Fluid` context.

## Decisions

### D1. Component instead of a directive
`<InputText />` renders an `<input>`; Leptos has no directives on an arbitrary element with the same convenience. The native `<input>` attributes that are needed most often are listed as explicit properties.

### D2. Two-way binding — `RwSignal<String>`
`value: Option<RwSignal<String>>`: the field reads the signal (`prop:value` plus the `value` attribute for SSR) and writes to it on `input`. Without `value` the field is uncontrolled: the value lives in the DOM, `p-filled` is not tracked (in PrimeNG it affects only floating labels). This is chosen so that a static `InputText` does not create a signal.
*Alternatives*: a `value` + `on_input` pair (clumsy, no two-way binding); an internal signal always (an extra signal for every field).

### D3. Events
`on_input`, `on_change`, `on_focus`, `on_blur` receive the original `web_sys::Event` (`FocusEvent` for focus). The callbacks are called after the value is written to the signal.

### D4. Variant from the configuration
`variant: OptProp<Variant>`; the resulting variant is `variant` or `PrimeConfig.input_variant` (read from the context when the instance is created).

### D5. Branches
The static branch is chosen with `all_static!(…)` and `value.is_none()`: `p-filled` is not needed, and the `<input>` has no value handlers. Otherwise the reactive one: attributes are closures, `prop:value` and `on:input`.

### D6. Tests
SSR: structure, attributes, classes, `data-p`, `pt`, branch parity, the value in `value`. Browser (`wasm-bindgen-test`): input updates the signal, a signal change updates the field, events, `disabled`/`readonly`.

## Risks / Trade-offs

- [An uncontrolled field does not track `p-filled`] → recorded in the spec and `docs/inputtext.md`.
- [`prop:value` in SSR] → checked by an SSR test (the value must get into the HTML as the `value` attribute).

## Migration Plan

No migration: a new crate. Rollback — `git revert`.

## Open Questions

None.
