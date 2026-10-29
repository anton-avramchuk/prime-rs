# Tasks

## 1. Крейт и компонент

- [x] 1.1 Создать крейт `crates/prime-select` (`publish = false`, features `ssr`/`hydrate`, зависимости `prime-core`, `prime-dom`, `prime-icons` с `chevrondown`, `search`, `times`, `check`, `spinner`, `web-sys` с нужными событиями и геометрией, dev-зависимости для wasm как у `prime-inputnumber`), добавить в workspace; проверить `cargo check -p prime-select --all-features`
- [x] 1.2 `options.rs`: `SelectOption`, `SelectGroup`, `Entry`, `visible_entries` (фильтр и группы), индексы (`first_valid`, `last_valid`, `next_valid`, `prev_valid`, `selected_index`, `first_focused`), `search_match`, `set_size`, `pos_in_set`; табличные тесты
- [x] 1.3 `state.rs`: классы корня, метки и варианта, `data-p` корня и метки по `selectstyle.ts` и `Select.dataP`; табличные тесты
- [x] 1.4 Реализовать `Select`, `SelectPt`, `SelectChange` (метка и подсказка, значок очистки, кнопка и значки, разметка панели с вариантами, группами, пустым сообщением, полем поиска и скрытыми границами, `aria-*`, шаблоны); SSR-тесты закрытого и открытого (через `open`) состояния
- [x] 1.5 Поведение панели: открытие и закрытие, позиционирование и z-index (`prime-dom`, `ZGroup::Overlay`), клик снаружи и `resize`, прокрутка выбранного в видимую область, возврат фокуса, `on_show`/`on_hide`, `disabled`/`readonly`/`loading`
- [x] 1.6 Выбор и клавиатура: клик и `focus_on_hover`, выбор и `on_change`, стрелки, `Alt+ArrowUp`, `Home`/`End`, `PageUp`/`PageDown`, `Enter`/`Space`, `Escape`, `Tab` (в том числе ловушка фокуса), `Delete`, поиск набором, `select_on_focus`, `auto_option_focus`, `aria-activedescendant`
- [x] 1.7 Поиск и группы: поле поиска, фильтрация, `on_filter`, `reset_filter_on_hide`, `autofocus_filter`, сообщения пустого списка и пустого результата, группы и их шаблон; очистка и `on_clear`
- [x] 1.8 Браузерные тесты `tests/web.rs`: открытие и закрытие всеми способами, выбор мышью и клавишей, пропуск отключённых, набор символов, фильтр, группы, очистка, `disabled`/`readonly`/`loading`, фокус и неизменность метки, внешний `value` и `open`, реактивные варианты
- [x] 1.9 rustdoc с примером; `cargo test -p prime-select --all-features`

## 2. Тема, showcase, e2e

- [x] 2.1 Добавить `select`, `iconfield` в `theme.components` и `select` в `primeng_extra` (`upstream.toml`), `cargo xtask gen-theme`, убедиться, что CSS других компонентов не изменился, подключить CSS
- [x] 2.2 Страница `/select` (`basic`, `checkmark`, `clearicon`, `disabled`, `filled`, `filter`, `fluid`, `group`, `invalid`, `loadingstate`, `sizes`, `template`), меню, маршрут, `stylesheet(&[..])`
- [x] 2.3 `e2e/tests/pages.json` и `e2e/tests/select.spec.ts` (клавиатура, мышь, фильтр, положение панели; ждать `networkidle`; `scrollIntoViewIfNeeded` перед координатами); перезапустить сервер showcase перед прогоном; `npx playwright test select`

## 3. Бенчмарки и бюджеты

- [x] 3.1 Сценарии `select-1`, `select-1000`, `case-select`, бюджеты по измерению (сначала `cargo bench -p render-bench --bench alloc`)

## 4. Интеграция

- [ ] 4.1 Добавить `prime-select` в браузерный job `ci.yml` (3 места); `docs/select.md` (отличия от PrimeNG: объём, оверлей, фильтр, значения-строки); clippy, тесты, deny, machete, `openspec validate port-select`; CI `dev` зелёный

## Workflow follow-up

- Архивировать после CI.
