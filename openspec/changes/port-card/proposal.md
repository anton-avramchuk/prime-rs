# Proposal

## Why

Card — контейнер с необязательными областями (шапка с изображением, заголовок, подзаголовок, содержимое, подвал). Это первый компонент волны 1 со слотами `ViewFn`, то есть он проверяет, как шаблон `Prop` + две ветви отрисовки работает для компонента, чья разметка — в основном вложенные необязательные блоки. Демонстрация `advanced` использует Button (уже портирован), так что Card показывает композицию компонентов.

## What Changes

- Новый крейт `prime-card` с компонентом `Card`: `header` и `subheader` (текст), слоты `header_template`, `title_template`, `subtitle_template`, `footer_template` (`ViewFn`), `children` (содержимое), `style`, `style_class`, `pt`; классы `p-card*` как в PrimeNG 21.1.10.
- Страница `/card` в showcase (`basic`, `advanced`), e2e, бенчмарки и бюджеты.
- **Не входит:** `BlockableUI` (интерфейс блокировки Angular), `unstyled`-режим.

## Capabilities

### New Capabilities
- `card`: компонент Card — области, слоты, содержимое, `pt`, производительность.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-card` (`publish = false`); зависимость `prime-core`.
- Изменяются `showcase` (страница использует Button), `benches/render`, `benches/wasm-size`, `benches/budgets.toml`, `benches/alloc-budgets.toml`, `e2e/tests`.
