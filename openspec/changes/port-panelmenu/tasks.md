# Tasks

## 1. Основа

- [x] 1.1 Добавить `panelmenu` в `theme.components` и в `primeng_extra` (`upstream.toml`; в `panelmenustyle.ts` есть блок `/*For PrimeNG*/`), `cargo xtask gen-theme`; проверить, что CSS остальных компонентов не изменился и есть классы `p-panelmenu*`
- [x] 1.2 Свериться с `onArrow*Key`, `flatItems`, `onItemToggle`, `changeFocusedItem` исходника; решить (D2), брать ли `prime-tieredmenu::model::{Node, build, item_at}`
- [x] 1.3 Значки `prime-icons`: chevronright, chevrondown (и для демонстрации, если нужны)

## 2. Крейт `prime-panelmenu`

- [x] 2.1 Чистая модель: `toggle_key`, видимые пункты, навигация (↓ ↑ ← → `Home` `End`), `aria` позиции; табличные тесты
- [x] 2.2 Компонент `PanelMenu`, `PanelMenuPt`, `PanelMenuItemContext`: корень, панели, заголовки, области, уровни, пункты, разделители, `item`/`submenu_icon`, SSR-тесты, rustdoc
- [x] 2.3 Раскрытие: одиночный и множественный режим, `expanded`, анимация `collapsible`, `PanelMenuRef` (`expand_all`/`collapse_all`/`toggle_all`)
- [x] 2.4 Фокус и клавиатура: `aria-activedescendant`, `p-focus`, переход между панелями, `Enter`/`Space`, `Tab`
- [x] 2.5 Браузерные `tests/web.rs` (один последовательный тест)

## 3. Showcase и e2e

- [x] 3.1 Страница `/panelmenu` (`basic`, `multiple`, `controlled`, `command`, `template`), меню, маршрут, `stylesheet(&[..])`, крейт в workspace и showcase
- [x] 3.2 `e2e/tests/pages.json` и `panelmenu.spec.ts` (стили, раскрытие, одиночный/множественный режим, клавиатура, команда); `networkidle`, перезапуск сервера showcase; `PW_CHANNEL=chrome npx playwright test panelmenu`

## 4. Бенчмарки и бюджеты

- [x] 4.1 Сценарии `-1` и `-1000`, `case-panelmenu` (с нужными `dep:`), бюджеты (сначала `cargo bench -p render-bench --bench alloc`; wasm — в фоне, отдельный файл бюджетов, не `xtask wasm-size --help`; бюджет в `budgets.toml` менять `sed` по номеру строки)

## 5. Интеграция

- [x] 5.1 `docs/panelmenu.md` (отличия от PrimeNG, что не проверялось); `prime-panelmenu` в браузерный job `ci.yml` (три места)
- [x] 5.2 `cargo fmt --all -- --check`, clippy с исключениями CI и `--all-features` (и для wasm), тесты (SSR по одному; wasm — по строке `test result`), deny, machete, `openspec validate port-panelmenu`; CI — один раз по завершении волны 5

## Workflow follow-up

- Архивировать; CI гоняется один раз по завершении волны 5.
- Дальше по плану волны 5: MegaMenu, Tree, TreeSelect, CascadeSelect, AutoComplete, InputMask.
