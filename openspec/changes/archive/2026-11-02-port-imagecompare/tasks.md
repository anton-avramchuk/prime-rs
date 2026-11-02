# Tasks

## 1. Основа

- [x] 1.1 Добавить `imagecompare` в `theme.components` (`upstream.toml`), `cargo xtask gen-theme`; проверить, что CSS остальных компонентов не изменился и что есть классы `p-imagecompare`, `p-imagecompare-slider`

## 2. Крейт

- [x] 2.1 `crates/prime-imagecompare`: `state.rs` (`clip_path`) с табличными тестами
- [x] 2.2 Компонент `ImageCompare`, `ImageComparePt`: корень, `left`/`right`, ползунок, атрибуты, обработчик ввода (`clip-path`, `rtl`), SSR-тесты, rustdoc
- [x] 2.3 Браузерные `tests/web.rs`: ввод значения, `rtl`, повторный ввод, ввод без соседа

## 3. Showcase и e2e

- [x] 3.1 Страница `/imagecompare` (`basic`, `responsive`), меню, маршрут, `stylesheet(&[..])`, крейт в workspace и showcase
- [x] 3.2 `e2e/tests/pages.json` и `imagecompare.spec.ts` (разметка, перетаскивание ручки, `clip-path`); ждать `networkidle`, перезапустить сервер showcase; `PW_CHANNEL=chrome npx playwright test imagecompare`

## 4. Бенчмарки и бюджеты

- [x] 4.1 Сценарии `-1` и `-1000`, `case-imagecompare` и бюджеты (сначала `cargo bench -p render-bench --bench alloc`; замер wasm — в фоне, с отдельным файлом бюджетов)

## 5. Интеграция

- [x] 5.1 `docs/imagecompare.md` (отличия от PrimeNG, что не проверялось); `prime-imagecompare` в браузерный job `ci.yml` (три места)
- [x] 5.2 `cargo fmt --all -- --check`, clippy с исключениями CI и `--all-features` (и для wasm-цели), тесты, deny, machete, `openspec validate port-imagecompare`; CI `dev` зелёный

## Workflow follow-up

- Архивировать после CI.
- Следующее: BlockUI.
