# Design

## Context

Requirements are in `specs/accordion` and the `specs/collapsible` delta. PrimeNG 21.1.10 sources: `packages/primeng/src/accordion/accordion.ts` (four components in one file), `accordion/style/accordionstyle.ts`, demos `apps/showcase/doc/accordion`. References: `crates/prime-panel`, `crates/prime-fieldset`, `prime-rs-core::collapsible`. In PrimeNG the animation runs on the `p-motion` element inside `p-accordioncontent`; the CSS `.p-accordioncontent .p-motion { display: grid; grid-template-rows: 1fr }` lives in the `/* For PrimeNG */` block of `accordionstyle.ts`, not in `@primeuix/styles`, so the theme generator does not pick it up yet.

## Decisions

### D1. Context between the parts
`Accordion` puts `AccordionCtx { id, value: RwSignal<Vec<String>>, multiple: Prop<bool>, select_on_focus: Prop<bool>, on_open, on_close, pt: Arc<AccordionPt> }` into the Leptos context, `AccordionPanel` — `PanelCtx { value: String, disabled: Prop<bool>, active: Signal<bool> }`. The header and content read both contexts and take no values themselves. Outside `Accordion` or `AccordionPanel` the parts panic with a clear message (`expect`).

### D2. Value
`value: Option<RwSignal<Vec<String>>>` and `default_value: Vec<String>`; the list is shared by both modes (in PrimeNG — `string | number | array`). Values are strings; numbers are converted with `to_string()`. Toggling: single mode — `[v]` or empty if `v` is already the only one; `multiple` — add or remove. This is a pure function `toggle_value(&[String], &str, multiple) -> Vec<String>` with a table of unit tests. `active = value.contains(panel_value)`.

### D3. Events
A click on the header (and Enter/Space, `select_on_focus`) computes `was_active` and `is_active` before and after `toggle`, as in PrimeNG: opening gives `on_open`, closing — `on_close`. The event is `AccordionToggle { original_event: web_sys::Event, index: String }`.

### D4. Animation via shared logic
`prime-rs-core::collapsible` gets `bind_region_with(region, collapsed: impl Fn() -> bool + 'static, on_done)`; `bind_region` reduces to it (`move || state.get()`), behaviour unchanged. Content: `div.p-accordioncontent[role=region] > div.p-motion (NodeRef, animated) > div.wrapper > div.content`. Initial style `display: none` on `p-motion` if the panel is inactive (`hidden_style`). Hiding is `display: none`, not `visibility`, as in Panel and Fieldset (a difference from PrimeNG `hideStrategy="visibility"`; recorded in the docs).

### D5. Theme CSS
`xtask gen-theme` reads from `<name>/style/<name>style.ts` the part after the `/* For PrimeNG */` marker up to the closing backtick, runs `dt('a.b.c')` through the same `resolved` and appends it to the component's CSS. For components without the marker nothing changes (the CSS of Panel, Fieldset and the rest stays byte-for-byte the same, checked with `git diff`). An unresolved `dt()` is still an error.

### D6. Keyboard
A `keydown` handler on the header (client): `ArrowDown/ArrowUp/Home/End` look for neighbouring `.p-accordionpanel` in the parent (`next/previous_element_sibling`), skipping `data-p-disabled="true"`, and focus the inner `.p-accordionheader`; the end of the list wraps around. `Enter/Space/NumpadEnter` call toggling and `preventDefault`. `select_on_focus` is a `focus` handler. DOM navigation is factored into functions, the pure part (picking the next index with wrapping and skipping) is covered by unit tests.

### D7. Branches and re-rendering
Activity depends on the shared signal, so there is no static branch: classes, `aria-*` and the icon are nested closures over `active`; the whole panel is not re-rendered. `NodeRef` only on `wasm32`. Markup is a `macro_rules!` per part.

### D8. Demos
`basic`, `multiple`, `disabled`, `controlled` (external signal and buttons), `dynamic` (`<For>`), `template` (icon slot and avatar/badge). The "plus/minus" icons are SVGs from `prime-icons`.

## Risks / Trade-offs

- [`p-motion` divergence from PrimeNG] → the element's class and position are the same, the name is in the docs; visual comparison in `pages.json`.
- [Values are strings only] → a compact API; numbers via `to_string`, recorded in the docs.
- [The xtask change affects all components] → the rule applies only when the marker is present; `git diff` over the generated CSS of the other components must be empty.
- [Nested Accordions] → the nearest context is taken (`use_context`), nesting works.

## Migration Plan

No migration: a new crate, a function added to `prime-rs-core`. Rollback — `git revert`.

## Open Questions

None.
