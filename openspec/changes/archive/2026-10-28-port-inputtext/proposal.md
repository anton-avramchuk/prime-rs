# Proposal

## Why

InputText — основа всех текстовых полей и первый компонент, который принимает ввод. В PrimeNG это директива `pInputText` на нативном `<input>`, связанная с формой через `ngModel`; в Leptos нужен компонент с двусторонней привязкой значения. Решения, принятые здесь (модель значения, события ввода, `invalid`, `variant`, `fluid`, размер), станут образцом для Checkbox, RadioButton, ToggleSwitch и остальных полей ввода.

## What Changes

- Новый крейт `prime-inputtext` с компонентом `InputText`: двусторонняя привязка `value` (`RwSignal<String>`), события `on_input`, `on_change`, `on_focus`, `on_blur`, атрибуты `input_type`, `id`, `name`, `placeholder`, `disabled`, `readonly`, `required`, `maxlength`, `autocomplete`, `aria_label`, `aria_describedby`; оформление `size` (`small`, `large`), `variant` (`filled`, `outlined`, значение по умолчанию из `PrimeConfig.input_variant`), `invalid`, `fluid`, `style_class`, `style`, `pt`.
- Классы `p-inputtext p-component` и модификаторы `p-filled`, `p-inputtext-sm`, `p-inputtext-lg`, `p-invalid`, `p-variant-filled`, `p-inputtext-fluid`, а также `data-p` как в PrimeNG 21.1.10.
- Страница `/inputtext` в showcase (`basic`, `filled`, `sizes`, `invalid`, `disabled`, `fluid`), e2e, бенчмарки, бюджеты.
- **Не входит:** интеграция с формами (валидаторы, dirty/touched), `KeyFilter`, `FloatLabel`/`IftaLabel`/`IconField`/`InputGroup` (отдельные компоненты), `Fluid`-контекст (есть свойство `fluid`).

## Capabilities

### New Capabilities
- `inputtext`: компонент InputText — значение и события, атрибуты, размер, вариант, `invalid`, `fluid`, `pt`, производительность.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-inputtext` (`publish = false`); зависимости `prime-core`, `web-sys`.
- Изменяются `showcase`, `benches/render`, `benches/wasm-size`, `benches/budgets.toml`, `benches/alloc-budgets.toml`, `e2e/tests`.
