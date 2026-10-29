# Tasks

## 1. Крейт и компоненты

- [x] 1.1 Создать крейт `crates/prime-inputgroup` (`publish = false`, features `ssr`/`hydrate`, зависимость `prime-core`), добавить в workspace; проверить `cargo check -p prime-inputgroup --all-features`
- [x] 1.2 Реализовать `InputGroup`, `InputGroupPt`, `InputGroupAddon`, `InputGroupAddonPt` (две ветви, `children`, `style_class`, `style`); проверить SSR-тестами (структура, классы, `style`, `pt`, вложенность) и паритетом ветвей
- [x] 1.3 rustdoc с примером; `cargo test -p prime-inputgroup --all-features`

## 2. Тема, showcase, e2e

- [x] 2.1 Добавить `inputgroup` в `theme.components` (`upstream.toml`), `cargo xtask gen-theme`, подключить CSS
- [x] 2.2 Страница `/inputgroup` (`basic`, `button`, `checkbox`, `multiple`), меню, маршрут, `stylesheet(&[..])`
- [x] 2.3 `e2e/tests/pages.json` и `e2e/tests/inputgroup.spec.ts`; `npx playwright test inputgroup`

## 3. Бенчмарки и бюджеты

- [x] 3.1 Сценарии `inputgroup-1`, `inputgroup-1000`, `case-inputgroup`, бюджеты по измерению

## 4. Интеграция

- [ ] 4.1 `docs/inputgroup.md`; clippy, тесты, deny, machete, `openspec validate port-inputgroup`; CI `dev` зелёный

## Workflow follow-up

- Архивировать после CI.
