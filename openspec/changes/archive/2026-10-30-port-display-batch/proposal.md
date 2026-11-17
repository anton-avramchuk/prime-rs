# Proposal

## Why

The next batch of wave 2 is display components without input and without the `MenuItem` model: `OverlayBadge` (a badge over arbitrary content), `Timeline` (a chain of events) and `MeterGroup` (a composite scale with labels). They rely only on the existing `Badge`, `prime-core` and the theme CSS, so they can be built as one package following the established cycle. `Breadcrumb` and `Steps` depend on a shared menu item model (`MenuItem`: links, `routerLink`, `command`, visibility, disabling) that the project does not have yet; it is more sensible to move them to a separate batch together with this model.

## What Changes

- Three new crates:
  - `prime-overlaybadge` (`OverlayBadge`): `<div class="p-overlaybadge">` with child content and a nested `Badge`; properties `value`, `severity`, `badge_size`, `badge_disabled`, `style_class`, `style`, `pt` (`root`, `pc_badge`).
  - `prime-timeline` (`Timeline<T>`): events from `value`, `align` (`left`/`right`/`top`/`bottom`/`alternate`, default `left`), `layout` (`Vertical`/`Horizontal`), templates `content`, `opposite`, `marker` as `Callback<T, AnyView>`; root `p-timeline p-component p-timeline-<align> p-timeline-<layout>`, parts `event`, `eventOpposite`, `eventSeparator`, `eventMarker`, `eventConnector` (absent on the last event), `eventContent`; `data-p`.
  - `prime-metergroup` (`MeterGroup`): `value: Vec<MeterItem>` (`label`, `color`, `value`, `icon`), `min`, `max`, `orientation`, `label_position`, `label_orientation`, templates `label`, `meter`, `start`, `end`, `icon`; `role="meter"` with `aria-valuemin/max/now`; segment width or height in percent, rounded and clamped to 0-100.
- Theme CSS `overlaybadge`, `timeline`, `metergroup` is added to `theme.components`.
- Showcase pages `/overlaybadge`, `/timeline`, `/metergroup` with demos from the PrimeNG documentation; e2e, benchmarks and budgets for the three crates, `docs/display-batch.md`.
- **Out of scope:** `Breadcrumb` and `Steps` (need the `MenuItem` model, a separate batch); `ng-content` templates like `PrimeTemplate` (replaced by callback properties); the deprecated `size` property of OverlayBadge; `hostName`; HTML labels (`escape = false`); icon fonts (`icon` of `MeterItem` is a `ViewFn`/icon class name from `prime-icons`, not `<i class>`).

## Capabilities

### New Capabilities
- `overlaybadge`: a badge overlaid on the top-right corner of the content.
- `timeline`: a vertical and horizontal chain of events with a marker, connector and labels.
- `metergroup`: a composite scale of several segments with labels and a total value.

### Modified Capabilities
<!-- None: existing requirements do not change. -->

## Impact

- New crates `crates/prime-overlaybadge`, `prime-timeline`, `prime-metergroup` (`publish = false`); dependencies: `prime-core`, `prime-badge` (for OverlayBadge).
- Changed: workspace `Cargo.toml`, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`.
