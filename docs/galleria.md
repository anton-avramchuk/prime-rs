# Galleria

A port of `p-galleria` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-galleria::Galleria`, `GalleriaPt`, `GalleriaImage`, `GalleriaPosition`, `GalleriaResponsive` | `packages/primeng/src/galleria/galleria.ts` (`Galleria`, `GalleriaContent`, `GalleriaItem`, `GalleriaThumbnails`), `galleria/style/galleriastyle.ts` |
| Thumbnail window, neighboring indices, responsive visible count, classes | `prime-galleria::model`: a pure port of `onAfterContentChecked`, `getMedianItemIndex`, `calculatePosition`, etc., table-driven tests |
| CSS | `@primeuix/styles/galleria` (`galleria` in `theme.components`) |
| Demos | `apps/showcase/doc/galleria/*-doc.ts` (`basic`, `autoplay`, `caption`, `controlled`, `thumbnail`, `indicator`, `fullscreen`) |

Spec: `openspec/specs/galleria`. The active index is an `RwSignal<usize>`, visibility in fullscreen mode is an `RwSignal<bool>`.

```rust
let active = RwSignal::new(0);
view! { <Galleria value=images active_index=active num_visible=5 circular=true show_item_navigators=true /> }
```

## What matches PrimeNG

- Thumbnail strip offset formulas (`shifted_items`), median index, page count, button availability; `p-galleria-thumbnail-item-current/-active/-start/-end` classes.
- Navigation: frame buttons, indicators (click, `Enter`/`Space`, hover), thumbnails (click, `Enter`/`Space`, `ArrowLeft`/`ArrowRight`/`Home`/`End`, `Tab`), page buttons, `circular`.
- Auto-advance that stops on user action (`should_stop_autoplay_by_click`).
- Thumbnail and indicator positions, `show_indicators_on_item`, `show_item_navigators_on_hover`, vertical thumbnails.
- Fullscreen mode: the `p-galleria-mask` mask in `body`, close button, closing by clicking the mask, scroll lock, focus trap, leave animation.

## Differences

| Difference | Reason |
|---|---|
| Data is `Vec<GalleriaImage>` (`item_image_src`, `thumbnail_image_src`, `alt`, `title`), templates are `Callback<GalleriaImage, AnyView>`; without a template the frame and thumbnail are rendered as `<img>` | Leptos has no `ng-template`; a typed model instead of `any`. |
| The strip offset is a function of `(active index, visible count)`; there is no separate `step` | In PrimeNG `step()` is always overridden by the recomputation on index change. |
| Thumbnails get `flex: 1 0 (100/n)%` as an inline style rather than a dynamic `<style>` with media queries | Responsiveness is computed in Rust from the window width (`resize`). |
| No system fullscreen (`requestFullscreen`, `fullscreenchange`), `Bind`, Angular forms, animation options | Out of the port scope. |
| No thumbnail swipe gestures | Touch was not verified on a device. |
| No `Escape` in fullscreen mode | Same in PrimeNG. |

## Not verified

- Touch and swipe on a real device.
- Screen reading: `aria-*` were checked against PrimeNG, screen reader behavior was not.
- Right-to-left layout.

## Performance

See `benches/` (`galleria-1`, `galleria-1000`, `case-galleria`) and `benches/*budgets.toml`.
