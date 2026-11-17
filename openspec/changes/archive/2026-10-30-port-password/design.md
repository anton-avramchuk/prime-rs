# Design

## Context

Requirements are in `specs/password`. PrimeNG 21.1.10 sources: `password/password.ts`, `password/style/passwordstyle.ts`; demos `apps/showcase/doc/password/*-doc.ts`. Models in the project: `prime-inputtext` (the field and value model), `prime-select` (panel inside the root, `align_overlay(Relative)`, z-index `ZGroup::Overlay`, `is_disposed()` check in timers), `prime-slider` (`window` listeners), `prime_rs_core::fluid`.

Observations about the source:
- This is `p-password` (the component); we do not port the `pPassword` directive.
- Estimate: `strongRegex` (`^(?=.*[a-z])(?=.*[A-Z])(?=.*[0-9])(?=.{8,})`) → 3, `mediumRegex` → 2, a non-empty string → 1, otherwise a prompt (`meter = null`).
- Panel: `onInputFocus` and `onKeyUp` open it, `onInputBlur` and `Escape` close it; `updateUI` is called on `keyup`, and therefore also when the value is written from outside (`writeControlValue`).
- The root gets `position: relative` with `appendTo = self`; the panel is `position: absolute`.
- The clear icon is shown when `value != null` (in PrimeNG an empty string also counts as a value); we show it for a non-empty string.

## Decisions

### D1. Crate and dependencies
`prime-password` with `Password`, `PasswordStrength { Weak, Medium, Strong }`, `PasswordPt`. Depends on `prime-rs-core`, `prime-dom`, `prime-inputtext`, `prime-icons` (`eye`, `eyeslash`, `times`), `web-sys`.

### D2. Pure logic (`state.rs`)
- `strength(value: &str) -> Option<PasswordStrength>` — `None` for an empty string; medium: length ≥ 6 (in characters) and at least two kinds; strong: length ≥ 8 and three kinds. Kinds: ASCII `a-z`, `A-Z`, `0-9` (as in the PrimeNG regular expressions).
- `meter_width(level)` — `33.33%`/`66.66%`/`100%`; `meter_class(level)`.
- `root_classes(...)`, `data_p(...)`.
Table-driven tests, including length boundaries and composition.

### D3. Difference: `strength` instead of regular expressions
`regex` does not support lookahead, and pulling in a second engine for two lines is excessive. The default estimate is equivalent to the default PrimeNG expressions; for other rules there is `strength: Callback<String, Option<PasswordStrength>>`. Recorded in `docs/password.md`.

### D4. Value model
As with `InputText`: an optional `RwSignal<String>`. Inside, the field is an `InputText` with its own `value`, to avoid duplicating state; the type (`password`/`text`) is switched by the `unmasked` signal. If `InputText` does not support a reactive `type`, the field is drawn by `Password` itself with the same classes (check during implementation; record the decision in docs).

### D5. Panel
Rendered in the root with a `Phase` analog: `Hidden → Shown → Leaving` (classes `p-password-enter`/`p-password-leave` from the `/* For PrimeNG */` block, a 300 ms timer with an `is_disposed()` check and a generation counter). Opens on `focus` and `keyup`/`input`, closes on `blur` and `Escape`. After showing: z-index `generate(Overlay, base)`, `align_overlay(Relative)`, `min-width` by the field width (CSS); on close — `zindex::revert`. On window `resize` the panel closes (unless it is a touch device, as in PrimeNG) — the listener is only on `wasm32`, removed in `on_cleanup`. Ancestor scrolling is not tracked (a difference, record in docs).

### D6. Icons
Clear, "eye" and "crossed-out eye" — `TimesIcon`, `EyeIcon`, `EyeSlashIcon` with PrimeNG classes; templates are `Option<ViewFn>`. Clicks are handled on the icons; the icons' `mousedown` must not take focus from the field (otherwise `blur` fires and the panel closes before the click).

### D7. Focus
The root gets `p-inputwrapper-focus` on the field's `focus`/`blur`. Autofocus (`autofocus`) is not ported in the first version.

### D8. CSS
`password` in `theme.components` and `primeng_extra`; `cargo xtask gen-theme`, check that the CSS of other components did not change. The `p-password.ng-invalid` selectors in the block are harmless.

### D9. Tests
SSR: default markup, `feedback = false`, `toggle_mask`, `show_clear`, `data-p`. Browser `tests/web.rs`: input and levels, opening and closing the panel, `Escape`, type switching, clearing, focus stays in the field on an icon click, z-index and its release. e2e: real input, panel visibility, the `Escape` key, show password. The crate is added in three places of `ci.yml`.

### D10. Benchmarks
Scenarios `password-1` and `-1000` (SSR, panel closed), `case-password` for wasm; first `cargo bench -p render-bench --bench alloc`, for wasm — a temporary large budget.

## Risks / Trade-offs

- [No `regex` with lookahead] → `strength` and equivalent default rules (D3).
- [Focus loss on icon clicks] → `prevent_default` on `mousedown` (D6), covered by a browser test.
- [Limit of 26 tachys attributes per element] → extra attributes via `Vec<AnyAttribute>`.
- [Touch input and screen readers] → not verified, honestly recorded in docs.

## Migration Plan

No migration. Rollback — `git revert`.

## Open Questions

None.
