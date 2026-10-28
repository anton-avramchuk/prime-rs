# Tasks

## 1. Крейт и логика

- [x] 1.1 Создать крейт `crates/prime-fieldset` (`publish = false`, features `ssr`/`hydrate`, зависимости `prime-core`, `prime-icons` с `minus`, `plus`, `web-sys`), добавить в workspace; проверить `cargo check -p prime-fieldset --all-features`
- [x] 1.2 Реализовать `write_root_classes`, `data_p` (`state.rs`) с таблицей юнит-тестов

## 2. Компонент

- [x] 2.1 Реализовать `Fieldset` и `FieldsetPt` (разметка `macro_rules!`, две ветви, легенда, кнопка, иконки и слоты, `bind_region`); проверить SSR-тестами (структура, классы, ARIA, слоты, `pt`) и паритетом ветвей
- [x] 2.2 Переключение, `on_before_toggle`, `on_after_toggle`; проверить `wasm-bindgen-test` (клик, сигнал, `aria-expanded`, анимация, `tabindex`)
- [x] 2.3 rustdoc с примером; `cargo test -p prime-fieldset --all-features`

## 3. Тема, showcase, e2e

- [x] 3.1 Добавить `fieldset` в `theme.components` (`upstream.toml`), `cargo xtask gen-theme`, подключить CSS
- [x] 3.2 Страница `/fieldset` (`basic`, `toggleable`, `template`), меню, маршрут, `stylesheet(&[..])`
- [x] 3.3 `e2e/tests/pages.json` и `e2e/tests/fieldset.spec.ts`; `npx playwright test fieldset`

## 4. Бенчмарки и бюджеты

- [x] 4.1 Сценарии `fieldset-1`, `fieldset-1000`, `case-fieldset`, бюджеты по измерению

## 5. Интеграция

- [x] 5.1 `docs/fieldset.md`; clippy, тесты, deny, machete, `openspec validate port-fieldset`; `prime-fieldset` в браузерный job CI; CI `dev` зелёный

## Workflow follow-up

- Архивировать после CI.
