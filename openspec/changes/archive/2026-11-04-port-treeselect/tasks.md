# Tasks

## 1. Основа

- [x] 1.1 Добавить `treeselect` в `theme.components` и `primeng_extra` (`upstream.toml`), `cargo xtask gen-theme`; проверить, что CSS остальных компонентов не изменился и есть классы `p-treeselect*`
- [x] 1.2 Свериться с `onKeyDown`, `onClick`, `show/hide`, `updateTreeState`, `label` исходника; значки `prime-icons`: chevrondown, times

## 2. Крейт `prime-treeselect`

- [x] 2.1 Чистая функция `ancestors_of`, метка выбранных узлов; табличные тесты
- [x] 2.2 Компонент `TreeSelect`, `TreeSelectPt`: корень, скрытое поле, метка (запятая/чипы), очистка, кнопка, панель с `Tree`, SSR-тесты, rustdoc
- [x] 2.3 Открытие и закрытие (клик, клик вне, `resize`, `Escape`), z-index, выравнивание, раскрытие предков, выбор в трёх режимах
- [x] 2.4 Клавиатура скрытого поля, фокус, `on_focus`/`on_blur`, события
- [x] 2.5 Браузерные `tests/web.rs` (один последовательный тест)

## 3. Showcase и e2e

- [x] 3.1 Страница `/treeselect`, меню, маршрут, `stylesheet(&[..])`, крейт в workspace и showcase
- [x] 3.2 `e2e/tests/pages.json` и `treeselect.spec.ts`; `networkidle`, перезапуск сервера showcase; `PW_CHANNEL=chrome npx playwright test treeselect`

## 4. Бенчмарки и бюджеты

- [x] 4.1 Сценарии `-1` и `-1000`, `case-treeselect`, бюджеты

## 5. Интеграция

- [x] 5.1 `docs/treeselect.md`; `prime-treeselect` в браузерный job `ci.yml` (три места)
- [x] 5.2 `cargo fmt --all -- --check`, clippy с исключениями CI и `--all-features` (и для wasm), тесты, deny, machete, `openspec validate port-treeselect`; CI — один раз по завершении волны 5 (не запускался)

## Workflow follow-up

- Архивировать; без пуша и ожидания CI до конца волны 5.
- Дальше по плану волны 5: CascadeSelect, AutoComplete, InputMask.
