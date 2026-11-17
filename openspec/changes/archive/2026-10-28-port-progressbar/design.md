# Design

## Context

Requirements are in `specs/progressbar`. PrimeNG 21.1.10 sources: `packages/primeng/src/progressbar/progressbar.ts`, `progressbar/style/progressbarstyle.ts`, demos `apps/showcase/doc/progressbar`. The component template is `docs/toggleswitch.md`, `docs/message.md`.

## Goals / Non-Goals

**Goals:** markup, classes, ARIA and `data-p` as in PrimeNG; reactive value and mode; a static branch for an unchanging indicator.
**Non-Goals:** own animations, form integration.

## Decisions

### D1. The root is a `<div>`
Instead of the `<p-progressbar>` host; ARIA attributes and `data-p` are on the root.

### D2. Value
`value: OptProp<f64>`; the number is rendered via `Display` (`50` without a fractional part). In `determinate` mode the width is `value%` (with no value - `0%`, like zero width in PrimeNG: `undefined%` is discarded by the browser). `aria-valuenow` and `aria-level` are rendered only when a value is set (PrimeNG renders `aria-level="undefined%"` - a deliberate difference).

### D3. Label
`div.p-progressbar-label` contains a `div` with the value and unit, hidden (`display: none`) when there is no value or it is zero; with `show_value = false` or a set `content_template` the inner `div` is not rendered, and the template receives the value.

### D4. Modes
`mode` may be a signal; the content is rebuilt when the mode changes. `indeterminate` renders only `div.p-progressbar-value` without width and label.

### D5. Branches
Static - with `all_static!`; otherwise reactive. The content is built by a function over the properties, the wrapper by a macro.

### D6. Tests
SSR: structure of both modes, ARIA, width, label, `color`, template, `pt`, branch parity. No browser tests: the component has no client-side behavior.

## Risks / Trade-offs

- [Number without locale formatting] -> as in PrimeNG; formatting is on the caller's side (`content_template`).

## Migration Plan

No migration: a new crate. Rollback is `git revert`.

## Open Questions

None.
