# Tasks

## 1. Крейт и компонент

- [x] 1.1 Создать крейт `crates/prime-scrollpanel` (`publish = false`, features `ssr`/`hydrate`, зависимости `prime-core`, `web-sys`), добавить в workspace; проверить `cargo check -p prime-scrollpanel --all-features`
- [x] 1.2 Реализовать `ScrollPanel` и `ScrollPanelPt` (разметка, ARIA полос, `id` содержимого, слот `content_template`, две ветви); SSR-тесты: структура, порядок, атрибуты, `style`, `pt`, паритет ветвей
- [x] 1.3 Чистая арифметика полос (`bar_size`, `bar_offset`, видимость) с нативными юнит-тестами
- [x] 1.4 Клиентская часть: `moveBar` через `requestAnimationFrame`, слушатели `scroll`/`mouseenter`/`resize`, расчёт высоты при `max-height`, перетаскивание полос, клавиатура, `ScrollPanelRef` (`refresh`, `scroll_top`), снятие слушателей
- [x] 1.5 Браузерные тесты `tests/web.rs` (скрытие полосы, размер и позиция, перетаскивание, стрелки и `keyup`, `scroll_top`, размонтирование)
- [x] 1.6 rustdoc с примером; `cargo test -p prime-scrollpanel --all-features`

## 2. Тема, showcase, e2e

- [x] 2.1 Добавить `scrollpanel` в `theme.components` (`upstream.toml`), `cargo xtask gen-theme`; проверить `.p-scrollpanel { display: block }` и при отсутствии добавить в `primeng_extra`; подключить CSS
- [x] 2.2 Страница `/scrollpanel` (`basic`, `custom`), меню, маршрут, `stylesheet(&[..])`
- [x] 2.3 `e2e/tests/pages.json` и `e2e/tests/scrollpanel.spec.ts` (видимость полосы, перетаскивание, стрелки); `npx playwright test scrollpanel`

## 3. Бенчмарки и бюджеты

- [x] 3.1 Сценарии `scrollpanel-1`, `scrollpanel-1000`, `case-scrollpanel`, бюджеты по измерению; добавить крейт в `benches/render`, `benches/wasm-size`, браузерный job `ci.yml` (3 места)

## 4. Интеграция

- [ ] 4.1 `docs/scrollpanel.md` (совпадения, отличия, производительность); clippy, тесты, deny, machete, `openspec validate port-scrollpanel`; CI `dev` зелёный

## Workflow follow-up

- Архивировать после CI.
