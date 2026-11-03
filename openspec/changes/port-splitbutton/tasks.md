# Tasks

## 1. Основа

- [x] 1.1 Добавить `splitbutton` в `theme.components` (`upstream.toml`), `cargo xtask gen-theme`; проверить, что CSS остальных компонентов не изменился и что есть классы `p-splitbutton*`

## 2. Крейт

- [x] 2.1 `crates/prime-splitbutton`: `state.rs` (`root_classes`, `is_toggle_key`) с табличными тестами
- [x] 2.2 Компонент `SplitButton`, `SplitButtonPt`: корень, основная кнопка (`Button`), кнопка раскрытия (своя разметка), `content`, модификаторы, SSR-тесты, rustdoc
- [x] 2.3 Меню: `Menu popup` с `MenuRef`, привязка к корню, переключение по клику и ↓/↑, закрытие по клику на основную кнопку, `aria-expanded`, события показа и скрытия; браузерные `tests/web.rs`

## 3. Showcase и e2e

- [x] 3.1 Страница `/splitbutton` (`basic`, `disabled`, `icons`, `outlined`, `raised`, `raisedtext`, `rounded`, `severity`, `sizes`, `template`, `text`), меню, маршрут, `stylesheet(&[..])`, крейт в workspace и showcase
- [x] 3.2 `e2e/tests/pages.json` и `splitbutton.spec.ts` (разметка, клики, клавиши, положение панели, команда пункта); ждать `networkidle`, перезапустить сервер showcase; `PW_CHANNEL=chrome npx playwright test splitbutton`

## 4. Бенчмарки и бюджеты

- [x] 4.1 Сценарии `-1` и `-1000`, `case-splitbutton` и бюджеты (сначала `cargo bench -p render-bench --bench alloc`; замер wasm — в фоне, с отдельным файлом бюджетов)

## 5. Интеграция

- [x] 5.1 `docs/splitbutton.md` (отличия от PrimeNG, что не проверялось); `prime-splitbutton` в браузерный job `ci.yml` (три места)
- [x] 5.2 `cargo fmt --all -- --check`, clippy с исключениями CI и `--all-features` (и для wasm-цели), тесты, deny, machete, `openspec validate port-splitbutton`; CI `dev` зелёный

## Workflow follow-up

- Архивировать после CI.
- Следующее: Dock.
