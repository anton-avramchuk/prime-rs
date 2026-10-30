# Tasks

## 1. Крейт

- [x] 1.1 `crates/prime-scrolltop`: `state.rs` (`is_visible`, `root_classes`, классы анимации) с табличными тестами
- [x] 1.2 Компонент `ScrollTop`: `ScrollTopTarget`, `ScrollTopBehavior`, `ScrollTopPt`, свойства, разметка поверх `Button`, SSR-тесты, rustdoc
- [x] 1.3 Поведение: подписка на `scroll` (окно/родитель) и её снятие, начальная проверка, фазы и анимация классами, z-index, клик с прокруткой; браузерные тесты `tests/web.rs`

## 2. Тема, showcase, e2e

- [x] 2.1 Добавить `scrolltop` в `theme.components` (`upstream.toml`), `cargo xtask gen-theme`, убедиться, что CSS других компонентов не изменился
- [x] 2.2 Страница `/scrolltop` (`basic`, `element`), меню, маршрут, `stylesheet(&[..])`, крейт в workspace и showcase
- [x] 2.3 `e2e/tests/pages.json` и `scrolltop.spec.ts` (порог, клик, возврат, `Parent`, клавиши); ждать `networkidle`, перезапустить сервер showcase; `PW_CHANNEL=chrome npx playwright test scrolltop`

## 3. Бенчмарки и бюджеты

- [x] 3.1 Сценарии `-1` и `-1000`, `case-scrolltop` и бюджеты (сначала `cargo bench -p render-bench --bench alloc`)

## 4. Интеграция

- [x] 4.1 `docs/scrolltop.md` (отличия от PrimeNG, что не проверялось); `prime-scrolltop` в браузерный job `ci.yml` (три места)
- [x] 4.2 `cargo fmt --all -- --check`, clippy с `--all-features` (и для wasm-цели), тесты, deny, machete, `openspec validate port-scrolltop`; CI `dev` зелёный

## Workflow follow-up

- Архивировать после CI.
- Следующее: Password.
