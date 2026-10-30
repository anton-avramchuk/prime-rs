# Tasks

## 1. Крейт

- [x] 1.1 `crates/prime-breadcrumb`: `BreadcrumbItem` (построитель), `BreadcrumbClick`, `BreadcrumbPt`, `click_plan` с табличным тестом
- [x] 1.2 Компонент `Breadcrumb`: корень, список, пункт «домой», пункты, разделители, шаблоны `item` и `separator`, клики; SSR-тесты по всем сценариям спецификации, rustdoc

## 2. Тема, showcase, e2e

- [x] 2.1 Добавить `breadcrumb` в `theme.components` (`upstream.toml`), `cargo xtask gen-theme`, убедиться, что CSS других компонентов не изменился
- [x] 2.2 Страница `/breadcrumb` (`basic`, `template`), меню, маршрут, `stylesheet(&[..])`, крейт в workspace и showcase
- [x] 2.3 `e2e/tests/pages.json` и `breadcrumb.spec.ts`; ждать `networkidle`, перезапустить сервер showcase; `PW_CHANNEL=chrome npx playwright test breadcrumb`

## 3. Бенчмарки и бюджеты

- [x] 3.1 Сценарии `-1` и `-1000`, `case-breadcrumb` и бюджеты (сначала `cargo bench -p render-bench --bench alloc`)

## 4. Интеграция

- [x] 4.1 `docs/breadcrumb.md` (отличия от PrimeNG: нет `routerLink`, `BreadcrumbItem` вместо `MenuItem`, `Steps` не переносится, демонстрация `router` пропущена)
- [x] 4.2 `cargo fmt --all -- --check`, clippy с `--all-features`, тесты, deny, machete, `openspec validate port-breadcrumb`; CI `dev` зелёный

## Workflow follow-up

- Архивировать после CI.
- Следующее: InputOtp, ScrollTop, Password.
