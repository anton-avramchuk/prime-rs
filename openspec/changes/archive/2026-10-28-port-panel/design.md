# Design

## Context

The requirements are in `specs/panel` and `specs/collapsible`. PrimeNG 21.1.10 sources: `packages/primeng/src/panel/panel.ts`, `panel/style/panelstyle.ts`, demos `apps/showcase/doc/panel`. Fieldset (`fieldset/fieldset.ts`) uses the same pair `pMotion="p-collapsible"` + an ARIA region, so the collapsing logic is shared. The animation CSS is already in `crates/prime-theme/css/base.css`: `.p-collapsible-enter-active` / `.p-collapsible-leave-active` run the keyframes `p-animate-collapsible-expand|collapse` (they change `grid-template-rows` `0fr`↔`1fr`, 0.2 s, `overflow: hidden`); the CSS `panel` will be added through `cargo xtask gen-theme` (`upstream.toml`).

## Decisions

### D1. The shared module `prime-rs-core::collapsible`
Not a separate crate: the logic is small and pulls in no dependencies except `leptos` and `web-sys` (already present in `prime-rs-core`). Contents:
- `CollapseState { collapsed: Signal<bool>, toggle() }` — over an external `Option<RwSignal<bool>>` or an internal `RwSignal` created from the initial value;
- `CollapseIds::new(id)` — `{id}_header`, `{id}_content`, `{id}-titlebar`; `id` is either the one given by the user or a unique one from a counter (deterministic during SSR and on the client, as in the existing components; check against the practice of Checkbox/RadioButton when applying);
- the pure functions `aria_expanded`, `aria_hidden`, `region_tabindex` for the attributes;
- `animate(el: &HtmlElement, expanding: bool, done: impl FnOnce())` — the client part (D3), `cfg(target_arch = "wasm32")`, empty during SSR.
Fieldset (the next change) takes this module without changes.

### D2. Two-way `collapsed`
The property `collapsed: Option<RwSignal<bool>>` (like `value` of InputText). Without a signal — an internal state; the initial value is a separate property `default_collapsed: bool` (the analogue of the one-way `[collapsed]` in Angular), so as not to mix "signal" and "literal" in one type. The root class and the ARIA are computed reactively from `collapsed`.

### D3. Animation
The container `p-panel-content-container` is always in the DOM. States:
- expanded: normal display (`display: grid` is set by the theme CSS);
- collapsed: `display: none` through the `hidden` style (during SSR — right away, without animation).
On the client, when the state changes (not on the first render): expanding — remove `display:none`, add `p-collapsible-enter-active`; collapsing — add `p-collapsible-leave-active`, after `animationend` remove the class and set `display:none`. `animationend` is listened to once; a fallback timer (`set_timeout` 400 ms) triggers the completion if the event does not arrive (`prefers-reduced-motion`, a hidden tab). The completion calls `on_after_toggle(collapsed)`. The first run of the effect is skipped with a flag, so that mounting is not animated.

### D4. Toggling by the header and by the button
`toggler = header`: a click handler on the header; the button in this mode also receives the click, but the header handler fires on bubbling, and the button in `header` mode has no toggling of its own — so there is no double call (in PrimeNG the same logic is done by checking `toggler`). Enter/Space on the button: the native `<button>` already turns them into `click`, so a separate `keydown` is not needed; by the browser default the event toggles once (in PrimeNG a `keydown` is needed because of `role=button` on `p-button`; checked by e2e). `on_before_toggle` receives the `MouseEvent` and the previous `collapsed`.

### D5. The toggle — its own button markup
`Button` does not fit: `aria-expanded` changes reactively, and `ButtonPt` accepts only static attributes. So the `<button type="button">` is rendered by Panel itself with the same classes and `data-p` as PrimeNG's `p-button`: `p-button p-component p-button-icon-only p-button-secondary p-button-rounded p-button-text p-panel-toggle-button`, inside it a `span.p-button-icon` with `MinusIcon`/`PlusIcon` (`prime-icons`). There is no dependency on `prime-button`; `pt.toggle_button` is applied to this button.

### D6. Slots
`ViewFn` instead of `ng-template`: `header_template`, `icons_template`, `header_icons_template` (`Callback<bool, AnyView>`, receives `collapsed`), `footer_template`. The content is `children`. Instead of `<p-header>`/`<p-footer>` — only slots.

### D7. Access to the hidden content
`tabindex="-1"` on the focusable descendants: a client `Effect` on a change of `collapsed` walks `input, button, select, a, textarea, [tabindex]` inside `p-panel-content`, as PrimeNG does. In SSR the collapsed panel is hidden with `display:none`, so the focus is unavailable even without this.

### D8. Branches and classes
The static branch — not `toggleable` and all properties static; otherwise reactive. The markup is in a `macro_rules!`; `Arc` only in the reactive branch. The order of the classes is as in `panelstyle.ts`. `data-p` — by `dataP` (`toggleable`), in the same way as `prime-rs-core::data_p`.

## Risks / Trade-offs

- [The animation of `grid-template-rows` depends on the theme CSS] → checked by e2e by the class change and the final visibility, not by pixels.
- [A double toggle with `toggler = header`] → a separate scenario in the browser test.
- [The module in `prime-rs-core` grows] → it holds only the collapsing logic; when Accordion appears it is split off, if needed.
- [`default_collapsed` diverges from Angular `[collapsed]`] → recorded in `docs/panel.md` in the table of differences.

## Migration Plan

No migration: a new crate and module. Rollback — `git revert`.

## Open Questions

None.
