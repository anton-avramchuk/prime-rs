# Tasks

## 1. Основа

- [x] 1.1 Добавить `stepper` в `theme.components` и `primeng_extra` (`upstream.toml`), `cargo xtask gen-theme`; проверить, что CSS остальных компонентов не изменился, что есть классы `p-step*`, `p-steppanel*`, `p-stepitem*`, `p-stepper-separator` и правило `.p-steppanel .p-motion`

## 2. Крейт

- [x] 2.1 `crates/prime-stepper`: `state.rs` (`step_id`, `panel_id`, `is_active`, `step_disabled`, `is_last`, классы) с табличными тестами
- [x] 2.2 `Stepper`, `StepList`, `Step`, `StepContext`, `StepperPt`: корень, значение, шаги с состояниями и разделителями, реестр шагов, SSR-тесты, rustdoc
- [x] 2.3 `StepPanels`, `StepPanel` (горизонтальная раскладка), `content`, переходы `activate`, линейный режим; браузерные `tests/web.rs`
- [x] 2.4 `StepItem` и вертикальная раскладка: значение элемента для `Step` и `StepPanel`, разделитель панели, скрытие и анимация раскрытия

## 3. Showcase и e2e

- [x] 3.1 Страница `/stepper` (`basic`, `linear`, `vertical`, `template`, `stepsonly`), меню, маршрут, `stylesheet(&[..])`, крейт в workspace и showcase
- [x] 3.2 `e2e/tests/pages.json` и `stepper.spec.ts` (разметка, клик по шагу, линейный режим, кнопки «Далее»/«Назад», вертикальный режим); ждать `networkidle`, перезапустить сервер showcase; `PW_CHANNEL=chrome npx playwright test stepper`

## 4. Бенчмарки и бюджеты

- [x] 4.1 Сценарии `-1` и `-1000`, `case-stepper` и бюджеты (сначала `cargo bench -p render-bench --bench alloc`; замер wasm — в фоне, с отдельным файлом бюджетов)

## 5. Интеграция

- [x] 5.1 `docs/stepper.md` (отличия от PrimeNG, что не проверялось); `prime-stepper` в браузерный job `ci.yml` (три места)
- [x] 5.2 `cargo fmt --all -- --check`, clippy с исключениями CI и `--all-features` (и для wasm-цели), тесты, deny, machete, `openspec validate port-stepper`; CI `dev` зелёный

## Workflow follow-up

- Архивировать после CI.
- Волна 3 закончена; дальше — по решению пользователя.
