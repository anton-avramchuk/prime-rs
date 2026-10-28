# Tasks

## 1. Крейт и логика

- [x] 1.1 Создать крейт `crates/prime-chip` (`publish = false`, features `ssr`/`hydrate`, зависимости `prime-core`, `prime-icons/timescircle`, `web-sys`) и добавить в workspace; проверить `cargo check -p prime-chip --all-features`
- [x] 1.2 Реализовать `ChipState`, `write_classes`, `data_p` и разбор клавиши удаления (`is_remove_key`: Enter, Backspace); проверить юнит-тестами сценарии спеки (классы, `data-p`, клавиши)

## 2. Компонент

- [x] 2.1 Реализовать `Chip`: корень, порядок `children`/изображение или иконка/метка/удаление, `pt` (`ChipPt`), статическая ветвь при `all_static && !removable`, реактивная с внутренним `visible`; проверить SSR-тестами структуры, классов, порядка, подписи из локали и `pt`
- [x] 2.2 Реализовать удаление (клик, Enter, Backspace, скрытие `display: none`, `on_remove` один раз) и `on_image_error`; проверить `wasm-bindgen-test` сценариев спеки, включая `disabled` (`tabindex=-1`)
- [x] 2.3 Добавить таблицу паритета ветвей и rustdoc с примером; проверить `cargo test -p prime-chip --all-features` и doc-тесты

## 3. Showcase и e2e

- [ ] 3.1 Добавить страницу `/chip` (`basic`, `icon`, `image`, `template`), пункт меню и CSS `chip` в оболочку; проверить `cargo leptos build` и страницу в Chrome без ошибок консоли
- [ ] 3.2 Добавить записи в `e2e/tests/pages.json` и `e2e/tests/chip.spec.ts` (удаление мышью и клавиатурой, скрытие, подпись); проверить `npx playwright test chip` локально

## 4. Бенчмарки и бюджеты

- [ ] 4.1 Добавить сценарии `chip-1`, `chip-1000` в `benches/render`, `case-chip` в `benches/wasm-size`, бюджеты по измерению; проверить `cargo bench -p render-bench --bench alloc`, `cargo xtask alloc-budget` и `cargo xtask wasm-size`

## 5. Интеграция

- [ ] 5.1 Написать `docs/chip.md` (соответствие PrimeNG, отличия); выполнить `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace --all-features` (кроме приложений), `cargo deny check`, `openspec validate port-chip`; убедиться, что CI на `dev` зелёный

## Workflow follow-up

- Архивировать change после прохождения CI; следующий компонент волны 1: Avatar.
