# Tasks

## 1. Крейт, формат и правка текста

- [x] 1.1 Создать крейт `crates/prime-inputnumber` (`publish = false`, features `ssr`/`hydrate`, зависимости `prime-core`, `prime-inputtext`, `prime-icons` с `angleup`, `angledown`, `times`, `web-sys` с `InputEvent`, `HtmlInputElement`, `ClipboardEvent`/`DataTransfer`, dev-зависимости для wasm как у `prime-slider`), добавить в workspace; проверить `cargo check -p prime-inputnumber --all-features`
- [x] 1.2 `locale.rs` и `format.rs`: таблица локалей и валют, `resolve_locale`, `format_number`, округление по кратчайшему десятичному представлению, группировка (в том числе 3-2-2), валюта и её положение, `prefix`/`suffix`; табличные тесты по всем локалям и валютам
- [x] 1.3 Разбор `parse_text` (префикс, суффикс, валюта, группы, минус, десятичный разделитель, состояние «только минус») и тесты обратимости `parse(format(x)) == x`
- [x] 1.4 `edit.rs`: `insert`, `delete_back`, `delete_forward`, `delete_range`, `paste`, `commit`, `clamp_caret`, сохранение набранного «хвоста» дроби и курсор по числу значимых символов; табличные тесты (набор, минус, разделитель, предел дроби, Backspace/Delete над разделителями и дробью, диапазоны, вставка, префикс и суффикс)
- [x] 1.5 `state.rs`: классы корня, кнопок и `data-p` по `inputnumberstyle.ts` и `InputNumber.dataP`; табличные тесты

## 2. Компонент

- [x] 2.1 Реализовать `InputNumber`, `InputNumberPt`, `InputNumberInput` (разметка, раскладки кнопок, значок очистки, атрибуты, `display` и форматирование, внешний сигнал); SSR-тесты (форматы, структура, классы и `data-p`, раскладки, очистка, атрибуты, `pt`, паритет ветвей)
- [x] 2.2 События ввода: `beforeinput` (набор, удаление, вставка), `keydown` (стрелки, `Enter`, `Tab`, `Home`, `End`), `focus`/`blur`, `click` (`clamp_caret`), фиксация значения, `on_input`, `on_key_down`
- [x] 2.3 Кнопки: шаг, повтор при удержании (500 мс, затем 40 мс), остановка по `mouseup`/`mouseleave`/`keyup` и в `on_cleanup`, `readonly`/`disabled`, `max_length`; очистка и `on_clear`
- [x] 2.4 Браузерные тесты `tests/web.rs`: набор и курсор, Backspace/Delete, вставка, фиксация, границы, стрелки, кнопки и повтор, `readonly`/`disabled`, очистка, внешний сигнал и смена `locale`, сохранение «1.» при наборе
- [x] 2.5 rustdoc с примером; `cargo test -p prime-inputnumber --all-features`

## 3. Тема, showcase, e2e

- [x] 3.1 Добавить `inputnumber` в `theme.components` (`upstream.toml`), `cargo xtask gen-theme`, убедиться, что CSS других компонентов не изменился, подключить CSS
- [x] 3.2 Страница `/inputnumber` (`locale`, `currency`, `numerals`, `prefixsuffix`, `buttons`, `vertical`, `clearicon`, `sizes`, `filled`, `invalid`, `disabled`, `fluid`), меню, маршрут, `stylesheet(&[..])`
- [x] 3.3 `e2e/tests/pages.json` и `e2e/tests/inputnumber.spec.ts` (набор с клавиатуры, стрелки, кнопки; ждать `networkidle`; `scrollIntoViewIfNeeded` перед координатами); перезапустить сервер showcase перед прогоном; `npx playwright test inputnumber`

## 4. Бенчмарки и бюджеты

- [x] 4.1 Сценарии `inputnumber-1`, `inputnumber-1000`, `case-inputnumber`, бюджеты по измерению (сначала `cargo bench -p render-bench --bench alloc`)

## 5. Интеграция

- [x] 5.1 Добавить `prime-inputnumber` в браузерный job `ci.yml` (3 места); `docs/inputnumber.md` (отличия от PrimeNG, в том числе локали, `beforeinput`, курсор); clippy, тесты, deny, machete, `openspec validate port-inputnumber`; CI `dev` зелёный

## Workflow follow-up

- Архивировать после CI.
