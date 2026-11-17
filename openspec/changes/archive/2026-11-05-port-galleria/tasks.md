# Tasks

## 1. Foundation

- [x] 1.1 `galleria` in `theme.components`, `cargo xtask gen-theme`; `p-galleria-thumbnail-item` and `p-items-hidden` are present
- [x] 1.2 Cross-check against the source's `GalleriaThumbnails` (`onAfterContentChecked`, `navForward/Backward`, `onItemClick`, the keys), `GalleriaItem`, `GalleriaContent` (auto-change)

## 2. Crate `prime-galleria`

- [x] 2.1 The model `model.rs` with table tests
- [x] 2.2 The `Galleria`, `GalleriaPt`, `GalleriaImage` component: markup, indicators, caption, thumbnails, SSR tests, rustdoc
- [x] 2.3 Navigation: frame buttons, indicators, thumbnails, keys, `circular`, touch
- [x] 2.4 Auto-change, adaptive `num_visible`
- [x] 2.5 Full-screen mode: `Portal`, mask, closing, scrolling, focus, exit
- [x] 2.6 Browser `tests/web.rs` (one sequential test)

## 3. Showcase and e2e

- [x] 3.1 The `/galleria` page, the menu, the route, `stylesheet(&[..])`, the crate in the workspace and the showcase
- [x] 3.2 `e2e/tests/pages.json` and `galleria.spec.ts`

## 4. Benchmarks and budgets

- [x] 4.1 Scenarios `-1` and `-1000`, `case-galleria`, budgets

## 5. Integration

- [x] 5.1 `docs/galleria.md`; `prime-galleria` in the browser job of `ci.yml` (three places)
- [x] 5.2 fmt, clippy, tests, deny, machete, `openspec validate port-galleria`

## Workflow follow-up

- Wave 6 — CI once at the end of the wave; commit and archive locally.
