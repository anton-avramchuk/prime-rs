# Design

## Context

Требования — в `specs/inputtext`. Исходники PrimeNG 21.1.10: `packages/primeng/src/inputtext/inputtext.ts` (директива `pInputText` на нативном `<input>`), `inputtext/style/inputtextstyle.ts`, демонстрации `apps/showcase/doc/inputtext`. Директива вешает классы и `data-p` на нативный `<input>`; значение живёт в самом элементе и синхронизируется с `ngModel`. `p-filled` определяется заполненностью модели; вариант — `variant || config.inputStyle || config.inputVariant`. Шаблон компонента — `docs/button.md`; `PrimeConfig.input_variant`, `Variant` и `Size` уже есть в `prime-core`.

## Goals / Non-Goals

**Goals:** модель значения и событий, которая станет образцом для Checkbox, RadioButton, ToggleSwitch; классы и `data-p` как в PrimeNG; статическая ветвь для неинтерактивного (без привязки) случая.
**Non-Goals:** интеграция с формами (валидаторы, dirty/touched), `KeyFilter`, `FloatLabel`, `IconField`, `InputGroup`, контекст `Fluid`.

## Decisions

### D1. Компонент вместо директивы
`<InputText />` отрисовывает `<input>`; в Leptos нет директив на произвольный элемент с тем же удобством. Атрибуты нативного `<input>`, которые чаще всего нужны, перечислены явными свойствами.

### D2. Двусторонняя привязка — `RwSignal<String>`
`value: Option<RwSignal<String>>`: поле читает сигнал (`prop:value` плюс атрибут `value` для SSR) и записывает в него при `input`. Без `value` поле неконтролируемое: значение живёт в DOM, `p-filled` не отслеживается (в PrimeNG он влияет только на плавающие подписи). Так выбрано, чтобы статический `InputText` не создавал сигнала.
*Альтернативы*: пара `value` + `on_input` (громоздко, нет двусторонности); внутренний сигнал всегда (лишний сигнал у каждого поля).

### D3. События
`on_input`, `on_change`, `on_focus`, `on_blur` получают исходное `web_sys::Event` (`FocusEvent` для фокуса). Колбэки вызываются после записи значения в сигнал.

### D4. Вариант из конфигурации
`variant: OptProp<Variant>`; итоговый вариант — `variant` либо `PrimeConfig.input_variant` (читается при создании экземпляра из контекста).

### D5. Ветви
Статическая ветвь выбирается при `all_static!(…)` и `value.is_none()`: `p-filled` не нужен, а `<input>` без обработчиков значения. Иначе — реактивная: атрибуты — замыкания, `prop:value` и `on:input`.

### D6. Тесты
SSR: структура, атрибуты, классы, `data-p`, `pt`, паритет ветвей, значение в `value`. Браузер (`wasm-bindgen-test`): ввод обновляет сигнал, изменение сигнала обновляет поле, события, `disabled`/`readonly`.

## Risks / Trade-offs

- [Неконтролируемое поле не отслеживает `p-filled`] → зафиксировано в спеке и `docs/inputtext.md`.
- [`prop:value` при SSR] → проверяется SSR-тестом (значение должно попасть в HTML атрибутом `value`).

## Migration Plan

Миграции нет: новый крейт. Откат — `git revert`.

## Open Questions

Нет.
