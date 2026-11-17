# Splitter

Port of `p-splitter` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-splitter::Splitter`, `SplitterLayout`, `SplitterStorage`, `SplitterResizeEvent`, `SplitterPt` | `splitter/splitter.ts`, `splitter/style/splitterstyle.ts` |
| CSS | `@primeuix/styles/splitter` - `cargo xtask gen-theme` (the component has no `/* For PrimeNG */` block) |
| Demos | `apps/showcase/doc/splitter/{horizontal,vertical,size,nested}-doc.ts` |

Spec: `openspec/specs/splitter`.

## What matches PrimeNG

- Markup and classes: `div.p-splitter.p-component.p-splitter-<layout>` (inline `display: flex; flex-wrap: nowrap`, a column with `vertical`), `div.p-splitterpanel` (`tabindex="-1"`, `data-pc-section="panel"`), between panels `div.p-splitter-gutter[role=separator]` with `div.p-splitter-gutter-handle[tabindex=0][aria-orientation][aria-valuenow]`; while resizing - `p-splitter-resizing` on the root, `p-splitter-gutter-resizing` and `data-p-gutter-resizing="true"` on the gutter.
- Panel size is `flex-basis: calc(<share>% - <(n-1)·gutter_size>px)`; missing and zero `panel_sizes` are split equally.
- Mouse dragging is computed in percent of the root size (`pageX`/`pageY`), in `rtl` the horizontal offset is inverted; arrows move the boundary by `step` pixels; `keyup` ends the resize.
- State is saved to `localStorage`/`sessionStorage` by `state_key` as a JSON array of numbers and restored on mount.

## Differences

| Difference | Reason |
|---|---|
| The root is a `<div>` without the `<p-splitter>` host | The host in Angular is a framework detail. |
| `panels: Vec<ViewFn>` instead of `ng-template #panel` and `ContentChildren` | Instead of content projection. |
| `nested` is an explicit property | In PrimeNG nesting is detected by the `contentChild(Splitter)` query. |
| `min_sizes[i]` applies to the previous panel, `min_sizes[i+1]` to the next one for gutter `i` | In PrimeNG `minSizes[0]` and `minSizes[1]` are checked for any gutter; for three or more panels this is a bug in the source. For two panels the behaviour matches. |
| A change violating `min_sizes` is rejected entirely, `aria-valuenow` is taken from the current panel size (each gutter has its own) | In PrimeNG a single shared `prevSize` is updated even on a rejected change. |
| An arrow applies the step on every `keydown` (including auto-repeat), without the 40 ms timer; `on_resize_start` is called once per series of presses | In PrimeNG `setTimer` calls `resizeStart`/`onResize` repeatedly. |
| The root size is measured once per drag | As in PrimeNG; no recomputation on window resize during dragging. |
| Events without `originalEvent`: `SplitterResizeEvent { sizes }` | A DOM event is not passed out through Leptos handlers. |
| The panel count, layout, step, `gutter_size`, `min_sizes` are read once; changing `panels` on the fly is not supported | A simple model; PrimeNG recomputes sizes when `panelSizes` changes. |
| Saved state is applied only when the panel count matches, storage errors are ignored | Protection against stale data. |
| Touch: `touchmove`/`touchend`/`touchcancel` listeners on `window`, without `preventDefault` | Not tested on real devices. |
| No `Bind`/`pt` with context | `pt` is plain additions of classes, style and attributes. |
| In the demos the root has `width: 100%`; Tailwind utility classes are replaced with inline styles | Our demo card is flex, PrimeNG's is block. |

## Cost

108 allocations per instance in SSR (two panels); contribution to wasm is about 313 KB (budget 350 KB in `benches/budgets.toml`).

## Not verified

Touch on real devices, `rtl` in a real browser (only the pure offset function was checked), screen readers. Markup, mouse, arrows, `min_sizes`, vertical and nested layouts, and storage were verified in Chrome (e2e and browser tests).
