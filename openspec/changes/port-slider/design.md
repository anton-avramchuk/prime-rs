# Design

## Context

Требования — в `specs/slider`. Исходники PrimeNG 21.1.10: `packages/primeng/src/slider/slider.ts`, `slider/style/sliderstyle.ts`, демонстрации `apps/showcase/doc/slider`. Образцы — `crates/prime-scrollpanel` (перетаскивание, слушатели документа с отпиской, ветвление `cfg(target_arch = "wasm32")`), `crates/prime-selectbutton` и `crates/prime-rating` (правила отслеживания в замыканиях: события и фокус без пересоздания элементов; в обработчиках только `*_untracked`).

Наблюдения по исходнику, влияющие на решения:
- Положение ручки всегда выводится из значения (`updateHandleValue` в конце `updateValue`), поэтому положение — чистая функция значения, а сырой процент перетаскивания не хранится.
- При заданном `step` итог получается не округлением до ближайшего шага, а `handleStepChange`: сдвигом от прежнего значения на целое число шагов в сторону указателя (`floor` вправо, `ceil` влево). Это переносится как есть и покрывается табличным тестом.
- Без `step` значение — `Math.floor` (до целого), а при дробном `step` — округление до числа его знаков после запятой.
- В диапазоне значение пишется в `values[handleIndex]` с ограничением `min..max`, а наружу уходит `[minVal, maxVal]`; ручки могут пересекаться, индекс активной не меняется. Если стартовая ручка стоит на `max`, нажатие выбирает её (`handleIndex = 0`).
- Клик по полосе двигает ручку `handleIndex` (последнюю активную, по умолчанию первую); клик по ручке не считается кликом по полосе (`sliderHandleClick`).
- После каждого изменения значения фокус возвращается на активную ручку.
- Касание: `floor(Δ·100 / размер) + стартовое положение` в процентах, шаг 1 %; события касания приходят на ручку, слушатели документа не нужны.

## Decisions

### D1. Значение
`value: Option<RwSignal<f64>>` и `values: Option<RwSignal<[f64; 2]>>`; без сигнала создаётся внутренний с `default_value` (`f64`, 0) или `default_values` (`Option<[f64; 2]>`, по умолчанию `[0, 0]`). Активен тот, что соответствует `range: Prop<bool>`; неактивный не читается. Числа — `f64` (дробный `step` в PrimeNG допустим).

### D2. Чистая логика (`state.rs`)
Функции без DOM, каждая с табличным тестом: `percent(value, min, max)` (с ограничением 0–100 и защитой от `min == max`), `value_from_percent`, `decimals(step)`, `normalize(value, step)`, `step_change(new, old, step)`, `key_target(code, current, step, range) -> Option<KeyAction>` (стрелки, `PageUp/Down`, `Home/End`), `clamp`, `bar_percent(pointer, start, size, vertical, rtl)`, `touch_percent(pointer, start, size, vertical, start_percent)`, классы корня и `data-p`. Нестандартные значения PrimeNG (шаг 1 для клавиш, 10 для `Page*`, в диапазоне `Page*` равны 1) закреплены здесь.

### D3. Состояние взаимодействия
`active: RwSignal<usize>` (индекс активной ручки), `dragging: RwSignal<bool>` (для `data-p-sliding` и снятия `p-slider-animate`), флаг `handle_pressed` (обычный `Cell`/`AtomicBool`, как `sliderHandleClick`): нажатие на ручку ставит его, клик по хосту сбрасывает и пропускает обработку полосы. Геометрия (`getBoundingClientRect`, `offsetWidth/Height`) считывается в момент события через `NodeRef` корня; направление (`getComputedStyle(root).direction`) определяется там же для RTL. Используются клиентские координаты и `rect.left/top`, что эквивалентно `pageX − (rect.left + scrollX)` PrimeNG.

