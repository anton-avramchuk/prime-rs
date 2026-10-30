# Отображение: OverlayBadge, Timeline, MeterGroup

Порт `p-overlaybadge`, `p-timeline` и `p-metergroup` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-overlaybadge::OverlayBadge` | `overlaybadge/overlaybadge.ts`, `overlaybadge/style/overlaybadgestyle.ts` |
| `prime-timeline::Timeline` | `timeline/timeline.ts`, `timeline/style/timelinestyle.ts` |
| `prime-metergroup::MeterGroup` | `metergroup/metergroup.ts` (с внутренним `MeterGroupLabel`), `metergroup/style/metergroupstyle.ts` |
| CSS | `@primeuix/styles/{overlaybadge,timeline,metergroup}` — `cargo xtask gen-theme` |
| Демонстрации | `apps/showcase/doc/badge/overlay-doc.ts`, `doc/timeline/*-doc.ts`, `doc/metergroup/*-doc.ts` |

Спеки: `openspec/specs/{overlaybadge,timeline,metergroup}`.

## Что совпадает с PrimeNG

- Классы и их порядок: `p-overlaybadge`; `p-timeline p-component p-timeline-<align> p-timeline-<layout>` и части `p-timeline-event*`; `p-metergroup p-component p-metergroup-<orientation>`, `p-metergroup-meters`, `p-metergroup-meter`, `p-metergroup-label-list*`, `p-metergroup-label*`.
- `OverlayBadge` передаёт `style_class` и `style` вложенному бейджу, а не корню.
- `Timeline`: соединителя у последнего события нет; `data-p` на корне и частях — `<layout> <align>`.
- `MeterGroup`: `role="meter"`, `aria-valuemin/max/now`; процент `round(clamp((v - min) / (max - min) * 100, 0, 100))`, при `min == max` — 100; отрезки с нулевым и отрицательным значением не выводятся; в подписи — `Метка (процент%)`.

## Отличия

| Отличие | Причина |
|---|---|
| Корни — `<div>`, а не хосты `<p-…>` | Хост в Angular — деталь фреймворка. |
| Шаблоны `content`, `opposite`, `marker`, `label`, `meter`, `start`, `end`, `icon` — свойства-колбэки (`Callback<…, AnyView>`) | Вместо `ng-template` и `PrimeTemplate`. |
| `Timeline<T>` обобщённый, `align` и `layout` — перечисления (`TimelineAlign` — `Left`, `Right`, `Top`, `Bottom`, `Alternate`) | В PrimeNG это произвольные строки, но CSS знает только эти значения. |
| `MeterItem::icon` — `ViewFn`, а не имя класса шрифта (`pi pi-…`) | В проекте иконки — SVG-компоненты `prime-icons`. |
| `OverlayBadge::style` читается один раз при создании | Бейдж принимает стиль только через `pt`; реактивный стиль не нужен ни одной демонстрации. |
| Нет устаревшего `size` у `OverlayBadge`, `escape = false` у меток, `hostName`, `getBlockableElement` у `Timeline` | Устарели или привязаны к Angular. |
| Нет демонстрации `template` у `Timeline` и `MeterGroup` | Опираются на карточки и иконки, которых нет в `prime-icons`. |
| В демонстрациях вместо `bell`, `envelope`, `table`, `inbox`, `image`, `cog` — `calendar`, `info-circle`, `exclamation-triangle`, `th-large`, `eye`, `refresh` | Этих иконок нет в `prime-icons`. |

## Стоимость

| | Аллокаций на экземпляр (SSR) | Вклад в wasm |
|---|---|---|
| `OverlayBadge` (со значком) | 18 | ~113 КБ (внутри — `Badge`) |
| `Timeline` (4 события, один шаблон) | 99 | ~123 КБ |
| `MeterGroup` (3 элемента) | 179 | ~196 КБ |

`MeterGroup` и `Timeline` — самые «тяжёлые» по аллокациям из-за строк стиля и `pt` на каждую часть; сигнал `value` перестраивает список целиком, поэтому читать его нужно только в замыкании компонента.

## Не проверено

Касание и экранные считыватели не проверялись на реальных устройствах: отрезки `MeterGroup` и события `Timeline` не интерактивны, проверена только разметка и вычисленные стили (e2e).
