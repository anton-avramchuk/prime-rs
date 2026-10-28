# Proposal

## Why

Divider — следующий компонент волны 1 (после Button, Badge, Tag, Chip, Avatar). Это самый «чистый» по логике компонент: разметка зависит только от свойств (направление, тип линии, выравнивание содержимого), без событий и состояния, но с нетривиальной таблицей классов выравнивания и inline-стилем `justify-content`/`align-items`. Он хорошо проверяет шаблон на условной логике классов и стилей.

## What Changes

- Новый крейт `prime-divider` с компонентом `Divider`: `layout` (`horizontal`, `vertical`), `type` (`solid`, `dashed`, `dotted`), `align` (`left`, `center`, `right`, `top`, `bottom`), `style`, `style_class`, `children`, `pt`; классы `p-divider*`, `role="separator"`, `aria-orientation`, `data-p` и inline-стиль выравнивания как в PrimeNG 21.1.10.
- Страница `/divider` в showcase (`basic`, `content`, `type`, `vertical`, `login`), e2e, бенчмарки и бюджеты.
- **Не входит:** `unstyled`-режим.

## Capabilities

### New Capabilities
- `divider`: компонент Divider — направление, тип, выравнивание, содержимое, доступность, `pt`, производительность.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-divider` (`publish = false`); зависимость `prime-core`.
- Изменяются `showcase`, `benches/render`, `benches/wasm-size`, `benches/budgets.toml`, `benches/alloc-budgets.toml`, `e2e/tests`.
