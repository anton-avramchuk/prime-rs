# Tasks

## 1. Крейт и логика

- [x] 1.1 Создать крейт `crates/prime-tag` (`publish = false`, features `ssr`/`hydrate`, зависимости `prime-core`) и добавить в workspace; проверить `cargo check -p prime-tag --all-features`
- [x] 1.2 Реализовать `TagSeverity`, `TagState`, `write_classes`, `data_p` по `tagstyle.ts` и `Tag.dataP`; проверить таблицей юнит-тестов по сценариям спеки (серьёзности, `rounded`, `data-p`)

## 2. Компонент

- [x] 2.1 Реализовать `Tag` с двумя ветвями отрисовки (`all_static!`, `Either`, разметка в макросе), блоками `children`, иконки и метки, `pt` (`TagPt`); проверить SSR-тестами всех сценариев спеки
- [x] 2.2 Добавить таблицу паритета (значения и сигналы, HTML совпадает) и rustdoc с примером; проверить `cargo test -p prime-tag --all-features` и doc-тесты

## 3. Showcase и e2e

- [ ] 3.1 Добавить страницу `/tag` (`basic`, `icon`, `pill`, `severity`, `template`) и пункт меню, включить CSS `tag` в оболочку; проверить `cargo leptos build` и страницу в Chrome без ошибок консоли
- [ ] 3.2 Добавить записи в `e2e/tests/pages.json` и `e2e/tests/tag.spec.ts` (демонстрации, классы серьёзности, порядок содержимого); проверить `npx playwright test tag` локально

## 4. Бенчмарки и бюджеты

- [ ] 4.1 Добавить сценарии `tag-1`, `tag-1000` в `benches/render`, `case-tag` в `benches/wasm-size`, бюджеты в `benches/alloc-budgets.toml` и `benches/budgets.toml` по измерению; проверить `cargo bench -p render-bench --bench alloc`, `cargo xtask alloc-budget` и `cargo xtask wasm-size`

## 5. Интеграция

- [ ] 5.1 Написать `docs/tag.md` (соответствие PrimeNG, отличия); выполнить `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace --all-features` (кроме приложений), `cargo deny check`, `openspec validate port-tag`; убедиться, что CI на `dev` зелёный

## Workflow follow-up

- Архивировать change после прохождения CI; следующий компонент волны 1: Chip.
