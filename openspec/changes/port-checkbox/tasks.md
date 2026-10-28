# Tasks

## 1. Крейт и логика

- [ ] 1.1 Создать крейт `crates/prime-checkbox` (`publish = false`, features `ssr`/`hydrate`, зависимости `prime-core`, `prime-icons`, `web-sys`), добавить в workspace; проверить `cargo check -p prime-checkbox --all-features`
- [ ] 1.2 Реализовать `CheckboxState`, `write_classes`, `data_p` по `checkboxstyle.ts` и `Checkbox.dataP`; проверить таблицей юнит-тестов

## 2. Компонент

- [ ] 2.1 Реализовать `Checkbox` (разметка, классы, атрибуты, `pt`, иконки, две ветви); проверить SSR-тестами и паритетом
- [ ] 2.2 Реализовать режимы `binary` и набор, `indeterminate`, `readonly`, события; проверить `wasm-bindgen-test`
- [ ] 2.3 Добавить rustdoc с примером; проверить `cargo test -p prime-checkbox --all-features`

## 3. Showcase и e2e

- [ ] 3.1 Страница `/checkbox` (`basic`, `multiple`, `filled`, `sizes`, `indeterminate`, `invalid`, `disabled`), меню, CSS `checkbox`
- [ ] 3.2 `e2e/tests/pages.json` и `e2e/tests/checkbox.spec.ts`; проверить `npx playwright test checkbox`

## 4. Бенчмарки и бюджеты

- [ ] 4.1 Сценарии `checkbox-1`, `checkbox-1000`, `case-checkbox`, бюджеты по измерению

## 5. Интеграция

- [ ] 5.1 `docs/checkbox.md`; clippy, тесты, deny, machete, `openspec validate port-checkbox`; `prime-checkbox` в браузерный job CI; CI `dev` зелёный

## Workflow follow-up

- Архивировать после CI; далее RadioButton, ToggleSwitch, Message, ProgressBar.
