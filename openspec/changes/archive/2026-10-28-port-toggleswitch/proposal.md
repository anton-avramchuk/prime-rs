# Proposal

## Why

ToggleSwitch — следующий компонент волны 1 после RadioButton: переключатель булева значения. Он повторяет модель Checkbox (сигнал `checked`, неконтролируемый режим, статическая и реактивная ветви), но имеет другую разметку (`slider` с `handle`) и срабатывает по клику на всём корне, как в PrimeNG.

## What Changes

- Новый крейт `prime-toggleswitch` с компонентом `ToggleSwitch`: двусторонний `checked: RwSignal<bool>`, `default_checked` для неконтролируемого случая; `disabled`, `readonly`, `required`, `invalid`, `input_id`, `name`, `tabindex`, `aria_label`, `aria_labelledby`, `style`, `style_class`, слот `handle_template` (получает `checked`), `pt` (`root`, `input`, `slider`, `handle`).
- События `on_change` (новое значение и исходное событие, как `onChange` PrimeNG), `on_focus`, `on_blur`.
- Разметка и классы как в PrimeNG 21.1.10: корень `p-toggleswitch p-component` со `style="position: relative"` и модификаторами `p-toggleswitch-checked`, `p-disabled`, `p-invalid`; `<input type="checkbox" role="switch" class="p-toggleswitch-input">` с `aria-checked`; `<div class="p-toggleswitch-slider">` с `<div class="p-toggleswitch-handle">`; атрибуты `data-p`, `data-p-checked`, `data-p-disabled`.
- Страница `/toggleswitch` (`basic`, `preselection`, `template`, `invalid`, `disabled`), e2e, бенчмарки, бюджеты.
- **Не входит:** интеграция с формами, `trueValue`/`falseValue` произвольных типов (только `bool`), `autofocus`, `size` (в PrimeNG не влияет на классы).

## Capabilities

### New Capabilities
- `toggleswitch`: компонент ToggleSwitch — состояние, события, атрибуты, классы, `pt`, производительность.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-toggleswitch` (`publish = false`); зависимости `prime-core`, `web-sys`.
- Изменяются `showcase`, `benches/render`, `benches/wasm-size`, бюджеты, `e2e/tests`, браузерный job CI.