### D4. Перетаскивание и слушатели
Нажатие на ручку (`mousedown`) ставит `dragging`, выбирает ручку, фокусирует её, отменяет действие по умолчанию и подписывает `mousemove` и `mouseup` на `window` (`leptos::prelude::window_event_listener`; обработчик живёт ровно до `mouseup` или размонтирования — `on_cleanup`). `mousemove` вычисляет значение по указателю, `mouseup` снимает слушатели, сбрасывает `dragging` и вызывает `on_slide_end`. Подписка создаётся только из обработчика события, то есть в браузере, поэтому SSR не затрагивается и отдельной ветки `cfg` не требуется. Касание — `touchstart/touchmove/touchend` на ручке по алгоритму `onDragStart/onDrag/onDragEnd`.

### D5. Обновление значения
Единая функция `update_value(raw, event, index)`: ограничение `min..=max`, нормализация, запись в сигнал (в диапазоне — в `values[index]`, наружу — упорядоченная пара), вызов `on_change` и фокус активной ручки. `on_change` вызывается на каждое применённое действие, даже если значение осталось прежним (как `updateValue` в PrimeNG). `set_value_from_handle` с `step` идёт через `step_change`.

### D6. Разметка и реактивность
Корень — `<div>`; положение ручек и заполнения — замыкания над значением и границами (`inset-inline-start`/`bottom`, `width`/`height`), собственные стили `position: absolute` статичны. Ручки и заполнение строятся один раз по `range` (структура зависит только от `range`; при смене `range` блок перестраивается, поэтому замыкание перестройки читает только `range`, а положение и значения читаются внутри ручек). Классы: `p-slider p-component`, `p-disabled`, `p-invalid`, `p-slider-horizontal|vertical`, `p-slider-animate` (при `animate` и не `dragging`), `style_class`, `pt.root`; `data-p` — ориентация. Ручка: `role=slider`, `aria-valuemin/now/max`, `aria-orientation`, `data-pc-section`, `tabindex` (нет при `disabled`), `p-slider-handle-active` у активной в диапазоне.

### D7. CSS
`slider` в `theme.components` (`upstream.toml`), `cargo xtask gen-theme`; блока `/* For PrimeNG */` у `sliderstyle.ts` нет, `primeng_extra` не меняется.

### D8. Демонстрации
`basic`, `range`, `step`, `vertical`, `filter`, `input`. В `filter` вместо внешней картинки градиентный блок с CSS-фильтрами `contrast`, `brightness`, `sepia`, переключаемыми `SelectButton`; три слайдера создаются по выбору. В `input` значение связано с `InputText` через два `Effect` (строка ↔ число). Ширина и высота задаются через `style`. В `basic` — `echo` значения для e2e.

### D9. Тесты
SSR: структура, положение и `aria-*` для одиночного режима и диапазона, ориентации, классы и `data-p`, `disabled`, `pt`, паритет статических и реактивных свойств. Браузерные (`tests/web.rs`): клавиши (шаг, страницы, `Home/End`, границы, фокус остаётся на ручке), клик по полосе, перетаскивание (синтетические `mousedown/mousemove/mouseup`: значение, `on_slide_end`, отсутствие слушателей после отпускания — проверкой, что `mousemove` после `mouseup` значение не меняет), `disabled`, диапазон (порядок, активная ручка), внешний сигнал и реактивные границы, сохранение ручек и фокуса после изменения. e2e: перетаскивание реальной мышью и клавиши. Касание покрыто чистыми функциями и ручной проверкой (синтетические `TouchEvent` в `wasm-bindgen-test` неудобны).

## Risks / Trade-offs

- [Много ветвей поведения (диапазон, RTL, вертикаль, шаг), легко разойтись с PrimeNG] → чистые функции с табличными тестами и сверка с исходником; отличия (`D1`, отсутствие сырого процента) записываются в `docs/slider.md`.
- [`window_event_listener` в обработчике: утечка слушателей при размонтировании посреди перетаскивания] → снятие в `on_cleanup` и при `mouseup`; тест на отсутствие реакции после `mouseup`.
- [Касание проверяется слабее мыши] → чистая функция `touch_percent` и ручной прогон в браузере.

## Migration Plan

Миграции нет: новый крейт. Откат — `git revert`.

## Open Questions

Нет.
