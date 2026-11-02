# Tasks

## 1. Основа

- [x] 1.1 Добавить `blockui` в `theme.components` (`upstream.toml`), `cargo xtask gen-theme`; проверить, что CSS остальных компонентов не изменился и что есть классы `p-blockui*`; убедиться, что `p-overlay-mask` и его анимации есть в `base.css`

## 2. Крейт

- [x] 2.1 `crates/prime-blockui`: `state.rs` (`mask_classes`, `aria_busy`, `mask_style`) с табличными тестами
- [x] 2.2 `BlockUI`, `BlockTarget`: свойства, вывод маски через `Portal` в цель или `body`, машина состояний, SSR-тест (маски нет), rustdoc
- [x] 2.3 Слой, блокировка прокрутки, `position` цели, выход по `animationend`/таймеру, повторное включение, освобождение при размонтировании; браузерные `tests/web.rs`

## 3. Showcase и e2e

- [x] 3.1 Страница `/blockui` (`basic`, `document`), меню, маршрут, `stylesheet(&[..])`, крейт в workspace и showcase
- [x] 3.2 `e2e/tests/pages.json` и `blockui.spec.ts` (маска над панелью, клик под маской, документ на три секунды); ждать `networkidle`, перезапустить сервер showcase; `PW_CHANNEL=chrome npx playwright test blockui`

## 4. Бенчмарки и бюджеты

- [x] 4.1 Сценарии `-1` и `-1000`, `case-blockui` и бюджеты (сначала `cargo bench -p render-bench --bench alloc`; замер wasm — в фоне, с отдельным файлом бюджетов)

## 5. Интеграция

- [x] 5.1 `docs/blockui.md` (отличия от PrimeNG, что не проверялось); `prime-blockui` в браузерный job `ci.yml` (три места)
- [x] 5.2 `cargo fmt --all -- --check`, clippy с исключениями CI и `--all-features` (и для wasm-цели), тесты, deny, machete, `openspec validate port-blockui`; CI `dev` зелёный

## Workflow follow-up

- Архивировать после CI.
- Следующее: SplitButton.
