# Tasks

## 1. Основа

- [x] 1.1 Добавить `tieredmenu` в `theme.components` (`upstream.toml`), `cargo xtask gen-theme`; проверить, что CSS остальных компонентов не изменился и что есть классы `p-tieredmenu*`
- [x] 1.2 Свериться с `nestedPosition` из `@primeuix/utils` и с набором значков `prime-icons` (angleright и значки для демонстрации)

## 2. Крейт `prime-tieredmenu`

- [x] 2.1 `model.rs`: узлы дерева, допустимость, поиск следующего/предыдущего/первого/последнего, поиск по символам, клик, смена пути, стрелки ←/→, `aria-setsize`/`aria-posinset`, сторона подменю, идентификаторы — с табличными тестами
- [x] 2.2 Компонент `TieredMenu`, `TieredMenuPt`: корень, корневой список, уровни и пункты, разделители, группы со стрелкой, `item`/`submenu_icon`, SSR-тесты, rustdoc
- [x] 2.3 Путь активных пунктов и фокус: клик, повторный клик, лист, наведение (`auto_display`, `dirty`, mobile), закрытие
- [x] 2.4 Клавиатура: ↓/↑/→/←/`Home`/`End`/`Enter`/`Space`/`Escape`/`Tab`/`Alt+↑`, поиск по символам; `aria-activedescendant`
- [x] 2.5 Положение подменю (сторона по месту), `matchMedia` для `p-tieredmenu-mobile`
- [x] 2.6 Режим `popup`: `TieredMenuRef`, выравнивание, слой, клик вне, `resize`, прокрутка, возврат фокуса, `on_show`/`on_hide`
- [x] 2.7 Браузерные `tests/web.rs` (один последовательный тест)

## 3. Showcase и e2e

- [x] 3.1 Страница `/tieredmenu` (`basic`, `popup`, `command`, `template`), меню, маршрут, `stylesheet(&[..])`, крейт в workspace и showcase
- [x] 3.2 `e2e/tests/pages.json` и `tieredmenu.spec.ts` (стили, раскрытие вправо, зеркалирование у края, клавиатура, popup, команда); ждать `networkidle`, перезапустить сервер showcase; `PW_CHANNEL=chrome npx playwright test tieredmenu`

## 4. Бенчмарки и бюджеты

- [x] 4.1 Сценарии `-1` и `-1000`, `case-tieredmenu` (с `dep:prime-menu`) и бюджеты (сначала `cargo bench -p render-bench --bench alloc`; замер wasm — в фоне, с отдельным файлом бюджетов)

## 5. Интеграция

- [x] 5.1 `docs/tieredmenu.md` (отличия от PrimeNG, что не проверялось); `prime-tieredmenu` в браузерный job `ci.yml` (три места)
- [x] 5.2 `cargo fmt --all -- --check`, clippy с исключениями CI и `--all-features` (и для wasm-цели), тесты (SSR по одному: `--exact`; эффекты только под `cfg(wasm32)`), deny, machete, `openspec validate port-tieredmenu`; CI `dev` зелёный

## Workflow follow-up

- Архивировать после CI.
- Волна 4 закончена; дальше — по плану следующей волны (остальные меню: ContextMenu, Menubar, MegaMenu, PanelMenu) и, отдельным изменением, `TieredMenu` внутри `SplitButton`.
