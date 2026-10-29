# Proposal

## Why

InputGroup группирует поле ввода с текстом, иконкой или кнопками рядом (префикс и суффикс). Это почти статичная разметка поверх уже готовых InputText и Button: общей логики и клиентского кода не требуется, риск минимальный, а компонент нужен для типовых форм.

## What Changes

- Новый крейт `prime-inputgroup` с двумя компонентами: `InputGroup` и `InputGroupAddon`.
- `InputGroup`: корень `<div class="p-inputgroup">` с проецируемым содержимым, `style_class`, `style`, `pt`.
- `InputGroupAddon`: `<div class="p-inputgroupaddon">` с проецируемым содержимым, `style_class`, `style`, `pt`.
- Разметка как в PrimeNG 21.1.10 (`inputgroup.ts`, `inputgroupaddon.ts`, `inputgroupstyle.ts`, `inputgroupaddonstyle.ts`): у хоста нет `role` и `data-p`.
- Страница `/inputgroup` (`basic`, `button`, `checkbox`, `multiple`), e2e, бенчмарки, бюджеты, `docs/inputgroup.md`, CSS темы `inputgroup`.
- **Не входит:** демонстрации `floatlabel` и `iftalabel` (FloatLabel и IftaLabel ещё нет в порте); блок `/*For PrimeNG*/` из `inputgroupstyle.ts` (правила для обёрток Angular `p-button`, `p-inputwrapper`, `p-inputmask`, которых в Leptos нет); свойство `fluid`.

## Capabilities

### New Capabilities
- `inputgroup`: компоненты InputGroup и InputGroupAddon — структура, классы, проецируемое содержимое, `style`, `pt`, производительность.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-inputgroup` (`publish = false`); зависимости `prime-core`.
- Изменяются `Cargo.toml` workspace, `upstream.toml` (`inputgroup` в `theme.components`), `showcase`, `benches/*`, `e2e/tests`.
