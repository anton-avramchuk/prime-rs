# ImageCompare

Порт `p-imagecompare` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-imagecompare::ImageCompare`, `ImageComparePt` | `packages/primeng/src/imagecompare/imagecompare.ts`, `imagecompare/style/imagecomparestyle.ts` |
| CSS | `@primeuix/styles/imagecompare` и пресет Aura — `cargo xtask gen-theme` (блока `/* For PrimeNG */` нет) |
| Демонстрации | `apps/showcase/doc/imagecompare/{basic,responsive}-doc.ts` |

Спека: `openspec/specs/imagecompare`.

## Что совпадает с PrimeNG

- Разметка: корень `div.p-imagecompare` (`tabindex`, `aria-label`, `aria-labelledby` на нём) с представлениями `left` и `right` без обёрток и `input[type=range].p-imagecompare-slider` (`min=0`, `max=100`, `value=50`).
- Ввод в ползунок задаёт `clip-path` элементу, стоящему непосредственно перед ним (правое изображение): `polygon(0 0, V% 0, V% 100%, 0 100%)`; при `dir="rtl"` у корня или предка — `polygon((100−V)% 0, 100% 0, 100% 100%, (100−V)% 100%)`.
- До первого ввода обрезку задаёт только правило темы `.p-imagecompare img + img` (50 %), поэтому `left` и `right` должны быть `<img>`: с другими элементами правило темы не сработает — как в PrimeNG.

## Отличия

| Отличие | Причина |
|---|---|
| Корень — `<div>`, а не хост `<p-imagecompare>` | Хост в Angular — деталь фреймворка. |
| `left`/`right` — `ViewFn` вместо шаблонов `#left`/`#right` | Нет `ng-template`. |
| Нет `MutationObserver` за атрибутом `dir` на `<html>`: направление вычисляется в момент ввода (`closest('[dir=rtl]')`) | PrimeNG применяет `isRTL` только в `onSlide`, поэтому результат тот же; наблюдатель не нужен. |
| `tabindex` по умолчанию не задан | Документация PrimeNG говорит о `0`, но в коде значение `undefined` — следуем коду. |
| В демонстрациях вместо фотографий с CDN два встроенных SVG (`data:`-ссылки) | Независимость от сети. |
| Нет `Bind`, `pTemplate`, `pt` с контекстом | `pt` — простые добавки классов, стиля и атрибутов (`root`, `slider`). |

## Что не проверялось

- Настоящее касание: перетаскивание ручки проверено мышью и клавишами.
- Чтение с экрана: у ползунка нет собственного `aria-label` (как в PrimeNG — подписи только на корне).

## Производительность

См. `benches/` (`imagecompare-1`, `imagecompare-1000`, `case-imagecompare`) и `benches/*budgets.toml`: 12 аллокаций на экземпляр при SSR.
