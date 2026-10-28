# Design

## Context

Требования — в `specs/card`. Исходники PrimeNG 21.1.10: `packages/primeng/src/card/card.ts`, `card/style/cardstyle.ts` (классы `root`, `header`, `body`, `title`, `subtitle`, `content`, `footer`; ещё `caption`, который в шаблоне не используется), демонстрации `apps/showcase/doc/card` (`basic`, `advanced`). Шаблон — `docs/button.md`.

## Goals / Non-Goals

**Goals:** Card с теми же блоками, порядком и приоритетом слотов; ветви без `RenderEffect` для статики; бюджеты.
**Non-Goals:** `BlockableUI`, `unstyled`, `p-header` и `p-footer` как фасеты (в Leptos это слоты `ViewFn`).

## Decisions

### D1. Слоты — `Option<ViewFn>`
Четыре слота (`header_template`, `title_template`, `subtitle_template`, `footer_template`) — `Option<ViewFn>`, содержимое — `children`. Имена повторяют шаблоны PrimeNG (`#header`, `#title`, `#subtitle`, `#footer`).

### D2. Статичность слотов
`ViewFn` и `children` статичны по природе (замыкания отрисовки), поэтому ветвь выбирается по `all_static!(header, subheader, style, style_class)`; блоки считаются функциями `header_block`, `title_block`, `subtitle_block`, `footer_block` с общим результатом `Option<AnyView>`.

### D3. Корень — `<div>`
Хост `<p-card>` — деталь Angular; дополнительный `display: block` из `cardstyle.ts` для `<div>` не нужен.

## Risks / Trade-offs

- [Слоты вычисляются один раз, а не по сигналам] → слоты принимают `ViewFn`, который сам может быть реактивным внутри; реактивны только текстовые свойства.
- [Расхождение ветвей] → параметризованный тест паритета.

## Migration Plan

Миграции нет: новый крейт. Откат — `git revert`.

## Open Questions

Нет.
