# Tasks

## 1. Реализация

- [x] 1.1 Chip: `Props`/`Pt` без `Arc` в статической ветви, блоки без `.into_any()`, `Pt::merge_style_parts`; проверить SSR- и браузерными тестами
- [x] 1.2 Button: `Props`/`Pt` без `Arc` в статической ветви, `NodeRef` только на `wasm32`, `EitherOf4`/`Option<impl IntoView>`, `icon_classes` → `Cow`; проверить SSR-, паритет- и браузерными тестами

## 2. Бюджеты и интеграция

- [x] 2.1 Пересчитать бюджеты `button-*`, `chip-*`, дополнить `docs/benchmarks.md`
- [x] 2.2 clippy, тесты, wasm-size, deny, machete, `openspec validate optimize-allocs-button-chip`; CI `dev` зелёный

## Workflow follow-up

- Архивировать после CI.
