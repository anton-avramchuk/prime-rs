# Tasks

## 1. Основа

- [x] 1.1 `xtask gen-theme`: распознавать маркер `/** For PrimeNG **/` (тест рядом с существующими), добавить `drawer` в `theme.components` и `primeng_extra`, `cargo xtask gen-theme`; проверить, что CSS остальных компонентов не изменился и что `@keyframes p-animate-drawer-*` попали в CSS (иначе — решить вопрос из design.md)
- [x] 1.2 `prime-dom`: `block_body_scroll()` / `unblock_body_scroll()` со счётчиком и классом `p-overflow-hidden`; табличные тесты счётчика, браузерный тест на DOM

## 2. Крейт

- [x] 2.1 `crates/prime-drawer`: `state.rs` (`DrawerPosition`, `root_classes`, `enter_class`, `leave_class`, `mask_classes`, `is_topmost`, `mask_z`) с табличными тестами
- [x] 2.2 Компонент `Drawer` и `DrawerPt`: разметка, `Portal`, заголовок, кнопка закрытия, подвал, `headless`, SSR-тесты, rustdoc
- [x] 2.3 Поведение: `visible`/`rendered` и анимации входа и выхода, маска и клик по ней, Escape верхней панели, `block_scroll`, `FocusTrap`, `z-index`, `on_show`/`on_hide`; браузерные `tests/web.rs`

## 3. Showcase и e2e

- [x] 3.1 Страница `/drawer` (`basic`, `position`, `size`, `fullscreen`, `template`, `headless`), меню, маршрут, `stylesheet(&[..])`, крейт в workspace и showcase
- [x] 3.2 `e2e/tests/pages.json` и `drawer.spec.ts` (открытие, закрытие кнопкой, маской, Escape, позиции, блокировка прокрутки, Tab-цикл); ждать `networkidle`, перезапустить сервер showcase; `PW_CHANNEL=chrome npx playwright test drawer`

## 4. Бенчмарки и бюджеты

- [x] 4.1 Сценарии `-1` и `-1000`, `case-drawer` и бюджеты (сначала `cargo bench -p render-bench --bench alloc`)

## 5. Интеграция

- [x] 5.1 `docs/drawer.md` (отличия от PrimeNG, что не проверялось); `prime-drawer` в браузерный job `ci.yml` (три места)
- [ ] 5.2 `cargo fmt --all -- --check`, clippy с `--all-features` (и для wasm-цели), тесты, deny, machete, `openspec validate port-drawer`; CI `dev` зелёный

## Workflow follow-up

- Архивировать после CI.
- Следующее: Dialog.
