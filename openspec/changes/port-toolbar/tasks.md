# Tasks

## 1. Крейт и компонент

- [x] 1.1 Создать крейт `crates/prime-toolbar` (`publish = false`, features `ssr`/`hydrate`, зависимость `prime-core`), добавить в workspace; проверить `cargo check -p prime-toolbar --all-features`
- [x] 1.2 Реализовать `Toolbar` и `ToolbarPt` (слоты, `role`, `aria-labelledby`, две ветви, `children` перед областями); проверить SSR-тестами (структура, порядок, пустые области, атрибуты, `style`, `pt`) и паритетом ветвей
- [x] 1.3 rustdoc с примером; `cargo test -p prime-toolbar --all-features`

## 2. Тема, showcase, e2e

- [x] 2.1 Добавить `toolbar` в `theme.components` (`upstream.toml`), `cargo xtask gen-theme`, подключить CSS
- [x] 2.2 Страница `/toolbar` (`basic`, `custom`), меню, маршрут, `stylesheet(&[..])`
- [x] 2.3 `e2e/tests/pages.json` и `e2e/tests/toolbar.spec.ts`; `npx playwright test toolbar`

## 3. Бенчмарки и бюджеты

- [x] 3.1 Сценарии `toolbar-1`, `toolbar-1000`, `case-toolbar`, бюджеты по измерению

## 4. Интеграция

- [x] 4.1 `docs/toolbar.md`; clippy, тесты, deny, machete, `openspec validate port-toolbar`; CI `dev` зелёный

## Workflow follow-up

- Архивировать после CI.
