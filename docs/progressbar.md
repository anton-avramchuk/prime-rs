# ProgressBar

Порт `p-progressbar` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-progressbar::ProgressBar` | `packages/primeng/src/progressbar/progressbar.ts`, `progressbar/style/progressbarstyle.ts` |
| CSS | `@primeuix/styles/progressbar` и пресет Aura (`cargo xtask gen-theme`) |
| Демонстрации | `apps/showcase/doc/progressbar/*-doc.ts` (`basic`, `dynamic`, `indeterminate`, `template`) |

Спека: `openspec/specs/progressbar`.

## Что совпадает с PrimeNG

- Корень с `role="progressbar"`, `aria-valuemin="0"`, `aria-valuemax="100"`, `aria-valuenow`, `aria-level`, классы `p-progressbar p-component p-progressbar-determinate|indeterminate`, `data-p` на корне и частях.
- Определённый режим: `div.p-progressbar-value` (`width: <value>%; display: flex`, при `color` — `background`) с `div.p-progressbar-label`, внутри значение и единица (скрыты при нуле и отсутствии значения).
- Неопределённый режим: только `div.p-progressbar-value`.
- Слот `content_template` получает значение (`content` PrimeNG).

## Отличия

| Отличие | Причина |
|---|---|
| Корень — `<div>`, а не хост `<p-progressbar>` | Хост в Angular — деталь фреймворка. |
| `aria-valuenow` и `aria-level` отсутствуют без значения | PrimeNG выводит `aria-level="undefined%"`. |
| `value` — `f64`, выводится через `Display` | Целые числа выводятся без дробной части, как в JavaScript. |
| `content_template: Callback<Option<f64>, AnyView>` | Слоты Leptos. |
| Демонстрация `dynamic` растёт на 10 каждые 500 мс | Детерминированный e2e; PrimeNG использует случайный шаг и `MessageService`. |
| Нет браузерных тестов компонента | Клиентского поведения нет, достаточно SSR-тестов и e2e. |

## Производительность

SSR: 36 аллокаций на экземпляр (бюджет 44); вклад в сырой wasm +136 КБ (бюджет 170 КБ).
Шаблон — как у ToggleSwitch (`docs/toggleswitch.md`).
