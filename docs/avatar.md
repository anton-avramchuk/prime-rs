# Avatar

Порт `p-avatar` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-avatar::Avatar` | `packages/primeng/src/avatar/avatar.ts`, `avatar/style/avatarstyle.ts` |
| CSS | `@primeuix/styles/avatar` и пресет Aura (`cargo xtask gen-theme`) |
| Демонстрации | `apps/showcase/doc/avatar/*-doc.ts` (`label`, `icon`, `image`, `shape`, `size`, `template`) |

Спека: `openspec/specs/avatar`.

## Что совпадает с PrimeNG

- Классы `p-avatar p-component`, `p-avatar-image`, `p-avatar-circle`, `p-avatar-lg`, `p-avatar-xl`, `p-avatar-label`, `p-avatar-icon`.
- `data-p` всегда содержит форму и размер, включая значения по умолчанию (`square normal`), и стоит на корне, метке, иконке и изображении.
- Приоритет блоков: метка, иконка, изображение; `aria-label` и `aria-labelledby` на корне, `aria-label` также на изображении.
- `p-avatar-image` на корне определяется по заданному `image` (как `image != null`).

## Отличия

| Отличие | Причина |
|---|---|
| Корень — `<div>`, а не хост `<p-avatar>` | Хост в Angular — деталь фреймворка. |
| Иконка — `ViewFn`, а не строка класса PrimeIcons | Шрифта нет. |
| Иконка `pi-user` в демонстрации заменена `HomeIcon` | В 53 иконках PrimeNG нет пользователя. |
| Нет `AvatarGroup` | Отдельный компонент PrimeNG, отдельный change. |

## Производительность

SSR: 15 аллокаций на экземпляр (бюджет 18); вклад в сырой wasm +137 КБ (бюджет 180 КБ, из них почти всё — общая реактивная среда Leptos).
Шаблон — как у Button (`docs/button.md`).
