# Carousel

Port of PrimeNG 21.1.10 `p-carousel` (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-carousel::Carousel<T>`, `CarouselPt`, `CarouselOrientation`, `Responsive`, pure `Track` model | `packages/primeng/src/carousel/carousel.ts`, `carousel/style/carouselstyle.ts` |
| CSS | `@primeuix/styles/carousel` and the Aura preset — `cargo xtask gen-theme` (there is no `/* For PrimeNG */` block); buttons take the `button` styles |
| Demos | `apps/showcase/doc/carousel/*-doc.ts` (`basic`, `responsive`, `circular`, `vertical`) |

Spec: `openspec/specs/carousel`.

```rust
view! {
    <Carousel value=products num_visible=3 num_scroll=1 circular=true autoplay_interval=3000
        item=Callback::new(|p: Product| view! { <ProductCard product=p /> }.into_any()) />
}
```

## What matches PrimeNG

- Markup and classes: `div.p-carousel.p-component.p-carousel-horizontal|vertical[role=region]` > `p-carousel-header` (if present), `p-carousel-content-container` > `p-carousel-content[aria-live]` with the `p-carousel-prev-button`/`-next-button` buttons (`p-disabled`) and `p-carousel-viewport` > `p-carousel-item-list` (`p-items-hidden`) > `p-carousel-item[role=group]` (`p-carousel-item-active|start|end`, `data-p-carousel-item-*`, `aria-hidden`, `aria-label`, `aria-roledescription`) and `p-carousel-item-clone` clones; `ul.p-carousel-indicator-list` > `li.p-carousel-indicator` (`-active`) > `button.p-carousel-indicator-button` (`aria-current`, `tabindex`); `p-carousel-footer`.
- Page arithmetic is an exact copy of the source, extracted into the pure `Track` model and covered by tables: page count `⌈(n − visible)/step⌉ + 1`, remainder on the last page (the last page shows the last `num_visible` items), `translate3d` offset in percent per item, forward/backward/to-page transitions, infinite mode (clones, crossing the boundary via clones and a jump without animation after `transitionend`), clamping the page when the value changes.
- Autoplay (`autoplay_interval`): the next page on a timer, from the last — to zero; manual paging stops it for good (`aria-live` becomes `off`).
- Responsiveness: the last option with `breakpoint ≥ window width`, page recomputation by the PrimeNG formula, clone rebuild, `on_page` events; swipe with a 20 px threshold; the ←/→ keys on indicators move focus; vertical orientation with `vertical_view_port_height`.

## Differences

| Difference | Reason |
|---|---|
| Item width is set by the inline style `flex: 1 0 <100/visible>%` and recomputed on `resize`, rather than by a dynamic `<style>` with media queries | No `Renderer2`; on the server the defaults apply, after hydration the layout is recomputed (in PrimeNG media queries work even before hydration). |
| No `p-header`/`p-footer` facets: header and footer are the `header`/`footer` views | No Angular content projection. |
| Instead of `prevButtonProps`/`nextButtonProps` — `prev_button_severity`/`next_button_severity` (buttons are always text and round) | The only property the demos change. |
| `page` is a two-way `RwSignal<usize>` instead of a `page`/`onPage` pair; an external value outside the pages is discarded | A Leptos idiom; in PrimeNG such a value left the component in an inconsistent state. |
| No `Bind`, `pTemplate`, `unstyled`; icons are `ViewFn` | Not ported in this project. |
| `onTabKey`, `Home`/`End` for indicators are not wired | In PrimeNG they are not called from the key handler. |
| In the demos product photos are replaced with colored tiles, PrimeIcons icons with `prime-icons` icons | Network independence, no PrimeIcons. |
| `circular` applies only while there are at least as many items as visible ones | As in PrimeNG (`isCircular`); infinite mode is not enabled for short lists. |

## Not verified

- Right-to-left layout.
- Real touches on a device: swipe was checked with synthetic `touch` events.
- Screen reading: `aria-*` were checked against PrimeNG, screen reader behavior was not.

## Performance

See `benches/` (`carousel-1`, `carousel-1000`, `case-carousel`) and `benches/*budgets.toml`.
