# Proposal

## Why

Accordion группирует содержимое во вкладки и использует ту же анимацию сворачивания, что Panel и Fieldset, но состояние у него общее на группу (`value`, режим `multiple`) и есть клавиатурная навигация между заголовками. Это третий потребитель `prime-core::collapsible` и первый составной компонент (Accordion, AccordionPanel, AccordionHeader, AccordionContent) с контекстом между частями.

## What Changes

- Новый крейт `prime-accordion` с компонентами `Accordion`, `AccordionPanel`, `AccordionHeader`, `AccordionContent`.
- `Accordion`: двусторонний `value` (`RwSignal<Vec<String>>`, список значений активных панелей; в одиночном режиме 0 или 1 значение) и `default_value`, `multiple`, `select_on_focus`, `id`, `style_class`, события `on_open` и `on_close` (исходное событие и значение панели), `pt`.
- `AccordionPanel`: `value` (обязательный, строка), `disabled`, `style_class`. `AccordionHeader`: слот `toggle_icon_template` (получает `active`), `style_class`. `AccordionContent`: `style_class`.
- Классы и атрибуты как в PrimeNG 21.1.10 (`accordionstyle.ts`, `accordion.ts`): `p-accordion p-component`, `p-accordionpanel` (+ `p-accordionpanel-active`, `p-disabled`), `p-accordionheader`, `p-accordionheader-toggle-icon`, `p-accordioncontent`, `p-accordioncontent-wrapper`, `p-accordioncontent-content`, `data-p-active`, `data-p-disabled`, `data-p`.
- Доступность: заголовок `role="button"`, `aria-expanded`, `aria-controls`, `aria-disabled`, `tabindex`; содержимое `role="region"`, `aria-labelledby`. Клавиатура: Enter/Space переключают, ArrowDown/ArrowUp/Home/End перемещают фокус между заголовками, пропуская отключённые.
- Анимация раскрытия — классами `p-collapsible-*` через общую логику; для этого в `prime-core::collapsible` добавляется привязка области к производному состоянию (`bind_region_with`), существующее поведение не меняется.
- `cargo xtask gen-theme` дополняется блоком `/* For PrimeNG */` из `<name>style.ts` (нужен Accordion: `.p-accordioncontent .p-motion { display: grid; grid-template-rows: 1fr }`, расположение иконки); CSS темы `accordion`.
- Страница `/accordion` (`basic`, `multiple`, `disabled`, `controlled`, `dynamic`, `template`), e2e, бенчмарки, бюджеты, `docs/accordion.md`.
- **Не входит:** `expandIcon`/`collapseIcon` как CSS-классы иконок (заменены слотом `toggle_icon_template`), ripple, `BlockableUI`, `motionOptions`/`transitionOptions`, числовые значения (значения — строки), демонстрация `accessibility`.

## Capabilities

### New Capabilities
- `accordion`: компоненты Accordion — структура, классы, значение и режим `multiple`, события, ARIA и клавиатура, анимация, слоты, `pt`, производительность.

### Modified Capabilities
- `collapsible`: добавляется привязка сворачиваемой области к производному (только читаемому) состоянию; текущие требования не меняются.

## Impact

- Новый крейт `crates/prime-accordion` (`publish = false`); зависимости `prime-core`, `prime-icons` (`chevrondown`, `chevronup`), `web-sys`.
- `prime-core::collapsible`: новая функция; `xtask`: чтение блока `For PrimeNG`.
- Изменяются `Cargo.toml` workspace, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, браузерный job CI.
