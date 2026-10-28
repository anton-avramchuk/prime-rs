# Proposal

## Why

Avatar — следующий компонент волны 1 (после Button, Badge, Tag, Chip). Он представляет человека меткой, иконкой или изображением и ложится на отработанный шаблон (`Prop`, статическая и реактивная ветви, паритет-тесты) без внутреннего состояния; дополнительно он добавляет `Prop` для перечислений с неявными значениями по умолчанию (`size = normal`, `shape = square`), которые попадают в `data-p`.

## What Changes

- Новый крейт `prime-avatar` с компонентом `Avatar`: `label`, `icon`, `image`, `size` (`normal`, `large`, `xlarge`), `shape` (`square`, `circle`), `style`, `style_class`, `aria_label`, `aria_labelledby`, `children`, `pt`, событие `on_image_error`; классы `p-avatar*` и `data-p` как в PrimeNG 21.1.10.
- Страница `/avatar` в showcase (`label`, `icon`, `image`, `shape`, `size`, `template`), e2e, бенчмарки и бюджеты.
- **Не входит:** `AvatarGroup` (отдельный компонент PrimeNG, отдельный change), шрифтовые иконки, `unstyled`-режим.

## Capabilities

### New Capabilities
- `avatar`: компонент Avatar — метка, иконка, изображение, размер, форма, доступность, `pt`, производительность.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-avatar` (`publish = false`); зависимости `prime-core`, `web-sys`.
- Изменяются `showcase`, `benches/render`, `benches/wasm-size`, `benches/budgets.toml`, `benches/alloc-budgets.toml`, `e2e/tests`.
