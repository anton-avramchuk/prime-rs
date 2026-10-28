# Tasks

## 1. Крейт и логика

- [x] 1.1 Создать крейт `crates/prime-textarea` (`publish = false`, features `ssr`/`hydrate`, зависимости `prime-core`, `web-sys`), добавить в workspace; проверить `cargo check -p prime-textarea --all-features`
- [x] 1.2 Реализовать `TextareaState`, `write_classes` по `textareastyle.ts` и чистую функцию `fit_height` для автоподгонки; проверить таблицей юнит-тестов

## 2. Компонент

- [x] 2.1 Реализовать `Textarea` (атрибуты, классы, значение дочерним текстом, вариант из `PrimeConfig`, `pt`, две ветви); проверить SSR-тестами и паритетом
- [x] 2.2 Реализовать двустороннюю привязку, события и `auto_resize` с `on_resize`; проверить `wasm-bindgen-test`
- [x] 2.3 Добавить rustdoc с примером; проверить `cargo test -p prime-textarea --all-features`

## 3. Showcase и e2e

- [x] 3.1 Страница `/textarea` (`basic`, `autoresize`, `filled`, `sizes`, `invalid`, `disabled`, `fluid`), меню, CSS `textarea`
- [x] 3.2 `e2e/tests/pages.json` и `e2e/tests/textarea.spec.ts`; проверить `npx playwright test textarea`

## 4. Бенчмарки и бюджеты

- [x] 4.1 Сценарии `textarea-1`, `textarea-1000`, `case-textarea`, бюджеты по измерению

## 5. Интеграция

- [x] 5.1 `docs/textarea.md`; clippy, тесты, deny, machete, `openspec validate port-textarea`; `prime-textarea` в браузерный job CI; CI `dev` зелёный

## Workflow follow-up

- Архивировать после CI.
