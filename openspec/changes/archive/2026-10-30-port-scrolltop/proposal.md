# Proposal

## Why

ScrollTop is a "back to top" button that appears after scrolling to a given depth. It is the next component of wave 2 (after InputOtp); it relies on the already ported Button and on a `scroll` subscription modeled on Slider/Select.

## What Changes

- New crate `prime-scrolltop` with the `ScrollTop` component (port of PrimeNG 21.1.10 `p-scrolltop`): a round `Button` with the classes `p-scrolltop` (+ `p-scrolltop-sticky` when `target = Parent`) and the `p-scrolltop-icon` icon (`ChevronUpIcon` by default).
- Props: `target` (`Window` by default | `Parent`), `threshold` (400), `behavior` (`Smooth` by default | `Auto`), `icon`, `aria_label`, `severity`, `raised`, `rounded` (`true` by default), `style`, `style_class`, `pt`.
- Visibility: a subscription to the `scroll` of the window or the parent element; the button is shown when the threshold is exceeded and hidden on return. Appearing and disappearing are animated with the theme's `p-scrolltop-enter-*`/`p-scrolltop-leave-*` classes, z-index — the `Overlay` group; position `fixed` for the window and `sticky` for the parent.
- A click scrolls the target to the start (`scroll({ top: 0, behavior })`).
- Showcase page `/scrolltop` with the `basic` and `element` (`target = Parent`) demos; e2e with real scrolling; benchmarks and budgets; `docs/scrolltop.md`.
- Styles: `scrolltop` is added to `theme.components`.
- **Out of scope:** full `buttonProps` (only `severity`, `raised`, `rounded` are available); `iconTemplate` (`icon: ViewFn` instead); `showTransitionOptions`/`hideTransitionOptions`/`motionOptions` (deprecated or tied to `@primeuix/motion`).

## Capabilities

### New Capabilities
- `scrolltop`: a button returning to the start of a scrollable area, visible after a scroll threshold.

### Modified Capabilities
<!-- None. -->

## Impact

- New crate `crates/prime-scrolltop` (`publish = false`); dependencies: `prime-core`, `prime-button`, `prime-icons` (`chevronup`), `web-sys` (`Window`, `Element`, `HtmlElement`, `ScrollToOptions`, `ScrollBehavior`).
- Changed: workspace `Cargo.toml`, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (browser job, three places).
