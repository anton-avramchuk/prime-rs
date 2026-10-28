# Tasks

## 1. Крейт и логика

- [x] 1.1 Создать крейт `crates/prime-divider` (`publish = false`, features `ssr`/`hydrate`, зависимость `prime-core`) и добавить в workspace; проверить `cargo check -p prime-divider --all-features`
- [x] 1.2 Реализовать `DividerLayout`, `DividerType`, `DividerAlign`, `DividerState`, `write_classes`, `align_style`, `data_p` по `dividerstyle.ts` и `Divider.dataP`; проверить таблицами юнит-тестов все сочетания `layout` × `align` (классы, стиль) и `data-p`

## 2. Компонент

- [x] 2.1 Реализовать `Divider` с двумя ветвями отрисовки, блоком `p-divider-content`, `role`, `aria-orientation`, inline-стилем и `pt` (`DividerPt`); проверить SSR-тестами всех сценариев спеки
- [x] 2.2 Добавить таблицу паритета ветвей и rustdoc с примером; проверить `cargo test -p prime-divider --all-features` и doc-тесты

## 3. Showcase и e2e

- [ ] 3.1 Добавить страницу `/divider` (`basic`, `content`, `type`, `vertical`, `login`), пункт меню и CSS `divider` в оболочку; проверить `cargo leptos build` и страницу в Chrome без ошибок консоли
- [ ] 3.2 Добавить записи в `e2e/tests/pages.json` и `e2e/tests/divider.spec.ts` (классы, `role`, стиль, содержимое); проверить `npx playwright test divider` локально

## 4. Бенчмарки и бюджеты

- [ ] 4.1 Добавить сценарии `divider-1`, `divider-1000` в `benches/render`, `case-divider` в `benches/wasm-size`, бюджеты по измерению; проверить `cargo bench -p render-bench --bench alloc`, `cargo xtask alloc-budget` и `cargo xtask wasm-size`

## 5. Интеграция

- [ ] 5.1 Написать `docs/divider.md` (соответствие PrimeNG, отличия); выполнить `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace --all-features` (кроме приложений), `cargo deny check`, `openspec validate port-divider`; убедиться, что CI на `dev` зелёный

## Workflow follow-up

- Архивировать change после прохождения CI; следующий компонент волны 1: Card.
