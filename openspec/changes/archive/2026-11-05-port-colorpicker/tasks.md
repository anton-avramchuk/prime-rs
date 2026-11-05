# Tasks

## 1. Основа

- [x] 1.1 `colorpicker` в `theme.components` (`upstream.toml`), `cargo xtask gen-theme`; CSS остальных компонентов не изменился, есть `p-colorpicker-hue-handle`
- [x] 1.2 Свериться с `pickColor`, `pickHue`, `updateUI`, `writeControlValue`, `onInputKeydown`, `HSBtoRGB`/`RGBtoHSB` исходника

## 2. Крейт `prime-colorpicker`

- [x] 2.1 Модуль `color.rs` с табличными тестами
- [x] 2.2 Компонент `ColorPicker`, `ColorPickerPt`, `ColorPickerRef`: образец, панель, `inline`, SSR-тесты, rustdoc
- [x] 2.3 Открытие/закрытие (клик, `Space`, `Escape`, `Tab`, клик снаружи, resize), `on_show`/`on_hide`
- [x] 2.4 Жесты мыши и касания по полю и шкале, `p-colorpicker-dragging`, `on_change`, форматы `hex`/`rgb`/`hsb`, внешнее значение, `disabled`
- [x] 2.5 Браузерные `tests/web.rs` (один последовательный тест)

## 3. Showcase и e2e

- [x] 3.1 Страница `/colorpicker` (`basic`, `inline`, `format`, `disabled`), меню, маршрут, `stylesheet(&[..])`, крейт в workspace и showcase
- [x] 3.2 `e2e/tests/pages.json` и `colorpicker.spec.ts`; `networkidle`, перезапуск сервера showcase; `PW_CHANNEL=chrome npx playwright test colorpicker`

## 4. Бенчмарки и бюджеты

- [x] 4.1 Сценарии `-1` и `-1000`, `case-colorpicker`, бюджеты (аллокации — `cargo bench -p render-bench --bench alloc`, wasm — временный большой бюджет)

## 5. Интеграция

- [x] 5.1 `docs/colorpicker.md`; `prime-colorpicker` в браузерный job `ci.yml` (три места)
- [x] 5.2 `cargo fmt --all -- --check`, clippy с исключениями CI и `--all-features`, тесты (`test result` wasm-тестов проверять строкой), deny, machete, `openspec validate port-colorpicker`

## Workflow follow-up

- Волна 6 — CI один раз в конце волны; после apply коммитить локально и архивировать.
