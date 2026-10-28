# Card

Порт `p-card` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-card::Card` | `packages/primeng/src/card/card.ts`, `card/style/cardstyle.ts` |
| CSS | `@primeuix/styles/card` и пресет Aura (`cargo xtask gen-theme`) |
| Демонстрации | `apps/showcase/doc/card/*-doc.ts` (`basic`, `advanced`) |

Спека: `openspec/specs/card`.

## Что совпадает с PrimeNG

- Классы `p-card p-component`, `p-card-header`, `p-card-body`, `p-card-title`, `p-card-subtitle`, `p-card-content`, `p-card-footer`.
- Структура: шапка (только при слоте) вне тела; в теле заголовок, подзаголовок (только при тексте или слоте), содержимое (всегда), подвал (только при слоте).
- Слот приоритетнее текстового свойства (`header_template` над `header` и т. д.).

## Отличия

| Отличие | Причина |
|---|---|
| Корень — `<div>`, а не хост `<p-card>` | Хост в Angular — деталь фреймворка (`display: block` для `<div>` не нужен). |
| Слоты — `ViewFn` (`header_template`, `title_template`, `subtitle_template`, `footer_template`), содержимое — `children` | Вместо шаблонов `#header`, `#title`, … и фасетов `p-header`/`p-footer`. |
| Нет `BlockableUI` | Интерфейс блокировки Angular. |
| Класс `p-card-caption` не используется | В шаблоне PrimeNG он тоже не применяется. |

## Производительность

SSR: 29 аллокаций на экземпляр с заголовком и подзаголовком (бюджет 35); вклад в сырой wasm +96 КБ (бюджет 130 КБ).
Шаблон — как у Button (`docs/button.md`).
