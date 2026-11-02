# Tasks

## 1. Основа

- [x] 1.1 Добавить `listbox` в `theme.components` и `primeng_extra` (`upstream.toml`), `cargo xtask gen-theme`; проверить, что CSS остальных компонентов не изменился и что классы `p-listbox-*` есть

## 2. Крейт

- [x] 2.1 `crates/prime-listbox`: `logic.rs` (`ListboxOption`, `ListboxGroup`, `Entry`, `visible_entries`, индексы, `select_single`, `select_multiple`, `select_range`, `select_all`, `all_selected`, `find_by_prefix`, `root_classes`, `option_classes`, `data_p`) с табличными тестами
- [x] 2.2 Компонент `Listbox` и `ListboxPt`: корень, шапка, поле поиска, список, группы, пустое сообщение, `option`/`group`, SSR-тесты, rustdoc
- [x] 2.3 Выбор: одиночный и множественный режимы, `meta_key_selection`, флажки варианта и «выбрать все», `checkmark`, `readonly`/`disabled`, `on_change`/`on_click`/`on_dbl_click`/`on_select_all_change`
- [x] 2.4 Фокус и клавиши: фокус списка, стрелки, Home/End, PageUp/PageDown, Enter/Space, `Shift`-диапазоны, `Ctrl+A`, поиск набором, `focus_on_hover`, `select_on_focus`, прокрутка к фокусному, `on_focus`/`on_blur`, поле поиска и `on_filter`; браузерные `tests/web.rs`

## 3. Showcase и e2e

- [x] 3.1 Страница `/listbox` (`basic`, `multiple`, `group`, `filter`, `checkbox`, `checkmark`, `disabled`, `invalid`, `template`), меню, маршрут, `stylesheet(&[..])`, крейт в workspace и showcase
- [x] 3.2 `e2e/tests/pages.json` и `listbox.spec.ts` (разметка, выбор мышью, множественный выбор с `Ctrl`/`Shift`, флажки, поиск, клавиши); ждать `networkidle`, перезапустить сервер showcase; `PW_CHANNEL=chrome npx playwright test listbox`

## 4. Бенчмарки и бюджеты

- [x] 4.1 Сценарии `-1` и `-1000`, `case-listbox` и бюджеты (сначала `cargo bench -p render-bench --bench alloc`; замер wasm — в фоне)

## 5. Интеграция

- [x] 5.1 `docs/listbox.md` (отличия от PrimeNG, что не проверялось); `prime-listbox` в браузерный job `ci.yml` (три места)
- [x] 5.2 `cargo fmt --all -- --check`, clippy с исключениями CI и `--all-features` (и для wasm-цели), тесты, deny, machete, `openspec validate port-listbox`; CI `dev` зелёный

## Workflow follow-up

- Архивировать после CI.
- Следующее: MultiSelect.
