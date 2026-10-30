# Tasks

## 1. Крейт

- [x] 1.1 `crates/prime-password`: `state.rs` (`strength`, `meter_width`, `meter_class`, классы корня, `data_p`) с табличными тестами
- [x] 1.2 Компонент `Password`: свойства, разметка поверх `InputText` (или собственное поле, см. D4), значки очистки и показа, панель-подсказка, шаблоны `header`/`footer`/`content`/значков, SSR-тесты, rustdoc
- [x] 1.3 Поведение: фокус/ввод/`Escape`/`blur`, фазы и анимация панели, z-index и `align_overlay`, закрытие на `resize`, локаль; браузерные тесты `tests/web.rs`

## 2. Тема, showcase, e2e

- [x] 2.1 Добавить `password` в `theme.components` и `primeng_extra` (`upstream.toml`), `cargo xtask gen-theme`, убедиться, что CSS других компонентов не изменился
- [x] 2.2 Страница `/password` (демонстрации из PrimeNG без форм Angular), меню, маршрут, `stylesheet(&[..])`, крейт в workspace и showcase
- [x] 2.3 `e2e/tests/pages.json` и `password.spec.ts` (панель по фокусу, уровни, `Escape`, показ пароля, очистка); ждать `networkidle`, перезапустить сервер showcase; `PW_CHANNEL=chrome npx playwright test password`

## 3. Бенчмарки и бюджеты

- [x] 3.1 Сценарии `-1` и `-1000`, `case-password` и бюджеты (сначала `cargo bench -p render-bench --bench alloc`)

## 4. Интеграция

- [x] 4.1 `docs/password.md` (отличия от PrimeNG, что не проверялось); `prime-password` в браузерный job `ci.yml` (три места)
- [x] 4.2 `cargo fmt --all -- --check`, clippy с `--all-features` (и для wasm-цели), тесты, deny, machete, `openspec validate port-password`; CI `dev` зелёный

## Workflow follow-up

- Архивировать после CI.
- Следующее: плана волны 2 больше нет — выбрать кандидатов следующей волны.
