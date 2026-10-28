# Proposal

## Why

Toolbar — простой группирующий контейнер для кнопок и другого содержимого с тремя областями (начало, центр, конец). Он не требует новой общей логики, имеет роль `toolbar` и подходит как следующий небольшой компонент после составных Panel, Fieldset и Accordion.

## What Changes

- Новый крейт `prime-toolbar` с компонентом `Toolbar`: слоты `start_template`, `center_template`, `end_template` (`ViewFn`), `aria_labelledby`, `style`, `style_class`, `children` (проецируемое содержимое перед областями), `pt`.
- Разметка как в PrimeNG 21.1.10 (`toolbar.ts`, `toolbarstyle.ts`): корень `<div class="p-toolbar p-component" role="toolbar">`, области `p-toolbar-start`, `p-toolbar-center`, `p-toolbar-end` выводятся только при заданном слоте и в таком порядке.
- Страница `/toolbar` (`basic`, `custom`), e2e, бенчмарки, бюджеты, `docs/toolbar.md`, CSS темы `toolbar`.
- **Не входит:** `BlockableUI`; старые имена шаблонов `left` и `right` (в PrimeNG они синонимы `start` и `end`); демонстрации с SplitButton, IconField и InputIcon, которых ещё нет в порте (`basic` собирается из Button и InputText).

## Capabilities

### New Capabilities
- `toolbar`: компонент Toolbar — структура, классы, роль и `aria-labelledby`, три области со слотами, `style`, `pt`, производительность.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-toolbar` (`publish = false`); зависимости `prime-core`.
- Изменяются `Cargo.toml` workspace, `upstream.toml` (`toolbar` в `theme.components`), `showcase`, `benches/*`, `e2e/tests`.
