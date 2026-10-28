# Tasks

## 1. Заготовки и общий помощник

- [x] 1.1 Сгенерировать заготовки `cargo xtask gen-spec badge` и `gen-spec button` во временный каталог и сверить, что все свойства, события и классы из них покрыты спеками этого change; результат сверки записать в `design.md` в разделе «Context», если найдены пропуски
- [x] 1.2 Добавить в `prime-core` помощник `DataP` (признаки через пробел, как `cn({...})`) и поле `style` в `Pt`; проверить юнит-тестами порядок, пустые значения и `Pt::merge_style`
- [x] 1.3 Задокументировать `DataP` и `Pt::style` в rustdoc с примерами и проверить `cargo test --doc -p prime-core`

## 2. Badge

- [x] 2.1 Создать крейт `crates/prime-badge` (`publish = false`, features `ssr`/`hydrate`) и добавить в workspace; проверить `cargo check -p prime-badge --all-features`
- [x] 2.2 Реализовать чистую функцию «свойства → классы и `data-p`» по `badgestyle.ts` и таблицу тестов по сценариям спеки `badge` (круг, точка, размеры, серьёзности); проверить `cargo test -p prime-badge`
- [x] 2.3 Реализовать компонент `Badge` (`value`, `severity`, `size`, `badge_disabled`, `style_class`, `pt`); проверить SSR-тестами структуры, `display: none` и `pt`
- [x] 2.4 Добавить `badge` в `crates/prime-theme` features приложения showcase и проверить, что `stylesheet(&["badge"])` содержит `.p-badge-dot`; задокументировать API (rustdoc, пример) и проверить `cargo test --doc -p prime-badge`

## 3. Button: логика и разметка

- [x] 3.1 Создать крейт `crates/prime-button` (`publish = false`, features `ssr`/`hydrate`, зависимости по D7); проверить `cargo check -p prime-button --all-features`
- [x] 3.2 Реализовать `ButtonState` и чистые функции классов и `data-p*` по `buttonstyle.ts` и `button.ts`; проверить таблицей юнит-тестов по всем модификаторам спеки (`icon-only`, `vertical`, `loading`, `text`/`outlined` из `variant`, размеры, `fluid`, серьёзности)
- [x] 3.3 Реализовать компонент `Button`: `<button>`, `children`, иконка, загрузочный индикатор, метка, `Badge`, `disabled` при `loading`, `aria-hidden` метки; проверить SSR-тестами порядка узлов и всех сценариев раздела «Иконка, метка, бейдж и содержимое»
- [x] 3.4 Реализовать `ButtonPt` и применение `pt` ко всем частям, `style_class`, `style`, `aria_label`, `tabindex`, `type`; проверить SSR-тестами сценариев `pt`, пользовательского класса и стиля, a11y-атрибутов
- [x] 3.5 Задокументировать API `Button` (rustdoc с примерами) и проверить `cargo test --doc -p prime-button`

## 4. Button: поведение в браузере

- [x] 4.1 Реализовать `on_click`, `on_focus`, `on_blur` и недоступность при `disabled`/`loading`; проверить `wasm-bindgen-test` (клик вызывает один раз, недоступная кнопка не вызывает)
- [x] 4.2 Реализовать `autofocus` и подключение `Ripple` по `PrimeConfig.ripple` с освобождением в `on_cleanup`; проверить `wasm-bindgen-test` сценариев автофокуса, ripple включён и выключен, отсутствия `p-ink` после размонтирования
- [x] 4.3 Проверить SSR-безопасность: `cargo test -p prime-button --features ssr` не обращается к `window`, а `Effect`-ы не выполняются; зафиксировать результат тестом

## 5. Бенчмарки и бюджеты

- [x] 5.1 Добавить сценарии `badge-1`, `badge-1000`, `button-1`, `button-1000` в `benches/render`; проверить `cargo bench -p render-bench --bench alloc` и `--bench time`
- [x] 5.2 Добавить случаи `case-badge` и `case-button` в `benches/wasm-size` и бюджеты в `benches/budgets.toml` (измеренный вклад с запасом); проверить `cargo xtask wasm-size`
- [x] 5.3 Проверить, что `class` не перезаписывается при изменении свойства, не влияющего на классы (счётчик обновлений в `wasm-bindgen-test`); записать результат в `docs/benchmarks.md` вместе с первыми значениями Button и Badge

## 6. Showcase и сравнение с PrimeNG

- [ ] 6.1 Добавить `leptos_router` в `showcase` (маршруты `/`, `/button`, `/badge`), включить features `button` и `badge` темы; проверить `cargo leptos build` и открытие страниц
- [ ] 6.2 Перенести демонстрации Button из `apps/showcase/doc/button` (все свойства спеки) и Badge в страницы showcase; проверить в Chrome, что каждая демонстрация отображается без ошибок консоли
- [ ] 6.3 Добавить страницы в `e2e/tests/pages.json` и e2e-проверки (`button.spec.ts`: клик, loading, disabled, ripple); проверить `npx playwright test` локально с запущенным showcase
- [ ] 6.4 Запустить `weekly.yml` вручную и проверить, что отчёт `visual.json` содержит `visual.button.mismatch_pct`; записать расхождения и их причины в `docs/button.md`

## 7. Интеграционная проверка

- [ ] 7.1 Добавить `openspec/` ссылки на исходники: в `docs/button.md` указать файлы PrimeNG и коммит, с которых портированы Button и Badge; проверить `cargo xtask upstream-diff` (изменений нет)
- [ ] 7.2 Выполнить `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace --all-features` (кроме приложений), `cargo fmt --all --check`, `cargo deny check` и `openspec validate port-button`; убедиться, что CI на PR зелёный

## Workflow follow-up

- Архивировать change после слияния и прохождения CI.
- Следующие компоненты волны 1: Tag, Chip, Avatar, Divider, Card, InputText, Checkbox, RadioButton, ToggleSwitch, Message, ProgressBar.
