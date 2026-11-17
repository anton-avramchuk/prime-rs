# ScrollTop

Port of PrimeNG 21.1.10 `p-scrolltop` (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-scrolltop::ScrollTop`, `ScrollTopTarget`, `ScrollTopBehavior`, `ScrollTopPt` | `scrolltop/scrolltop.ts`, `scrolltop/style/scrolltopstyle.ts` |
| CSS | `@primeuix/styles/scrolltop` — `cargo xtask gen-theme` |
| Demos | `apps/showcase/doc/scrolltop/{basic,element}-doc.ts` |

Spec: `openspec/specs/scrolltop`.

## What matches PrimeNG

- The root is a `Button` with the classes `p-scrolltop` (and `p-scrolltop-sticky` for `Parent`), rounded by default; the icon is `ChevronUpIcon` with the `p-scrolltop-icon` class, `aria-label` from the locale.
- The button is shown when the scroll is strictly greater than the threshold (400 by default); `Window` — `position: fixed`, `Parent` — `sticky`.
- Showing and hiding are animated with the theme's `p-scrolltop-enter-*` / `leave-*` classes (300 ms); the z-index is taken from the `Overlay` group on show and returned on hide.
- A click scrolls the window or the parent to the start (`top = 0`, `Smooth` behavior by default).

## Differences

| Difference | Reason |
|---|---|
| The root is a `<button>` without a `<p-scrolltop>` host | The host in Angular is a framework detail. |
| Initial check on mount | In PrimeNG, until the first `scroll` arrives there is no button, even if the page is already scrolled. |
| With `Window` the button is not in the DOM while hidden; with `Parent` it is always in the DOM with `display: none` | The parent can only be found from the component's own element. |
| The animation uses classes and a 300 ms timer | No `@primeuix/motion` / `pMotion`. |
| No `buttonProps` and no `icon` as a class name | `severity`, `raised`, `rounded` are props; the icon is a `ViewFn`. |

## Cost

29 allocations per instance in SSR (in SSR the `Window` variant is hidden, so an "empty" instance is measured), contribution to wasm ≈ 542 KB (mostly the shared `Button`; the budget is 600 KB in `benches/budgets.toml`).

## Not verified

Touch and inertial scrolling on mobile devices, screen readers. The threshold, position, click and `Enter`/`Space` keys were verified in Chrome (e2e and browser tests).
