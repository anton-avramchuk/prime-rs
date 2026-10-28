# Design

## Context

Требования — в `specs/progressbar`. Исходники PrimeNG 21.1.10: `packages/primeng/src/progressbar/progressbar.ts`, `progressbar/style/progressbarstyle.ts`, демонстрации `apps/showcase/doc/progressbar`. Шаблон компонента — `docs/toggleswitch.md`, `docs/message.md`.

## Goals / Non-Goals

**Goals:** разметка, классы, ARIA и `data-p` как в PrimeNG; реактивное значение и режим; статическая ветвь для неизменного индикатора.
**Non-Goals:** собственные анимации, интеграция с формами.

## Decisions

### D1. Корень — `<div>`
Вместо хоста `<p-progressbar>`; ARIA-атрибуты и `data-p` — на корне.

### D2. Значение
`value: OptProp<f64>`; число выводится через `Display` (`50` без дробной части). В режиме `determinate` ширина — `value%` (при отсутствии значения — `0%`, как нулевая ширина в PrimeNG: `undefined%` отбрасывается браузером). `aria-valuenow` и `aria-level` выводятся только при заданном значении (PrimeNG выводит `aria-level="undefined%"` — осознанное отличие).

### D3. Подпись
`div.p-progressbar-label` содержит `div` со значением и единицей, скрытый (`display: none`) при отсутствии значения или нуле; при `show_value = false` или заданном `content_template` внутренний `div` не выводится, а шаблон получает значение.

### D4. Режимы
`mode` может быть сигналом; содержимое пересобирается при смене режима. `indeterminate` выводит только `div.p-progressbar-value` без ширины и подписи.

### D5. Ветви
Статическая — при `all_static!`; иначе реактивная. Содержимое строится функцией над свойствами, обёртка — макросом.

### D6. Тесты
SSR: структура обоих режимов, ARIA, ширина, подпись, `color`, шаблон, `pt`, паритет ветвей. Браузерных тестов нет: у компонента нет клиентского поведения.

## Risks / Trade-offs

- [Число без форматирования локали] → как в PrimeNG; форматирование — на стороне вызывающего (`content_template`).

## Migration Plan

Миграции нет: новый крейт. Откат — `git revert`.

## Open Questions

Нет.
