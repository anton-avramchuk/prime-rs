# Proposal

## Why

Inplace is the first component of wave 3 and the simplest in it: an area that on click replaces the "view" with the "content" (editing in place, lazy loading). It needs no overlays and relies on the already ported `Button` and `TimesIcon`.

## What Changes

- A new crate `prime-inplace` with the `Inplace` component (a port of `p-inplace` from PrimeNG 21.1.10): the root `div.p-inplace.p-component` (`aria-live="polite"`) with one of two parts: `div.p-inplace-display` (`tabindex="0"`, `role="button"`, `p-disabled` when `disabled`) or `div.p-inplace-content` with the content and, when `closable`, a close button `Button` with the `TimesIcon` icon.
- Properties: `active` (a two-way `RwSignal<bool>`), `closable`, `disabled`, `prevent_click`, `close_icon` (`ViewFn`), `close_aria_label`, `style`, `style_class`, `pt`; the parts — `display: ViewFn`, `content: Callback<Callback<()>, AnyView>` (receives the close function, like `closeCallback`); the callbacks `on_activate`, `on_deactivate` (`Callback<()>`, without an event).
- Behaviour: a click on the view and `Enter` activate (the click does not fire with `prevent_click`, `Enter` always fires, as in PrimeNG); closing by the close button and by the `close` callback; with `disabled` activation and closing are not performed.
- The showcase page `/inplace` with the demos `basic`, `input`, `image`, `lazy`; e2e; benchmarks and budgets; `docs/inplace.md`.
- Styles: `inplace` is added to `theme.components`.
- **Not included:** `Ripple`, `closeIcon` as the name of a font class (`ViewFn` is used instead), the selectors `pInplaceDisplay`/`pInplaceContent` (the `display` and `content` properties are used instead), `Table` in the `lazy` demo (a plain `<table>` is used), `pAutoFocus` in the `input` demo (the field gets focus manually).

## Capabilities

### New Capabilities
- `inplace`: an area that switches between the view and the content on click.

### Modified Capabilities
<!-- None. -->

## Impact

- A new crate `crates/prime-inplace` (`publish = false`); dependencies: `prime-core`, `prime-button`, `prime-icons` (`times`), `web-sys`.
- Changes to `Cargo.toml` of the workspace, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (the browser job, three places, if `tests/web.rs` appears).
