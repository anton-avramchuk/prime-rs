# ProgressSpinner

Порт `p-progressspinner` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-progressspinner::ProgressSpinner` | `packages/primeng/src/progressspinner/progressspinner.ts`, `progressspinner/style/progressspinnerstyle.ts` |
| CSS | `@primeuix/styles/progressspinner` и пресет Aura (`cargo xtask gen-theme`) |
| Демонстрации | `apps/showcase/doc/progressspinner/*-doc.ts` (`basic`, `custom`) |

Спека: `openspec/specs/progressspinner`.

## Что совпадает с PrimeNG

- Корень `div.p-progressspinner` с `role="progressbar"`, `aria-busy="true"` и `aria-label`; внутри `svg.p-progressspinner-spin` (`viewBox="25 25 50 50"`, `animation-duration` в `style`) и `circle.p-progressspinner-circle` (`cx="50" cy="50" r="20"`, `fill`, `stroke-width`, `stroke-miterlimit="10"`).
- Значения по умолчанию: `stroke_width="2"`, `fill="none"`, `animation_duration="2s"`.

## Отличия

| Отличие | Причина |
|---|---|
| Корень — `<div>`, а не хост `<p-progressspinner>` | Хост в Angular — деталь фреймворка. |
| Нет синонимов селектора и `BlockableUI` | Устаревшие имена и интерфейс Angular. |
| `style_class` вместо `styleClass` | Устаревшее имя, в PrimeNG заменено на `class`. |

## Производительность

SSR: 14 аллокаций на экземпляр (бюджет 16); вклад в сырой wasm +95 КБ (бюджет 105 КБ).
Шаблон — как у Toolbar (`docs/toolbar.md`).
