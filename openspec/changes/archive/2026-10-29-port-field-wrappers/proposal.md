# Proposal

## Why

Волна 2 начинается с набора крошечных обёрток форм — `ButtonGroup`, `Fluid`, `IconField` с `InputIcon`, `FloatLabel`, `IftaLabel`. Каждая в PrimeNG занимает 30–60 строк (контейнер с классами и слотом), но без них нельзя собрать типичную форму: значок в поле, плавающая метка, группа кнопок, «растянуть всё». От них же зависят демонстрации, которые я пропускал в InputText, InputNumber и Select (`floatlabel`, `iftalabel`), а Select уже использует разметку `p-iconfield` для поля поиска.

## What Changes

- Пять новых крейтов, по одному на обёртку: `prime-buttongroup` (`ButtonGroup`), `prime-fluid` (`Fluid`), `prime-iconfield` (`IconField` и `InputIcon`, как `InputGroup` и `InputGroupAddon` в одном крейте), `prime-floatlabel` (`FloatLabel`), `prime-iftalabel` (`IftaLabel`). У всех: дочернее содержимое, `style`, `style_class`, `pt` с частью `root`.
  - `ButtonGroup` — `<span class="p-buttongroup p-component" role="group">`.
  - `IconField` — `<span class="p-iconfield p-iconfield-left|right">` (`icon_position`, по умолчанию слева), `InputIcon` — `<span class="p-inputicon">` с иконкой внутри.
  - `FloatLabel` — `<span class="p-floatlabel p-floatlabel-over|on|in">` (`variant`, по умолчанию `Over`), `IftaLabel` — `<span class="p-iftalabel">`.
  - `Fluid` — `<div class="p-fluid">`, который задаёт контекст «растянуть вложенные поля».
- Наследование `fluid` через контекст: в `prime-core` появляется `FluidContext` (`provide_fluid`, `in_fluid`, `resolve_fluid`); `Button`, `InputText`, `Textarea`, `InputNumber` и `Select` становятся потребителями — их свойство `fluid` меняется с `Prop<bool>` на `OptProp<bool>`: явное значение (в том числе `false`) главнее контекста, а без него компонент внутри `Fluid` растягивается, как `hasFluid = fluid() ?? !!pcFluid` в PrimeNG. Вызовы вида `fluid=true` остаются прежними.
- CSS темы `buttongroup`, `floatlabel`, `iftalabel` (`iconfield` уже подключён); у `fluid` стилей в PrimeNG нет.
- Страницы showcase `/buttongroup` (демонстрации из `button/buttongroup-doc.ts`), `/fluid`, `/iconfield` (`basic`, `sizes`, `template`, `floatlabel`, `iftalabel`), `/floatlabel` (`basic`, `variants`, `invalid`), `/iftalabel` (`basic`, `invalid`); демонстрации `floatlabel` и `iftalabel`, пропущенные ранее, добавляются на страницы InputText, InputNumber, Select и Textarea; e2e, бенчмарки и бюджеты для пяти крейтов, `docs/field-wrappers.md`.
- **Не входит:** интеграция с Angular Forms (`ng-invalid`, правила `:has(.ng-invalid.ng-dirty)` в блоках `/* For PrimeNG */` — они не применимы, `primeng_extra` не меняется); правила `.p-buttongroup p-button …` из блока `/* For PrimeNG */` у ButtonGroup (в PrimeNG они нужны хост-элементу `<p-button>`, наш `Button` рисует `<button class="p-button">` сам, и базовый CSS темы его уже покрывает); `hostName`; поддержка `fluid` в `ToggleButton` и `SelectButton` (у них в PrimeNG нет наследования от `Fluid`).

## Capabilities

### New Capabilities
- `buttongroup`: группа слитных кнопок.
- `fluid`: контекст растягивания вложенных полей и кнопок.
- `iconfield`: поле со значком внутри (`IconField` и `InputIcon`).
- `floatlabel`: плавающая метка трёх вариантов.
- `iftalabel`: метка над введённым значением.

### Modified Capabilities
<!-- Поведение существующих компонентов расширяется требованием из `fluid` (наследование от контекста); их собственные требования остаются в силе, поэтому дельты не нужны. -->

## Impact

- Новые крейты `crates/prime-buttongroup`, `prime-fluid`, `prime-iconfield`, `prime-floatlabel`, `prime-iftalabel` (`publish = false`); зависимости `prime-core`.
- Изменяются `prime-core` (`fluid`), `prime-button`, `prime-inputtext`, `prime-textarea`, `prime-inputnumber`, `prime-select` (тип и разрешение `fluid`), `Cargo.toml` workspace, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`.
