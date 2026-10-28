# Proposal

## Why

Fieldset — группирующий контейнер с `<legend>` и необязательным сворачиванием, парный к Panel. Общая логика сворачивания (`prime-core::collapsible`) уже есть, поэтому порт сводится к разметке `<fieldset>`/`<legend>` и кнопке в легенде.

## What Changes

- Новый крейт `prime-fieldset` с компонентом `Fieldset`: `legend`, `toggleable`, двусторонний `collapsed` (`RwSignal<bool>`) и `default_collapsed`, `id`, `style`, `style_class`, слоты `header_template`, `expand_icon_template`, `collapse_icon_template`, события `on_before_toggle`, `on_after_toggle`, `pt`.
- Классы как в PrimeNG 21.1.10 (`fieldsetstyle.ts`): `p-fieldset p-component`, `p-fieldset-toggleable`, `p-fieldset-collapsed`, `p-fieldset-legend`, `p-fieldset-legend-label`, `p-fieldset-toggle-button`, `p-fieldset-toggle-icon`, `p-fieldset-content-container`, `p-fieldset-content-wrapper`, `p-fieldset-content`; `data-p="toggleable"`.
- При `toggleable` легенда содержит `<button>` с `aria-controls`, `aria-expanded`, `aria-label` (текст легенды); область содержимого — `role="region"`, `aria-labelledby`, `aria-hidden`; анимация и `tabindex` потомков — через `prime-core::collapsible`.
- Страница `/fieldset` (`basic`, `toggleable`, `template`), e2e, бенчмарки, бюджеты, `docs/fieldset.md`, CSS темы `fieldset`.
- **Не входит:** `BlockableUI`, `motionOptions`/`transitionOptions`, фасет `<p-header>`; изменения `prime-core::collapsible` (используется как есть).

## Capabilities

### New Capabilities
- `fieldset`: компонент Fieldset — структура и классы, сворачивание и двусторонний `collapsed`, ARIA, анимация и `on_after_toggle`, слоты, `pt`, производительность.

### Modified Capabilities
<!-- Нет: collapsible переиспользуется без изменения требований. -->

## Impact

- Новый крейт `crates/prime-fieldset` (`publish = false`); зависимости `prime-core`, `prime-icons` (`minus`, `plus`), `web-sys`.
- Изменяются `Cargo.toml` workspace, `upstream.toml` (`fieldset` в `theme.components`), `showcase`, `benches/*`, `e2e/tests`, браузерный job CI.
