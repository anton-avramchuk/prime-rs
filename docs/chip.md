# Chip

Порт `p-chip` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-chip::Chip` | `packages/primeng/src/chip/chip.ts`, `chip/style/chipstyle.ts` |
| CSS | `@primeuix/styles/chip` и пресет Aura (`cargo xtask gen-theme`) |
| Демонстрации | `apps/showcase/doc/chip/*-doc.ts` (`basic`, `icon`, `image`, `template`) |

Спека: `openspec/specs/chip`.

## Что совпадает с PrimeNG

- Классы `p-chip p-component`, `p-disabled`, `p-chip-image`, `p-chip-icon`, `p-chip-label`, `p-chip-remove-icon`; `data-p="removable"`.
- Порядок содержимого: `children`, изображение (иначе иконка), метка, элемент удаления.
- Удаление по клику и по Enter и Backspace, скрытие `display: none` на корне, `on_remove`, `on_image_error`, `tabindex` -1 при `disabled`,
  `aria-label` из локали (`aria.removeLabel`).

## Отличия

| Отличие | Причина |
|---|---|
| Корень — `<div>`, а не хост `<p-chip>` | Хост в Angular — деталь фреймворка. |
| Элемент удаления — `span[role=button]` с `TimesCircleIcon` внутри, а не `<svg role=button>` | Иконка `prime-icons` выставляет собственную a11y-семантику; роль и обработчики должны быть на одном элементе. |
| Иконка и иконка удаления — `ViewFn`, а не строки классов PrimeIcons | Шрифта нет. |
| `on_remove` получает `web_sys::Event` (`MouseEvent` или `KeyboardEvent`) | В PrimeNG тип `MouseEvent`, но при клавиатуре приходит `KeyboardEvent`. |
| Нет `chipProps` | В Rust это обычные свойства. |
| Удаляемый Chip всегда идёт реактивным путём | Корню нужно внутреннее состояние видимости. |
| Иконки брендов демонстрации `icon` заменены иконками `prime-icons` | В 53 иконках PrimeNG нет Apple, Facebook, Google, Microsoft. |

## Производительность

SSR, аллокаций на экземпляр: обычный Chip 13 (бюджет 16), удаляемый 41 (бюджет 50; реактивная ветвь, SVG иконки удаления, локаль); вклад в сырой wasm +179 КБ
(бюджет 230 КБ; удаляемый путь тянет обработчики, иконку и локаль). Шаблон — как у Button (`docs/button.md`).
