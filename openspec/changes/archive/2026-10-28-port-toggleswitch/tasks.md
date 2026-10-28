# Tasks

## 1. Крейт и логика

- [x] 1.1 Создать крейт `crates/prime-toggleswitch` (`publish = false`, features `ssr`/`hydrate`, зависимости `prime-core`, `web-sys`), добавить в workspace; проверить `cargo check -p prime-toggleswitch --all-features`
- [x] 1.2 Реализовать `ToggleSwitchState`, `write_classes`, `data_p` по `toggleswitchstyle.ts` и `ToggleSwitch.dataP`; проверить таблицей юнит-тестов

## 2. Компонент

- [x] 2.1 Реализовать `ToggleSwitch` (разметка, классы, стиль корня, атрибуты, `pt`, слот, две ветви); проверить SSR-тестами и паритетом
- [x] 2.2 Реализовать клик по корню, `readonly`/`disabled`, возврат фокуса и события; проверить `wasm-bindgen-test`
- [x] 2.3 Добавить rustdoc с примером; проверить `cargo test -p prime-toggleswitch --all-features`

## 3. Showcase и e2e

- [x] 3.1 Страница `/toggleswitch` (`basic`, `preselection`, `template`, `invalid`, `disabled`), меню, CSS `toggleswitch`
- [x] 3.2 `e2e/tests/pages.json` и `e2e/tests/toggleswitch.spec.ts`; проверить `npx playwright test toggleswitch`

## 4. Бенчмарки и бюджеты

- [x] 4.1 Сценарии `toggleswitch-1`, `toggleswitch-1000`, `case-toggleswitch`, бюджеты по измерению

## 5. Интеграция

- [x] 5.1 `docs/toggleswitch.md`; clippy, тесты, deny, machete, `openspec validate port-toggleswitch`; `prime-toggleswitch` в браузерный job CI; CI `dev` зелёный

## Workflow follow-up

- Архивировать после CI; далее Message, ProgressBar.
