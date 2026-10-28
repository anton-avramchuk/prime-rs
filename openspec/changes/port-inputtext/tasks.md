# Tasks

## 1. Крейт и логика

- [ ] 1.1 Создать крейт `crates/prime-inputtext` (`publish = false`, features `ssr`/`hydrate`, зависимости `prime-core`, `web-sys`) и добавить в workspace; проверить `cargo check -p prime-inputtext --all-features`
- [ ] 1.2 Реализовать `InputTextState`, `write_classes`, `data_p` по `inputtextstyle.ts` и `InputText.dataP`; проверить таблицей юнит-тестов сценарии спеки (размер, вариант, `invalid`, `fluid`, `filled`, порядок `data-p`)

## 2. Компонент

- [ ] 2.1 Реализовать `InputText` (атрибуты, классы, `data-p`, вариант из `PrimeConfig`, `pt`, две ветви отрисовки); проверить SSR-тестами структуры, атрибутов, классов, значения в HTML и паритета ветвей
- [ ] 2.2 Реализовать двустороннюю привязку `value` и события `on_input`, `on_change`, `on_focus`, `on_blur`; проверить `wasm-bindgen-test` (ввод пишет в сигнал, сигнал обновляет поле, события, `disabled`/`readonly`)
- [ ] 2.3 Добавить rustdoc с примером; проверить `cargo test -p prime-inputtext --all-features` и doc-тесты

## 3. Showcase и e2e

- [ ] 3.1 Добавить страницу `/inputtext` (`basic`, `filled`, `sizes`, `invalid`, `disabled`, `fluid`), пункт меню и CSS `inputtext`; проверить `cargo leptos build` и страницу в Chrome без ошибок консоли
- [ ] 3.2 Добавить записи в `e2e/tests/pages.json` и `e2e/tests/inputtext.spec.ts` (ввод, классы, `invalid`, `disabled`); проверить `npx playwright test inputtext` локально

## 4. Бенчмарки и бюджеты

- [ ] 4.1 Добавить сценарии `inputtext-1`, `inputtext-1000` в `benches/render`, `case-inputtext` в `benches/wasm-size`, бюджеты по измерению; проверить `cargo bench -p render-bench --bench alloc`, `cargo xtask alloc-budget` и `cargo xtask wasm-size`

## 5. Интеграция

- [ ] 5.1 Написать `docs/inputtext.md` (соответствие PrimeNG, отличия, модель значения как образец для полей ввода); выполнить `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace --all-features` (кроме приложений), `cargo deny check`, `openspec validate port-inputtext`; убедиться, что CI на `dev` зелёный

## Workflow follow-up

- Архивировать change после прохождения CI; следующие компоненты волны 1: Checkbox, RadioButton, ToggleSwitch, Message, ProgressBar.
