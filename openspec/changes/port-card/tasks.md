# Tasks

## 1. Крейт

- [x] 1.1 Создать крейт `crates/prime-card` (`publish = false`, features `ssr`/`hydrate`, зависимость `prime-core`) и добавить в workspace; проверить `cargo check -p prime-card --all-features`

## 2. Компонент

- [x] 2.1 Реализовать `Card` с двумя ветвями отрисовки, блоками header/body/title/subtitle/content/footer, приоритетом слотов над текстом и `pt` (`CardPt`); проверить SSR-тестами всех сценариев спеки
- [x] 2.2 Добавить таблицу паритета ветвей и rustdoc с примером; проверить `cargo test -p prime-card --all-features` и doc-тесты

## 3. Showcase и e2e

- [x] 3.1 Добавить страницу `/card` (`basic`, `advanced` с Button в подвале), пункт меню и CSS `card` (уже в оболочке); проверить `cargo leptos build` и страницу в Chrome без ошибок консоли
- [x] 3.2 Добавить записи в `e2e/tests/pages.json` и `e2e/tests/card.spec.ts` (структура, порядок блоков, подвал с кнопками); проверить `npx playwright test card` локально

## 4. Бенчмарки и бюджеты

- [x] 4.1 Добавить сценарии `card-1`, `card-1000` в `benches/render`, `case-card` в `benches/wasm-size`, бюджеты по измерению; проверить `cargo bench -p render-bench --bench alloc`, `cargo xtask alloc-budget` и `cargo xtask wasm-size`

## 5. Интеграция

- [x] 5.1 Написать `docs/card.md` (соответствие PrimeNG, отличия); выполнить `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace --all-features` (кроме приложений), `cargo deny check`, `openspec validate port-card`; убедиться, что CI на `dev` зелёный

## Workflow follow-up

- Архивировать change после прохождения CI; следующие компоненты волны 1: InputText, Checkbox, RadioButton, ToggleSwitch, Message, ProgressBar.
