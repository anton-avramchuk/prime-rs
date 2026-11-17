# Proposal

## Why

Carousel is the ninth component of wave 4: a scrollable row of items with "previous/next" buttons, dot indicators, autoplay, infinite mode and adaptation to window width. It is the most "arithmetic" component of the wave: the main value of the port is an exact copy of the page, offset and clone logic, so it is extracted into a pure model with table tests.

## What Changes

- New crate `prime-carousel` (port of PrimeNG 21.1.10 `p-carousel`).
- `Carousel<T>`: root `div.p-carousel.p-component.p-carousel-horizontal|vertical[role=region]` (`id`) > optional `div.p-carousel-header` > `div.p-carousel-content-container` (+ `content_class`) > `div.p-carousel-content[aria-live]` with the `p-carousel-prev-button` / `p-carousel-next-button` buttons (text, round, `p-disabled`, `aria-label` from translations) and the viewport `div.p-carousel-viewport` with `div.p-carousel-item-list` (`transform: translate3d(...)`, `transition`, the `p-items-hidden` class after the transition) > `div.p-carousel-item[role=group]` items (`p-carousel-item-active|start|end`, `aria-hidden`, `aria-label`, `aria-roledescription`, `data-p-carousel-item-*`) and `p-carousel-item-clone` clones (with `circular`) > `ul.p-carousel-indicator-list` with `li.p-carousel-indicator` (`-active`) and `p-carousel-indicator-button` buttons (`aria-label`, `aria-current`, `tabindex`) > optional `div.p-carousel-footer`.
- Page model (pure logic, as in PrimeNG): `num_visible`, `num_scroll`, dot count `⌈(n − visible)/scroll⌉ + 1`, remainder `remaining_items`, offset `total_shifted_items`, forward/backward/to-page transitions, infinite mode with clones (`circular` applies while there are at least `num_visible` items), going out of bounds when the page count shrinks.
- Responsiveness: `responsive_options` (`breakpoint`, `num_visible`, `num_scroll`) are recomputed on window `resize`; item width is set by the inline style `flex: 1 0 <100/visible>%`.
- Behavior: buttons and dots switch pages (`on_page`), `page` is a two-way signal; autoplay by `autoplay_interval` until the first manual action; swipe (20 px threshold) via touch events; the ←/→ keys on indicators move focus between dots (roving `tabindex`).
- Props: `value`, `page`, `num_visible`, `num_scroll`, `responsive_options`, `orientation`, `vertical_view_port_height` (`300px`), `content_class`, `indicators_content_class`, `indicators_content_style`, `indicator_style_class`, `indicator_style`, `circular`, `show_indicators`, `show_navigators`, `autoplay_interval`, `id`, `style`, `style_class`, `prev_button_severity`/`next_button_severity`, `pt`; views `item` (required), `header`, `footer`, `previous_icon`, `next_icon`; event `on_page`.
- Showcase page `/carousel` with demos `basic`, `responsive`, `circular` (autoplay), `vertical` and product cards (local data, no CDN); e2e; benchmarks and budgets; `docs/carousel.md`.
- Styles: `carousel` is added to `theme.components`.
- **Out of scope:** `Header`/`Footer` as facets (`p-header`/`p-footer`) — only the `header`/`footer` views, `Bind`, `pTemplate`, `unstyled`, full `ButtonProps` (severity is kept), dynamic `<style>` with media queries (responsiveness is recomputed on `resize`).

## Capabilities

### New Capabilities
- `carousel`: a scrollable row of items with pages, indicators, autoplay and infinite mode.

### Modified Capabilities
<!-- None. -->

## Impact

- New crate `crates/prime-carousel` (`publish = false`); dependencies: `prime-core`, `prime-button`, `prime-icons` (chevronleft/right/up/down), `web-sys`.
- Changed: workspace `Cargo.toml`, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (browser job, three places: `prime-carousel`).
