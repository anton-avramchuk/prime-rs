# Tasks

## 1. Крейт и компонент

- [x] 1.1 Создать крейт `crates/prime-progressspinner` (`publish = false`, features `ssr`/`hydrate`, зависимость `prime-core`), добавить в workspace; проверить `cargo check -p prime-progressspinner --all-features`
- [x] 1.2 Реализовать `ProgressSpinner` и `ProgressSpinnerPt` (разметка, SVG, ARIA, `stroke_width`, `fill`, `animation_duration`, две ветви); SSR-тесты: структура, значения по умолчанию, свои значения, `aria-label`, `style`, `pt`, паритет ветвей, реактивное обновление
- [x] 1.3 rustdoc с примером; `cargo test -p prime-progressspinner --all-features`

## 2. Тема, showcase, e2e

- [x] 2.1 Добавить `progressspinner` в `theme.components` (`upstream.toml`), `cargo xtask gen-theme`, подключить CSS
- [x] 2.2 Страница `/progressspinner` (`basic`, `custom`), меню, маршрут, `stylesheet(&[..])`
- [x] 2.3 `e2e/tests/pages.json` и `e2e/tests/progressspinner.spec.ts` (разметка, анимация, отсутствие ошибок гидрации); `npx playwright test progressspinner`

## 3. Бенчмарки и бюджеты

- [x] 3.1 Сценарии `progressspinner-1`, `progressspinner-1000`, `case-progressspinner`, бюджеты по измерению

## 4. Интеграция

- [ ] 4.1 `docs/progressspinner.md`; clippy, тесты, deny, machete, `openspec validate port-progressspinner`; CI `dev` зелёный

## Workflow follow-up

- Архивировать после CI.
