# Design

## Context

Requirements are in `specs/scrollpanel`. PrimeNG 21.1.10 sources: `packages/primeng/src/scrollpanel/scrollpanel.ts`, `scrollpanel/style/scrollpanelstyle.ts`, demos `apps/showcase/doc/scrollpanel` (`basic`, `custom`). The `scrollpanel` CSS will be added via `cargo xtask gen-theme` (`upstream.toml`); the `/* For PrimeNG */` block is not needed, but `.p-scrollpanel { display: block }` from `scrollpanelstyle.ts` should be checked: if it is missing from the generated CSS, add the component to `primeng_extra`. The crate template is `prime-toolbar` (two branches, `macro_rules!`, `Props`), the client part follows `prime-rs-core::collapsible` (code under `cfg(target_arch = "wasm32")`, empty SSR path).

## Decisions

### D1. Markup and branches
The static branch is used when `style` and `style_class` are static; otherwise the reactive one. The markup is a `macro_rules!`, the root attribute expressions vary. The content `id` is `pn_id_{n}_content` from a `CollapseIds`-like counter of `prime-rs-core` (the same `next_unique`, deterministic in SSR/hydration); if needed, expose a small public function `unique_id(suffix)` in `collapsible` rather than duplicating the counter.

### D2. State lives in handlers, not signals
`lastScrollLeft/Top`, `orientation`, `scrollXRatio/YRatio`, `lastPageX/Y`, drag flags are mutable instance state. Stored in `Rc<RefCell<State>>`/`StoredValue`, not in signals: recalculation must not trigger a Leptos re-render. The exception is `aria-valuenow`: PrimeNG updates it via change detection; we write the attribute directly to the DOM (`set_attribute`) from the `scroll` handler, so reactivity is not needed.

### D3. Bar styles: direct DOM writes
`moveBar` measures the content and writes `style.cssText` and `data-p-scrollpanel-hidden`/the `p-scrollpanel-hidden` class on the bar elements via `web-sys`, inside `requestAnimationFrame` (as in the original). The bars' `style` attribute is not handed to Leptos (otherwise a reactive rewrite would wipe the calculation). For `pt.bar_*.style` Leptos does not write `style` on these elements; such a `pt` style is applied to the bar once on mount via `cssText` before the first `moveBar` (and is overwritten by it, a difference recorded in docs).

### D4. Listeners and cleanup
A client `Effect`/`on_mount` after mounting: `scroll` and `mouseenter` on the content, `resize` on the window, `mousedown` on both bars (via Leptos `on:` for the bars and the content; `resize` and document ones manually via `Closure`). Document `mousemove`/`mouseup` are attached on `mousedown` and removed on `mouseup` and in `on_cleanup`. Closures are kept in the state for removal. `window.request_animation_frame` is used; if unavailable, `setTimeout(0)` (a fallback like `timeoutFrame`).

### D5. Keyboard
`keydown`/`keyup`/`focus`/`blur` on the bars use Leptos `on:`. The 40 ms timer is `set_timeout`, the previous one is cleared. `event.code` is read as in the original. Difference: `orientation` is kept in `State`.

### D6. `calculateContainerHeight`
Runs once after mounting, only in the browser; PrimeUI's `getHeight` is replaced by `offsetHeight` minus paddings and borders (`getComputedStyle`). The result is written to the root `style.height`; since Leptos may manage the root `style`, the write happens only if `style`/`pt.root` did not set `height`, otherwise Leptos would overwrite it on a reactive update; the case of a reactive `style` together with `max-height` is documented as a limitation.

### D7. Public methods
`ScrollPanelRef` is a cloneable handle (`StoredValue<Option<…>>` with the content `NodeRef` and a `refresh` closure). `refresh()` calls `moveBar`; `scroll_top` clamps the value. Outside wasm it is a no-op.

### D8. Content slot
`content_template: Option<ViewFn>`; when present it is rendered instead of `children`. For `ng-content` + `ngTemplateOutlet` the original renders both; ours are mutually exclusive (like the `@if` in the original template).

### D9. Tests
SSR tests: structure, ARIA, `pt`, branch parity. Browser tests (`tests/web.rs`, `wasm-bindgen-test`): bar hiding, size/position formulas, dragging (synthetic `MouseEvent`), arrows, `scroll_top` with clamping, listener removal. Pure arithmetic (`bar_size`, `bar_offset`) is extracted into ordinary functions and checked by native unit tests.

## Risks / Trade-offs

- [Writing bar `style` outside Leptos] → bars without a reactive `style`; the `pt.style` scenario is described in docs.
- [Tests depend on layout geometry] → the browser test sets fixed sizes inline and checks relative values.
- [Document `mousemove` and closure leaks] → removal in `mouseup` and `on_cleanup`; an unmount test.
- [Possible `display: block` from `scrollpanelstyle.ts`] → check after `gen-theme`.

## Migration Plan

No migration: a new crate. Rollback is `git revert`.

## Open Questions

None.
