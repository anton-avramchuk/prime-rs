# Skeleton

Порт `p-skeleton` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-skeleton::Skeleton` | `packages/primeng/src/skeleton/skeleton.ts`, `skeleton/style/skeletonstyle.ts` |
| CSS | `@primeuix/styles/skeleton` и пресет Aura (`cargo xtask gen-theme`) |
| Демонстрации | `apps/showcase/doc/skeleton/*-doc.ts` (`shapes`, `card`, `list`) |

Спека: `openspec/specs/skeleton`.

## Что совпадает с PrimeNG

- Корень `aria-hidden="true"`, классы `p-skeleton p-component`, `p-skeleton-circle`, `p-skeleton-animation-none`, `data-p` — имя формы.
- Встроенный стиль `position: relative`, `width` (`100%`), `height` (`1rem`), `border-radius`; `size` задаёт ширину и высоту разом.

## Отличия

| Отличие | Причина |
|---|---|
| Корень — `<div>`, а не хост `<p-skeleton>` | Хост в Angular — деталь фреймворка. |
| `shape` и `animation` — перечисления `SkeletonShape`, `SkeletonAnimation` | В PrimeNG это произвольные строки; допустимые значения те же. |
| Демонстрация `datatable` не перенесена, утилиты Tailwind заменены инлайн-стилями | Нужна таблица; Tailwind не поставляется. |
| Встроенный стиль пользователя только через `pt.root` | В PrimeNG у Skeleton нет входа `style`. |

## Производительность

SSR: 6 аллокаций на экземпляр (бюджет 8); вклад в сырой wasm +86 КБ (бюджет 105 КБ).
Шаблон — как у AvatarGroup (`docs/avatargroup.md`).
