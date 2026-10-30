# Tasks

## 1. Крейт

- [x] 1.1 `crates/prime-splitter`: `state.rs` (`initial_sizes`, `flex_basis`, `drag_delta`, `apply`, `key_step`, классы, `data_p`) с табличными тестами
- [x] 1.2 Компонент `Splitter`: свойства, разметка (панели, разделители, ручки), `SplitterPt`, SSR-тесты, rustdoc
- [x] 1.3 Поведение: перетаскивание мышью и касанием, `min_sizes`, клавиатура, события, хранилище; браузерные тесты `tests/web.rs`

## 2. Тема, showcase, e2e

- [x] 2.1 Добавить `splitter` в `theme.components` (`upstream.toml`), `cargo xtask gen-theme`, убедиться, что CSS других компонентов не изменился
- [x] 2.2 Страница `/splitter` (`horizontal`, `vertical`, `size`, `nested`), меню, маршрут, `stylesheet(&[..])`, крейт в workspace и showcase
- [x] 2.3 `e2e/tests/pages.json` и `splitter.spec.ts` (перетаскивание мышью, стрелки, минимальные размеры, вложенность); ждать `networkidle`, перезапустить сервер showcase; `PW_CHANNEL=chrome npx playwright test splitter`

## 3. Бенчмарки и бюджеты

- [x] 3.1 Сценарии `-1` и `-1000`, `case-splitter` и бюджеты (сначала `cargo bench -p render-bench --bench alloc`)

## 4. Интеграция

- [x] 4.1 `docs/splitter.md` (отличия от PrimeNG, что не проверялось); `prime-splitter` в браузерный job `ci.yml` (три места)
- [x] 4.2 `cargo fmt --all -- --check`, clippy с `--all-features` (и для wasm-цели), тесты, deny, machete, `openspec validate port-splitter`; CI `dev` зелёный

## Workflow follow-up

- Архивировано после CI.
- Следующее: Tabs.
