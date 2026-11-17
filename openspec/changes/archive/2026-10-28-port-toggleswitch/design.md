# Design

## Context

The requirements are in `specs/toggleswitch`. PrimeNG 21.1.10 sources: `packages/primeng/src/toggleswitch/toggleswitch.ts`, `toggleswitch/style/toggleswitchstyle.ts`, demos `apps/showcase/doc/toggleswitch`. In PrimeNG the click is handled on the host (`@HostListener('click')`): `modelValue` is toggled, `onChange` is emitted, the focus returns to `<input>`. The component template and the value model are in `docs/checkbox.md`.

## Goals / Non-Goals

**Goals:** the classes, `data-p*` and the inline style of the root as in PrimeNG; a click on any part of the root; the handle slot; a static branch without a signal.
**Non-Goals:** forms, `trueValue`/`falseValue`, `autofocus`, `size`.

## Decisions

### D1. Root — `<div>`
Instead of the `<p-toggleswitch>` host; `style="position: relative"` — like `inlineStyles.root` in PrimeNG; `style` and `pt.root` are attached to it.

### D2. State
`checked: Option<RwSignal<bool>>`; without it — `default_checked`, and after that the state lives in the DOM, the root classes are not tracked.

### D3. Click
The `on:click` handler on the root: when `!disabled && !readonly` it toggles the signal, calls `on_change(ToggleSwitchChange { checked, original_event })`, and returns the focus to `<input>`. The native state of `<input>` is aligned with the final value after processing (with `readonly`/`disabled` it returns to the previous one). A separate `change` on `<input>` is not handled, so as not to fire the event twice.

### D4. Slot
`handle_template: Option<Callback<bool, AnyView>>` inside `div.p-toggleswitch-handle`, receives `checked`.

### D5. Branches
Static — when `all_static!` and `checked.is_none()`; otherwise reactive. The markup and the helper `rx(props, pt, fn)` — as in Checkbox.

### D6. Tests
SSR: structure, the root style, classes, `data-p*`, `aria-checked`, `pt`, the slot, parity. Browser: a click on the root and on `<input>` toggles, `readonly`/`disabled`, events.

## Risks / Trade-offs

- [A click on `<input>` bubbles to the root and is handled once] → checked by a browser test.

## Migration Plan

No migration: a new crate. Rollback — `git revert`.

## Open Questions

None.
