# Design

## Context

Требования — в `specs/message`. Исходники PrimeNG 21.1.10: `packages/primeng/src/message/message.ts`, `message/style/messagestyle.ts`, демонстрации `apps/showcase/doc/message`. Шаблон компонента — `docs/toggleswitch.md`, `docs/checkbox.md`.

## Goals / Non-Goals

**Goals:** разметка, классы и `data-p` как в PrimeNG; закрытие и `life`; слоты; статическая ветвь для неинтерактивного сообщения.
**Non-Goals:** `escape=false`, анимации `@primeuix/motion`, ripple на кнопке закрытия, `MessageService`/`Toast`.

## Decisions

### D1. Корень — `<div>`
Вместо хоста `<p-message>`; `role="alert"`, `aria-live="polite"`, `data-p` — на корне.

### D2. Текст и содержимое
Как в PrimeNG: при непустом `text` выводится `span.p-message-text` с текстом, затем всегда второй `span.p-message-text` с дочерним содержимым (пустой, если детей нет). `container_template` заменяет оба и получает колбэк закрытия.

### D3. Закрытие и `life`
Состояние `visible` создаётся только если сообщение закрываемое или имеет `life`. `close` скрывает сообщение и вызывает `on_close`. Скрытие — класс `p-message-leave-active` и `display: none` (PrimeNG убирает элемент после анимации). `life` запускает таймер `set_timeout` только в браузере.

### D4. Иконки
`icon` — CSS-класс (`<i class="p-message-icon …">`); `icon_template` выводится без обёртки (как в PrimeNG); по умолчанию кнопка закрытия содержит `TimesIcon` с классом `p-message-close-icon`; `aria-label` берётся из локали (`aria.close`).

### D5. Ветви
Статическая — при `all_static!`, `!closable` и без `life`; иначе реактивная. Разметка одна, в `macro_rules!`; части (иконка, текст, кнопка) строятся функциями над свойствами.

### D6. Тесты
SSR: структура, классы, `data-p`, роль, текст и дети, слоты, `pt`, паритет. Браузер: закрытие по клику, `on_close`, автоскрытие по `life`.

## Risks / Trade-offs

- [Без анимации скрытие мгновенное] → зафиксировано в `docs/message.md`.

## Migration Plan

Миграции нет: новый крейт. Откат — `git revert`.

## Open Questions

Нет.
