# Proposal

## Why

RadioButton — следующий компонент волны 1 после Checkbox. Он переиспользует модель Checkbox (корень + `input` + `box`, два класса данных, сигнал значения), добавляя выбор одного значения из группы по общему сигналу.

## What Changes

- Новый крейт `prime-radiobutton` с компонентом `RadioButton`: `value` флажка и двусторонний `model: RwSignal<Option<String>>` группы; `default_checked` для неконтролируемого случая; `disabled`, `required`, `invalid`, `size`, `variant`, `input_id`, `name`, `tabindex`, `aria_label`, `aria_labelledby`, `style`, `style_class`, `pt` (`root`, `input`, `box`, `icon`).
- События `on_click` (значение и исходное событие, как `onClick` PrimeNG), `on_focus`, `on_blur`.
- Разметка и классы как в PrimeNG 21.1.10: `p-radiobutton p-component` и модификаторы `p-radiobutton-checked`, `p-disabled`, `p-invalid`, `p-variant-filled`, `p-radiobutton-sm p-inputfield-sm`, `p-radiobutton-lg p-inputfield-lg`; `<input type="radio" class="p-radiobutton-input">` с `aria-checked`, `<div class="p-radiobutton-box">` с `<div class="p-radiobutton-icon">`; атрибуты `data-p`, `data-p-checked`, `data-p-disabled`.
- Страница `/radiobutton` (`group`, `filled`, `sizes`, `invalid`, `disabled`), e2e, бенчмарки, бюджеты.
- **Не входит:** интеграция с формами и реестр `RadioControlRegistry` (группа определяется общим сигналом), `binary`, `autofocus`.

## Capabilities

### New Capabilities
- `radiobutton`: компонент RadioButton — состояние, события, атрибуты, классы, `pt`, производительность.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-radiobutton` (`publish = false`); зависимости `prime-core`, `web-sys`.
- Изменяются `showcase`, `benches/render`, `benches/wasm-size`, бюджеты, `e2e/tests`, браузерный job CI.
