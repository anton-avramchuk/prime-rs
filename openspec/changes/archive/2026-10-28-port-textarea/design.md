# Design

## Context

The requirements are in `specs/textarea`. PrimeNG 21.1.10 sources: `packages/primeng/src/textarea/textarea.ts`, `textarea/style/textareastyle.ts`, demos `apps/showcase/doc/textarea`. In PrimeNG it is a directive on `<textarea>`; the component template and the value model are in `docs/inputtext.md`.

## Decisions

### D1. A component instead of a directive
`<Textarea />` renders a `<textarea>`; the value is an optional `RwSignal<String>` (as in InputText). Without `value` the field is uncontrolled, and `p-filled` is not tracked.

### D2. Value in SSR
The text of `<textarea>` is its child node, not the `value` attribute: the initial value is rendered as child text, and updates on the client go through `prop:value`.

### D3. Height auto-resize
`auto_resize` creates a client `Effect` (only `wasm32`, only when enabled): `style.height = auto`, then `scrollHeight` in pixels; when `max-height` is reached — `overflow-y: scroll` and the height equals `max-height`, otherwise `overflow: hidden`. `on_resize` is called after each adjustment. The logic is a pure function over numbers and is covered by a unit test.

### D4. Classes
The order is as in `textareastyle.ts`; `p-textarea-resizable` is set with `auto_resize`. The variant is `variant` or `PrimeConfig.input_variant`.

### D5. Branches
Static — when `all_static!` and `value.is_none()`; otherwise reactive. The markup is in a `macro_rules!`; `Arc` only in the reactive branch and in the handlers.

### D6. Tests
SSR: structure, attributes, classes, the value as child text, `pt`, parity. Browser: input writes to the signal, the signal changes the field, auto-resize changes the height and calls `on_resize`, events.

## Risks / Trade-offs

- [The adjustment depends on the layout] → checked in the browser by the change of height when the text grows.

## Migration Plan

No migration: a new crate. Rollback — `git revert`.

## Open Questions

None.
