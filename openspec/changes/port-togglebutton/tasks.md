# Tasks

## 1. Крейт и компонент

- [x] 1.1 Создать крейт `crates/prime-togglebutton` (`publish = false`, features `ssr`/`hydrate`, зависимость `prime-core`, dev-зависимости для wasm как у `prime-toggleswitch`), добавить в workspace; проверить `cargo check -p prime-togglebutton --all-features`
- [x] 1.2 `state.rs`: классы корня и `data-p` по `togglebuttonstyle.ts` и `ToggleButton.dataP`, табличный тест
- [x] 1.3 Реализовать `ToggleButton`, `ToggleButtonPt`, `ToggleButtonChange`, `IconPos` (состояние, метки и иконки, слоты, атрибуты, клик и клавиши, `allow_empty`); SSR-тесты (структура, классы, `data-p`, атрибуты, `tabindex`, слоты, `pt`)
- [x] 1.4 Браузерные тесты `tests/web.rs`: клик, Enter и Space, `disabled`, `allow_empty`, один `on_change` на нажатие, `on_focus` и `on_blur`
- [x] 1.5 rustdoc с примером; `cargo test -p prime-togglebutton --all-features`

## 2. Тема, showcase, e2e

- [x] 2.1 Расширить `primeng_extra_block` в `xtask/src/gen_theme.rs` (маркер `/* For PrimeNG (…) */`), добавить `togglebutton` в `theme.components` и `primeng_extra` (`upstream.toml`), `cargo xtask gen-theme`, убедиться, что CSS других компонентов не изменился, подключить CSS
- [x] 2.2 Страница `/togglebutton` (`basic`, `customized`, `disabled`, `fluid`, `invalid`, `sizes`), меню, маршрут, `stylesheet(&[..])`
- [x] 2.3 `e2e/tests/pages.json` и `e2e/tests/togglebutton.spec.ts`; `npx playwright test togglebutton`

## 3. Бенчмарки и бюджеты

- [x] 3.1 Сценарии `togglebutton-1`, `togglebutton-1000`, `case-togglebutton`, бюджеты по измерению

## 4. Интеграция

- [ ] 4.1 Добавить `prime-togglebutton` в браузерный job `ci.yml` (3 места); `docs/togglebutton.md`; clippy, тесты, deny, machete, `openspec validate port-togglebutton`; CI `dev` зелёный

## Workflow follow-up

- Архивировать после CI.
