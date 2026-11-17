# Proposal

## Why

ScrollPanel is a cross-browser themable replacement for the native scrollbar. It is the first component that reads DOM geometry (`scrollWidth`, `clientHeight`, `scrollTop`…), moves bars with `requestAnimationFrame`, drags them with the mouse via document listeners and is controlled from the keyboard. It rehearses the client "measure → write style" layer without SSR dependencies, needed later by Listbox, Table and virtual scroll.

## What Changes

- New crate `prime-scrollpanel` with the `ScrollPanel` component: `step` (keyboard step, default 5), `style`, `style_class`, the content slot `content_template` (replacing the `#content` template), `children`, `pt` (parts `root`, `content_container`, `content`, `bar_x`, `bar_y`).
- Markup and classes as in PrimeNG 21.1.10 (`scrollpanelstyle.ts`): root `div.p-scrollpanel.p-component`, `p-scrollpanel-content-container`, `p-scrollpanel-content`, two bars `p-scrollpanel-bar p-scrollpanel-bar-x|bar-y` (`role="scrollbar"`, `tabindex="0"`, `aria-orientation`, `aria-valuenow`, `aria-controls`, `data-pc-group-section="bar"`).
- Client behavior: bar recalculation (`moveBar`) on `scroll`, `mouseenter` on the content and window `resize`; the `p-scrollpanel-hidden` class/`data-p-scrollpanel-hidden` attribute when scrolling on an axis is not needed; container height calculation with `max-height` (`calculateContainerHeight`); bar dragging with the mouse with `p-scrollpanel-grabbed` on the bar and `body`; arrows when the bar is focused (repeat after 40 ms until `keyup`).
- Public methods `refresh()` and `scroll_top(value)` via `ScrollPanelRef` (a handle passed through the `panel_ref` property).
- Page `/scrollpanel` (`basic`, `custom`), e2e, benchmarks, budgets, `docs/scrollpanel.md`, the `scrollpanel` theme CSS.
- **Out of scope:** `BlockableUI`, template synonyms, `appendTo`; the `scrollTop` method as a public method of the Angular instance is replaced by a handle.

## Capabilities

### New Capabilities
- `scrollpanel`: the ScrollPanel component: structure and classes, bar ARIA, bar calculation and hiding, dragging, keyboard, `refresh`/`scroll_top`, properties and `pt`, performance.

### Modified Capabilities
<!-- None. -->

## Impact

- New crate `crates/prime-scrollpanel` (`publish = false`); dependencies `prime-core`, `web-sys` (elements, `MouseEvent`, `KeyboardEvent`).
- Changed: workspace `Cargo.toml`, `upstream.toml` (`scrollpanel` in `theme.components`), `showcase`, `benches/*`, `e2e/tests`, the browser CI job.
