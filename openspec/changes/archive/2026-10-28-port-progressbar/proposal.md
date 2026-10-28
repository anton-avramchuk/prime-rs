# Proposal

## Why

ProgressBar — последний компонент волны 1: индикатор выполнения (определённый и неопределённый). Он без поведения на стороне клиента, поэтому проверяет, что шаблон компонента (две ветви отрисовки, `pt`, `data-p`) хорошо ложится и на чисто отображающие компоненты со значением-числом.

## What Changes

- Новый крейт `prime-progressbar` с компонентом `ProgressBar`: `value` (число, 0–100), `mode` (`determinate` по умолчанию, `indeterminate`), `show_value` (по умолчанию `true`), `unit` (по умолчанию `%`), `color`, `style`, `style_class`, `value_style_class`, слот `content_template` (получает значение), `pt` (`root`, `value`, `label`).
- Разметка и классы как в PrimeNG 21.1.10: корень `role="progressbar"` с `aria-valuemin="0"`, `aria-valuemax="100"`, `aria-valuenow`, `aria-level` и классами `p-progressbar p-component p-progressbar-determinate|indeterminate`; в режиме `determinate` — `div.p-progressbar-value` (`width: <value>%; display: flex`, при `color` — `background`) с `div.p-progressbar-label`; в режиме `indeterminate` — только `div.p-progressbar-value`; `data-p` на корне и частях.
- Страница `/progressbar` (`basic`, `dynamic`, `indeterminate`, `template`), e2e, бенчмарки, бюджеты.
- **Не входит:** анимации обновления (CSS темы), интеграция с формами.

## Capabilities

### New Capabilities
- `progressbar`: компонент ProgressBar — режимы, значение, подпись, классы, `pt`, производительность.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-progressbar` (`publish = false`); зависимость только `prime-core`.
- Изменяются `showcase`, `benches/render`, `benches/wasm-size`, бюджеты, `e2e/tests`.
