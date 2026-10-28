# Button и Badge

Порт `p-button` и `p-badge` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-button::Button` | `packages/primeng/src/button/button.ts` (компонент `Button`, `p-button`), `button/style/buttonstyle.ts` |
| `prime-badge::Badge` | `packages/primeng/src/badge/badge.ts` (компонент `Badge`, `p-badge`), `badge/style/badgestyle.ts` |
| CSS | `@primeuix/styles/button`, `@primeuix/styles/badge` и пресет Aura (`cargo xtask gen-theme`) |
| Демонстрации | `apps/showcase/doc/button/*-doc.ts`, `apps/showcase/doc/badge/*-doc.ts` |

Спеки: `openspec/specs/button`, `openspec/specs/badge` (после архивации change `port-button`).

## Что совпадает с PrimeNG

- Классы `p-button*`, `p-badge*` и атрибуты `data-p`, `data-p-disabled`, `data-p-severity` считаются теми же правилами (`state.rs` обоих крейтов,
  таблицы юнит-тестов по `buttonstyle.ts` и `badgestyle.ts`).
- Порядок содержимого кнопки: `children`, иконка или индикатор загрузки, метка, бейдж.
- Свойства: см. спеку; Angular-имена переведены в snake_case (`iconPos` → `icon_pos`, `badgeSeverity` → `badge_severity`, `styleClass` → `style_class`).

## Отличия и их причины

| Отличие | Причина |
|---|---|
| Нет хост-элементов `<p-button>`, у Badge вместо `<p-badge>` — `<span>` | Хост в Angular — деталь фреймворка; CSS темы привязан к классам. |
| Иконка — `ViewFn` в оболочке `span.p-button-icon`, а не строка класса PrimeIcons | Шрифта PrimeIcons нет; оболочка нужна, чтобы CSS (`order`, отступы) работал для любой иконки. |
| Индикатор загрузки по умолчанию — `SpinnerIcon` без числовых классов | В PrimeNG `spinnerIcon` формирует классы `0 1 2 …` из `Object.entries` строки; это побочный эффект, а не стиль. |
| Свой индикатор загрузки без класса `pi-spin` | `pi-spin` — анимация шрифта PrimeIcons; у `SpinnerIcon` свой `p-icon-spin`. |
| Нет директивы `pButton`, `buttonProps`, контекста `Fluid` (есть свойство `fluid`), `unstyled` | Вне объёма change; см. `proposal.md`. |
| Иконки демонстраций заменены ближайшими из `prime-icons` | В 53 иконках PrimeNG нет пользователя, закладки, колокольчика, сердца и пользователей: `pi-user` → `InfoCircleIcon`, `pi-bookmark` → `StarIcon`, `pi-bell` → `ExclamationTriangleIcon`, `pi-heart` → `StarFillIcon`. |

## Проверки

- `cargo test -p prime-button -p prime-badge --all-features` — юнит и SSR (структура DOM, классы, `pt`, a11y, реактивность).
- `cargo test -p prime-button --target wasm32-unknown-unknown --test web` — клик, `disabled`/`loading`, фокус, автофокус, ripple, перезапись `class`.
- `e2e/tests/button.spec.ts` — страница `/button` в showcase (Playwright).
- Еженедельное сравнение скриншотов с PrimeNG (`e2e/tests/visual.spec.ts`, `pages.json`): отчёт `visual.json`, не блокирует PR.

## Производительность

Измерения и бюджеты — `docs/benchmarks.md`. Кратко: Badge ≈ 41, Button ≈ 70 аллокаций на экземпляр при SSR (в основном реактивные примитивы),
вклад в сырой wasm +113 КБ и +379 КБ.

## Известная проблема, найденная при портировании

Генератор темы оставлял в CSS компонентов нерешённые `dt('button.primary.color')` (значения токенов не подставлялись, кнопки выглядели как
нативные). Исправлено в `xtask/theme/extract.mjs` (текст стиля проходит через тег `Css`, как в `BaseStyle.load` PrimeNG), `gen-theme` теперь
отклоняет CSS с `dt(`, а тесты `prime-theme` проверяют подстановку токенов.
