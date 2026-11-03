# Tasks

## 1. Основа

- [x] 1.1 Добавить `confirmdialog` в `theme.components` (`upstream.toml`), `cargo xtask gen-theme`; проверить, что CSS остальных компонентов не изменился и что есть классы `p-confirmdialog*`
- [x] 1.2 `prime-confirmation`: `ConfirmEventType` (`Accept`, `Reject`, `Cancel`, `as_str`), тест

## 2. Крейт `prime-confirmdialog`

- [x] 2.1 `state.rs` (`pick`, `position_of`, подпись кнопки, значения по умолчанию) с табличными тестами
- [x] 2.2 Компонент `ConfirmDialog`, `ConfirmDialogPt`, `ConfirmDialogHeadless`: подписка на сервис, обёртка над `Dialog`, тело, подвал с кнопками, SSR-тесты, rustdoc
- [x] 2.3 Закрытие: кнопки, крестик/`Escape`/маска как отказ с `Cancel`, `Close` сервиса, `on_hide` с типом, забывание подтверждения после выхода, замена подтверждения при открытом окне
- [x] 2.4 Фокус по `default_focus`; шаблоны `header_content`, `footer`, `message_content`, `icon_content`, `headless`, значки кнопок
- [x] 2.5 Браузерные `tests/web.rs` (один последовательный тест)

## 3. Showcase и e2e

- [x] 3.1 Страница `/confirmdialog` (`basic`, `position`, `template`, `headless`), меню, маршрут, `stylesheet(&[..])`, крейт в workspace и showcase
- [x] 3.2 `e2e/tests/pages.json` и `confirmdialog.spec.ts` (стили, accept/reject, крестик, `Escape`, позиции, шаблоны); ждать `networkidle`, перезапустить сервер showcase; `PW_CHANNEL=chrome npx playwright test confirmdialog`

## 4. Бенчмарки и бюджеты

- [x] 4.1 Сценарии `-1` и `-1000`, `case-confirmdialog` и бюджеты (сначала `cargo bench -p render-bench --bench alloc`; замер wasm — в фоне, с отдельным файлом бюджетов)

## 5. Интеграция

- [x] 5.1 `docs/confirmdialog.md` (отличия от PrimeNG, что не проверялось); `prime-confirmdialog` в браузерный job `ci.yml` (три места); `docs/confirmation.md` — `ConfirmEventType`
- [x] 5.2 `cargo fmt --all -- --check`, clippy с исключениями CI и `--all-features` (и для wasm-цели), тесты, deny, machete, `openspec validate port-confirmdialog`; CI `dev` зелёный

## Workflow follow-up

- Архивировать после CI.
- Следующее: SpeedDial.
