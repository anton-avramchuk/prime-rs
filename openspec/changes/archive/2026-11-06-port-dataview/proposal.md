# Proposal

## Why

DataView is data as a list or a grid with a paginator, sorting and a filter. The first component of wave 7 (DataView -> Table -> TreeTable): small and builds on the ready-made Paginator.

## What Changes

- New crate `prime-dataview` (port of `p-dataview` from PrimeNG 21.1.10): generic `DataView<T>`.
- Pure module `data.rs`: `prepare` (filter and stable sort by order 1/-1/0), `page`, `total`, `clamp_first`; table-driven tests.
- Markup `div.p-dataview.p-component.p-dataview-list|grid` > `p-dataview-loading` (mask and icon), `p-dataview-header`, paginator `p-dataview-paginator-top|bottom`, `p-dataview-content` (the `list`/`grid` templates receive the page items), `p-dataview-empty-message`, `p-dataview-footer`.
- Properties: `value`, `list`, `grid`, `layout`, `paginator`, `first`, `rows`, `total_records`, paginator settings, `paginator_position`, `lazy`, `lazy_load_on_init`, `filter` (predicate), `compare` and `sort_order`, `empty_message`, `loading`, `header`, `footer`, `empty`, `style`, `style_class`; events `on_page`, `on_lazy_load`.
- The `/dataview` page (`basic`, `layout`, `pagination`, `sorting`, `loading`), e2e, benchmarks and budgets, `docs/dataview.md`, `dataview` in `theme.components`.
- **Not included:** `sortField`/`filterBy` by field name (replaced by `compare` and `filter`), `trackBy`, `loadingIcon`, `onChangeLayout`/`onSort`, Angular templates.

## Capabilities

### New Capabilities
- `dataview`: data as a list or a grid with a paginator, sorting and a filter.

### Modified Capabilities
<!-- None. -->

## Impact

- New crate `crates/prime-dataview` (`publish = false`); dependencies: `prime-core`, `prime-paginator`, `prime-icons`.
- Changed: workspace `Cargo.toml`, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml`.
