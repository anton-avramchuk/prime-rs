# Design

## Context

Требования — в `specs/avatargroup`. Исходники PrimeNG 21.1.10: `packages/primeng/src/avatargroup/avatargroup.ts`, `avatargroup/style/avatargroupstyle.ts`, демонстрация `apps/showcase/doc/avatar/avatargroup-doc.ts`. Компонент — контейнер: шаблон `<ng-content>`, класс `p-avatar-group p-component`, `styleClass`, `style`. Шаблон крейта — `docs/divider.md`, `docs/card.md`.

## Goals / Non-Goals

**Goals:** разметка и классы как в PrimeNG; `pt.root`; статическая ветвь.
**Non-Goals:** сворачивание лишних аватаров, зависимость от `prime-avatar` (контейнер принимает любые дочерние элементы).

## Decisions

### D1. Корень — `<div>`
Вместо хоста `<p-avatar-group>`; CSS темы нацелен на `.p-avatar-group`.

### D2. Ветви
Статическая — при `all_static!(style, style_class)`: готовые значения; иначе замыкания. Разметка одна, в `macro_rules!`.

### D3. Тесты
SSR: структура, классы, дети, `style`/`style_class`, `pt`, паритет ветвей. Браузерных тестов нет: поведения нет.

## Risks / Trade-offs

- [Перекрытие аватаров целиком в CSS темы] → проверяется e2e (отрицательный отступ у второго аватара).

## Migration Plan

Миграции нет: новый крейт. Откат — `git revert`.

## Open Questions

Нет.
