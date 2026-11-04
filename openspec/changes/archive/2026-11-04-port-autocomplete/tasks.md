# Tasks

## 1. Основа

- [x] 1.1 Добавить `autocomplete` в `theme.components` и `primeng_extra` (`upstream.toml`), `cargo xtask gen-theme`; проверить, что CSS остальных компонентов не изменился и есть классы `p-autocomplete*`
- [x] 1.2 Свериться с `onInput`, `onKeyDown`, `onOptionSelect`, `search`, `updateInputWithForceSelection`, `handleSuggestionsChange` исходника; значки `prime-icons`: chevrondown, times, timescircle, spinner

## 2. Крейт `prime-autocomplete`

- [x] 2.1 Модель `AutoCompleteOption`, чистая логика `state.rs` (плоский список, навигация, подпись значения, `force_selection`, разделитель) с табличными тестами
- [x] 2.2 Компонент `AutoComplete`, `AutoCompletePt`: корень, поле/чипы, очистка, спиннер, кнопка, панель, варианты, группы, сообщение о пустоте, SSR-тесты, rustdoc
- [x] 2.3 Ввод, задержка и `on_complete`, обновление `suggestions`, выбор, `multiple`, чипы, кнопка-стрелка, очистка, `force_selection`, `complete_on_focus`
- [x] 2.4 Клавиатура поля и контейнера чипов, `typeahead = false` (добавление по `Enter`/разделителю/`Tab`/blur/вставке)
- [x] 2.5 Браузерные `tests/web.rs` (один последовательный тест)

## 3. Showcase и e2e

- [x] 3.1 Страница `/autocomplete`, меню, маршрут, `stylesheet(&[..])`, крейт в workspace и showcase
- [x] 3.2 `e2e/tests/pages.json` и `autocomplete.spec.ts`; `networkidle`, перезапуск сервера showcase; `PW_CHANNEL=chrome npx playwright test autocomplete`

## 4. Бенчмарки и бюджеты

- [x] 4.1 Сценарии `-1` и `-1000`, `case-autocomplete`, бюджеты

## 5. Интеграция

- [x] 5.1 `docs/autocomplete.md`; `prime-autocomplete` в браузерный job `ci.yml` (три места)
- [x] 5.2 `cargo fmt --all -- --check`, clippy с исключениями CI и `--all-features` (и для wasm), тесты, deny, machete, `openspec validate port-autocomplete`; CI — один раз по завершении волны 5 (не запускался)

## Workflow follow-up

- Архивировать; без пуша и ожидания CI до конца волны 5.
- Дальше по плану волны 5: InputMask.
