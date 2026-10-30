# Tasks

## 1. Крейт

- [x] 1.1 `crates/prime-inplace`: `state.rs` (`display_classes`, `next_state`) с табличными тестами
- [x] 1.2 Компонент `Inplace`: свойства, разметка, кнопка закрытия, `InplacePt`, SSR-тесты, rustdoc
- [x] 1.3 Поведение: клик, `Enter`, `prevent_click`, `disabled`, закрытие, внешний сигнал, колбэки; браузерные тесты `tests/web.rs`

## 2. Тема, showcase, e2e

- [x] 2.1 Добавить `inplace` в `theme.components` (`upstream.toml`), `cargo xtask gen-theme`, убедиться, что CSS других компонентов не изменился
- [x] 2.2 Страница `/inplace` (`basic`, `input`, `image`, `lazy`), меню, маршрут, `stylesheet(&[..])`, крейт в workspace и showcase
- [x] 2.3 `e2e/tests/pages.json` и `inplace.spec.ts`; ждать `networkidle`, перезапустить сервер showcase; `PW_CHANNEL=chrome npx playwright test inplace`

## 3. Бенчмарки и бюджеты

- [x] 3.1 Сценарии `-1` и `-1000`, `case-inplace` и бюджеты (сначала `cargo bench -p render-bench --bench alloc`)

## 4. Интеграция

- [x] 4.1 `docs/inplace.md` (отличия от PrimeNG, что не проверялось); `prime-inplace` в браузерный job `ci.yml` (три места)
- [x] 4.2 `cargo fmt --all -- --check`, clippy с `--all-features`, тесты, deny, machete, `openspec validate port-inplace`; CI `dev` зелёный

## Workflow follow-up

- Архивировано после CI.
- Следующее: Splitter.
