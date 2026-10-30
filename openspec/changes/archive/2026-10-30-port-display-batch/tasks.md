# Tasks

## 1. Новые крейты

- [x] 1.1 `crates/prime-overlaybadge` (`OverlayBadge`, `OverlayBadgePt`): крейт, компонент, SSR-тесты, rustdoc
- [x] 1.2 `crates/prime-timeline` (`Timeline`, `TimelineAlign`, `TimelineLayout`, `TimelinePt`): `state.rs` с классами и `data_p` (табличный тест), компонент с шаблонами, SSR-тесты, rustdoc
- [x] 1.3 `crates/prime-metergroup` (`MeterGroup`, `MeterItem`, `MeterGroupPt`, перечисления): `state.rs` с `percent`/`total_percent`/`cumulative` (табличные тесты), компонент с подписями и шаблонами, SSR-тесты, rustdoc

## 2. Тема, showcase, e2e

- [x] 2.1 Добавить `overlaybadge`, `timeline`, `metergroup` в `theme.components` (`upstream.toml`), `cargo xtask gen-theme`, убедиться, что CSS других компонентов не изменился
- [x] 2.2 Страницы `/overlaybadge`, `/timeline`, `/metergroup`, меню, маршруты, `stylesheet(&[..])`; новые крейты в workspace
- [x] 2.3 `e2e/tests/pages.json` и spec-файлы (угол бейджа, направление и позиции таймлайна, ширина отрезков и подписи); ждать `networkidle`, перезапустить сервер showcase; `PW_CHANNEL=chrome npx playwright test`

## 3. Бенчмарки и бюджеты

- [x] 3.1 Сценарии `-1` и `-1000`, `case-*` и бюджеты для трёх крейтов (сначала `cargo bench -p render-bench --bench alloc`)

## 4. Интеграция

- [x] 4.1 `docs/display-batch.md` (отличия от PrimeNG: перечисление `align`, шаблоны-колбэки, иконки `MeterItem`, демонстрации без недостающих иконок)
- [x] 4.2 `cargo fmt --all -- --check`, clippy с исключениями CI и с `--all-features`, тесты, deny, machete, `openspec validate port-display-batch`; CI `dev` зелёный

## Workflow follow-up

- Архивировать после CI.
- Следующая партия: модель `MenuItem` + `Breadcrumb` + `Steps`, затем InputOtp, ScrollTop, Password.
