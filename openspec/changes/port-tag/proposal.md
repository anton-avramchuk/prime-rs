# Proposal

## Why

Tag — следующий компонент волны 1 после Button и Badge: категоризирует контент (значение, серьёзность, иконка, скругление). Он простой по логике и целиком ложится на отработанный шаблон (`Prop`, две ветви отрисовки, тесты паритета), поэтому служит проверкой того, что шаблон переносится на новый компонент без изменений инфраструктуры.

## What Changes

- Новый крейт `prime-tag` с компонентом `Tag`: свойства `value`, `severity`, `rounded`, `icon`, `style`, `style_class`, `children`, `pt`; классы `p-tag*`, `data-p` как в PrimeNG 21.1.10.
- Статическая и реактивная ветви отрисовки, свойства — `Prop`/`OptProp`, бюджеты аллокаций и размера wasm.
- Страница `/tag` в showcase с демонстрациями PrimeNG (`basic`, `icon`, `pill`, `severity`, `template`) и сравнение скриншотов в еженедельном job.
- **Не входит:** шрифтовые иконки PrimeIcons (иконка — `ViewFn`), `unstyled`-режим.

## Capabilities

### New Capabilities
- `tag`: компонент Tag — значение, серьёзность, скругление, иконка, содержимое, `pt`, производительность.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-tag` (`publish = false`).
- Изменяются `showcase` (страница, меню), `benches/render` (сценарии), `benches/wasm-size` (`case-tag`), `benches/budgets.toml`, `benches/alloc-budgets.toml`, `e2e/tests/pages.json`, `e2e/tests` (спека Tag), `crates/prime-theme` (CSS `tag` уже сгенерирован).
- Зависимости: `prime-core`; внешних новых нет.
