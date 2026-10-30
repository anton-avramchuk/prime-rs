# Tasks

## 1. Крейт

- [x] 1.1 `crates/prime-popover`: `state.rs` (`root_classes`, `arrow_left`, `is_flipped`, `outside_click`, `retarget`) с табличными тестами
- [x] 1.2 `PopoverRef` (`show`, `show_at`, `toggle`, `toggle_at`, `hide`, `visible`), компонент `Popover` и `PopoverPt`: разметка, SSR-тесты, rustdoc
- [x] 1.3 Поведение: выравнивание, стрелка и `flipped`, `z-index`, клик снаружи/внутри/по цели, Escape, `resize`, прокрутка предка, смена цели, `autofocus`, `on_show`/`on_hide`; браузерные `tests/web.rs`

## 2. Тема, showcase, e2e

- [x] 2.1 Добавить модуль `popover` в `theme.components` (`upstream.toml`), `cargo xtask gen-theme`, убедиться, что CSS других компонентов не изменился
- [x] 2.2 Страница `/popover` (`basic`, `target`, `selectdata`, `template`), меню, маршрут, `stylesheet(&[..])`, крейт в workspace и showcase
- [x] 2.3 `e2e/tests/pages.json` и `popover.spec.ts` (клик, клик снаружи, Escape, другая цель, размещение над целью); ждать `networkidle`, перезапустить сервер showcase; `PW_CHANNEL=chrome npx playwright test popover`

## 3. Бенчмарки и бюджеты

- [x] 3.1 Сценарии `-1` и `-1000`, `case-popover` и бюджеты (сначала `cargo bench -p render-bench --bench alloc`)

## 4. Интеграция

- [x] 4.1 `docs/popover.md` (отличия от PrimeNG, что не проверялось); `prime-popover` в браузерный job `ci.yml` (три места)
- [x] 4.2 `cargo fmt --all -- --check`, clippy с `--all-features` (и для wasm-цели), тесты, deny, machete, `openspec validate port-popover`; CI `dev` зелёный

## Workflow follow-up

- Архивировать после CI.
- Следующее: Drawer.
