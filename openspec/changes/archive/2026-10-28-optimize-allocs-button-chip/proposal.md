# Proposal

## Why

После `optimize-allocs` Button (19 аллокаций на экземпляр) и Chip (10, removable — 35) остались самыми дорогими: обработчики событий держали `Arc<Props>` и в статической ветви, `Arc<Pt>` создавался всегда, `NodeRef` — при SSR, а блоки собирались через `.into_any()`.

## What Changes

- Button и Chip: `Props` и `Pt` — обычные значения; `Arc` создаётся только в реактивной ветви. Обработчик клика статической кнопки захватывает готовый `bool`.
- Button: `NodeRef` и `node_ref` создаются только в браузере (`wasm32`); блоки индикатора, метки и бейджа — `EitherOf4`/`Option<impl IntoView>` вместо `AnyView`; `icon_classes` возвращает `Cow<'static, str>`.
- Chip: блоки изображения, метки и удаления без `.into_any()`; стиль корня собирается одной строкой (`Pt::merge_style_parts`).
- Бюджеты в `benches/alloc-budgets.toml` для `button-*` и `chip-*` снижаются по новым измерениям.
- **Не входит:** публичный API и разметка (HTML не меняется).

## Capabilities

### New Capabilities
<!-- Нет. -->

### Modified Capabilities
- `benchmarking`: добавляется требование об отсутствии `NodeRef` и `Arc` в статической ветви при SSR.

## Impact

- `crates/prime-button`, `crates/prime-chip`, `crates/prime-core` (`Pt::merge_style_parts`), `benches/alloc-budgets.toml`, `docs/benchmarks.md`.
