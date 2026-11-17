# Design

## Context

The requirements are in `specs/togglebutton`. PrimeNG 21.1.10 sources: `packages/primeng/src/togglebutton/togglebutton.ts`, `togglebutton/style/togglebuttonstyle.ts`, demos `apps/showcase/doc/togglebutton`. The templates are `crates/prime-toggleswitch` (`docs/toggleswitch.md`: `checked: RwSignal<bool>`, `on_change`, the `rx` closures, the `macro_rules!` markup, browser tests) and `crates/prime-button` (the icon is a `ViewFn` in a `span` wrapper).

## Decisions

### D1. State
`checked: Option<RwSignal<bool>>`. Without it the component creates an internal `RwSignal` with `default_checked` — in PrimeNG the component also always keeps the state itself (`checked`), there is no native `<input>`, so there is no DOM state to align. There is no static branch: any press changes `class`, `aria-pressed`, `data-*` and the label, so the markup is always built with reactive `rx(props, pt, fn)` closures.

### D2. Toggling
A `toggle` handler as in PrimeNG: `if !disabled && !(allow_empty == Some(false) && checked)`. It is attached to `click` and to `keydown` (`Enter`, `Space` by `event.code`, with `preventDefault`). `allow_empty` is a `Prop<Option<bool>>` in the PrimeNG sense: the restriction applies only when `false` is set explicitly. `on_change` receives `ToggleButtonChange { checked, original_event }`.

### D3. Markup and attributes
The root is a `<div>` (instead of the `<p-togglebutton>` host). The order of the classes as in `togglebuttonstyle.ts`: `p-togglebutton p-component`, `p-togglebutton-checked`, `p-invalid`, `p-disabled`, `p-togglebutton-sm p-inputfield-sm` / `p-togglebutton-lg p-inputfield-lg`, `p-togglebutton-fluid`, then `style_class` and `pt.root`. `tabindex`: the value given, otherwise `0`, with `disabled` `-1`. `data-p` — `checked invalid <size>` (`DataP`). An empty label is replaced by a space (`' '`), as in the PrimeNG template. The pure logic (classes, `data-p`) is in `state.rs` with a table test, as for ToggleSwitch.

### D4. Icons and slots
`on_icon`/`off_icon` — `Option<ViewFn>` in a `span.p-togglebutton-icon` wrapper (+ `-left`/`-right`): there is no PrimeIcons font, the wrapper is needed for the CSS (`order`). `icon_template: Callback<bool, AnyView>` replaces the default icon, `content_template: Callback<bool, AnyView>` — the whole content after `span.p-togglebutton-content`. `IconPos` is the crate's own enum (`Left`, `Right`): a dependency on `prime-button` for a single enumeration is not needed.

### D5. CSS
In `togglebuttonstyle.ts` the block is called `/* For PrimeNG (iconPos) */`, while `xtask gen-theme` looks for the exact marker `/* For PrimeNG */`. We fix `primeng_extra_block`: it accepts the marker `/* For PrimeNG` with an optional explanation before `*/`, and we add `togglebutton` to `primeng_extra`. The rule `.p-togglebutton.ng-invalid.ng-dirty` refers to the Angular Forms classes and is harmless (there are none in Leptos).

### D6. Demos
`basic`, `customized`, `disabled`, `fluid`, `invalid`, `sizes`. The `prime-icons` set has no `lock` and `lock-open`, so `customized` uses `CheckIcon` and `TimesIcon` instead (in the source `onIcon`/`offIcon` are set twice, and `pi-lock`/`pi-lock-open` take effect). The demos with forms are skipped.

### D7. Tests
SSR: structure, default labels, icons and position, slots, classes and `data-p` for all combinations, attributes, `tabindex`, `pt`. Browser (`tests/web.rs`, wasm-bindgen-test): click, Enter and Space, `disabled`, `allow_empty`, one call of `on_change` per press, focus and loss of focus. A new crate with `tests/web.rs` is added to the browser job of `ci.yml` (3 places).

## Risks / Trade-offs

- [There is no static branch, so there are more allocations than in the static components] → a budget by measurement, as for ToggleSwitch.
- [The change of `xtask gen-theme` affects all the components] → the marker is extended in a backward-compatible way; after the change `cargo xtask gen-theme` must not change the CSS of the other components (check with `git diff`).

## Migration Plan

No migration: a new crate. Rollback — `git revert`.

## Open Questions

None.
