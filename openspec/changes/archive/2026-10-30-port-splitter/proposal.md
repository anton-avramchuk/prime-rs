# Proposal

## Why

Splitter - the second component of wave 3: a set of panels whose sizes change by dragging gutters or with arrows. It uses no overlays, relies on `window` subscriptions (following Slider) and introduces state persistence in `localStorage`/`sessionStorage` into the project.

## What Changes

- New crate `prime-splitter` with the `Splitter` component (port of `p-splitter` from PrimeNG 21.1.10): root `div.p-splitter.p-component.p-splitter-<layout>` (`display: flex`, a column with `vertical`), panels `div.p-splitterpanel`, between panels a gutter `div.p-splitter-gutter[role=separator]` with a handle `div.p-splitter-gutter-handle[tabindex=0][aria-orientation][aria-valuenow]`.
- Properties: `panels: Vec<ViewFn>` (panel content in order, instead of `ng-template #panel`), `layout` (`Horizontal` by default | `Vertical`), `panel_sizes: Vec<f64>` (percent; missing and zero ones are split equally), `min_sizes: Vec<f64>`, `gutter_size` (4), `step` (5, pixels), `state_key` and `state_storage` (`Session` by default | `Local`), `nested`, `style`, `style_class`, `panel_style`, `panel_style_class`, `pt`; `on_resize_start`, `on_resize_end` with the event `SplitterResizeEvent { sizes }`.
- Panel size: `flex-basis: calc(<share>% - <(n-1)*gutter_size>px)`. Mouse (and touch) dragging and arrows on the handle (along the `layout` axis) change two adjacent panels; `min_sizes` limit the change; state is saved by `state_key` and restored on mount.
- Showcase page `/splitter` with demos `horizontal`, `vertical`, `size`, `nested`; e2e with real dragging; benchmarks and budgets; `docs/splitter.md`.
- Styles: `splitter` is added to `theme.components`.
- **Out of scope:** templates via `ng-template` and `ContentChildren` (`panels` instead), automatic nesting detection (`nested` instead), `originalEvent` in events, `Bind`/`pt` with context, key repeat via a timer (`setTimer` 40 ms): the step is applied on every `keydown` (including key auto-repeat), the finish on `keyup`.

## Capabilities

### New Capabilities
- `splitter`: panels with resizable sizes and gutters.

### Modified Capabilities
<!-- None. -->

## Impact

- New crate `crates/prime-splitter` (`publish = false`); dependencies: `prime-rs-core`, `web-sys` (`Window`, `Storage`, `MouseEvent`, `TouchEvent`, `KeyboardEvent`, `Element`, `DomRect`).
- Changed: workspace `Cargo.toml`, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (browser job, three places).
