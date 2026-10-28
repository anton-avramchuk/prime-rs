# Divider

Порт `p-divider` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-divider::Divider` | `packages/primeng/src/divider/divider.ts`, `divider/style/dividerstyle.ts` |
| CSS | `@primeuix/styles/divider` и пресет Aura (`cargo xtask gen-theme`) |
| Демонстрации | `apps/showcase/doc/divider/*-doc.ts` (`basic`, `content`, `type`, `vertical`) |

Спека: `openspec/specs/divider`.

## Что совпадает с PrimeNG

- Классы `p-divider p-component`, `p-divider-<layout>`, `p-divider-<type>` и класс выравнивания по той же таблице, что в `dividerstyle.ts`
  (все сочетания проверены таблицей юнит-тестов, включая недопустимые: у них класса выравнивания нет).
- `role="separator"`, `aria-orientation`, inline-стиль `justify-content` или `align-items`, `data-p` (`align`, `layout`, тип).
- Блок `div.p-divider-content` присутствует всегда.

## Отличия

| Отличие | Причина |
|---|---|
| Корень — `<div>`, а не хост `<p-divider>` | Хост в Angular — деталь фреймворка. |
| Свойство `type` называется `divider_type` | `type` — ключевое слово Rust. |
| Демонстрация `login` не перенесена | Ей нужен InputText, который ещё не портирован; будет добавлена вместе с ним. |
| Тексты-заполнители в демонстрациях сокращены | Размер страницы. |

## Производительность

SSR: 22 аллокации на экземпляр (бюджет 27); вклад в сырой wasm +95 КБ (бюджет 130 КБ). Шаблон — как у Button (`docs/button.md`).
