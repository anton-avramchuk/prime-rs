# Tasks

## 1. Крейт

- [x] 1.1 `crates/prime-inputotp`: `state.rs` (`tokens_of`, `join`, `key_decision`, `paste_value`, `input_move`) с табличными тестами
- [x] 1.2 Компонент `InputOtp`: разметка, значение (контролируемое и нет), атрибуты, `Pt`, SSR-тесты по сценариям разметки и значения, rustdoc
- [x] 1.3 Поведение: ввод и автопереход, клавиши, вставка, выделение при фокусе, `on_change`/`on_focus`/`on_blur`, шаблон `input` с `InputOtpEvents`; браузерные тесты `tests/web.rs`

## 2. Тема, showcase, e2e

- [x] 2.1 Добавить `inputotp` в `theme.components` (`upstream.toml`), `cargo xtask gen-theme`, убедиться, что CSS других компонентов не изменился
- [x] 2.2 Страница `/inputotp` (`basic`, `mask`, `integeronly`, `sizes`, `template`), меню, маршрут, `stylesheet(&[..])`, крейт в workspace и showcase
- [x] 2.3 `e2e/tests/pages.json` и `inputotp.spec.ts` (ввод, автопереход, Backspace, стрелки, вставка, integer_only); ждать `networkidle`, перезапустить сервер showcase; `PW_CHANNEL=chrome npx playwright test inputotp`

## 3. Бенчмарки и бюджеты

- [x] 3.1 Сценарии `-1` и `-1000`, `case-inputotp` и бюджеты (сначала `cargo bench -p render-bench --bench alloc`)

## 4. Интеграция

- [x] 4.1 `docs/inputotp.md` (отличия от PrimeNG, модель значения, что не проверялось); браузерный job в `ci.yml` (три места), если добавлен `tests/web.rs`
- [x] 4.2 `cargo fmt --all -- --check`, clippy с `--all-features`, тесты, deny, machete, `openspec validate port-inputotp`; CI `dev` зелёный

## Workflow follow-up

- Архивировать после CI.
- Следующее: ScrollTop, Password.
