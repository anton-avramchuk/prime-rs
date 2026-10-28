# Proposal

## Why

Textarea — многострочное поле ввода, второй компонент волны 2. Он переиспользует модель значения InputText (двусторонний `RwSignal<String>`, события, вариант из `PrimeConfig`) и добавляет клиентское поведение — автоматическую подгонку высоты (`autoResize`) с событием `onResize`.

## What Changes

- Новый крейт `prime-textarea` с компонентом `Textarea` (`<textarea>`): двусторонний `value`, `rows`, `cols`, `id`, `name`, `placeholder`, `disabled`, `readonly`, `required`, `maxlength`, `aria_label`, `aria_describedby`, `auto_resize`, `size`, `variant`, `invalid`, `fluid`, `style`, `style_class`, события `on_input`, `on_change`, `on_focus`, `on_blur`, `on_resize`, `pt`.
- Классы как в PrimeNG 21.1.10: `p-textarea p-component` и `p-filled`, `p-textarea-resizable`, `p-variant-filled`, `p-textarea-fluid`, `p-inputfield-sm p-textarea-sm`, `p-textarea-lg p-inputfield-lg`, `p-invalid` (у директивы PrimeNG нет `data-p`).
- `auto_resize`: при монтировании и при каждом изменении значения высота выставляется по `scrollHeight` (с учётом `max-height`), только в браузере.
- Страница `/textarea` (`basic`, `autoresize`, `filled`, `sizes`, `invalid`, `disabled`, `fluid`), e2e, бенчмарки, бюджеты.
- **Не входит:** интеграция с формами, `KeyFilter`, `FloatLabel`/`IftaLabel`, контекст `Fluid`.

## Capabilities

### New Capabilities
- `textarea`: компонент Textarea — значение и события, атрибуты, классы, автоподгонка высоты, `pt`, производительность.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-textarea` (`publish = false`); зависимости `prime-core`, `web-sys`.
- Изменяются `showcase`, `benches/*`, `e2e/tests`, браузерный job CI.
