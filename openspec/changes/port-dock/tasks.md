# Tasks

## 1. Основа

- [x] 1.1 Добавить `dock` в `theme.components` (`upstream.toml`), `cargo xtask gen-theme`; проверить, что CSS остальных компонентов не изменился и что есть классы `p-dock*`

## 2. Крейт

- [x] 2.1 `crates/prime-dock`: `state.rs` (`root_classes`, `orientation`, `item_classes`, `item_id`, `key_action`, `step`) с табличными тестами
- [x] 2.2 Компонент `Dock`, `DockItem`, `DockCommand`, `DockPosition`, `DockPt`: корень, список, пункты, значок, бейдж, `item`, SSR-тесты, rustdoc
- [x] 2.3 Клик по пункту и `command`, фокус списка, клавиши по ориентации, `Home`/`End`/`Enter`/`Space`, `on_focus`/`on_blur`
- [x] 2.4 Мобильный режим по `matchMedia` и `breakpoint`; браузерные `tests/web.rs`

## 3. Showcase и e2e

- [x] 3.1 Страница `/dock` (`basic` с выбором положения), меню, маршрут, `stylesheet(&[..])`, крейт в workspace и showcase
- [x] 3.2 `e2e/tests/pages.json` и `dock.spec.ts` (разметка и положение, клик, клавиши, мобильный режим); ждать `networkidle`, перезапустить сервер showcase; `PW_CHANNEL=chrome npx playwright test dock`

## 4. Бенчмарки и бюджеты

- [x] 4.1 Сценарии `-1` и `-1000`, `case-dock` и бюджеты (сначала `cargo bench -p render-bench --bench alloc`; замер wasm — в фоне, с отдельным файлом бюджетов)

## 5. Интеграция

- [x] 5.1 `docs/dock.md` (отличия от PrimeNG, что не проверялось); `prime-dock` в браузерный job `ci.yml` (три места)
- [x] 5.2 `cargo fmt --all -- --check`, clippy с исключениями CI и `--all-features` (и для wasm-цели), тесты, deny, machete, `openspec validate port-dock`; CI `dev` зелёный

## Workflow follow-up

- Архивировать после CI.
- Следующее: Image.
