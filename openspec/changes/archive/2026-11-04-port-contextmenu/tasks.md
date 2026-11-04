# Tasks

## 1. Основа

- [x] 1.1 Добавить `contextmenu` в `theme.components` (`upstream.toml`), `cargo xtask gen-theme`; проверить, что CSS остальных компонентов не изменился и есть классы `p-contextmenu*`
- [x] 1.2 Расширить `pub use model::…` в `prime-tieredmenu` нужными функциями (без изменения поведения)

## 2. Крейт `prime-contextmenu`

- [x] 2.1 `place` (положение у курсора) с табличными тестами
- [x] 2.2 Компонент `ContextMenu`, `ContextMenuPt`, `ContextMenuRef`: корень, список, уровни, пункты, разделители, группы, `item`/`submenu_icon`, SSR-тесты, rustdoc
- [x] 2.3 Путь активных пунктов, фокус, наведение (`dirty`, mobile), клавиатура, поиск, положение подменю
- [x] 2.4 Триггер: `target`, `global`, `trigger_event`, долгое касание; показ, перепозиционирование, `on_show`/`on_hide`
- [x] 2.5 Закрытие: клик вне (без правой кнопки и Ctrl), `resize`, лист, `Escape`, `Tab`; слой `Menu`; снятие слушателей
- [x] 2.6 Браузерные `tests/web.rs` (один последовательный тест)

## 3. Showcase и e2e

- [x] 3.1 Страница `/contextmenu` (`basic`, `command`, `template`), меню, маршрут, `stylesheet(&[..])`, крейт в workspace и showcase
- [x] 3.2 `e2e/tests/pages.json` и `contextmenu.spec.ts` (стили, положение у курсора, переворот у края, подменю, клавиатура, команда); `networkidle`, перезапуск сервера showcase; `PW_CHANNEL=chrome npx playwright test contextmenu`

## 4. Бенчмарки и бюджеты

- [x] 4.1 Сценарии `-1` и `-1000`, `case-contextmenu` (с `dep:prime-menu`, `dep:prime-tieredmenu`), бюджеты (сначала `cargo bench -p render-bench --bench alloc`; wasm — в фоне, отдельный файл бюджетов)

## 5. Интеграция

- [x] 5.1 `docs/contextmenu.md` (отличия от PrimeNG, что не проверялось); `prime-contextmenu` в браузерный job `ci.yml` (три места)
- [x] 5.2 `cargo fmt --all -- --check`, clippy с исключениями CI и `--all-features`, тесты (SSR по одному: `--exact`; эффекты только под `cfg(wasm32)`; результат wasm-тестов проверять по `test result`), deny, machete, `openspec validate port-contextmenu`; CI `dev` зелёный

## Workflow follow-up

- Архивировать после CI.
- Дальше по плану волны 5: Menubar.
