# Proposal

## Why

ColorPicker — выбор цвета: пользователь двигает указатель по полю насыщенности/яркости и по шкале оттенка, результат уходит в значение в формате `hex`, `rgb` или `hsb`. Компонент самодостаточен: чистая математика цветов (HSB/RGB/HEX) плюс панель с двумя перетаскиваемыми областями. Открывается как оверлей (поле-образец и панель) либо встроен (`inline`). Первый компонент волны 6.

## What Changes

- Новый крейт `prime-colorpicker` (порт `p-colorpicker` из PrimeNG 21.1.10).
- Чистый модуль `color.rs`: `Hsb`, `Rgb`, `hsb_to_rgb`, `rgb_to_hsb`, `hex_to_rgb`, `hsb_to_hex`, ограничение значений; табличные тесты на формулах PrimeNG (включая округления).
- `ColorPicker`: корень `div.p-colorpicker.p-component` (`p-colorpicker-overlay` без `inline`, `p-colorpicker-dragging` во время жеста); `input.p-colorpicker-preview` (`readonly`, `p-disabled`, фон — текущий цвет, `aria-label` «Select Color») и панель `div.p-colorpicker-panel` > `.p-colorpicker-content` > `.p-colorpicker-color-selector` (> `.p-colorpicker-color-background` > `.p-colorpicker-color-handle`) и `.p-colorpicker-hue` (> `.p-colorpicker-hue-handle`). Размер областей — 150 px.
- Значение — `RwSignal<ColorValue>` (`Hex(String)`, `Rgb(Rgb)`, `Hsb(Hsb)`) по `format`; пустое значение показывает `default_color` (`ff0000`).
- Поведение: клик/`Space` на образце переключает панель; `Escape`/`Tab` закрывают; нажатие и перетаскивание мышью и касанием по полю и шкале меняют цвет (слушатели `mousemove`/`mouseup` окна на время жеста); клик снаружи и resize закрывают панель; `disabled` блокирует жесты.
- Свойства: `value`, `format`, `inline`, `default_color`, `disabled`, `tabindex`, `input_id`, `autofocus`, `auto_z_index`, `id`, `style`, `style_class`; события `on_change`, `on_show`, `on_hide`; `ColorPickerRef` (`show`, `hide`).
- Страница `/colorpicker` (`basic`, `inline`, `format`, `disabled`), e2e, бенчмарки и бюджеты, `docs/colorpicker.md`, `colorpicker` в `theme.components`.
- **Не входит:** формы Angular, `Bind`, `pt` через хост, `overlayOptions`/`appendTo`/`motionOptions` (панель внутри корня, как у Select/Popover), `showTransitionOptions`/`hideTransitionOptions`.

## Capabilities

### New Capabilities
- `colorpicker`: выбор цвета на панели с полем яркости/насыщенности и шкалой оттенка.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-colorpicker` (`publish = false`); зависимости: `prime-core`, `prime-dom`.
- Изменяются `Cargo.toml` workspace, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml`.
