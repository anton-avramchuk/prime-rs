# Tasks

## 1. Ядро, крейт и компонент

- [x] 1.1 `prime-core`: вынести `next_unique` из `collapsible` в `ids` (`pub fn next_unique`), использовать его в `collapsible`; `cargo test -p prime-core --all-features` и тесты Panel, Fieldset, Accordion без правок
- [x] 1.2 Создать крейт `crates/prime-rating` (`publish = false`, features `ssr`/`hydrate`, зависимости `prime-core` и `prime-icons` с `star`, `starfill`, dev-зависимости для wasm как у `prime-selectbutton`), добавить в workspace; проверить `cargo check -p prime-rating --all-features`
- [x] 1.3 `state.rs`: `next_value`, классы корня, опции и иконок, `data-p` по `ratingstyle.ts` и `Rating.dataP`; табличные тесты
- [x] 1.4 Реализовать `Rating`, `RatingPt`, `RatingRate`, `RatingIcon` (значение, звёзды, радиокнопки и подписи, фокус и `p-focus-visible`, иконки и слоты, атрибуты, события); SSR-тесты (структура, число звёзд, классы, `data-p`, `checked`, подписи, атрибуты, иконки и слоты, `invalid`, `pt`)
- [x] 1.5 Браузерные тесты `tests/web.rs`: выбор и сброс кликом, `readonly`, `disabled`, один `on_rate` на действие, `change` радиокнопки без сброса, внешний сигнал, реактивное `stars`, `p-focus-visible` и фокус, сохранение звёзд и фокуса после выбора
- [x] 1.6 rustdoc с примером; `cargo test -p prime-rating --all-features`

## 2. Тема, showcase, e2e

- [x] 2.1 Добавить `rating` в `theme.components` (`upstream.toml`), `cargo xtask gen-theme`, убедиться, что CSS других компонентов не изменился, подключить CSS
- [x] 2.2 Страница `/rating` (`basic`, `disabled`, `numberofstars`, `readonly`, `template`), меню, маршрут, `stylesheet(&[..])`
- [x] 2.3 `e2e/tests/pages.json` и `e2e/tests/rating.spec.ts` (включая стрелки); перезапустить сервер showcase перед прогоном; `npx playwright test rating`

## 3. Бенчмарки и бюджеты

- [x] 3.1 Сценарии `rating-1`, `rating-1000`, `case-rating`, бюджеты по измерению (сначала `cargo bench -p render-bench --bench alloc`)

## 4. Интеграция

- [ ] 4.1 Добавить `prime-rating` в браузерный job `ci.yml` (3 места); `docs/rating.md`; clippy, тесты, deny, machete, `openspec validate port-rating`; CI `dev` зелёный

## Workflow follow-up

- Архивировать после CI.
