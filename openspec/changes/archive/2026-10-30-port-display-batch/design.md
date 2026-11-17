# Design

## Context

Requirements are in `specs/{overlaybadge,timeline,metergroup}`. PrimeNG 21.1.10 sources: `overlaybadge/overlaybadge.ts`, `timeline/timeline.ts`, `metergroup/metergroup.ts` and their `style/*style.ts`. Reference implementations in the project: `crates/prime-badge` (nested component, `state.rs` with pure class and `data_p` logic), `crates/prime-avatargroup` (container with a slot), `prime-accordion` (templates as `Callback<_, AnyView>`), `prime_rs_core::Pt`/`OptProp`/`Prop`/`all_static!`.

Observations about the source:
- `OverlayBadge` is a `div.p-overlaybadge` with a slot and a `p-badge`; `styleClass`/`style` go to the badge, not the root; all positioning is in CSS.
- `Timeline` is an `*ngFor` over `value`, three parts per event plus a marker and a connector (the last event has no connector); `align` and `layout` are strings that end up in classes and `data-p`.
- `MeterGroup` is a root with ARIA `meter`, the segments compute the percentage via `percent()`: `round(clamp((v - min) / (max - min) * 100, 0, 100))`, 100 when `min == max`; labels are moved into `MeterGroupLabel`. Labels are rendered once: before the segments with `labelPosition = start`, after with `end` (the source has two blocks conditioned on the position).
- These three have no `/* For PrimeNG */` blocks, so `primeng_extra` does not change.

## Decisions

### D1. Crates and dependencies
`prime-overlaybadge` (depends on `prime-rs-core`, `prime-badge`), `prime-timeline`, `prime-metergroup` (only `prime-rs-core`). Each with `ssr`/`hydrate`, rustdoc with an example, SSR tests; public `…Pt { … }` structured by PrimeNG parts: `OverlayBadgePt { root, pc_badge: BadgePt }`, `TimelinePt { root, event, event_opposite, event_separator, event_marker, event_connector, event_content }`, `MeterGroupPt { root, meters, meter, label_list, label, label_icon, label_marker, label_text }`.

### D2. OverlayBadge
A simple container: `children: Children` and a `Badge` with property forwarding. Properties are `OptProp`/`Prop`, as in `Badge`; to avoid duplicating types, `BadgeSeverity` and `BadgeSize` are re-exported. There is no static branch: the root is a single element, the badge decides for itself whether it is reactive.

### D3. Timeline
Generic `Timeline<T>`: `value: Prop<Vec<T>>` (a signal recreates the list), `align: Prop<TimelineAlign>`, `layout: Prop<TimelineLayout>`; templates `content`, `opposite`, `marker` are `Option<Callback<T, AnyView>>` (`T: Clone + Send + Sync + 'static`). Events are rendered by a `For`-like traversal by index, the connector by `index + 1 < len`. Pure functions `root_classes`, `data_p` in `state.rs` with a table-driven test. The `align` classes are only the five valid values (an enum type instead of PrimeNG's arbitrary string; the difference is recorded in the documentation).

### D4. MeterGroup
`MeterItem { label: String, color: String, value: f64, icon: Option<ViewFn> }` (`Clone`). Pure functions `percent(value, min, max) -> u32`, `total_percent`, `cumulative` live in `state.rs` and are covered by table-driven tests (boundaries, `min == max`, negatives, above `max`). The `min`/`max` properties are `Prop<f64>`, `orientation`, `label_position`, `label_orientation` are `Prop` of enums. Templates `label`, `meter`, `start`, `end`, `icon` are a `Callback` with the context `MeterGroupContext { items, total_percent, percentages }` (for `meter` and `icon` the item and index). Labels are a separate internal markup function, called in one of two places by `label_position`. Width/height is via `style` with `width`/`height` in percent and `background-color`; with a `match` on orientation the second dimension is not emitted (in PrimeNG it is `false`, i.e. also not emitted).

### D5. CSS
`overlaybadge`, `timeline`, `metergroup` are added to `theme.components`; `cargo xtask gen-theme`, then a check that the CSS of the other components has not changed.

### D6. Demos
`/overlaybadge` (`basic`, `avatar`, `button`), `/timeline` (`basic`, `alignment`, `opposite`, `horizontal`, `customized`, without icons that are not in `prime-icons`), `/metergroup` (`basic`, `multiple`, `icon`, `vertical`, `label`, `min-max`). Missing icons are replaced by available ones (`CheckIcon`, `SearchIcon`, `TimesIcon`, `SpinnerIcon`); the difference is in `docs/display-batch.md`. A demo heading must not coincide with the ids of the component's own elements.

### D7. Tests
SSR for each spec scenario; for `percent`, table-driven tests of the pure logic; templates are tested with `Owner::new().with(...)`. No browser tests: the components are static; e2e check computed styles (badge corner, segment width, timeline line direction).

### D8. Benchmarks
Scenarios `overlaybadge`, `timeline`, `metergroup` (`-1`, `-1000`) and `case-*` for wasm; first `cargo bench -p render-bench --bench alloc`, then budgets.

## Risks / Trade-offs

- [Generic `Timeline<T>` grows monomorphization code] → one event type in the demos; the wasm size is checked by the budget.
- [Rebuilding the list when `value` changes] → acceptable for static display components; the `value` signal is read only in the list closure.
- [`align` is limited to five values] → PrimeNG accepts any string, but the CSS knows only these five.
- [Icons in `MeterItem`: no font classes] → `ViewFn` instead of a class name.

## Migration Plan

No migration. Rollback is `git revert`.

## Open Questions

None. `Breadcrumb` and `Steps` are moved to the next batch together with the `MenuItem` model.
