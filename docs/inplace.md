# Inplace

Port of `p-inplace` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-inplace::Inplace`, `InplacePt` | `inplace/inplace.ts`, `inplace/style/inplacestyle.ts` |
| CSS | `@primeuix/styles/inplace` — `cargo xtask gen-theme` (the component has no `/* For PrimeNG */` block) |
| Demos | `apps/showcase/doc/inplace/{basic,input,image,lazy}-doc.ts` |

Spec: `openspec/specs/inplace`.

## What matches PrimeNG

- Markup and classes: `div.p-inplace.p-component[aria-live=polite]` > `div.p-inplace-display[tabindex=0][role=button]` (with `p-disabled` and `data-p-disabled`) or `div.p-inplace-content` with the content and, when `closable`, a close button with the `TimesIcon` icon.
- A click on the view and `Enter` activate; `Enter` cancels the default action. `prevent_click` disables the click and closing, but not `Enter` (as in PrimeNG). When `disabled`, nothing happens.
- The content receives the close function (`closeCallback`); it follows the same rules as the close button.
- The content is created on each show and removed when returning to the view (like `*ngIf`).

## Differences

| Difference | Reason |
|---|---|
| The root is a `<div>` without a `<p-inplace>` host | The Angular host is a framework detail. |
| `display: ViewFn` and `content: Callback<Callback<()>, AnyView>` instead of the `display` and `content` templates and the `pInplaceDisplay` / `pInplaceContent` selectors | Instead of `ng-template` and content projection. |
| `close_icon` is a `ViewFn`, not a font class name; there is no `closeicon` template | The project icons are SVG components. |
| `on_activate` and `on_deactivate` are `Callback<()>` without an event | The event is mainly needed for `preventDefault`; a state change is not tied to an event (closing can come from the content callback). |
| `active` is a two-way `RwSignal<bool>` | In PrimeNG it is an ordinary property that the component changes itself. |
| No `Ripple` on the close button | `Ripple` is not ported as a separate directive. |
| In the `lazy` demo a plain `<table>` replaces `Table`; `input` has no `pAutoFocus`; in `image` `EyeIcon` replaces the `pi-image` icon | `Table` is not ported; there is no `image` in `prime-icons`; `InputText` has no `node_ref`. |
| A click that closed the content does not activate the new view | The same click keeps bubbling to the newly created view; a target that has already been removed from the document is ignored. In PrimeNG the event is already handled by Angular before the replacement. |

## Cost

32 allocations per instance with SSR (the view), contribution to wasm ≈ 500 KB (budget 550 KB in `benches/budgets.toml`; mostly the shared `Button` and icon).

## Not verified

Touch and screen readers: the markup, click, `Enter`, closing and callbacks were verified in Chrome (e2e and browser tests). Focus is not moved on switching — as in PrimeNG.
