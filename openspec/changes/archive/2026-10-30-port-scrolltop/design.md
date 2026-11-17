# Design

## Context

Requirements are in `specs/scrolltop`. PrimeNG 21.1.10 sources: `scrolltop/scrolltop.ts`, `scrolltop/style/scrolltopstyle.ts`; demos `apps/showcase/doc/scrolltop/{basic,element}-doc.ts`. Models in the project: `prime-button` (button), `prime-slider` (a `window` subscription from a handler, the handle in `StoredValue<_, LocalStorage>`, removal in `on_cleanup`), `prime-select` (`Overlay` group z-index, an `is_disposed()` check in timer closures), `prime_core::zindex`.

Observations on the source:
- The root is `p-button` itself with `styleClass = cn(cx('root'), styleClass)`; `buttonProps` default to `{ rounded: true }`. Icon: a custom class (`icon`), otherwise `ChevronUpIcon` with the `p-scrolltop-icon` class.
- `checkVisibility(scrollY)`: `scrollY > threshold` → `visible = true` and `render = true`; otherwise `visible = false`. The element is removed (`render = false`) in `onAfterLeave` of the `pMotion` animation (name `p-scrolltop`).
- `onBeforeEnter`: `position = sticky` for `parent`, otherwise `fixed`; `ZIndexUtils.set('overlay', …)`; `onBeforeLeave` releases the z-index.
- `onClick`: the target is `defaultView` or the host's parent; `scroll({ top: 0, behavior })`.
- The initial visibility is not computed: until the first `scroll` event arrives there is no button, even if the page is already scrolled. We do the initial check (in the spec).

## Decisions

### D1. Crate and dependencies
`prime-scrolltop` with `ScrollTop`, `ScrollTopTarget { Window, Parent }`, `ScrollTopBehavior { Smooth, Auto }`, `ScrollTopPt`. Depends on `prime-core`, `prime-button`, `prime-icons` (feature `chevronup`) and `web-sys`. The button is a `Button` with `rounded` (default `true`), `severity` and `raised` from props; root classes are added via `style_class`.

### D2. Pure logic (`state.rs`)
- `is_visible(scroll: f64, threshold: f64) -> bool` — strict `>`.
- `root_classes(target) -> &'static str` — `p-scrolltop` or `p-scrolltop p-scrolltop-sticky`.
- `enter_classes(started)` / `leave_classes(started)` — animation classes.
Table tests.

### D3. Visibility and lifecycle
State `phase: RwSignal<Phase>`, `Phase { Hidden, Shown, Leaving }`. The `scroll` subscription is set only under `cfg(target_arch = "wasm32")`.

Parent: `Parent` needs the component's own `parent_element()`, and while the button is hidden the component has no element. So for `Parent` the button is always in the DOM, and in the `Hidden` phase is hidden with `display: none` (for `Window` the button is not rendered at all in the `Hidden` phase). After mounting the parent is taken via the button's `NodeRef`; the subscription is set on it, and the initial check runs immediately. For `Window` the subscription and the check are done in an `Effect` after mounting, the listener is `window_event_listener`, the handle in `StoredValue<_, LocalStorage>`, removal in `on_cleanup`. This difference is recorded in `docs/scrolltop.md`.

### D4. Animation
The classes `p-scrolltop-enter-from|active|to` and `leave-*` come from the theme CSS: on enter `enter-from` + `enter-active` are set, on the next frame `enter-from` is replaced with `enter-to`; on completion (a 300 ms timer per the theme's `transition: opacity 300ms`, with an `is_disposed()` check and a generation counter against stale timers) the classes are removed. For exit the same with `leave-*`, then the `Hidden` phase and z-index release. Timers and listeners are removed in `on_cleanup`.

### D5. Position and z-index
The position is set by the theme CSS (`position: fixed !important` for `.p-scrolltop`, `sticky !important` for `.p-scrolltop-sticky`), only `z-index: N` stays inline, where `N = zindex::generate(Overlay, config.z_index.base(Overlay))`; on exit — `zindex::revert`. On SSR the `Window` button is not rendered, so no z-index is issued.

### D6. Action
The click handler calls `Window::scroll_with_scroll_to_options` or the parent's `Element::scroll_with_scroll_to_options` with `top = 0` and `behavior`.

### D7. CSS
`scrolltop` is added to `theme.components`; `cargo xtask gen-theme`, check that the CSS of the others did not change. If the CSS turns out to have a `/* For PrimeNG */` block, it is picked up via `primeng_extra`.

### D8. Tests
SSR: hidden by default, classes by `target`. Browser `tests/web.rs`: threshold (400/401), return, initial check, `Parent`, click, position, z-index and its return. e2e: real scrolling of the window and a nested container, click, `Enter`/`Space` keys. The crate is added in three places of `ci.yml`.

### D9. Benchmarks
Scenarios `scrolltop-1` and `-1000` (in SSR the `Window` variant is hidden, so an "empty" instance is measured; for honesty a second scenario is not needed), `case-scrolltop` for wasm; first `cargo bench -p render-bench --bench alloc`.

## Risks / Trade-offs

- [`Parent` always in the DOM, `Window` only when shown] → the only way to find the parent before the first show; recorded in docs.
- [No `@primeuix/motion`] → animation by classes and a timer by the transition duration from the theme (300 ms).
- [Touch and inertial scrolling on mobile] → not verified, honestly recorded in docs.

## Migration Plan

No migration. Rollback — `git revert`.

## Open Questions

None.
