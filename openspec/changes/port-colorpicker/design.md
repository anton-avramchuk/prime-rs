# Design

## Context

Требования — в `specs/colorpicker`. Исходники PrimeNG 21.1.10: `colorpicker/colorpicker.ts` (`pickColor`, `pickHue`, `updateUI`, `HSBtoRGB`, `RGBtoHSB`, `writeControlValue`), `style/colorpickerstyle.ts`, `@primeuix/styles/colorpicker`. Образцы в проекте: `prime-popover` (панель внутри корня, `align_overlay`, клик снаружи, `watch_scroll`), `prime-slider`/`prime-knob` (жест через слушатели окна, `StoredValue<_, LocalStorage>`, снятие в `on_cleanup`), `prime-select` (образец + панель).

## Goals / Non-Goals

**Goals:** тот же DOM, те же формулы и значения, что в PrimeNG.

**Non-Goals:** формы Angular, `appendTo`, `overlayOptions`, `motionOptions`, `Bind`.

## Decisions

### D1. Чистый модуль цветов (`color.rs`)
`Hsb {h,s,b: f64}`, `Rgb {r,g,b: u8-подобные f64/i32}`; функции `hsb_to_rgb`, `rgb_to_hsb`, `hex_to_rgb` (строка с `#` или без; мусор → чёрный), `rgb_to_hex`, `hsb_to_hex`, `validate_hsb`, `pick_field(x, y) -> (s, b)`, `pick_hue(y) -> h`. Всё без DOM, табличные тесты на формулах TS (в том числе `h == 360`, серые, граничные).

### D2. Значение
`enum ColorValue { Hex(String), Rgb(Rgb), Hsb(Hsb) }` и `value: RwSignal<Option<ColorValue>>`; внутренний сигнал `hsb` — источник истины для отображения. Эффект (под `cfg(wasm32)`) переводит внешнее значение в `hsb` по `format`; `None` — `default_color`. Запись из жеста обновляет `hsb` и `value` одним действием, без петли эффекта (сравнение с последним записанным).

### D3. Оверлей
Панель — внутри корня, как у Select/Popover: `prime_dom::align_overlay(Relative)`, `ZGroup::Overlay`, клик снаружи и resize на `window` (слушатели вешаются в `set_timeout(0)` после показа), `prime_dom::watch_scroll` вместо своего слушателя прокрутки. Панель скрыта до расчёта положения. `inline` — без образца и оверлея.

### D4. Жесты
Размер области — константа 150. Координаты: `clientX/Y` минус `getBoundingClientRect` области (как у Slider/Knob, а не `pageX/Y` — результат тот же, но не зависит от прокрутки). Мышь: `mousedown` на области → слушатели `mousemove`/`mouseup` окна на время жеста, `StoredValue<_, LocalStorage>`, снятие в `on_cleanup`. Касание: `touchstart` на области, `touchmove`/`touchend` на ней же с `preventDefault`. Касание НЕ проверяется на реальном устройстве — честно записать в docs.

### D5. Положение указателей
Стили указателей и фона поля вычисляются из `hsb` реактивно (не императивно, как в TS): `style:left/top`, `background-color`. Образец: `style:background-color`.

### D6. SSR
Начальный `hsb` из `value`/`default_color` без DOM; образец с фоном и закрытая панель (либо встроенная) — в разметке.

### D7. Тесты
Цвета — таблицы. SSR: разметка закрытого/встроенного, классы, формат, блокировка. Браузерные `tests/web.rs` одним последовательным тестом: открытие `Space`/клик, `Escape`, клик снаружи, клик по полю и шкале (значения), перетаскивание до выхода за границы, `format` rgb/hsb, внешнее значение, `disabled`. e2e: реальные клики по полю и шкале, перетаскивание, `Space`.

## Risks / Trade-offs

- [Округление] `floor` и `round` из TS воспроизводятся точно; сравнение с эталонными значениями PrimeNG в таблицах.
- [Касание] Только эмуляция событий.
- [Позиция панели] Образец — `input`, панель позиционируется относительно корня.

## Open Questions

- Нет.

## Migration Plan

Миграции нет. Откат — `git revert`.
