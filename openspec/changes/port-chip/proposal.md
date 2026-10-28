# Proposal

## Why

Chip — следующий компонент волны 1 (Tag портирован). В отличие от Badge и Tag, он интерактивный: удаляемый (`removable`), реагирует на клавиатуру, имеет события и состояние «скрыт после удаления», локализуемую подпись, изображение с обработкой ошибки загрузки. Это первый компонент, где статическая ветвь отрисовки должна уживаться с внутренним состоянием, поэтому он проверяет шаблон `Prop` + две ветви на поведении, а не только на разметке.

## What Changes

- Новый крейт `prime-chip` с компонентом `Chip`: `label`, `icon`, `image`, `alt`, `disabled`, `removable`, `remove_icon`, `style`, `style_class`, `children`, `pt`, события `on_remove`, `on_image_error`; классы `p-chip*` и `data-p` как в PrimeNG 21.1.10.
- Удаление по клику и по клавишам Enter и Backspace; после удаления Chip скрывается (`display: none`), как в PrimeNG; `aria-label` кнопки удаления берётся из `PrimeConfig.locale` (`aria.removeLabel`).
- Статическая ветвь отрисовки используется, когда все свойства статические и Chip не удаляемый (удаляемому нужно внутреннее состояние видимости, поэтому он идёт реактивным путём).
- Страница `/chip` в showcase (`basic`, `icon`, `image`, `template`), e2e, бенчмарки и бюджеты.
- **Не входит:** `chipProps`, шрифтовые иконки, `unstyled`-режим.

## Capabilities

### New Capabilities
- `chip`: компонент Chip — метка, иконка, изображение, удаление, события, доступность, `pt`, производительность.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-chip` (`publish = false`); зависит от `prime-core` и `prime-icons` (feature `timescircle`).
- Изменяются `showcase`, `benches/render`, `benches/wasm-size`, `benches/budgets.toml`, `benches/alloc-budgets.toml`, `e2e/tests`.
