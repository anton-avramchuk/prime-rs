# Display: OverlayBadge, Timeline, MeterGroup

Port of `p-overlaybadge`, `p-timeline` and `p-metergroup` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-overlaybadge::OverlayBadge` | `overlaybadge/overlaybadge.ts`, `overlaybadge/style/overlaybadgestyle.ts` |
| `prime-timeline::Timeline` | `timeline/timeline.ts`, `timeline/style/timelinestyle.ts` |
| `prime-metergroup::MeterGroup` | `metergroup/metergroup.ts` (with the internal `MeterGroupLabel`), `metergroup/style/metergroupstyle.ts` |
| CSS | `@primeuix/styles/{overlaybadge,timeline,metergroup}` — `cargo xtask gen-theme` |
| Demos | `apps/showcase/doc/badge/overlay-doc.ts`, `doc/timeline/*-doc.ts`, `doc/metergroup/*-doc.ts` |

Specs: `openspec/specs/{overlaybadge,timeline,metergroup}`.

## What matches PrimeNG

- Classes and their order: `p-overlaybadge`; `p-timeline p-component p-timeline-<align> p-timeline-<layout>` and the `p-timeline-event*` parts; `p-metergroup p-component p-metergroup-<orientation>`, `p-metergroup-meters`, `p-metergroup-meter`, `p-metergroup-label-list*`, `p-metergroup-label*`.
- `OverlayBadge` passes `style_class` and `style` to the nested badge, not to the root.
- `Timeline`: the last event has no connector; `data-p` on the root and parts is `<layout> <align>`.
- `MeterGroup`: `role="meter"`, `aria-valuemin/max/now`; percent is `round(clamp((v - min) / (max - min) * 100, 0, 100))`, 100 when `min == max`; segments with zero or negative values are not rendered; the label is `Label (percent%)`.

## Differences

| Difference | Reason |
|---|---|
| Roots are `<div>`, not `<p-…>` hosts | The host in Angular is a framework detail. |
| The `content`, `opposite`, `marker`, `label`, `meter`, `start`, `end`, `icon` templates are callback props (`Callback<…, AnyView>`) | Instead of `ng-template` and `PrimeTemplate`. |
| `Timeline<T>` is generic, `align` and `layout` are enums (`TimelineAlign` is `Left`, `Right`, `Top`, `Bottom`, `Alternate`) | In PrimeNG they are arbitrary strings, but the CSS knows only these values. |
| `MeterItem::icon` is a `ViewFn`, not a font class name (`pi pi-…`) | In this project icons are `prime-icons` SVG components. |
| `OverlayBadge::style` is read once at creation | The badge accepts style only through `pt`; no demo needs reactive style. |
| No deprecated `size` on `OverlayBadge`, no `escape = false` on labels, `hostName`, or `getBlockableElement` on `Timeline` | Deprecated or tied to Angular. |
| No `template` demo for `Timeline` and `MeterGroup` | They rely on cards and icons that are not in `prime-icons`. |
| Demos use `calendar`, `info-circle`, `exclamation-triangle`, `th-large`, `eye`, `refresh` instead of `bell`, `envelope`, `table`, `inbox`, `image`, `cog` | These icons are not in `prime-icons`. |

## Cost

| | Allocations per instance (SSR) | Contribution to wasm |
|---|---|---|
| `OverlayBadge` (with a badge) | 18 | ~113 KB (includes `Badge`) |
| `Timeline` (4 events, one template) | 99 | ~123 KB |
| `MeterGroup` (3 items) | 179 | ~196 KB |

`MeterGroup` and `Timeline` are the "heaviest" in allocations because of the style strings and `pt` on every part; the `value` signal rebuilds the whole list, so it must be read only inside the component closure.

## Not verified

Touch and screen readers were not tested on real devices: `MeterGroup` segments and `Timeline` events are not interactive; only the markup and computed styles were verified (e2e).
