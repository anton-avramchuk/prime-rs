# Proposal

## Why

Slider — выбор числа или диапазона перетаскиванием ручки; один из базовых компонентов форм, заметно сложнее уже портированных: перетаскивание мышью и касанием, клавиатура, вертикальная ориентация, режим диапазона. Закрывает следующий шаг после Rating и задаёт образец для Knob.

## What Changes

- Новый крейт `prime-slider` с компонентом `Slider`: одиночное значение через двусторонний `value` (`RwSignal<f64>`) или диапазон (`range`) через `values` (`RwSignal<[f64; 2]>`); без сигнала компонент ведёт внутреннее состояние (`default_value`, `default_values`). Свойства `min` (0), `max` (100), `step`, `orientation` (`Horizontal`/`Vertical`), `animate`, `disabled`, `invalid`, `tabindex`, `aria_label`, `aria_labelledby`, `style`, `style_class`; события `on_change` (значение и исходное событие) и `on_slide_end`; `pt` (`root`, `range`, `handle`, `start_handler`, `end_handler`).
- Разметка как в PrimeNG 21.1.10 (`slider.ts`, `sliderstyle.ts`): корень `<div class="p-slider p-component">` с `data-pc-name`, `data-pc-section`, `data-p`, `data-p-sliding`; внутри `span.p-slider-range` и одна (`role=slider`) или две ручки `span.p-slider-handle` с `aria-valuemin/now/max`, `aria-orientation`.
- Взаимодействие как в PrimeNG: перетаскивание ручки мышью (обработчики на документе только на время перетаскивания) и касанием, клик по полосе, клавиши стрелок, `PageUp`/`PageDown`, `Home`/`End`; шаг и нормализация значения как в `handleStepChange` и `getNormalizedValue`; поддержка RTL.
- Страница `/slider` (`basic`, `range`, `step`, `vertical`, `filter`, `input`), e2e, браузерные тесты, бенчмарки, бюджеты, `docs/slider.md`, CSS темы `slider`.
- **Не входит:** Angular Forms (`ngModel`, реактивные формы); `autofocus`; устаревший `styleClass` в виде отдельной логики (есть `style_class`); демонстрации `reactiveforms` и `templatedrivenforms`; внешняя картинка в `filter` (вместо неё градиентный блок с CSS-фильтрами).

## Capabilities

### New Capabilities
- `slider`: компонент Slider — значение и диапазон, ввод мышью, касанием и клавиатурой, ориентации, шаг, структура и доступность, `pt`, производительность.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-slider` (`publish = false`); зависимости `prime-core`.
- Изменяются `Cargo.toml` workspace, `upstream.toml` (`slider` в `theme.components`), `showcase`, `benches/*`, `e2e/tests`, `ci.yml` (браузерный job для `tests/web.rs`).
