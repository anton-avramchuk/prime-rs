# Tasks

## 1. Общая логика и тема

- [x] 1.1 `prime-core::collapsible::bind_region_with` (привязка к производному состоянию), `bind_region` через неё; проверить, что тесты Panel и Fieldset (`wasm-bindgen-test`) проходят без изменений
- [x] 1.2 `xtask gen-theme`: блок `/* For PrimeNG */` из `<name>style.ts` с разрешением `dt()`; проверить, что CSS остальных компонентов не изменился (`git diff`), и юнит-тестом разбора блока

## 2. Крейт и компоненты

- [x] 2.1 Создать крейт `crates/prime-accordion` (`publish = false`, features `ssr`/`hydrate`, зависимости `prime-core`, `prime-icons` с `chevrondown`, `chevronup`, `web-sys`), добавить в workspace; проверить `cargo check -p prime-accordion --all-features`
- [x] 2.2 `toggle_value` и выбор соседнего заголовка (`state.rs`) с таблицами юнит-тестов
- [x] 2.3 `Accordion`, `AccordionPanel`, `AccordionHeader`, `AccordionContent`, `AccordionPt`, контекст, классы и ARIA; проверить SSR-тестами (структура, классы, ARIA, `pt`, слот иконки, начальная скрытость)
- [x] 2.4 Клик, клавиатура, `select_on_focus`, `on_open`/`on_close`, анимация; проверить `wasm-bindgen-test` (одиночный режим и `multiple`, отключённая панель, стрелки с пропуском и зацикливанием, Home/End, анимационный класс, `tabindex` потомков)
- [x] 2.5 rustdoc с примером; `cargo test -p prime-accordion --all-features`

## 3. Showcase и e2e

- [x] 3.1 Добавить `accordion` в `theme.components` (`upstream.toml`), `cargo xtask gen-theme`, подключить CSS
- [x] 3.2 Страница `/accordion` (`basic`, `multiple`, `disabled`, `controlled`, `dynamic`, `template`), меню, маршрут, `stylesheet(&[..])`
- [x] 3.3 `e2e/tests/pages.json` и `e2e/tests/accordion.spec.ts`; `npx playwright test accordion`

## 4. Бенчмарки и бюджеты

- [x] 4.1 Сценарии `accordion-1`, `accordion-1000` (три панели на экземпляр), `case-accordion`, бюджеты по измерению

## 5. Интеграция

- [x] 5.1 `docs/accordion.md`; clippy, тесты, deny, machete, `openspec validate port-accordion`; `prime-accordion` в браузерный job CI; CI `dev` зелёный

## Workflow follow-up

- Архивировать после CI.
