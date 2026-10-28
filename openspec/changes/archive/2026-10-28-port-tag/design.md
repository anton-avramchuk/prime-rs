# Design

## Context

Требования — в `specs/tag`. Исходники PrimeNG 21.1.10: `packages/primeng/src/tag/tag.ts`, `tag/style/tagstyle.ts`; демонстрации — `apps/showcase/doc/tag`. Tag — хост `<p-tag>` с классами на нём, внутри `ng-content`, иконка (`span.p-tag-icon`, если задана) и метка (`span.p-tag-label` — всегда, даже пустая). Классы корня: `p-tag p-component`, серьёзность, `p-tag-rounded`; `data-p`: `rounded`, серьёзность.

Шаблон из `docs/button.md`: свойства `Prop`/`OptProp`, без `Memo`, свойства в одном `Arc<Props>`, `all_static!` выбирает статическую или реактивную ветвь, разметка один раз в `macro_rules!`, тесты паритета.

## Goals / Non-Goals

**Goals:** Tag, совпадающий с PrimeNG по классам, `data-p` и порядку содержимого; ветви без `RenderEffect` для статических свойств; бюджеты.
**Non-Goals:** шрифтовые иконки, `unstyled`-режим.

## Decisions

### D1. Корень — `<span>`
Как у Badge: хост `<p-tag>` — деталь Angular, CSS привязан к классам.

### D2. Иконка и содержимое
`icon: Option<ViewFn>` в `<span class="p-tag-icon">` (всегда оболочка, как в PrimeNG при шаблонной иконке); `children: Option<Children>` первыми. Метка выводится всегда (в PrimeNG так же).

### D3. Серьёзность
Собственный enum `TagSeverity` (`Secondary`, `Info`, `Success`, `Warn`, `Danger`, `Contrast`) — как у `BadgeSeverity`, но отдельный тип крейта, чтобы компоненты не зависели друг от друга.

### D4. Состояние и ветви
Чистые функции `write_classes`/`data_p` над заимствованным `TagState`; ветвь по `all_static!(value, severity, rounded, style, style_class)`; блоки иконки и метки считаются функциями `icon_of`/`label_of`.

## Risks / Trade-offs

- [Дублирование enum серьёзности с Badge] → осознанно: компоненты независимы; общий тип может появиться в `prime-core`, когда их станет больше трёх.
- [Расхождение ветвей] → параметризованный тест паритета.

## Migration Plan

Миграции нет: новый крейт. Откат — `git revert`.

## Open Questions

Нет.
