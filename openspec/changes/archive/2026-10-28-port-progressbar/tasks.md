# Tasks

## 1. Крейт и логика

- [x] 1.1 Создать крейт `crates/prime-progressbar` (`publish = false`, features `ssr`/`hydrate`, зависимость `prime-core`), добавить в workspace; проверить `cargo check -p prime-progressbar --all-features`
- [x] 1.2 Реализовать `ProgressBarMode`, `ProgressBarState`, `write_classes`, `data_p` по `progressbarstyle.ts` и `ProgressBar.dataP`; проверить таблицей юнит-тестов

## 2. Компонент

- [x] 2.1 Реализовать `ProgressBar` (ARIA, оба режима, подпись, `color`, шаблон, `pt`, две ветви); проверить SSR-тестами и паритетом
- [x] 2.2 Добавить rustdoc с примером; проверить `cargo test -p prime-progressbar --all-features`

## 3. Showcase и e2e

- [x] 3.1 Страница `/progressbar` (`basic`, `dynamic`, `indeterminate`, `template`), меню, CSS `progressbar`
- [x] 3.2 `e2e/tests/pages.json` и `e2e/tests/progressbar.spec.ts`; проверить `npx playwright test progressbar`

## 4. Бенчмарки и бюджеты

- [x] 4.1 Сценарии `progressbar-1`, `progressbar-1000`, `case-progressbar`, бюджеты по измерению

## 5. Интеграция

- [x] 5.1 `docs/progressbar.md`; clippy, тесты, deny, machete, `openspec validate port-progressbar`; CI `dev` зелёный

## Workflow follow-up

- Архивировать после CI; волна 1 завершена (Button, Badge, Tag, Chip, Avatar, Divider, Card, InputText, Checkbox, RadioButton, ToggleSwitch, Message, ProgressBar).
