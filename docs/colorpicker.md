# ColorPicker

Порт `p-colorpicker` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-colorpicker::ColorPicker`, `ColorPickerPt`, `ColorPickerRef`, `ColorValue`, `ColorFormat` | `packages/primeng/src/colorpicker/colorpicker.ts`, `colorpicker/style/colorpickerstyle.ts` |
| Преобразования HSB / RGB / HEX, расчёт точки по полю и шкале | `prime-colorpicker::color` — чистый порт `HSBtoRGB`, `RGBtoHSB`, `HEXtoRGB`, `pickColor`, `pickHue`, `updateUI`, табличные тесты |
| CSS | `@primeuix/styles/colorpicker` (`colorpicker` в `theme.components`) |
| Демонстрации | `apps/showcase/doc/colorpicker/*-doc.ts` (`basic`, `inline`, `format`, `disabled`) |

Спека: `openspec/specs/colorpicker`. Значение — `RwSignal<Option<ColorValue>>` (`Hex("#rrggbb")`, `Rgb`, `Hsb`).

```rust
let value = RwSignal::new(None);
view! { <ColorPicker value=value format=ColorFormat::Rgb on_change=Callback::new(|c| log(c)) /> }
```

## Что совпадает с PrimeNG

- Формулы и округления — те же `f64`, поэтому результаты побитово те же, в том числе `h = 300` у серых цветов (так даёт `RGBtoHSB` при нулевой насыщенности).
- Поле и шкала 150 px: `s = floor(100·x/150)`, `b = floor(100·(150−y)/150)`, `h = floor(360·(150−y)/150)` с ограничением координат областью.
- Образец (`readonly`-поле с цветом фона), открытие по клику и `Space`, закрытие по `Escape`/`Tab`, клику снаружи и resize; `inline` без образца.
- Жест мыши продолжается за пределами области до `mouseup`; во время жеста у корня `p-colorpicker-dragging`.
- Разметка и классы: `p-colorpicker`, `-preview`, `-panel`, `-content`, `-color-selector`, `-color-background`, `-color-handle`, `-hue`, `-hue-handle`.

## Отличия

| Отличие | Причина |
|---|---|
| Панель выводится внутри корня (`align_overlay(Relative)`), а не в `p-overlay`; нет `appendTo`, `overlayOptions`, `motionOptions`, `showTransitionOptions` | Тот же подход, что у Select и Popover. |
| Координаты жеста — `clientX/Y` минус `getBoundingClientRect` области, а не `pageX/Y` с поправкой на прокрутку | Результат тот же, но не зависит от прокрутки (как у Slider и Knob). |
| Положение указателей и фон поля вычисляются реактивно из текущего HSB, а не императивно в `updateUI` | Естественно для Leptos; значения те же. |
| `aria-label` образца — константа «Select Color» | В проекте нет переводов (`TranslationKeys.ARIA`). |
| Нет форм Angular, `Bind` и `pt` через хост | Вне объёма порта. |

## Что не проверялось

- Касание: события `touchstart`/`touchmove`/`touchend` проверены только эмуляцией, не на реальном устройстве.
- Чтение с экрана: `aria-*` сверены с PrimeNG, поведение скринридеров — нет.
- Раскладка справа налево.

## Производительность

См. `benches/` (`colorpicker-1`, `colorpicker-1000`, `case-colorpicker`) и `benches/*budgets.toml`.
