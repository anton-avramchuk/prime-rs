# Tasks

## 1. Крейт

- [x] 1.1 `crates/prime-tabs`: `state.rs` (идентификаторы, классы, `tabindex_for`, `key_action`, `wrap_target`, `should_render`, прокрутка кнопок) с табличными тестами
- [x] 1.2 Компоненты `Tabs`, `TabList`, `Tab`, `TabPanels`, `TabPanel`, `TabListPt`: контекст, разметка, SSR-тесты, rustdoc
- [x] 1.3 Поведение: клик, фокус, клавиши, roving `tabindex`, индикатор, кнопки прокрутки, `lazy`; браузерные тесты `tests/web.rs`

## 2. Тема, showcase, e2e

- [x] 2.1 Добавить модуль `tabs` в `theme.components` (`upstream.toml`), `cargo xtask gen-theme`, убедиться, что CSS других компонентов не изменился
- [x] 2.2 Страница `/tabs` (`basic`, `controlled`, `disabled`, `scrollable`, `dynamic`, `lazy`, `customtemplate`), меню, маршрут, `stylesheet(&[..])`, крейт в workspace и showcase
- [x] 2.3 `e2e/tests/pages.json` и `tabs.spec.ts` (клик, клавиатура, отключённая, индикатор, прокрутка, ленивая); ждать `networkidle`, перезапустить сервер showcase; `PW_CHANNEL=chrome npx playwright test tabs`

## 3. Бенчмарки и бюджеты

- [x] 3.1 Сценарии `-1` и `-1000`, `case-tabs` и бюджеты (сначала `cargo bench -p render-bench --bench alloc`)

## 4. Интеграция

- [x] 4.1 `docs/tabs.md` (отличия от PrimeNG, что не проверялось); `prime-tabs` в браузерный job `ci.yml` (три места)
- [x] 4.2 `cargo fmt --all -- --check`, clippy с `--all-features` (и для wasm-цели), тесты, deny, machete, `openspec validate port-tabs`; CI `dev` зелёный

## Workflow follow-up

- Архивировать после CI.
- Следующее: Popover.
