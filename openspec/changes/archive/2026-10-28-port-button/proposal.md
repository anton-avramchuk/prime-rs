# Proposal

## Why

Инфраструктура готова (`bootstrap-workspace` архивирован), но ни одного компонента пока нет. Button — первый порт волны 1: самый простой по логике, но затрагивает все сквозные механизмы (классы `p-*`, `pt`, ripple, иконки, конфигурация, тема, бенчмарки, бюджеты размера), поэтому он становится эталоном, по которому портируются остальные компоненты. Внутри `p-button` используется `p-badge`, поэтому Badge портируется вместе с ним.

## What Changes

- Новый крейт `prime-badge` с компонентом `Badge`: тот же набор свойств, классы `p-badge*` и `data-p`, как в PrimeNG 21.1.10.
- Новый крейт `prime-button` с компонентом `Button`: свойства, события, классы `p-button*`, `data-p*`, состояния `loading`/`disabled`, иконка, метка, бейдж, `fluid`, ripple, базовый `pt`.
- Иконка передаётся как `ViewFn` (например, `<CheckIcon />`), а не строкой имени класса: шрифта PrimeIcons в проекте нет.
- Сквозные дополнения: тип `Pt` расширяется до набора частей компонента; `Ripple` подключается по `PrimeConfig.ripple`; в `upstream.toml` фиксируются исходники, с которых портированы компоненты.
- Инфраструктура: сценарии `render-bench`, случаи `case-button` и `case-badge` в `wasm-size` и бюджеты, страницы Button и Badge в showcase, запись в `e2e/tests/pages.json` для сравнения с PrimeNG.
- **Не входит:** директива `pButton` и её вспомогательные `ButtonLabel`/`ButtonIcon`, `buttonProps`, контекст `Fluid`, `unstyled`-режим, `SplitButton`, `ButtonGroup`.

## Capabilities

### New Capabilities
- `badge`: компонент Badge — значение, серьёзность, размер, точка и круг, скрытие.
- `button`: компонент Button — вид, состояния, иконка, метка, бейдж, события, доступность, ripple, `pt`.

### Modified Capabilities
<!-- Нет: существующие спеки (core-runtime, theme-css, …) не меняются; Button использует их как есть. -->

## Impact

- Новые крейты `crates/prime-badge`, `crates/prime-button` (`publish = false`).
- Изменяются: `prime-core` (расширение `Pt`, без изменения требований), `upstream.toml` (список CSS-компонентов уже включает `button` и `badge`), `benches/render`, `benches/wasm-size`, `benches/budgets.toml`, `showcase`, `e2e/tests/pages.json`.
- Зависимости: `prime-core`, `prime-dom` (ripple, autofocus), `prime-icons` (`spinner`), `prime-theme` (`button`, `badge`), `prime-badge`. Внешних новых зависимостей нет.
- Зависящие в будущем: `Chip`, `Tag`, `SplitButton` и т. д. возьмут отсюда схему `pt`, работы с иконками и тестов.
