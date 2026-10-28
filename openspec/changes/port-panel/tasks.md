# Tasks

## 1. Общая логика сворачивания

- [x] 1.1 Модуль `prime-core::collapsible`: `CollapseState` (внешний `RwSignal<bool>` либо внутренний), `CollapseIds`, функции атрибутов ARIA; проверить юнит-тестами (таблица состояний, идентификаторы)
- [x] 1.2 Клиентская `animate` (классы `p-collapsible-*-active`, `animationend` + запасной таймер, завершение) и проставление `tabindex` потомкам; проверить `wasm-bindgen-test`

## 2. Крейт и компонент

- [x] 2.1 Создать крейт `crates/prime-panel` (`publish = false`, features `ssr`/`hydrate`, зависимости `prime-core`, `prime-icons` с `minus`, `plus`), добавить в workspace; проверить `cargo check -p prime-panel --all-features`
- [x] 2.2 Реализовать `PanelPt`, классы по `panelstyle.ts`, разметку (`macro_rules!`), две ветви, слоты; проверить SSR-тестами (структура, классы, ARIA, `show_header`, `icon_pos`, `pt`) и паритетом ветвей
- [x] 2.3 Переключение (`toggler = icon|header`), `on_before_toggle`, `on_after_toggle`, смена иконки; проверить `wasm-bindgen-test` (клик, клавиатура, нет двойного переключения, `aria-expanded`, `tabindex`)
- [x] 2.4 rustdoc с примером; `cargo test -p prime-panel --all-features`

## 3. Тема, showcase, e2e

- [x] 3.1 Добавить `panel` в `theme.components` (`upstream.toml`), `cargo xtask gen-theme`, подключить CSS
- [x] 3.2 Страница `/panel` (`basic`, `toggleable`, `header-toggler`, `template`), меню, маршрут, `stylesheet(&[..])`
- [x] 3.3 `e2e/tests/pages.json` и `e2e/tests/panel.spec.ts` (раскрытие/сворачивание, `aria-expanded`, анимационный класс); `npx playwright test panel`

## 4. Бенчмарки и бюджеты

- [x] 4.1 Сценарии `panel-1`, `panel-1000`, `case-panel` в `benches/render`, `benches/wasm-size`, бюджеты по измерению

## 5. Интеграция

- [x] 5.1 `docs/panel.md` (соответствие PrimeNG, отличия, производительность); clippy, тесты, deny, machete, `openspec validate port-panel`; `prime-panel` в браузерный job `ci.yml` (3 места); CI `dev` зелёный

## Workflow follow-up

- Архивировать после CI; следующим change — `port-fieldset` поверх `prime-core::collapsible`.
