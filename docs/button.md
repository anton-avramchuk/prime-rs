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

## Сравнение скриншотов с PrimeNG

Еженедельный job (`weekly.yml`, прогон от 2026-10-28) сравнил 17 демонстраций с запущенным showcase PrimeNG на закреплённом коммите.
Доля отличающихся пикселей (`pixelmatch`, порог 0,1), сначала самые большие:

| Демонстрация | Расхождение | Демонстрация | Расхождение |
|---|---:|---|---:|
| button-severity | 24,3 % | button-badge | 10,7 % |
| button-icons | 23,8 % | button-loading | 8,2 % |
| button-raised | 23,8 % | button-basic | 6,0 % |
| button-rounded | 21,3 % | button-disabled | 5,9 % |
| button-sizes | 21,1 % | badge-severity | 5,3 % |
| button-iconsonly | 20,5 % | badge-size | 4,7 % |
| button-link | 12,5 % | button-outlined | 4,4 % |
| button-text | 2,4 % | button-template | 1,2 % |
| badge-basic | 1,0 % | | |

Расхождение определяется не компонентами, а окружением showcase; по картинкам `e2e-results` видно три причины:

1. **Основной цвет.** В showcase PrimeNG основной цвет «noir» (почти чёрный), у нашего showcase стандартный emerald пресета Aura; кнопки `Primary`
   и их `outlined`/`text`-варианты отличаются цветом. Остальные серьёзности (`secondary`, `success`, `info`, `warn`, `help`, `danger`) совпадают по цвету.
2. **Шрифт.** У PrimeNG свой шрифт showcase, у нас системный `sans-serif`: меняются ширина кнопок и высота строки.
3. **Ширина карточки.** Карточка PrimeNG уже, поэтому ряд из восьми кнопок переносится на вторую строку, у нас остаётся в одну; из-за этого сильнее всего
   отличаются демонстрации с многими кнопками (`severity`, `raised`, `rounded`, `icons`).

Следующий шаг для честного сравнения (не в этом change): привести showcase к тем же основному цвету, шрифту и ширине контента.
Метрика остаётся отчётом и PR не блокирует.

## Производительность

Измерения и бюджеты — `docs/benchmarks.md`. После `optimize-reactive-props` Badge делает 8, Button 20 аллокаций на экземпляр при SSR
(было 41 и 67), вклад в сырой wasm +84 КБ и +327 КБ (было +113 КБ и +379 КБ: `Memo`, `Signal` и `class_memo` перестали попадать в бинарник).

Шаблон для следующих компонентов:
- свойства — `Prop<T>`/`OptProp<T>` из `prime-core` (значение не создаёт сигнала, сигнал подписывает атрибуты);
- без `Memo`: состояние заимствует значения (`Option<&str>`) и собирается внутри атрибутных замыканий;
- свойства экземпляра лежат в одном `Arc<Props>`, замыкания клонируют только счётчик ссылок;
- клиентские эффекты (`Effect`, ripple, автофокус) создаются только под `cfg(target_arch = "wasm32")`;
- в `view!` идентификатор перед `{..attrs}` оборачивается в `{}` (`data-p={x} {..attrs}`), иначе макрос читает его как литерал структуры.

## Известная проблема, найденная при портировании

Генератор темы оставлял в CSS компонентов нерешённые `dt('button.primary.color')` (значения токенов не подставлялись, кнопки выглядели как
нативные). Исправлено в `xtask/theme/extract.mjs` (текст стиля проходит через тег `Css`, как в `BaseStyle.load` PrimeNG), `gen-theme` теперь
отклоняет CSS с `dt(`, а тесты `prime-theme` проверяют подстановку токенов.
