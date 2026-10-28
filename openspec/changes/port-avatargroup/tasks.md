# Tasks

## 1. Крейт и компонент

- [x] 1.1 Создать крейт `crates/prime-avatargroup` (`publish = false`, features `ssr`/`hydrate`, зависимость `prime-core`), добавить в workspace; проверить `cargo check -p prime-avatargroup --all-features`
- [x] 1.2 Реализовать `AvatarGroup` (корень, классы, `style`, `style_class`, `pt`, две ветви) и rustdoc с примером; проверить SSR-тестами, паритетом и `cargo test -p prime-avatargroup --all-features`

## 2. Showcase и e2e

- [x] 2.1 Добавить демонстрацию `avatargroup` на страницу `/avatar`, CSS темы уже подключён; проверить `cargo leptos build` и страницу в Chrome
- [x] 2.2 Добавить запись в `e2e/tests/pages.json` и проверки в `e2e/tests/avatar.spec.ts` (перекрытие аватаров); проверить `npx playwright test avatar`

## 3. Бенчмарки и бюджеты

- [x] 3.1 Сценарии `avatargroup-1`, `avatargroup-1000`, `case-avatargroup`, бюджеты по измерению

## 4. Интеграция

- [x] 4.1 `docs/avatargroup.md`, правка `docs/avatar.md`; clippy, тесты, deny, machete, `openspec validate port-avatargroup`; CI `dev` зелёный

## Workflow follow-up

- Архивировать после CI.
