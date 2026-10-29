# Proposal

## Why

SelectButton — выбор одного или нескольких вариантов из списка кнопками; строится из `ToggleButton`, который уже портирован (`port-togglebutton`). Это следующий небольшой порт и первый компонент поверх другого компонента prime-rs.

## What Changes

- Новый крейт `prime-selectbutton` с компонентом `SelectButton` и типом `SelectButtonOption` (`label`, `value`, `disabled`): список `options`, одиночный выбор через двусторонний `value` (`RwSignal<Option<String>>`) или множественный (`multiple`) через `values` (`RwSignal<Vec<String>>`); без сигнала компонент ведёт внутреннее состояние (`default_value`, `default_values`).
- `allow_empty`, `disabled`, `invalid`, `fluid`, `size`, `aria_labelledby`, `style` (корень), `style_class` (как в PrimeNG, передаётся каждой кнопке, а не корню), слот `item_template` (получает вариант и индекс), события `on_change` (новое значение и исходное событие), `on_option_click` (вариант, индекс, исходное событие), `pt` (`root` и `toggle_button` для вложенных `ToggleButton`).
- Разметка как в PrimeNG 21.1.10 (`selectbutton.ts`, `selectbuttonstyle.ts`): корень `<div class="p-selectbutton p-component" role="group" aria-labelledby data-p>`, внутри по `ToggleButton` на вариант с `on_label = off_label = label`.
- Страница `/selectbutton` (`basic`, `disabled`, `fluid`, `invalid`, `multiple`, `sizes`, `template`), e2e, браузерные тесты, бенчмарки, бюджеты, `docs/selectbutton.md`, CSS темы `selectbutton`.
- **Не входит:** Angular Forms (`ngModel`, реактивные формы); устаревший `unselectable` (его заменяет `allow_empty`); `autofocus`; `tabindex` (в PrimeNG не используется в шаблоне, у каждой кнопки свой `tabindex`); `option_label`/`option_value`/`option_disabled`/`data_key` (поля произвольного объекта) — вместо них типизированный `SelectButtonOption`; блок `/* For PrimeNG */` (`.ng-invalid.ng-dirty`) не переносится; демонстрации `reactiveforms` и `templatedrivenforms`.

## Capabilities

### New Capabilities
- `selectbutton`: компонент SelectButton — варианты и значение, одиночный и множественный выбор, события, структура и атрибуты, `pt`, производительность.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-selectbutton` (`publish = false`); зависимости `prime-core`, `prime-togglebutton`.
- Изменяются `Cargo.toml` workspace, `upstream.toml` (`selectbutton` в `theme.components`), `showcase`, `benches/*`, `e2e/tests`, `ci.yml` (браузерный job для `tests/web.rs`).
- `ToggleButton` не меняется.
