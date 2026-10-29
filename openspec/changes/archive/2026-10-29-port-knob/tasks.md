# Tasks

## 1. Крейт и компонент

- [x] 1.1 Создать крейт `crates/prime-knob` (`publish = false`, features `ssr`/`hydrate`, зависимость `prime-core`, `web-sys` с событиями мыши, касания, `DomRect`, dev-зависимости для wasm как у `prime-slider`), добавить в workspace; проверить `cargo check -p prime-knob --all-features`
- [x] 1.2 `state.rs`: `map_range`, `value_from_angle`, `zero_radians`, `value_radians`, `range_path`, `value_path`, `key_target`, `display_text`, классы корня по `knobstyle.ts`; табличные тесты
- [x] 1.3 Реализовать `Knob`, `KnobPt` (значение, SVG и дуги, текст и шаблон, цвета и размер, `aria-*`, `tabindex`, клавиатура, `on_change`); SSR-тесты (структура, `aria-*`, размер, пути в ключевых точках, шаблон и цвета, `show_value`, `disabled`/`readonly`, `pt`, паритет ветвей)
- [x] 1.4 Ввод указателем: клик, перетаскивание мышью и касанием со слушателями на `window` только на время нажатия, фокус при нажатии, пропуск `click` после перетаскивания
- [x] 1.5 Браузерные тесты `tests/web.rs`: клавиши, клик по характерным точкам и разрыву, шаг, перетаскивание с выходом за SVG, снятие слушателей, пропуск `click`, `readonly`/`disabled`, внешний сигнал, реактивные границы
- [x] 1.6 rustdoc с примером; `cargo test -p prime-knob --all-features`

## 2. Тема, showcase, e2e

- [x] 2.1 Добавить `knob` в `theme.components` (`upstream.toml`), `cargo xtask gen-theme`, убедиться, что CSS других компонентов не изменился, подключить CSS
- [x] 2.2 Страница `/knob` (`basic`, `minmax`, `reactive`, `size`, `step`, `stroke`, `template`, `color`, `readonly`, `disabled`), меню, маршрут, `stylesheet(&[..])`
- [x] 2.3 `e2e/tests/pages.json` и `e2e/tests/knob.spec.ts` (перетаскивание и клавиши; ждать `networkidle`); перезапустить сервер showcase перед прогоном; `npx playwright test knob`

## 3. Бенчмарки и бюджеты

- [x] 3.1 Сценарии `knob-1`, `knob-1000`, `case-knob`, бюджеты по измерению (сначала `cargo bench -p render-bench --bench alloc`)

## 4. Интеграция

- [x] 4.1 Добавить `prime-knob` в браузерный job `ci.yml` (3 места); `docs/knob.md`; clippy, тесты, deny, machete, `openspec validate port-knob`; CI `dev` зелёный

## Workflow follow-up

- Архивировать после CI.
