# Tasks

## 1. Крейт и компонент

- [x] 1.1 Создать крейт `crates/prime-slider` (`publish = false`, features `ssr`/`hydrate`, зависимость `prime-core`, `web-sys` с нужными событиями и геометрией, dev-зависимости для wasm как у `prime-scrollpanel`), добавить в workspace; проверить `cargo check -p prime-slider --all-features`
- [x] 1.2 `state.rs`: `percent`, `value_from_percent`, `normalize`, `step_change`, `key_target`, `bar_percent`, `touch_percent`, классы корня и `data-p` по `sliderstyle.ts` и `Slider.dataP`; табличные тесты
- [x] 1.3 Реализовать `Slider`, `SliderPt`, `SliderChange`, `SliderSlideEnd`, `SliderOrientation` (значение и диапазон, разметка и положение ручек, `aria-*`, клавиатура, обновление значения, события); SSR-тесты (структура, положение, `aria-*`, ориентации, классы, `data-p`, `disabled`, `pt`, паритет ветвей)
- [x] 1.4 Ввод указателем: клик по полосе, перетаскивание мышью со слушателями на `window` только на время перетаскивания, касание, `animate`, `data-p-sliding`, RTL
- [x] 1.5 Браузерные тесты `tests/web.rs`: клавиши, клик по полосе, перетаскивание и снятие слушателей, `disabled`, диапазон, внешний сигнал и реактивные границы, сохранение ручек и фокуса
- [x] 1.6 rustdoc с примером; `cargo test -p prime-slider --all-features`

## 2. Тема, showcase, e2e

- [x] 2.1 Добавить `slider` в `theme.components` (`upstream.toml`), `cargo xtask gen-theme`, убедиться, что CSS других компонентов не изменился, подключить CSS
- [x] 2.2 Страница `/slider` (`basic`, `range`, `step`, `vertical`, `filter`, `input`), меню, маршрут, `stylesheet(&[..])`
- [x] 2.3 `e2e/tests/pages.json` и `e2e/tests/slider.spec.ts` (перетаскивание и клавиши; ждать `networkidle`); перезапустить сервер showcase перед прогоном; `npx playwright test slider`

## 3. Бенчмарки и бюджеты

- [x] 3.1 Сценарии `slider-1`, `slider-1000`, `case-slider`, бюджеты по измерению (сначала `cargo bench -p render-bench --bench alloc`)

## 4. Интеграция

- [x] 4.1 Добавить `prime-slider` в браузерный job `ci.yml` (3 места); `docs/slider.md`; clippy, тесты, deny, machete, `openspec validate port-slider`; CI `dev` зелёный

## Workflow follow-up

- Архивировать после CI.
