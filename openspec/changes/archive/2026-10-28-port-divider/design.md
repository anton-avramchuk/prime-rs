# Design

## Context

Требования — в `specs/divider`. Исходники PrimeNG 21.1.10: `packages/primeng/src/divider/divider.ts`, `divider/style/dividerstyle.ts` (классы и `inlineStyles`), демонстрации `apps/showcase/doc/divider`. Хост `<p-divider>` несёт классы, `role=separator`, `aria-orientation`, inline-стиль и `data-p`; внутри всегда `<div class="p-divider-content">`. `data-p` строится из `align`, `layout`, `type` (порядок именно такой, `align` только если задан). Шаблон — `docs/button.md`.

## Goals / Non-Goals

**Goals:** Divider с теми же классами, стилем и атрибутами; статическая ветвь без `RenderEffect`; бюджеты.
**Non-Goals:** `unstyled`.

## Decisions

### D1. Корень — `<div>`
Хост `<p-divider>` — деталь Angular.

### D2. Имя свойства `type`
В Rust свойство называется `divider_type` (как `button_type` у Button): `type` — ключевое слово. Атрибута в DOM нет, поэтому на разметку это не влияет.

### D3. Перечисления
`DividerLayout` (`Horizontal`, `Vertical`), `DividerType` (`Solid`, `Dashed`, `Dotted`), `DividerAlign` (`Left`, `Center`, `Right`, `Top`, `Bottom`); у первых двух `Default` (`Horizontal`, `Solid`), `align` — `OptProp`.

### D4. Классы и стиль — чистые функции
`write_classes(cn, &state)` и `align_style(&state) -> Option<(&str, &str)>` (имя свойства и значение) покрываются таблицами юнит-тестов по `dividerstyle.ts` (все 2 × 5 сочетаний); `data_p` — отдельная функция.

### D5. Ветви
`all_static!(layout, divider_type, align, style, style_class)`; блок содержимого — одно значение `Option<AnyView>` (`children`), общее для ветвей.

## Risks / Trade-offs

- [Таблица классов выравнивания легко ошибиться] → таблица тестов на все сочетания и сравнение скриншотов с PrimeNG.

## Migration Plan

Миграции нет: новый крейт. Откат — `git revert`.

## Open Questions

Нет.
