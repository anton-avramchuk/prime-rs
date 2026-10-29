# Proposal

## Why

Knob — числовой ввод в виде круговой шкалы, перетаскиваемой мышью и касанием; по устройству это ближайший родственник Slider (значение, границы, шаг, перетаскивание с подпиской на `window`, клавиши), только вместо полосы SVG-дуга. Порт следует за `port-slider` и переиспользует его приёмы.

## What Changes

- Новый крейт `prime-knob` с компонентом `Knob`: двусторонний `value` (`RwSignal<f64>`) или внутреннее состояние с `default_value` (без него показывается `min`), свойства `min` (0), `max` (100), `step` (1), `size` (100 px), `stroke_width` (14), `show_value` (`true`), `value_template` (`{value}`), `value_color`, `range_color`, `text_color`, `readonly`, `disabled`, `required`, `name`, `tabindex`, `aria_label`, `aria_labelledby`, `style`, `style_class`; событие `on_change` (новое значение) и `pt` (`root`, `svg`, `range`, `value`, `text`).
- Разметка как в PrimeNG 21.1.10 (`knob.ts`, `knobstyle.ts`): корень `<div class="p-knob p-component">` (`p-disabled`), внутри `<svg viewBox="0 0 100 100" role="slider">` с `path.p-knob-range`, `path.p-knob-value` и `text.p-knob-text`; дуга от −120° до +120° (с разрывом внизу), значение рисуется от нуля шкалы.
- Взаимодействие как в PrimeNG: клик по шкале, перетаскивание мышью и касанием (слушатели на `window` только на время нажатия), клавиши стрелок (±1), `PageUp`/`PageDown` (±10), `Home`/`End`.
- Страница `/knob` (`basic`, `minmax`, `reactive`, `size`, `step`, `stroke`, `template`, `color`, `readonly`, `disabled`), e2e, браузерные тесты, бенчмарки, бюджеты, `docs/knob.md`, CSS темы `knob`.
- **Не входит:** Angular Forms (`ngModel`, реактивные формы); `styleClass` отдельной логикой (есть `style_class`); демонстрации `reactiveforms` и `templatedrivenforms`.

## Capabilities

### New Capabilities
- `knob`: компонент Knob — значение и шкала, ввод мышью, касанием и клавиатурой, структура и доступность, `pt`, производительность.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-knob` (`publish = false`); зависимости `prime-core`.
- Изменяются `Cargo.toml` workspace, `upstream.toml` (`knob` в `theme.components`), `showcase`, `benches/*`, `e2e/tests`, `ci.yml` (браузерный job для `tests/web.rs`).
