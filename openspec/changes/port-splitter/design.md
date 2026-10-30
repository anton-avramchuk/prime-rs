# Design

## Context

Требования — в `specs/splitter`. Исходники PrimeNG 21.1.10: `splitter/splitter.ts`, `splitter/style/splitterstyle.ts`; демонстрации `apps/showcase/doc/splitter/{horizontal,vertical,size,nested}-doc.ts`. Образцы в проекте: `prime-slider` (перетаскивание: `window_event_listener` из обработчика, handle в `StoredValue<_, LocalStorage>`, снятие в `on_cleanup`), `prime-knob` (координаты указателя), `prime-fieldset` (двусторонний сигнал).

Наблюдения по исходнику:
- Размеры панелей PrimeNG записывает прямо в `style.flexBasis` DOM-элементов; мы держим `RwSignal<Vec<f64>>` и выводим `flex-basis` реактивно.
- `validateResize` сравнивает `minSizes[0]` с предыдущей и `minSizes[1]` со следующей панелью независимо от индекса разделителя (для трёх и более панелей это ошибка исходника); мы берём `minSizes[i]` и `minSizes[i+1]`.
- Клавиатура: `keydown` ставит таймер 40 мс и вызывает `resizeStart(keydown)` + `onResize(step)`, где размеры соседей берутся в пикселях; `keyup` завершает. Мы применяем шаг на каждый `keydown` без таймера: `Δ% = 100 · step / размер_корня`.
- Касание: `touchstart`/`touchmove`/`touchend` на разделителе плюс слушатели документа; в первой версии касание обрабатываем так же, как мышь, через слушатели `window`, без проверки на устройстве.
- `p-splitterpanel-nested` и `nested` в `data-p` в PrimeNG вычисляются по `contentChild(Splitter)`; мы вводим явное свойство `nested`.
- Разделитель выводится после каждой панели, кроме последней (`*ngIf="i !== panels.length - 1"`).

## Decisions

### D1. Крейт и зависимости
`prime-splitter` с `Splitter`, `SplitterLayout { Horizontal, Vertical }`, `SplitterStorage { Session, Local }`, `SplitterResizeEvent { sizes }`, `SplitterPt { root, panel, gutter, gutter_handle }`. Зависит от `prime-core` и `web-sys`.

### D2. Чистая логика (`state.rs`)
- `initial_sizes(panel_sizes, n) -> Vec<f64>` — заданные ненулевые или `100/n`.
- `flex_basis(size, n, gutter) -> String` — `calc(<size>% - <(n-1)*gutter>px)`.
- `drag_delta(start, current, container, rtl, horizontal) -> f64` — смещение в процентах.
- `apply(sizes, index, prev, next, min_sizes) -> Option<(f64, f64)>` — проверка `min_sizes` для пары.
- `key_step(layout, key, step) -> Option<f64>` — знак шага по клавише и раскладке.
- `root_classes`, `panel_classes`, `data_p`.
Табличные тесты на каждую функцию.

### D3. Состояние
`sizes: RwSignal<Vec<f64>>` (проценты), `drag: StoredValue<Option<Drag>>` (индекс, стартовая позиция, стартовые размеры пары, размер корня), `resizing: RwSignal<Option<usize>>` для классов и `data-p-gutter-resizing`, `prev_sizes: RwSignal<Vec<f64>>` не нужен: `aria-valuenow` берётся из `sizes[i]`.

### D4. Перетаскивание
`on:mousedown` на разделителе: измеряет размер корня (`getBoundingClientRect` по оси), запоминает стартовую позицию (`page_x/page_y` — как в оригинале) и размеры пары, вызывает `on_resize_start`, ставит `window_event_listener` на `mousemove` и `mouseup`/`touchmove`/`touchend`. Handle слушателей лежит в `StoredValue<_, LocalStorage>`, снимается по `mouseup`/`touchend` и в `on_cleanup`.

### D5. Состояние в хранилище
`state_key` + `state_storage`; чтение в `Effect` после монтирования (только `wasm32`), запись при завершении изменения. Ошибки хранилища (запрещено, переполнено) молча игнорируются. Формат — JSON-массив чисел, как в PrimeNG; разбор — вручную без новых зависимостей (`serde_json` не подключаем).

### D6. Клавиатура
Ручка: `on:keydown` — применяет шаг к паре `(i, i+1)`, всегда `preventDefault` для стрелок (как в оригинале, даже для «чужой» оси); `on:keyup` — завершает (сохранение, `on_resize_end`). `on_resize_start` вызывается на первом `keydown` серии (пока не было `keyup`).

### D7. CSS
`splitter` добавляется в `theme.components`; `cargo xtask gen-theme`; проверка, что CSS остальных не изменился.

### D8. Тесты
SSR: разметка, `layout`, размеры по умолчанию и заданные, `nested`, `pt`. Браузерные `tests/web.rs`: перетаскивание (синтетические `mousedown`/`mousemove`/`mouseup`), `min_sizes`, клавиши, `on_resize_*`, хранилище, снятие слушателей. e2e: настоящее перетаскивание мышью и стрелки в Chrome, вложенный Splitter. Крейт добавляется в три места `ci.yml`.

### D9. Бенчмарки
Сценарии `splitter-1`/`-1000` (две панели), `case-splitter`; сначала `cargo bench -p render-bench --bench alloc`, для wasm — временный большой бюджет.

## Risks / Trade-offs

- [Расхождения с PrimeNG в `min_sizes` для трёх и более панелей] → намеренное исправление, записывается в docs.
- [Касание не проверено на реальных устройствах] → честно записано в docs.
- [Хранилище недоступно в SSR] → чтение только на клиенте; на сервере всегда начальные размеры.

## Migration Plan

Миграции нет. Откат — `git revert`.

## Open Questions

Нет.
