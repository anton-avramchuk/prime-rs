# Tasks

## 1. Крейт и компонент

- [x] 1.1 Создать крейт `crates/prime-selectbutton` (`publish = false`, features `ssr`/`hydrate`, зависимости `prime-core` и `prime-togglebutton`, dev-зависимости для wasm как у `prime-togglebutton`), добавить в workspace; проверить `cargo check -p prime-selectbutton --all-features`
- [x] 1.2 `state.rs`: `SelectButtonOption`, `SelectButtonValue`, чистая логика выбора (одиночный и множественный режимы, `allow_empty`, `getAllowEmpty`), классы корня и `data-p` по `selectbuttonstyle.ts` и `SelectButton.dataP`; табличные тесты
- [x] 1.3 Реализовать `SelectButton`, `SelectButtonPt`, `SelectButtonChange`, `SelectButtonOptionClick` (варианты, значение, синхронизация сигналов кнопок, слот `item_template`, атрибуты, события); SSR-тесты (структура, метки, выбранный вариант, `disabled`, классы, `data-p`, `size`/`fluid`/`style_class`, слот, `pt`)
- [x] 1.4 Браузерные тесты `tests/web.rs`: выбор и снятие, `allow_empty`, множественный выбор, `disabled`, Enter и Space, один `on_change` на нажатие, синхронность `aria-pressed` после отклонённого клика, внешний сигнал и смена `options`
- [x] 1.5 rustdoc с примером; `cargo test -p prime-selectbutton --all-features`

## 2. Тема, showcase, e2e

- [x] 2.1 Добавить `selectbutton` в `theme.components` (`upstream.toml`), `cargo xtask gen-theme`, убедиться, что CSS других компонентов не изменился, подключить CSS (`selectbutton` вместе с `togglebutton`)
- [x] 2.2 Страница `/selectbutton` (`basic`, `disabled`, `fluid`, `invalid`, `multiple`, `sizes`, `template`), меню, маршрут, `stylesheet(&[..])`
- [x] 2.3 `e2e/tests/pages.json` и `e2e/tests/selectbutton.spec.ts`; `npx playwright test selectbutton`

## 3. Бенчмарки и бюджеты

- [x] 3.1 Сценарии `selectbutton-1`, `selectbutton-1000`, `case-selectbutton`, бюджеты по измерению (сначала `cargo bench -p render-bench --bench alloc`)

## 4. Интеграция

- [ ] 4.1 Добавить `prime-selectbutton` в браузерный job `ci.yml` (3 места); `docs/selectbutton.md`; clippy, тесты, deny, machete, `openspec validate port-selectbutton`; CI `dev` зелёный

## Workflow follow-up

- Архивировать после CI.
