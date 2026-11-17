# Design

## Context

The requirements are in `specs/inplace`. PrimeNG 21.1.10 sources: `inplace/inplace.ts`, `inplace/style/inplacestyle.ts`; demos `apps/showcase/doc/inplace/{basic,input,image,lazy}-doc.ts`. Project templates: `prime-button` (the close button), `prime-scrolltop` (markup over `Button`), `prime-fieldset` (the two-way signal model and `Pt`).

Observations on the source:
- `onActivateClick`/`onDeactivateClick` check `preventClick`; `onKeydown` for `Enter` calls `activate` without this check and `preventDefault`.
- `activate`/`deactivate` do nothing when `disabled`.
- The close button without `closeIcon` contains `TimesIcon`; with `closeIcon` — an icon from a font class.

## Decisions

### D1. Crate and dependencies
`prime-inplace` with `Inplace` and `InplacePt { root, display, content, close_button }`. It depends on `prime-rs-core`, `prime-button`, `prime-icons` (`times`), `web-sys`.

### D2. Pure logic (`state.rs`)
`display_classes(disabled)` — `p-inplace-display` and `p-disabled`; `next_state(active, action, disabled, prevent_click) -> bool` — a table-driven decision for `Click`, `Enter`, `Close`. The table test covers all combinations.

### D3. State
`active: Option<RwSignal<bool>>`; without it a local signal with the initial value `false`. The `Callback<Callback<()>, AnyView>` for `content` is called each time the content is shown; the close function `Callback<()>` performs `deactivate` (without an event — `on_deactivate` receives the button event, while from the content callback it is not called with an event, so `on_deactivate: Callback<()>` takes no argument).

### D4. Callbacks without an event
`on_activate` and `on_deactivate` are `Callback<()>`: in a Leptos handler the DOM event is needed only for `prevent_default` and is otherwise of no interest to consumers (the difference is recorded in the docs).

### D5. Markup
The content is selected by a closure over `active`. Display is a `div` with `on:click` and `on:keydown`; Content is a `div` with `content.run(close)` and, when `closable`, a `Button` with `button_type="button"` and the PrimeNG default `text` variant (no `text`, as in the original).

### D6. CSS
`inplace` is added to `theme.components`; `cargo xtask gen-theme`, a check that the CSS of the other components did not change.

### D7. Tests
SSR: the default view, the active state, `disabled`, `closable`, `pt`. Browser `tests/web.rs`: click, `Enter`, `prevent_click`, `disabled`, the close button, the close callback, the external `active` signal, the calls of `on_activate`/`on_deactivate`. e2e: real clicks and keys on the page. The crate is added in the three places of `ci.yml`.

### D8. Benchmarks
Scenarios `inplace-1`/`-1000` (SSR, the view), `case-inplace`; first `cargo bench -p render-bench --bench alloc`, for wasm a temporary large budget.

## Risks / Trade-offs

- [Re-rendering of the content on each show] → this is how `*ngIf` behaves in the original; recorded in the docs.
- [Focus when switching] → in the original the focus is not moved; we keep it that way.

## Migration Plan

No migration. Rollback — `git revert`.

## Open Questions

None.
