# Design

## Context

The requirements are in `specs/fieldset`. PrimeNG 21.1.10 sources: `packages/primeng/src/fieldset/fieldset.ts`, `fieldset/style/fieldsetstyle.ts`, demos `apps/showcase/doc/fieldset`. The template is `crates/prime-panel` (`docs/panel.md`): the same `CollapseState`, `CollapseIds`, `bind_region`, `aria_*`, `hidden_style`, the structure of two branches and `macro_rules!`. The collapsing logic does not change, so the `collapsible` spec is not affected.

## Decisions

### D1. Only the markup and the binding
`Fieldset` creates `CollapseState::new(collapsed, default_collapsed)` and `CollapseIds::new(id)`, calls `bind_region` (only `wasm32` and `toggleable`), and takes the ARIA attributes from the functions of `prime_rs_core::collapsible`. There are no copies of the animation, `tabindex` or identifiers in the crate.

### D2. Identifiers
`{id}_header` is the `id` of the button, to which the `aria-labelledby` of the region points; `{id}_content` is the identifier of the region. Without `toggleable` there is no button, so `aria-labelledby` is not rendered (in PrimeNG it remains a dangling reference). `{id}-titlebar` is not used.

### D3. Legend
- without `toggleable`: `<legend><span.p-fieldset-legend-label>{legend}</span>{header_template}</legend>`;
- with `toggleable`: `<legend><button …>{icon}<span.p-fieldset-legend-label>{legend}</span>{header_template}</button></legend>`.
The icon: `span.p-fieldset-toggle-icon` around the slot `expand_icon_template`/`collapse_icon_template`, and without slots — an svg `PlusIcon`/`MinusIcon` with the class `p-fieldset-toggle-icon`. The icon depends on `collapsed`, so it is read in a nested closure, and the legend is not re-rendered as a whole.

### D4. Button
Its own `<button type="button">` with `aria-controls`, `aria-expanded`, `aria-label`. Enter/Space — the native `click` (as in Panel). `on_before_toggle` receives the `MouseEvent` and the previous `collapsed`; `preventDefault` is called.

### D5. Branches and classes
The static branch — not `toggleable` and all properties static. The markup is in a `macro_rules!`; `Arc<Props>` and `Arc<FieldsetPt>`; `NodeRef` only on `wasm32` (as in Panel). `data-p` on the root and the legend. The order of the classes is as in `fieldsetstyle.ts`. The root is `<fieldset>`, as in PrimeNG.

### D6. Tests
SSR: structure, classes, ARIA, slots, `pt`, parity. Browser: click, signal, `aria-expanded`, the animation class and `on_after_toggle`, the `tabindex` of the descendants, events. e2e — the real CSS animation.

## Risks / Trade-offs

- [The `<button>` inside `<legend>` depends on the theme styles] → checked in e2e by visibility and click.
- [The markup of Panel and Fieldset is similar] → only the logic is shared (`prime-rs-core`), the markup differs; extracting a common component is premature.

## Migration Plan

No migration: a new crate. Rollback — `git revert`.

## Open Questions

None.
