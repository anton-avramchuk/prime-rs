# Tasks

## 1. Основа

- [x] 1.1 Спайк `use:` на корне компонентов проекта (`Button`, `InputText`) и на обычном элементе: принимается ли, вызывается ли при построении, сколько раз при `rebuild`; результат записать в design.md (D1) и при необходимости выбрать запасной вариант
- [x] 1.2 Добавить `tooltip` в `theme.components` (`upstream.toml`), `cargo xtask gen-theme`; проверить, что CSS остальных компонентов не изменился, а стрелка и позиция подсказки в CSS есть

## 2. Крейт

- [x] 2.1 `crates/prime-tooltip`: `state.rs` (`TooltipPosition`, `TooltipEvent`, `priority`, `place`, `arrow_style`, `out_of_bounds`, `root_classes`, `activation_delay`, `has_ellipsis`, `leave_keeps_open`) с табличными тестами
- [x] 2.2 `TooltipOptions` (строитель, `TooltipContent`) и директива `tooltip`: таблица состояний по элементу, слушатели `hover`/`focus`/`both`, SSR-тесты (разметка не меняется), rustdoc
- [x] 2.3 Показ и скрытие: создание подсказки в `body`, позиция и перенос по приоритету, стрелка, задержки и `life`, `auto_hide`, Escape/resize/scroll, слой, содержимое (текст, HTML, `View`), реактивные текст и `disabled`, `show_on_ellipsis`, очистка; браузерные `tests/web.rs`

## 3. Showcase и e2e

- [x] 3.1 Страница `/tooltip` (`position`, `event`, `delay`, `autohide`, `options`, `custom`), меню, маршрут, `stylesheet(&[..])`, крейт в workspace и showcase
- [x] 3.2 `e2e/tests/pages.json` и `tooltip.spec.ts` (наведение мышью, фокус клавиатурой, четыре стороны, задержка, автоскрытие, Escape); ждать `networkidle`, перезапустить сервер showcase; `PW_CHANNEL=chrome npx playwright test tooltip`

## 4. Бенчмарки и бюджеты

- [x] 4.1 Сценарии `-1` и `-1000`, `case-tooltip` и бюджеты (сначала `cargo bench -p render-bench --bench alloc`)

## 5. Интеграция

- [x] 5.1 `docs/tooltip.md` (отличия от PrimeNG, что не проверялось); `prime-tooltip` в браузерный job `ci.yml` (три места)
- [ ] 5.2 `cargo fmt --all -- --check`, clippy с `--all-features` (и для wasm-цели), тесты, deny, machete, `openspec validate port-tooltip`; CI `dev` зелёный

## Workflow follow-up

- Архивировать после CI.
- Следующее: Toast.
