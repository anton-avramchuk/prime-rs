# Tasks

## 1. Основа

- [x] 1.1 Добавить `multiselect` в `theme.components` (`upstream.toml`), `cargo xtask gen-theme`; проверить, что CSS остальных компонентов не изменился, что классы `p-multiselect-*` есть и что `Chip` и `iconfield` покрывают чипы и поле поиска

## 2. Крейт

- [x] 2.1 `crates/prime-multiselect`: `logic.rs` (модель, плоский список, фильтр, индексы, выбор, диапазоны, `toggle_all`, `all_selected`, предел выбора, `label_text`, `chip_labels`, `summary`, классы, `data_p`) с табличными тестами
- [x] 2.2 Компонент `MultiSelect` и `MultiSelectPt`: корень, скрытое поле, метка (`comma`/`chip`), значки очистки и стрелки/загрузки, панель с шапкой, поиском, списком, группами и пустым сообщением, представления, SSR-тесты, rustdoc
- [x] 2.3 Панель: открытие/закрытие, выравнивание, z-index, клик вне, `resize`, фокус поля поиска, `reset_filter_on_hide`, `on_show`/`on_hide`, внешний `open`
- [x] 2.4 Выбор: клик по варианту, предел выбора, флажок «выбрать все» (с поиском), чипы и их удаление, очистка, `on_change`/`on_remove`/`on_clear`/`on_select_all_change`, `readonly`/`disabled`/`loading`
- [x] 2.5 Клавиши: навигация и открытие панели, диапазоны, `Ctrl+A`, `Alt+↑`, `Tab`, `Escape`, поиск набором, `focus_on_hover`, `select_on_focus`, прокрутка к фокусному, `on_focus`/`on_blur`, поле поиска и `on_filter`; браузерные `tests/web.rs`

## 3. Showcase и e2e

- [x] 3.1 Страница `/multiselect` (`basic`, `chips`, `clearicon`, `disabled`, `filled`, `filter`, `fluid`, `group`, `invalid`, `loadingstate`, `sizes`, `template`, `floatlabel`, `iftalabel`), меню, маршрут, `stylesheet(&[..])`, крейт в workspace и showcase
- [x] 3.2 `e2e/tests/pages.json` и `multiselect.spec.ts` (разметка, открытие и положение панели, выбор мышью, чипы, «выбрать все», поиск, клавиши); ждать `networkidle`, перезапустить сервер showcase; `PW_CHANNEL=chrome npx playwright test multiselect`

## 4. Бенчмарки и бюджеты

- [x] 4.1 Сценарии `-1` и `-1000`, `case-multiselect` и бюджеты (сначала `cargo bench -p render-bench --bench alloc`; замер wasm — в фоне, с отдельным файлом бюджетов)

## 5. Интеграция

- [x] 5.1 `docs/multiselect.md` (отличия от PrimeNG, что не проверялось); `prime-multiselect` в браузерный job `ci.yml` (три места)
- [x] 5.2 `cargo fmt --all -- --check`, clippy с исключениями CI и `--all-features` (и для wasm-цели), тесты, deny, machete, `openspec validate port-multiselect`; CI `dev` зелёный

## Workflow follow-up

- Архивировать после CI.
- Следующее: Paginator, затем Stepper.
