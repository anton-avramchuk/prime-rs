# Design

## Context

Requirements are in `specs/carousel`. PrimeNG 21.1.10 sources: `carousel/carousel.ts` (about 1000 lines, about 550 of them logic), `carousel/style/carouselstyle.ts`; styles `@primeuix/styles/carousel`; demos `apps/showcase/doc/carousel/{basic,responsive,circular,vertical}-doc.ts`. Models in the project: `prime-timeline` (generic `Vec<T>` and slot template), `prime-paginator` (pages and boundaries), `prime-splitbutton`/`prime-speeddial` (buttons), `prime-dock` (`matchMedia`/window).

Observations on the source (exact arithmetic):
- State: `_page`, `_numVisible`, `_numScroll`, `totalShiftedItems`, `remainingItems = (len − numVisible) % numScroll`, `isRemainingItemsAdded`. Dots `totalDots = ceil((len − numVisible)/numScroll) + 1` (0 for an empty list).
- Recompute (`onAfterContentChecked` when `numScroll`/`numVisible`/value length changes): the remainder; if the page went past the last one — it is clamped; `totalShifted = −page·numScroll` (and `−numVisible` in infinite mode); on the last page with a remainder the offset is adjusted by `−remaining + numScroll`.
- `step(dir, page?)`: with an explicit page `totalShifted = −numScroll·page (− numVisible)`; otherwise a shift by `numScroll·dir` (accounting for the already added remainder), page from the offset. Special cases: infinite mode on the last page forward → offset `−(len + numVisible)` and page 0 (transition to a clone); on zero backward → offset 0 and the last page; last page with a remainder → adjustment by the remainder. Then `transform`/`transition: transform 500ms ease 0s`, the `p-items-hidden` class is removed.
- `onTransitionEnd`: `transition` is reset, `p-items-hidden` is added; in infinite mode on the edge pages `transform` is recomputed from the logical offset without animation (a jump from a clone to the real item). After each step (`afterContentChecked`) the logical offset is settled: page 0 → `−numVisible`; offset 0 → `−len`.
- Autoplay: `setInterval(autoplayInterval)`: to the next page, from the last — to zero; any manual action stops it (`allowAutoplay = false`); `aria-live` = `polite` while autoplay is allowed.
- Responsiveness: `calculatePosition` picks the last option with `breakpoint ≥ window width`, recomputes the page (`floor(page·oldScroll/newScroll)`), the offset and `numVisible`, rebuilds clones; a `resize` listener. Item width is set by a generated `<style>` (`flex: 1 0 100/visible %`) with media queries.
- Indicators: `tabindex` 0 on the active page, ←/→ move focus between dots.
- Swipe: `touchstart` remembers the point, `touchend` with `|diff| > 20` pages; `touchmove` suppresses page scrolling.

## Goals / Non-Goals

**Goals:** identical classes, roles, `aria-*`, page and clone arithmetic, autoplay, responsiveness, swipe.

**Non-Goals:** `p-header`/`p-footer` facets, `Bind`, `pTemplate`, `unstyled`, `ButtonProps`, dynamic `<style>`.

## Decisions

### D1. Pure model (`model.rs`)
`Track { page, num_visible, num_scroll, total_shifted, remaining, remaining_added }` with pure methods: `total_dots(len)`, `is_circular(circular, len)`, `recalc(len, circular) -> Track` (the `afterContentChecked` block), `step(len, circular, dir, page) -> Step { track, applied_shift }` (`applied_shift` is the offset for `transform` right after the step, before settling), `settle(len, circular)` (settling after a step), `after_transition_shift(circular)`, `first_index`, `last_index`, `nav_disabled(forward/backward)`, `responsive(options, window_width, defaults) -> Matched`, `swipe(diff)`, `autoplay_next(page, dots)`, `translate(shift, num_visible, vertical) -> String`. Tests: tables for the regular, infinite, remainder and responsive modes (several lengths and steps), both edges, changing the item count.

### D2. Component state
`RwSignal<Track>`, `applied_shift: RwSignal<i32>`, `transitioning: RwSignal<bool>` (for `transition`), `items_hidden: RwSignal<bool>` (the `p-items-hidden` class; initially `false`). The page is exposed outward as `page: Option<RwSignal<usize>>`: an external change (different from the internal page) triggers `step` to a page (like the `page` setter in PrimeNG: autoplay stops; forward — `step(-1, val)`, backward — `step(1, val)`).

### D3. Generic component
`Carousel<T>` with `T: Clone + Send + Sync + 'static`, `value: Prop<Vec<T>>`, `item: Callback<T, AnyView>` (required). Items are built once per `value` change; positions, classes and `aria-*` are reactive attributes. Clones (`circular`) are copies of the edge `num_visible` items; they are rendered by the same `item`.

### D4. Responsiveness
`responsive_options: Vec<CarouselResponsive>`; the window width is read on mount and on `resize` (browser only); the `Track.responsive` result updates `num_visible`/`num_scroll` and rebuilds clones. Item width is inline `flex: 1 0 <100/num_visible>%` on each `div.p-carousel-item`. Server output uses the defaults, after hydration the page is recomputed — a known difference from PrimeNG (where media queries work already on the server).

### D5. Autoplay and swipe
`set_interval` (browser only) via `StoredValue<Option<IntervalHandle>>`, stopped on a manual action and on unmount. Swipe — `touchstart`/`touchend`/`touchmove` handlers on the viewport; the threshold logic is a pure function.

### D6. Indicators and focus
`tabindex` is reactive: `0` on the dot with `focus_dot` (the active page by default), the rest `-1`; ←/→ change `focus_dot` and focus the button; a page change returns `focus_dot` to it.

### D7. Navigation buttons
`Button` from `prime-button` (`text`, `rounded`, `severity = Secondary` by default, `on_click`), reactive `style_class` (`p-carousel-prev-button`, `p-disabled`); the icon is `previous_icon`/`next_icon` or chevrons by orientation (left/up, right/down). `aria-label` comes from the `aria.prevPageLabel`/`aria.nextPageLabel` translations.

### D8. Translations
`aria.slide`, `aria.slideNumber` (with `{slideNumber}`), `aria.pageLabel` (with `{page}`) already exist in `translation_defaults.rs`; substitution is a pure function.

### D9. Tests
Model: table tests (`model.rs`). SSR: markup, roles, `aria-*`, clones, indicators, absence of navigators/indicators by flags, `pt`, orientation, templates. Browser `tests/web.rs` (one sequential test because of timers and `resize`): buttons and dots, boundaries, `p-disabled`, `on_page`, external `page`, infinite mode (transition to a clone and a jump after `transitionend` — via a forced event), remainder, autoplay (short interval) and its stop, indicator keyboard, `responsive_options` (cannot change the container width — use a `breakpoint` larger than the window width), swipe (synthetic `touch` events), vertical orientation. e2e: theme styles, paging with buttons and dots, infinite mode, autoplay, responsiveness (window resize), vertical mode, swipe (touch context).

### D10. Demos and benchmarks
Products are a local list of 9 items (no network images, colored tiles). Bench `carousel-1`/`-1000` and `case-carousel`.

## Risks / Trade-offs

- [Arithmetic complexity] An error at page edges is visible only visually; we defend with table tests of the model and checking against the source for every branch.
- [Animation in tests] Without the theme CSS `transitionend` never arrives; in wasm tests we finish the transition manually and check transitions in e2e.
- [SSR and responsiveness] The server uses the defaults; after hydration the layout is recomputed.

## Open Questions

- Whether an `unstyled` mode is needed: not ported, like the other components.

## Migration Plan

No migration. Rollback — `git revert`.
