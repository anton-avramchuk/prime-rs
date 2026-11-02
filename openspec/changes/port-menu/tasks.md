# Tasks

## 1. Основа

- [x] 1.1 Добавить `menu` в `theme.components` (`upstream.toml`), `cargo xtask gen-theme`; проверить, что CSS остальных компонентов не изменился и что `p-menu-*` и `p-menu-overlay` есть (иначе — решить вопрос из design.md)

## 2. Крейт

- [x] 2.1 `crates/prime-menu`: `state.rs` (`MenuItem`, `flatten`, `item_id`, `root_classes`, `item_classes`, `root_style`, `enabled_ids`, `next_id`, `prev_id`, `should_prevent_default`, `can_run_command`) с табличными тестами
- [x] 2.2 Компонент `Menu` и `MenuPt`: корень, `start`/`end`, список, пункты, группы, разделители, `item`/`submenu_header`, SSR-тесты, rustdoc
- [x] 2.3 Поведение статичного режима: клик и `command`, отключённые пункты, фокус и клавиши, `on_focus`/`on_blur`
- [x] 2.4 `MenuRef` и всплывающий режим: показ/скрытие, выравнивание, слой, клик вне, resize, прокрутка, Escape/Tab/Alt+↑, `on_show`/`on_hide`; браузерные `tests/web.rs`

## 3. Showcase и e2e

- [x] 3.1 Страница `/menu` (`basic`, `group`, `command`, `popup`, `template`), меню, маршрут, `stylesheet(&[..])`, крейт в workspace и showcase
- [x] 3.2 `e2e/tests/pages.json` и `menu.spec.ts` (разметка, группы, клик и команда, клавиши, popup: показ, клик вне, Escape); ждать `networkidle`, перезапустить сервер showcase; `PW_CHANNEL=chrome npx playwright test menu`

## 4. Бенчмарки и бюджеты

- [x] 4.1 Сценарии `-1` и `-1000`, `case-menu` и бюджеты (сначала `cargo bench -p render-bench --bench alloc`; замер wasm — в фоне)

## 5. Интеграция

- [x] 5.1 `docs/menu.md` (отличия от PrimeNG, что не проверялось); `prime-menu` в браузерный job `ci.yml` (три места)
- [x] 5.2 `cargo fmt --all -- --check`, clippy с `--all-features` (и для wasm-цели), тесты, deny, machete, `openspec validate port-menu`; CI `dev` зелёный

## Workflow follow-up

- Архивировать после CI.
- Следующее: Listbox.
