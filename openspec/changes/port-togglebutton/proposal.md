# Proposal

## Why

ToggleButton — кнопка с булевым состоянием (включена/выключена), первый шаг к SelectButton, который строится из таких кнопок. Модель значения и события те же, что у Checkbox и ToggleSwitch, нативного `<input>` нет, общей логики в `prime-core` не требуется, поэтому компонент хорошо подходит как следующий небольшой порт.

## What Changes

- Новый крейт `prime-togglebutton` с компонентом `ToggleButton`: двусторонний `checked` (`RwSignal<bool>`) или внутреннее состояние с `default_checked`, `on_label`/`off_label` (по умолчанию «Yes»/«No»), `on_icon`/`off_icon` (`ViewFn`), `icon_pos`, `size`, `disabled`, `invalid`, `allow_empty`, `fluid`, `tabindex`, `aria_label`, `aria_labelledby`, `style`, `style_class`, слоты `icon_template` и `content_template`, события `on_change`, `on_focus`, `on_blur`, `pt`.
- Разметка как в PrimeNG 21.1.10 (`togglebutton.ts`, `togglebuttonstyle.ts`): корень `<div class="p-togglebutton p-component" role="button" aria-pressed tabindex>` с `data-pc-name`, `data-p-checked`, `data-p-disabled`, `data-p`; внутри `span.p-togglebutton-content` с иконкой и `span.p-togglebutton-label`. Переключение по клику и по клавишам Enter и Space.
- `xtask gen-theme` принимает маркер `/* For PrimeNG (…) */` (в `togglebuttonstyle.ts` он с пояснением), `togglebutton` добавляется в `primeng_extra`.
- Страница `/togglebutton` (`basic`, `customized`, `disabled`, `fluid`, `invalid`, `sizes`), e2e, браузерные тесты, бенчмарки, бюджеты, `docs/togglebutton.md`, CSS темы `togglebutton`.
- **Не входит:** интеграция с Angular Forms (`ngModel`, реактивные формы); ripple; `autofocus`; демонстрации `reactiveforms` и `templatedrivenforms`; иконка-класс PrimeIcons (вместо неё `ViewFn`).

## Capabilities

### New Capabilities
- `togglebutton`: компонент ToggleButton — состояние и события, клавиатура, структура и классы, атрибуты доступности, `pt`, производительность.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-togglebutton` (`publish = false`); зависимости `prime-core`.
- Изменяются `Cargo.toml` workspace, `upstream.toml` (`togglebutton` в `theme.components` и `primeng_extra`), `xtask/src/gen_theme.rs`, `showcase`, `benches/*`, `e2e/tests`, `ci.yml` (браузерный job для `tests/web.rs`).
