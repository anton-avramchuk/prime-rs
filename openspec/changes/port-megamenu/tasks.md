# Tasks

## 1. Основа

- [x] 1.1 Добавить `megamenu` в `theme.components` (`upstream.toml`; блока `For PrimeNG` в `megamenustyle.ts` нет), `cargo xtask gen-theme`; проверить, что CSS остальных компонентов не изменился и есть классы `p-megamenu*`
- [x] 1.2 Свериться с `onArrow*Key`, `onItemChange`, `findNextItemIndex`/`findPrevItemIndex`, `onItemClick`, `hide`, `searchItems` исходника; решить (D1), что переиспользовать из `prime-tieredmenu`
- [x] 1.3 Значки `prime-icons`: angledown, angleright, bars (и для демонстрации)

## 2. Крейт `prime-megamenu`

- [x] 2.1 Модель `MegaMenuItem`/`MegaMenuSubmenu`, `MegaMenuOrientation`; чистая логика `state.rs` (узлы с ключами, видимые, переходы обеих ориентаций, поиск, `column_class`) с табличными тестами
- [x] 2.2 Компонент `MegaMenu`, `MegaMenuPt`, `MegaMenuItemContext`: корень, `start`/`end`, кнопка, корневой список, пункты, наложение, колонки, подменю, разделители, `item`/`submenu_icon`/`menu_icon`, SSR-тесты, rustdoc
- [x] 2.3 Путь активных пунктов, фокус, наведение (`dirty`, mobile), клик вне, `resize`
- [x] 2.4 Клавиатура корня и наложения, поиск, `aria-activedescendant`, `on_focus`/`on_blur`
- [x] 2.5 Мобильный режим: `matchMedia`, кнопка, слой `Menu`, возврат фокуса, клавиши кнопки
- [x] 2.6 Браузерные `tests/web.rs` (один последовательный тест)

## 3. Showcase и e2e

- [x] 3.1 Страница `/megamenu` (`basic`, `vertical`, `command`, `template`), меню, маршрут, `stylesheet(&[..])`, крейт в workspace и showcase
- [x] 3.2 `e2e/tests/pages.json` и `megamenu.spec.ts` (стили колонок, вертикальная ориентация, клавиатура, команда, мобильная кнопка); `networkidle`, перезапуск сервера showcase; `PW_CHANNEL=chrome npx playwright test megamenu`

## 4. Бенчмарки и бюджеты

- [x] 4.1 Сценарии `-1` и `-1000`, `case-megamenu` (с нужными `dep:`), бюджеты (сначала `cargo bench -p render-bench --bench alloc`; wasm — в фоне, отдельный файл бюджетов, не `xtask wasm-size --help`; бюджет менять `sed` по номеру строки)

## 5. Интеграция

- [x] 5.1 `docs/megamenu.md` (отличия от PrimeNG, что не проверялось); `prime-megamenu` в браузерный job `ci.yml` (три места)
- [x] 5.2 `cargo fmt --all -- --check`, clippy с исключениями CI и `--all-features` (и для wasm), тесты (SSR по одному; wasm — по строке `test result`), deny, machete, `openspec validate port-megamenu`; CI — один раз по завершении волны 5 (не запускался)

## Workflow follow-up

- Архивировать; без пуша и ожидания CI до конца волны 5.
- Дальше по плану волны 5: Tree, TreeSelect, CascadeSelect, AutoComplete, InputMask.
