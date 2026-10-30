# Splitter

Порт `p-splitter` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-splitter::Splitter`, `SplitterLayout`, `SplitterStorage`, `SplitterResizeEvent`, `SplitterPt` | `splitter/splitter.ts`, `splitter/style/splitterstyle.ts` |
| CSS | `@primeuix/styles/splitter` — `cargo xtask gen-theme` (блока `/* For PrimeNG */` у компонента нет) |
| Демонстрации | `apps/showcase/doc/splitter/{horizontal,vertical,size,nested}-doc.ts` |

Спека: `openspec/specs/splitter`.

## Что совпадает с PrimeNG

- Разметка и классы: `div.p-splitter.p-component.p-splitter-<layout>` (inline `display: flex; flex-wrap: nowrap`, колонкой при `vertical`), `div.p-splitterpanel` (`tabindex="-1"`, `data-pc-section="panel"`), между панелями `div.p-splitter-gutter[role=separator]` с `div.p-splitter-gutter-handle[tabindex=0][aria-orientation][aria-valuenow]`; при изменении — `p-splitter-resizing` на корне, `p-splitter-gutter-resizing` и `data-p-gutter-resizing="true"` на разделителе.
- Размер панели — `flex-basis: calc(<доля>% - <(n-1)·gutter_size>px)`; недостающие и нулевые `panel_sizes` делятся поровну.
- Перетаскивание мышью считается в процентах размера корня (`pageX`/`pageY`), в `rtl` горизонтальное смещение инвертируется; стрелки двигают границу на `step` пикселей; `keyup` завершает изменение.
- Состояние сохраняется в `localStorage`/`sessionStorage` по `state_key` как JSON-массив чисел и восстанавливается при монтировании.

## Отличия

| Отличие | Причина |
|---|---|
| Корень — `<div>` без хоста `<p-splitter>` | Хост в Angular — деталь фреймворка. |
| `panels: Vec<ViewFn>` вместо `ng-template #panel` и `ContentChildren` | Вместо проекции содержимого. |
| `nested` — явное свойство | В PrimeNG вложенность определяется запросом `contentChild(Splitter)`. |
| `min_sizes[i]` относится к предыдущей панели, `min_sizes[i+1]` — к следующей для разделителя `i` | В PrimeNG для любого разделителя проверяются `minSizes[0]` и `minSizes[1]`; для трёх и более панелей это ошибка исходника. Для двух панелей поведение совпадает. |
| Изменение, нарушающее `min_sizes`, отвергается целиком, `aria-valuenow` берётся из текущего размера панели (у каждого разделителя свой) | В PrimeNG один общий `prevSize` обновляется и при отвергнутом изменении. |
| Стрелка применяет шаг на каждый `keydown` (включая автоповтор), без таймера в 40 мс; `on_resize_start` вызывается один раз за серию нажатий | В PrimeNG `setTimer` повторно вызывает `resizeStart`/`onResize`. |
| Размер корня измеряется один раз за перетаскивание | Как в PrimeNG; пересчёт при изменении размеров окна во время перетаскивания не делается. |
| События без `originalEvent`: `SplitterResizeEvent { sizes }` | Событие DOM в Leptos-обработчиках не передаётся наружу. |
| Число панелей, раскладка, шаг, `gutter_size`, `min_sizes` читаются один раз; изменение `panels` на лету не поддерживается | Простая модель; PrimeNG пересчитывает размеры при смене `panelSizes`. |
| Сохранённое состояние применяется только при совпадении числа панелей, ошибки хранилища игнорируются | Защита от устаревших данных. |
| Касание: слушатели `touchmove`/`touchend`/`touchcancel` на `window`, без `preventDefault` | Не проверялось на реальных устройствах. |
| Нет `Bind`/`pt` с контекстом | `pt` — простые добавки классов, стиля и атрибутов. |
| В демонстрациях корню задано `width: 100%`; утилитные классы Tailwind заменены встроенными стилями | Карточка демонстрации у нас flex, у PrimeNG блочная. |

## Стоимость

108 аллокаций на экземпляр при SSR (две панели); вклад в wasm ≈ 313 КБ (бюджет 350 КБ в `benches/budgets.toml`).

## Не проверено

Касание на реальных устройствах, `rtl` в реальном браузере (проверена только чистая функция смещения), экранные считыватели. Проверены разметка, мышь, стрелки, `min_sizes`, вертикальная и вложенная раскладки, хранилище в Chrome (e2e и браузерные тесты).
