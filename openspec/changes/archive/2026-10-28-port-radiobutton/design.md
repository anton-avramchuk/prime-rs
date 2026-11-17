# Design

## Context

The requirements are in `specs/radiobutton`. PrimeNG 21.1.10 sources: `packages/primeng/src/radiobutton/radiobutton.ts`, `radiobutton/style/radiobuttonstyle.ts`, demos `apps/showcase/doc/radiobutton`. The component template and the value model are in `docs/checkbox.md` and `docs/inputtext.md`.

## Goals / Non-Goals

**Goals:** classes and `data-p*` as in PrimeNG; selecting one value of a group through a shared signal; a static branch for the case without a signal.
**Non-Goals:** forms, `RadioControlRegistry`, `binary`, `autofocus`.

## Decisions

### D1. Root — `<div>`
Instead of the `<p-radiobutton>` host; `data-p*` — on the root.

### D2. Group through a shared signal
`model: Option<RwSignal<Option<String>>>` and `value: OptProp<String>`: checked if `model == Some(value)`. A choice writes `Some(value)` to `model`; the other buttons of the group update reactively, no registry is needed. Without `model` — an uncontrolled button with `default_checked` (native grouping by `name`).

### D3. The `value` attribute
In PrimeNG `[attr.value]="modelValue()"` renders the boolean state. We render our own `value` (otherwise the native form loses its meaning); the difference is recorded in `docs/radiobutton.md`.

### D4. Events
`on_click` receives `RadioButtonClick { value, original_event }` after writing to `model`; with `disabled` the choice is ignored (the state returns to `model`).

### D5. Branches
Static — when `all_static!` holds and `model.is_none()`; otherwise reactive. The markup and the helper `rx(props, pt, fn)` — as in Checkbox.

### D6. Tests
SSR: structure, classes, `data-p*`, attributes, group, `pt`, parity. Browser: a click selects the value, the group clears the others, `disabled`, events.

## Risks / Trade-offs

- [Two buttons with one `name` without `model` behave natively] → documented.

## Migration Plan

No migration: a new crate. Rollback — `git revert`.

## Open Questions

None.
