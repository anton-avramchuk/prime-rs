# AvatarGroup

Порт `p-avatar-group` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-avatargroup::AvatarGroup` | `packages/primeng/src/avatargroup/avatargroup.ts`, `avatargroup/style/avatargroupstyle.ts` |
| CSS | `.p-avatar-group` входит в `@primeuix/styles/avatar` (feature `avatar` в `prime-theme`) |
| Демонстрации | `apps/showcase/doc/avatar/avatargroup-doc.ts` (на странице `/avatar`, демонстрация `avatargroup`) |

Спека: `openspec/specs/avatargroup`.

## Что совпадает с PrimeNG

- Классы корня `p-avatar-group p-component`, дочернее содержимое без обёрток, `style` и `style_class`.
- Перекрытие и выравнивание аватаров целиком в CSS темы.

## Отличия

| Отличие | Причина |
|---|---|
| Корень — `<div>`, а не хост `<p-avatar-group>` | Хост в Angular — деталь фреймворка. |
| Нет зависимости от `prime-avatar` | Контейнер принимает любые дочерние элементы; в PrimeNG связь тоже только через CSS. |

## Производительность

SSR: 6 аллокаций на экземпляр (бюджет 8); вклад в сырой wasm +64 КБ (бюджет 80 КБ).
Шаблон — как у Divider (`docs/divider.md`).
