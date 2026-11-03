# Tasks

## 1. Основа

- [x] 1.1 Добавить `speeddial` в `theme.components` (`upstream.toml`), `cargo xtask gen-theme`; проверить, что CSS остальных компонентов не изменился и что есть классы `p-speeddial*`
- [x] 1.2 Проверить наличие нужных значков в `prime-icons` для демонстраций (`plus`, а также аналоги pencil/refresh/trash/upload/external-link); недостающие заменить близкими

## 2. Крейт `prime-speeddial`

- [x] 2.1 `state.rs` (классы, flex-стили, `point_style` для четырёх типов, `transition_delay`, `arrow_step`, `step_index`, значок кнопки) с табличными тестами
- [x] 2.2 Компонент `SpeedDial`, `SpeedDialItem`, `SpeedDialCommand`, `SpeedDialPt`: корень, кнопка, список, пункты, маска, SSR-тесты, rustdoc
- [x] 2.3 Показ и скрытие: кнопка, `command`, `on_*`, управляемый `visible`, `disabled`, клик вне, `Escape`, маска, `--item-diff-x/y`
- [x] 2.4 Клавиатура: кнопка и список, восемь направлений, `Home`/`End`/`Enter`/`Space`, фокус по `aria-activedescendant`, пропуск отключённых
- [x] 2.5 `tooltip_options`, `button_severity`, `button_content`/`item_content`/`icon_content`
- [x] 2.6 Браузерные `tests/web.rs` (один последовательный тест)

## 3. Showcase и e2e

- [x] 3.1 Страница `/speeddial` (`linear`, `circle`, `semicircle`, `quartercircle`, `mask`, `tooltip`, `template`), меню, маршрут, `stylesheet(&[..])`, крейт в workspace и showcase
- [x] 3.2 `e2e/tests/pages.json` и `speeddial.spec.ts` (стили, раскрытие по направлениям, раскладки, маска, подсказка, клавиатура, шаблоны); ждать `networkidle`, перезапустить сервер showcase; `PW_CHANNEL=chrome npx playwright test speeddial`

## 4. Бенчмарки и бюджеты

- [x] 4.1 Сценарии `-1` и `-1000`, `case-speeddial` и бюджеты (сначала `cargo bench -p render-bench --bench alloc`; замер wasm — в фоне, с отдельным файлом бюджетов)

## 5. Интеграция

- [x] 5.1 `docs/speeddial.md` (отличия от PrimeNG, что не проверялось); `prime-speeddial` в браузерный job `ci.yml` (три места)
- [x] 5.2 `cargo fmt --all -- --check`, clippy с исключениями CI и `--all-features` (и для wasm-цели), тесты (SSR по одному: `--exact`), deny, machete, `openspec validate port-speeddial`; CI `dev` зелёный

## Workflow follow-up

- Архивировать после CI.
- Следующее: Carousel.
