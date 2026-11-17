# Design

## Context

The motivation is in `proposal.md`, the requirements are in `specs/badge` and `specs/button`. PrimeNG 21.1.10 sources: `packages/primeng/src/button/button.ts` (the `p-button` component, lines 570–920), `button/style/buttonstyle.ts` (`classes`), `badge/badge.ts`, `badge/style/badgestyle.ts`.

What was found during the study:
- `p-button` in Angular is a host element without classes; inside it is `<button class="p-button p-component …">`. All classes and `data-p*` live on the `<button>`.
- The order of the content: projected content, then the loading indicator or the icon, then the label (`span.p-button-label`), then `p-badge`. The label and the badge are not rendered with a custom content template.
- In PrimeNG the icon is a string with a class name (`pi pi-check`) or a template; we have no font, the icons are `prime-icons` components.
- `p-badge` is a `<p-badge>` host element with the classes on itself, inside only the text.
- Ready in `bootstrap-workspace`: `Pt`, `ClassNames`/`class_memo`, `PrimeConfig` (including `ripple`), `Ripple`, `FocusTrap`, `SpinnerIcon` (feature `spinner`), the CSS `button.css` and `badge.css` in `prime-theme`, benchmark scenarios, budgets, the showcase, `gen-spec`.

Check against `cargo xtask gen-spec` (task 1.1): all inputs of `p-button` and `p-badge` are covered by specs, except those deliberately not ported: `hostName` (internal), `badgeClass` (outdated, replaced by `badgeSeverity`), `buttonProps` (in Rust these are ordinary properties), `pButtonPT`, `ptButtonDirective`, `ptButtonLabel`, `ptButtonIcon`, `*Unstyled` (directives and the `unstyled` mode are out of scope), `badgeStyle`, `badgeStyleClass`, `badgeDisabled` of the `pBadge` directive (the directive is not ported; `badgeDisabled` is covered for `p-badge`). The outdated `size` of Badge is replaced by `badgeSize`: in Rust there is one input `size`. All three outputs of `p-button` (`onClick`, `onFocus`, `onBlur`) are covered.

## Goals / Non-Goals

**Goals:**
- Button and Badge that match PrimeNG in classes, `data-p*` attributes, order and DOM composition (except the host elements).
- Decisions that become the template for the other components: the form of the properties, `pt`, icons, ripple, tests, benchmarks, the showcase page.

**Non-Goals:**
- The `pButton` directive, `ButtonLabel`, `ButtonIcon`, `buttonProps`, the `Fluid` context (`fluid` is only a property), the `unstyled` mode, `SplitButton`, `ButtonGroup`.
- The full set of PrimeNG `pt` hooks (`pt:*` on any part, the `ptOptions` options): for now the root and the listed parts.

## Decisions

### D1. Reactivity of the properties
The properties that in practice change at runtime (`label`, `loading`, `disabled`, `severity`, `badge`, `icon_pos`, the `value` of Badge) are accepted as `Signal<T>` (`#[prop(into)]`; values and closures are converted automatically). The others (`type`, `tabindex`, `aria_label`) are plain values. This allows passing both a constant and a signal without `.into()` on the caller side, and static buttons do not pay for subscriptions.
*Alternative*: all properties are `Signal` (simpler uniformity, but extra subscriptions on thousands of static buttons); all plain (`loading` cannot be changed without recreating).

### D2. Icons and content
`icon` and `loading_icon` are `Option<ViewFn>`. The icon is wrapped in `<span class="p-button-icon …" data-p="…">` (as the string icon in PrimeNG); the default loading indicator is `<SpinnerIcon spin=true class="p-button-loading-icon …">`. `children` is `Option<ChildrenFn>`: they are rendered first.
*Difference from PrimeNG*: with a template icon PrimeNG passes the class to the SVG itself; we always keep the `span` wrapper, so that the CSS (`order`, paddings) works for any icon without knowing its markup.

### D3. DOM without a host element
Only the `<button>` is rendered. The `<p-button>`/`<p-badge>` host in PrimeNG is an Angular detail; the theme CSS is bound to classes. For Badge the root becomes `<span class="p-badge …">`.

### D4. Classes and `data-p`
One pure function per component builds the set of modifiers from the structure "snapshot of properties" (`ButtonState`); both `class` and `data-p*` are derived from it. The classes are assembled by `ClassNames` inside `class_memo`, so `class` is updated only when the string changes. The functions are tested without rendering (a table "properties → classes"), and are easy to check against `buttonstyle.ts`.
A common helper for `data-p` (flags separated by spaces, like `cn({...})`) is added to `prime-core` (`DataP`).

### D5. Pass-through
`prime-core::Pt` gets `style`; the component defines the structure of its parts: `ButtonPt { root, label, icon, loading_icon, badge }` (each one is a `Pt`), `BadgePt { root }`. The classes from `Pt` are added after the own ones (`Pt::merge_class`), the attributes — through `Pt::attrs()` with `{..}`. The structure of the parts lives in the component crate, the common mechanism — in `prime-core`.

### D6. Ripple and autofocus
The root always has the class `p-ripple`. `Ripple::attach` is called from an `Effect` (only in the browser, after mounting) when `PrimeConfig.ripple` is set; `Ripple` is kept and released through `on_cleanup`. Autofocus is the same `Effect`, with `NodeRef<Button>`; on the server the `Effect` does not run, so SSR is safe.

### D7. Crate dependencies
`prime-button` → `prime-core`, `prime-dom`, `prime-icons` (feature `spinner`), `prime-badge`. `prime-badge` → `prime-core`. The theme is connected by the application (`prime-theme`, features `button`, `badge`); the component crates do not pull in `prime-theme`, so as not to impose CSS.

### D8. Tests
1. Unit: a table "properties → classes and `data-p`" for both components (independent of SSR).
2. SSR (`to_html`): DOM structure, order, attributes, `pt`, a11y.
3. `wasm-bindgen-test` (Chrome): click and unavailability, focus and autofocus, ripple.
4. Benchmarks: `button-1`, `button-1000`, `badge-1`, `badge-1000`; a browser measurement — a scenario in the showcase.
5. Comparison with PrimeNG: the `/button` page in the showcase and an entry in `e2e/tests/pages.json`.

### D9. Showcase
Routing by component appears: `/`, `/button`, `/badge` through `leptos_router` (0.8). The component page repeats the PrimeNG demos (`apps/showcase/doc/button/*`), whose list is taken from the sources.
*Alternative*: one long page — not suitable for comparing screenshots page by page.

## Risks / Trade-offs

- [The `span` wrapper around the icon differs from PrimeNG with a template icon] → recorded in the spec; the visual comparison in the weekly job will show the differences.
- [The `Signal` properties increase the size of the component] → the budget `case-button`; if the contribution turns out large, some properties will become plain values.
- [The Leptos router will increase the showcase and `target/`] → it is needed only by the showcase and does not get into the library crates.
- [The list of PrimeNG demos is large] → we port those that cover all the properties of the spec; the others are added as needed.

## Migration Plan

No migration: new crates. Rollback — `git revert`. The budgets in `benches/budgets.toml` are revised after the first measurement of Button.

## Open Questions

None. The field `type` is named `button_type` (the attribute in the DOM stays `type`). The label with `aria-hidden` from PrimeNG (`icon && !label`) is not needed in Leptos: `span.p-button-label` is rendered only when the label is not empty.
