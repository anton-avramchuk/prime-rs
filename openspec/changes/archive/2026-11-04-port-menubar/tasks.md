# Tasks

## 1. Основа

- [x] 1.1 Добавить `menubar` в `theme.components` (`upstream.toml`; в `primeng_extra` — если в `menubarstyle.ts` есть блок `For PrimeNG`), `cargo xtask gen-theme`; проверить, что CSS остальных компонентов не изменился и есть классы `p-menubar*`
- [x] 1.2 Свериться с `onArrow*Key`/`onItemChange` исходника; добавить в `prime-tieredmenu::model` функции горизонтального корня (← → ↓) с табличными тестами, не меняя поведение `TieredMenu`
- [x] 1.3 Значки `prime-icons`: angleright, angledown, bars (и для демонстрации)

## 2. Крейт `prime-menubar`

- [x] 2.1 Компонент `Menubar`, `MenubarPt`, `MenubarItemContext`: корень, `start`/`end`, кнопка, корневой список, уровни, пункты, разделители, группы, `item`/`submenu_icon`/`menu_icon`, SSR-тесты, rustdoc
- [x] 2.2 Путь активных пунктов, фокус, наведение (`auto_display`, mobile), `auto_hide`, клик вне, `resize`
- [x] 2.3 Клавиатура корня и подменю, поиск, `aria-activedescendant`, `on_focus`/`on_blur`
- [x] 2.4 Мобильный режим: `matchMedia`, кнопка, слой `Menu`, возврат фокуса, клавиши кнопки
- [x] 2.5 Положение вложенных подменю: отдельной логики нет, положение задаёт CSS темы (как в PrimeNG)
- [x] 2.6 Браузерные `tests/web.rs` (один последовательный тест)

## 3. Showcase и e2e

- [x] 3.1 Страница `/menubar` (`basic`, `command`, `template`), меню, маршрут, `stylesheet(&[..])`, крейт в workspace и showcase
- [x] 3.2 `e2e/tests/pages.json` и `menubar.spec.ts` (стили, подменю под пунктом, клавиатура, команда, мобильная кнопка); `networkidle`, перезапуск сервера showcase; `PW_CHANNEL=chrome npx playwright test menubar`

## 4. Бенчмарки и бюджеты

- [x] 4.1 Сценарии `-1` и `-1000`, `case-menubar` (с `dep:prime-menu`, `dep:prime-tieredmenu`), бюджеты (сначала `cargo bench -p render-bench --bench alloc`; wasm — в фоне, отдельный файл бюджетов, не `xtask wasm-size --help`)

## 5. Интеграция

- [x] 5.1 `docs/menubar.md` (отличия от PrimeNG, что не проверялось); `prime-menubar` в браузерный job `ci.yml` (три места)
- [x] 5.2 `cargo fmt --all -- --check`, clippy с исключениями CI и `--all-features` (и для wasm), тесты (SSR по одному; wasm — по строке `test result`), deny, machete, `openspec validate port-menubar`; CI `dev` зелёный

## Workflow follow-up

- Архивировать после CI.
- Дальше по плану волны 5: PanelMenu, MegaMenu, Tree, TreeSelect, CascadeSelect, AutoComplete, InputMask.
