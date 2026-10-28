# Proposal

## Why

Skeleton — заглушка загрузки; первый компонент волны 2. Он чисто отображающий (без поведения), поэтому служит проверкой шаблона на компоненте, чья разметка целиком определяется стилем (`width`, `height`, `border-radius`), и открывает волну 2 вместе с Textarea.

## What Changes

- Новый крейт `prime-skeleton` с компонентом `Skeleton`: `shape` (`rectangle` по умолчанию, `circle`), `animation` (`wave` по умолчанию, `none`), `border_radius`, `size`, `width` (по умолчанию `100%`), `height` (по умолчанию `1rem`), `style_class`, `pt` (`root`).
- Разметка и классы как в PrimeNG 21.1.10: `<div class="p-skeleton p-component" aria-hidden="true">` с модификаторами `p-skeleton-circle`, `p-skeleton-animation-none`, `data-p` (форма) и встроенным стилем `position: relative; width: …; height: …; border-radius: …` (при `size` ширина и высота равны `size`).
- Страница `/skeleton` (`shapes`, `card`, `list`), e2e, бенчмарки, бюджеты. CSS темы `skeleton` и `textarea` сгенерирован `cargo xtask gen-theme` (список компонентов в `upstream.toml`).
- **Не входит:** демонстрация `datatable` (нужна таблица), Tailwind-утилиты демонстраций заменены инлайн-стилями.

## Capabilities

### New Capabilities
- `skeleton`: компонент Skeleton — разметка, классы, стиль, `pt`, производительность.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-skeleton` (`publish = false`); зависимость только `prime-core`.
- Изменяются `upstream.toml`, `crates/prime-theme` (сгенерированные CSS и features), `showcase`, `benches/*`, `e2e/tests`.
