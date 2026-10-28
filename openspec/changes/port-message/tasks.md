# Tasks

## 1. Крейт и логика

- [x] 1.1 Создать крейт `crates/prime-message` (`publish = false`, features `ssr`/`hydrate`, зависимости `prime-core`, `prime-icons` (`times`), `web-sys`), добавить в workspace; проверить `cargo check -p prime-message --all-features`
- [x] 1.2 Реализовать `MessageSeverity`, `MessageVariant`, `MessageState`, `write_classes`, `data_p` по `messagestyle.ts` и `Message.dataP`; проверить таблицей юнит-тестов

## 2. Компонент

- [x] 2.1 Реализовать `Message` (разметка, классы, текст и дети, слоты, `pt`, две ветви); проверить SSR-тестами и паритетом
- [x] 2.2 Реализовать закрытие, `on_close` и `life`; проверить `wasm-bindgen-test`
- [x] 2.3 Добавить rustdoc с примером; проверить `cargo test -p prime-message --all-features`

## 3. Showcase и e2e

- [x] 3.1 Страница `/message` (`basic`, `closable`, `severity`, `outlined`, `simple`, `sizes`, `icon`, `life`), меню, CSS `message`
- [x] 3.2 `e2e/tests/pages.json` и `e2e/tests/message.spec.ts`; проверить `npx playwright test message`

## 4. Бенчмарки и бюджеты

- [x] 4.1 Сценарии `message-1`, `message-1000`, `case-message`, бюджеты по измерению

## 5. Интеграция

- [x] 5.1 `docs/message.md`; clippy, тесты, deny, machete, `openspec validate port-message`; `prime-message` в браузерный job CI; CI `dev` зелёный

## Workflow follow-up

- Архивировать после CI; далее ProgressBar.
