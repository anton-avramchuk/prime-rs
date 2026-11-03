# Tasks

## 1. Основа

- [x] 1.1 Добавить `confirmpopup` в `theme.components` (`upstream.toml`), `cargo xtask gen-theme`; проверить, что CSS остальных компонентов не изменился и что есть классы `p-confirmpopup*`
- [x] 1.2 Проверить переводы `accept` и `reject` в `translation_defaults.rs`

## 2. Крейт `prime-confirmation`

- [x] 2.1 `Confirmation`, `ConfirmButtonProps`, `ConfirmFocus`, `ConfirmEvent`, `ConfirmationService`, `provide_confirmation_service`/`use_confirmation_service`, rustdoc, нативные тесты (подписчики, `confirm`, `close`, `unsubscribe`, контекст)

## 3. Крейт `prime-confirmpopup`

- [x] 3.1 `state.rs` (`root_classes`, `arrow_left`, `is_flipped`, `outside_click`, `focus_target`, `accepts_key`, `button_label`) с табличными тестами
- [x] 3.2 Компонент `ConfirmPopup`, `ConfirmPopupPt`: подписка на сервис, разметка панели, кнопки, `content`/`headless`/значки, SSR-тесты, rustdoc
- [x] 3.3 Показ у цели: выравнивание, стрелка, переворот, слой, фокус по `default_focus`, `accept`/`reject`, возврат фокуса на цель, перенацеливание, `visible`, `on_show`/`on_hide`
- [x] 3.4 Закрытие: `Escape`, клик снаружи, `resize`, прокрутка предка, `close()`; `close_on_escape`, `dismissable_mask`
- [x] 3.5 Браузерные `tests/web.rs` (один последовательный тест)

## 4. Showcase и e2e

- [x] 4.1 Страница `/confirmpopup` (`basic`, `template`, `headless`), меню, маршрут, `stylesheet(&[..])`, крейты в workspace и showcase
- [x] 4.2 `e2e/tests/pages.json` и `confirmpopup.spec.ts` (стили, положение и стрелка, accept/reject, Escape, клик снаружи, шаблоны); ждать `networkidle`, перезапустить сервер showcase; `PW_CHANNEL=chrome npx playwright test confirmpopup`

## 5. Бенчмарки и бюджеты

- [x] 5.1 Сценарии `-1` и `-1000`, `case-confirmpopup` и бюджеты (сначала `cargo bench -p render-bench --bench alloc`; замер wasm — в фоне, с отдельным файлом бюджетов)

## 6. Интеграция

- [x] 6.1 `docs/confirmpopup.md`, `docs/confirmation.md` (отличия от PrimeNG, что не проверялось); `prime-confirmpopup` в браузерный job `ci.yml` (три места)
- [x] 6.2 `cargo fmt --all -- --check`, clippy с исключениями CI и `--all-features` (и для wasm-цели), тесты, deny, machete, `openspec validate port-confirmpopup`; CI `dev` зелёный

## Workflow follow-up

- Архивировать после CI.
- Следующее: ConfirmDialog (на `prime-confirmation`).
