# Design

## Context

Требования — в `specs/skeleton`. Исходники PrimeNG 21.1.10: `packages/primeng/src/skeleton/skeleton.ts`, `skeleton/style/skeletonstyle.ts`, демонстрации `apps/showcase/doc/skeleton`. Шаблон крейта — `docs/progressbar.md`, `docs/avatargroup.md`.

## Decisions

### D1. Корень — `<div>`
Вместо хоста `<p-skeleton>`; `aria-hidden="true"` и `data-p` — на корне.

### D2. Стиль
Как `containerStyle` PrimeNG: `position: relative`, затем `width`/`height` (при заданном `size` оба равны ему) и `border-radius` (только при заданном `border_radius`). Строка собирается одним `String` через `write!`; завершающую `;` добавляет Leptos при выводе атрибута.

### D3. Классы и `data-p`
`p-skeleton p-component`, `p-skeleton-circle` при `shape = circle`, `p-skeleton-animation-none` при `animation = none`; `data-p` — имя формы (`rectangle`/`circle`).

### D4. Ветви
Статическая — при `all_static!`; иначе реактивная. Разметка — `macro_rules!`; `Props`/`Pt` без `Arc` в статической ветви.

### D5. Тесты
SSR: структура, стиль, классы, `size`, `pt`, паритет ветвей. Браузерных тестов нет: поведения нет.

## Risks / Trade-offs

- [Строки `shape`/`animation` в PrimeNG — произвольные] → типизированные перечисления с теми же значениями.

## Migration Plan

Миграции нет: новый крейт. Откат — `git revert`.

## Open Questions

Нет.
