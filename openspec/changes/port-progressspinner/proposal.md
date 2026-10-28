# Proposal

## Why

ProgressSpinner — индикатор неопределённого ожидания: один SVG с вращающейся окружностью и без клиентской логики. Он нужен как самостоятельный компонент и как заготовка для будущих индикаторов загрузки (Button, Table, Dropdown). Порт мал и закрывает ещё один паттерн: SVG-разметка с настраиваемыми атрибутами окружности.

## What Changes

- Новый крейт `prime-progressspinner` с компонентом `ProgressSpinner`: `stroke_width` (по умолчанию `"2"`), `fill` (`"none"`), `animation_duration` (`"2s"`), `aria_label`, `style`, `style_class`, `pt` (части `root`, `spin`, `circle`).
- Разметка и классы как в PrimeNG 21.1.10 (`progressspinnerstyle.ts`): корень `div.p-progressspinner` с `role="progressbar"`, `aria-busy="true"` и `aria-label`; внутри `svg.p-progressspinner-spin` (`viewBox="25 25 50 50"`, `animation-duration` в `style`) и `circle.p-progressspinner-circle` (`cx="50" cy="50" r="20"`, `fill`, `stroke-width`, `stroke-miterlimit="10"`).
- Страница `/progressspinner` (`basic`, `custom`), e2e, бенчмарки, бюджеты, `docs/progressspinner.md`, CSS темы `progressspinner`.
- **Не входит:** синоним селектора `p-progressSpinner`, `BlockableUI`; устаревший `styleClass` заменён `style_class`.

## Capabilities

### New Capabilities
- `progressspinner`: компонент ProgressSpinner — структура и классы, ARIA, свойства окружности и анимации, `pt`, производительность.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-progressspinner` (`publish = false`); зависимость `prime-core`.
- Изменяются `Cargo.toml` workspace, `upstream.toml` (`progressspinner` в `theme.components`), `showcase`, `benches/*`, `e2e/tests`. Браузерных тестов нет, job `ci.yml` не меняется.
