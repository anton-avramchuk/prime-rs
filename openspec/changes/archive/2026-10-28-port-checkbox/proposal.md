# Proposal

## Why

Checkbox — следующий компонент волны 1 и первое поле с булевым или множественным значением. Модель значения InputText (двусторонний сигнал, события с исходным `web_sys::Event`) здесь расширяется на режим `binary` и набор значений, как в PrimeNG; решения станут образцом для RadioButton и ToggleSwitch.

## What Changes

- Новый крейт `prime-checkbox` с компонентом `Checkbox`: режим `binary` (значение `RwSignal<bool>`) и режим набора (`value` + `RwSignal<Vec<String>>`); `indeterminate`, `disabled`, `readonly`, `required`, `invalid`, `size`, `variant`, `input_id`, `name`, `tabindex`, `aria_label`, `aria_labelledby`, `input_class`, `input_style`, `style_class`, `style`, слот `icon_template`, `pt` (`root`, `input`, `box`, `icon`).
- События `on_change` (новое значение `checked` и исходное событие), `on_focus`, `on_blur`.
- Разметка и классы как в PrimeNG 21.1.10: `p-checkbox p-component` и модификаторы `p-checkbox-checked p-highlight`, `p-disabled`, `p-invalid`, `p-variant-filled`, `p-checkbox-sm p-inputfield-sm`, `p-checkbox-lg p-inputfield-lg`; `<input type="checkbox" class="p-checkbox-input">`, `<div class="p-checkbox-box">` с иконкой `check` или `minus`; атрибуты `data-p`, `data-p-checked`, `data-p-highlight`, `data-p-disabled`.
- Страница `/checkbox` (`basic`, `multiple`, `filled`, `sizes`, `indeterminate`, `invalid`, `disabled`), e2e, бенчмарки, бюджеты.
- **Не входит:** интеграция с формами, `trueValue`/`falseValue` произвольных типов (только `bool`), `checkboxIcon` как CSS-класс (есть `icon_template`).

## Capabilities

### New Capabilities
- `checkbox`: компонент Checkbox — состояние, события, атрибуты, классы, `pt`, производительность.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-checkbox` (`publish = false`); зависимости `prime-core`, `prime-icons` (`check`, `minus`), `web-sys`.
- Изменяются `showcase`, `benches/render`, `benches/wasm-size`, бюджеты, `e2e/tests`, браузерный job CI.
