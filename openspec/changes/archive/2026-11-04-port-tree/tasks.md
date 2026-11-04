# Tasks

## 1. Основа

- [x] 1.1 Добавить `tree` в `theme.components` (`upstream.toml`), `cargo xtask gen-theme`; проверить, что CSS остальных компонентов не изменился и есть классы `p-tree*`
- [x] 1.2 Свериться с `onNodeClick`, `propagateUp/Down`, `_filter`, `onArrow*` исходника; значки `prime-icons`: chevronright, chevrondown, spinner, search, check, minus

## 2. Крейт `prime-tree`

- [x] 2.1 Модель `TreeNode`, чистая логика `state.rs` (видимые, выбор всех режимов, распространение, частичный выбор, фильтр, клавиши) с табличными тестами
- [x] 2.2 Компонент `Tree`, `TreePt`, `TreeNodeContext`: корень, маска, фильтр, список, узел, кнопка, флажок, значок, подпись, дети, пустое сообщение, SSR-тесты, rustdoc
- [x] 2.3 Раскрытие, выбор (`Single`/`Multiple`/`Checkbox`, `meta`, `selectable`), события, внешние сигналы `expanded_keys`/`selection`
- [x] 2.4 Фильтр: поле, режимы, раскрытие совпавших предков, `on_filter`
- [x] 2.5 Клавиатура и roving tabindex, `on_node_double_click`
- [x] 2.6 Браузерные `tests/web.rs` (один последовательный тест)

## 3. Showcase и e2e

- [x] 3.1 Страница `/tree` (`basic`, `single`, `multiple`, `checkbox`, `controlled`, `event`, `filter`, `lazy`, `template`), меню, маршрут, `stylesheet(&[..])`, крейт в workspace и showcase
- [x] 3.2 `e2e/tests/pages.json` и `tree.spec.ts`; `networkidle`, перезапуск сервера showcase; `PW_CHANNEL=chrome npx playwright test tree`

## 4. Бенчмарки и бюджеты

- [x] 4.1 Сценарии `-1` и `-1000`, `case-tree`, бюджеты (сначала `cargo bench -p render-bench --bench alloc`; wasm — в фоне, отдельный файл бюджетов)

## 5. Интеграция

- [x] 5.1 `docs/tree.md`; `prime-tree` в браузерный job `ci.yml` (три места)
- [x] 5.2 `cargo fmt --all -- --check`, clippy с исключениями CI и `--all-features` (и для wasm), тесты, deny, machete, `openspec validate port-tree`; CI — один раз по завершении волны 5 (не запускался)

## Workflow follow-up

- Архивировать; без пуша и ожидания CI до конца волны 5.
- Дальше по плану волны 5: TreeSelect, CascadeSelect, AutoComplete, InputMask.
