# Tasks

## 1. Крейт и логика

- [x] 1.1 Создать крейт `crates/prime-radiobutton` (`publish = false`, features `ssr`/`hydrate`, зависимости `prime-core`, `web-sys`), добавить в workspace; проверить `cargo check -p prime-radiobutton --all-features`
- [x] 1.2 Реализовать `RadioButtonState`, `write_classes`, `data_p` по `radiobuttonstyle.ts` и `RadioButton.dataP`; проверить таблицей юнит-тестов

## 2. Компонент

- [x] 2.1 Реализовать `RadioButton` (разметка, классы, атрибуты, `pt`, две ветви); проверить SSR-тестами и паритетом
- [x] 2.2 Реализовать выбор через `model`, `disabled` и события; проверить `wasm-bindgen-test`
- [x] 2.3 Добавить rustdoc с примером; проверить `cargo test -p prime-radiobutton --all-features`

## 3. Showcase и e2e

- [x] 3.1 Страница `/radiobutton` (`group`, `filled`, `sizes`, `invalid`, `disabled`), меню, CSS `radiobutton`
- [x] 3.2 `e2e/tests/pages.json` и `e2e/tests/radiobutton.spec.ts`; проверить `npx playwright test radiobutton`

## 4. Бенчмарки и бюджеты

- [x] 4.1 Сценарии `radiobutton-1`, `radiobutton-1000`, `case-radiobutton`, бюджеты по измерению

## 5. Интеграция

- [x] 5.1 `docs/radiobutton.md`; clippy, тесты, deny, machete, `openspec validate port-radiobutton`; `prime-radiobutton` в браузерный job CI; CI `dev` зелёный

## Workflow follow-up

- Архивировать после CI; далее ToggleSwitch, Message, ProgressBar.
