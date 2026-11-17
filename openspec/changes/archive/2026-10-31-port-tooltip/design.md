# Design

## Context

Requirements are in `specs/tooltip`. PrimeNG 21.1.10 sources: `tooltip/tooltip.ts`, `tooltip/style/tooltipstyle.ts`; demos `apps/showcase/doc/tooltip/*-doc.ts`. Project references: `prime-popover` (positioning, window listeners, `scroll` capture, cleanup closures), `prime-core::zindex` (`ZGroup::Tooltip` already exists), `prime_dom::dom::{viewport, scroll}`.

Observations about the source:
- It is a **directive** (`[pTooltip]`), not a component: no host markup, everything works on the target `ElementRef`; the `mouseenter`/`mouseleave`/`click`/`touchstart`/`touchend` (hover) and `focus`/`blur` (focus) listeners are attached to the target; for `focus` a `.p-component` inside is looked up, and for `p-inputwrapper` the nested `input`.
- The tooltip is created imperatively on show (`create()`): `div.p-tooltip.p-component[role=tooltip]` > `div.p-tooltip-arrow` + `div.p-tooltip-text`; it is placed in `body` (`appendTo` defaults to `body`); removed on hide.
- The position is computed from the target `getBoundingClientRect` + window scroll; the primary side and fallbacks by priority, `isOutOfBounds` by window bounds; the arrow is placed by edge and center (for `top`/`bottom` the tooltip midpoint is written into the arrow `left`).
- Options are assembled into `_tooltipOptions` from separate inputs and `tooltipOptions`; when the text changes while shown, `updateText()` + `align()` or `hide()` is called.
- Hiding on Escape is a document `keydown.escape` listener while shown; on window `resize` and on ancestor scroll (`ConnectedOverlayScrollHandler`).
- Appearance is the JS function `fadeIn(container, 250)`.
- Leptos supports `use:` directives (`tachys::html::directive`): the function `handler(el: web_sys::Element, param: P)` is called on element build and hydration (not called in SSR) and on every attribute `rebuild`; the directive has no cleanup hook of its own, but the handler runs in the element reactive owner, so `on_cleanup` inside it fires on unmount.

## Goals / Non-Goals

**Goals:** the same look, positioning and events as `pTooltip`; attaching without wrappers or markup changes; SSR safety.

**Non-Goals:** `appendTo`, `pt`, `Bind` (see proposal).

## Decisions

### D1. API shape: the `use:tooltip` directive
`pub fn tooltip(el: web_sys::Element, options: TooltipOptions)`; usage `<input use:tooltip=TooltipOptions::text("…") />`. Alternatives rejected: a wrapper component (changes the DOM and breaks `:hover` styles of nested selectors, an extra element around every field), a `use_tooltip(NodeRef)` hook (needs a `NodeRef` on every element). Verified by a spike (wasm test): `use:` is called once on a plain element and on the root of `Button` and `InputText` without any changes to those components; the fallback (`NodeRef` + `Effect`) is not needed.

### D2. Per-element state
The directive `rebuild` is called repeatedly, so the state must not be duplicated: a `thread_local` table `id → Rc<State>`; the `id` is written into a JS property of the element (`__primeTooltip`, no DOM attribute). A repeated call updates the options of the existing state (`RwSignal<TooltipOptions>`-like), the first one creates the state and subscribes the listeners. `on_cleanup` (from the element owner) removes the listeners, removes the tooltip and the table entry.

### D3. Pure logic (`state.rs`)
- `TooltipPosition`, `TooltipEvent`, `priority(position) -> [TooltipPosition; 4]`.
- `place(position, host: Rect, tip: Size) -> (left, top)`: offsets of the primary side relative to the element without `position_top/left`.
- `arrow_style(position, tip_width) -> ArrowStyle` (`top`/`right`/`bottom`/`left`).
- `out_of_bounds(left, top, width, height, viewport_w, viewport_h) -> bool` under the same conditions as `isOutOfBounds`.
- `root_classes(position, style_class) -> String`.
- `activation_delay(show_delay, life) -> (Option<u32>, Option<u32>)` (when to show and when to hide by `life`).
- `has_ellipsis(offset_w, scroll_w, offset_h, scroll_h) -> bool`.
- `leave_keeps_open(auto_hide, related_target_is_tooltip) -> bool`.
Table-driven tests for each.

### D4. Tooltip lifecycle
`activate()`: checks (`show_on_ellipsis`, `disabled`, empty content) → show timer (`set_timeout`) → `show()` creates the DOM and positions it (`display: none` while building, then `inline-block`; opacity 0 → 1 with a 250 ms transition via `style.transition`); `deactivate()`: cancel the show, `hide_delay`, `hide()`. Window listeners (`keydown` Escape, `resize`, `scroll` capture) are attached while shown and removed in `hide()`. `scroll` hides the tooltip only if a nested ancestor of the element was scrolled: scrolling the page (document) itself does not hide it, because the tooltip is in page coordinates and moves with the page (otherwise the first show after scrolling to the element would be hidden immediately). The container, timers and z-index are released in one place (`remove()`).

### D5. Content
`Text(Prop<String>)` is written to `text_content` (escaping), `Html(String)` to `inner_html`, `View(ViewFn)` is mounted inside `p-tooltip-text` via `leptos::mount::mount_to`, the `UnmountHandle` is kept until hide. Reactive `Text` and `disabled` are tracked by an `Effect`: a change while shown → `update_text()` + `align()` or `hide()`.

### D6. Touch
`touchstart` shows, `touchend` hides with `auto_hide`; without `auto_hide` a `touchstart` listener is attached to the document that hides the tooltip on a touch outside the target and outside the tooltip. Not verified on real input (as with `Slider`/`Knob`); recorded in docs.

### D7. CSS
`tooltip` is added to `theme.components`; `tooltipstyle.ts` has no `For PrimeNG` block. Check that the CSS has the arrow and positioning (`.p-tooltip-arrow` via `border`), and that `fadeIn` is replaced by a transition (the theme has nothing for this).

### D8. Tests
SSR: the directive renders nothing and does not change the element markup. Browser `tests/web.rs` (one sequential run, since window listeners are shared): show/hide on hover, focus, both; click hides; delays (`show_delay` is canceled by leaving, `hide_delay`, `life`); `auto_hide=false` and hovering the tooltip; 4 sides and fallback when there is not enough room; offsets; Escape, resize, scroll; HTML/text/View; reactive text change and `disabled`; `show_on_ellipsis`; `z-index` above `Dialog`; cleanup on unmount; a repeated `use:` does not duplicate listeners. e2e: a real mouse and keyboard on the `/tooltip` page. The crate is added in three places of `ci.yml`.

### D9. Benchmarks
Scenarios `tooltip-1`/`-1000` (an element with the directive in SSR: the directive is not called, the cost of building `TooltipOptions` is measured) and `case-tooltip` (element + directive in wasm); first `cargo bench -p render-bench --bench alloc`, for wasm a temporary large budget (replace by a unique line).

## Risks / Trade-offs

- [`use:` on a component root] verified on `Button` and `InputText`; for other project components it is checked in the showcase.
- [directive `rebuild`] a repeated call must not duplicate listeners → D2 and a separate test.
- [No cleanup hook on the directive] we rely on the owner `on_cleanup`; verified by an unmount test.
- [Touch not verified on real input] recorded in docs.
- [Opacity transition instead of `fadeIn`] the visual difference is negligible; recorded in docs.

## Open Questions

- Closed: `use:` is accepted on the root of `Button` and `InputText` (spike in the first task).

## Migration Plan

No migration. Rollback is `git revert`.
