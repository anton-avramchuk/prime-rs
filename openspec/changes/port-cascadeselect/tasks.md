# Tasks

## 1. Основа

- [x] 1.1 Добавить `cascadeselect` в `theme.components` и `primeng_extra` (`upstream.toml`), `cargo xtask gen-theme`; проверить, что CSS остальных компонентов не изменился и есть классы `p-cascadeselect*`
- [x] 1.2 Свериться с `onOptionChange`, `onOptionClick`, `onArrow*Key`, `show/hide`, `onOptionMouseEnter` исходника; значки `prime-icons`: chevrondown, angleright, times, spinner

## 2. Крейт `prime-cascadeselect`

- [x] 2.1 Модель `CascadeOption`, чистая логика `state.rs` (узлы, путь, фокус, клавиши, поиск) с табличными тестами
- [x] 2.2 Компонент `CascadeSelect`, `CascadeSelectPt`: корень, скрытое поле, метка, очистка, кнопка, панель, списки уровней, SSR-тесты, rustdoc
- [x] 2.3 Открытие и закрытие, каскад кликом и наведением, выбор листа, очистка, `breakpoint`
- [x] 2.4 Клавиатура, поиск, фокус, события
- [x] 2.5 Браузерные `tests/web.rs` (один последовательный тест)

## 3. Showcase и e2e

- [x] 3.1 Страница `/cascadeselect`, меню, маршрут, `stylesheet(&[..])`, крейт в workspace и showcase
- [x] 3.2 `e2e/tests/pages.json` и `cascadeselect.spec.ts`; `networkidle`, перезапуск сервера showcase; `PW_CHANNEL=chrome npx playwright test cascadeselect`

## 4. Бенчмарки и бюджеты

- [x] 4.1 Сценарии `-1` и `-1000`, `case-cascadeselect`, бюджеты

## 5. Интеграция

- [x] 5.1 `docs/cascadeselect.md`; `prime-cascadeselect` в браузерный job `ci.yml` (три места)
- [x] 5.2 `cargo fmt --all -- --check`, clippy с исключениями CI и `--all-features` (и для wasm), тесты, deny, machete, `openspec validate port-cascadeselect`; CI — один раз по завершении волны 5 (не запускался)

## Workflow follow-up

- Архивировать; без пуша и ожидания CI до конца волны 5.
- Дальше по плану волны 5: AutoComplete, InputMask.
