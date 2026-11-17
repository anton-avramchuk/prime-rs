# Design

## Context

The requirements are in `specs/chip`. PrimeNG 21.1.10 sources: `packages/primeng/src/chip/chip.ts`, `chip/style/chipstyle.ts`, demos `apps/showcase/doc/chip`. Behaviour: `close()` sets `visible = false` and calls `onRemove`; `visible = false` gives the inline style `display: none` on the root; Enter and Backspace on the remove element also close it; `aria-label` is taken from `config.getTranslation(ARIA).removeLabel` (our locale already has the key `aria.removeLabel`). The component template — `docs/button.md`.

## Goals / Non-Goals

**Goals:** Chip with the same classes, `data-p`, content order and removal behaviour; a static branch where it applies.
**Non-Goals:** `chipProps`, icon fonts, `unstyled`.

## Decisions

### D1. Root — `<div>`
The `<p-chip>` host in Angular is a framework detail; ours is `<div class="p-chip …">`.

### D2. Visibility state
`visible` is an internal `RwSignal<bool>`, created only in the reactive branch. The static branch is selected by the condition `all_static!(…) && !removable`: a Chip without removal cannot hide, while a removable one needs root reactivity (`display: none`), so it takes the reactive path even if all properties are static. In the reactive branch the root style is a closure that depends on `visible`.

### D3. Remove element
`span.p-chip-remove-icon[role=button][tabindex][aria-label]` with `TimesCircleIcon` (decorative) inside, or `remove_icon`. A difference from PrimeNG: for the default icon PrimeNG puts these attributes directly on the `<svg>`; ours is a `span` wrapper, because the `prime-icons` icon sets its own a11y semantics, and `role="button"` must be on the element with the handlers. The label is taken from `PrimeConfig.locale` when the block is created.

### D4. Events
`on_remove: Callback<web_sys::Event>` receives a `MouseEvent` or a `KeyboardEvent` as an `Event`; `on_image_error: Callback<web_sys::Event>`. The Enter and Backspace keys are handled on the `keydown` of the remove element.

### D5. Tests
SSR: structure, classes, order, `pt`, the label from the locale, branch parity. Browser (`wasm-bindgen-test`): click, Enter, Backspace, one call of `on_remove`, hiding, image error, `tabindex` with `disabled` — for the static (non-removable) and the reactive variants where applicable.

## Risks / Trade-offs

- [A removable Chip is always reactive] → deliberate: the root depends on internal state; this is recorded in the spec.
- [The `span` wrapper around the remove icon differs from PrimeNG] → documented in `docs/chip.md`, checked by comparing screenshots.

## Migration Plan

No migration: a new crate. Rollback — `git revert`.

## Open Questions

None.
