# Tasks

## 1. Крейт и компонент

- [x] 1.1 Добавить `skeleton` и `textarea` в `[theme] components` (`upstream.toml`) и выполнить `cargo xtask gen-theme`; создать крейт `crates/prime-skeleton` (`publish = false`, features `ssr`/`hydrate`, зависимость `prime-core`), добавить в workspace; проверить `cargo check -p prime-skeleton --all-features`
- [x] 1.2 Реализовать `Skeleton`, `SkeletonShape`, `SkeletonAnimation`, классы, стиль, `data-p`, `pt`, две ветви, rustdoc с примером и юнит-тест таблицы классов; проверить SSR-тестами и паритетом

## 2. Showcase и e2e

- [x] 2.1 Страница `/skeleton` (`shapes`, `card`, `list`), меню, CSS `skeleton`; проверить `cargo leptos build` и страницу в Chrome
- [x] 2.2 `e2e/tests/pages.json` и `e2e/tests/skeleton.spec.ts`; проверить `npx playwright test skeleton`

## 3. Бенчмарки и бюджеты

- [x] 3.1 Сценарии `skeleton-1`, `skeleton-1000`, `case-skeleton`, бюджеты по измерению

## 4. Интеграция

- [x] 4.1 `docs/skeleton.md`; clippy, тесты, deny, machete, `openspec validate port-skeleton`; CI `dev` зелёный

## Workflow follow-up

- Архивировать после CI; далее Textarea.
