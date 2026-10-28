# Tasks

## 1. Крейт и логика

- [ ] 1.1 Создать крейт `crates/prime-avatar` (`publish = false`, features `ssr`/`hydrate`, зависимости `prime-core`, `web-sys`) и добавить в workspace; проверить `cargo check -p prime-avatar --all-features`
- [ ] 1.2 Реализовать `AvatarSize`, `AvatarShape` (с `Default`), `AvatarState`, `write_classes`, `data_p` по `avatarstyle.ts` и `Avatar.dataP`; проверить таблицей юнит-тестов по сценариям спеки (размеры, формы, `p-avatar-image`, `data-p` по умолчанию)

## 2. Компонент

- [ ] 2.1 Реализовать `Avatar` с двумя ветвями отрисовки, блоками метка/иконка/изображение с приоритетом, `aria-*` на корне, `pt` (`AvatarPt`) и `on_image_error`; проверить SSR-тестами всех сценариев спеки
- [ ] 2.2 Проверить `on_image_error` и поведение в браузере (`wasm-bindgen-test`: ошибка загрузки вызывает колбэк один раз)
- [ ] 2.3 Добавить таблицу паритета ветвей и rustdoc с примером; проверить `cargo test -p prime-avatar --all-features` и doc-тесты

## 3. Showcase и e2e

- [ ] 3.1 Добавить страницу `/avatar` (`label`, `icon`, `image`, `shape`, `size`, `template`), пункт меню и CSS `avatar` в оболочку; проверить `cargo leptos build` и страницу в Chrome без ошибок консоли
- [ ] 3.2 Добавить записи в `e2e/tests/pages.json` и `e2e/tests/avatar.spec.ts` (классы, `data-p`, приоритет блоков); проверить `npx playwright test avatar` локально

## 4. Бенчмарки и бюджеты

- [ ] 4.1 Добавить сценарии `avatar-1`, `avatar-1000` в `benches/render`, `case-avatar` в `benches/wasm-size`, бюджеты по измерению; проверить `cargo bench -p render-bench --bench alloc`, `cargo xtask alloc-budget` и `cargo xtask wasm-size`

## 5. Интеграция

- [ ] 5.1 Написать `docs/avatar.md` (соответствие PrimeNG, отличия); выполнить `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace --all-features` (кроме приложений), `cargo deny check`, `openspec validate port-avatar`; убедиться, что CI на `dev` зелёный

## Workflow follow-up

- Архивировать change после прохождения CI; следующие компоненты волны 1: Divider, Card.
