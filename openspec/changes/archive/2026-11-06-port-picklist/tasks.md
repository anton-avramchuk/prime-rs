# Tasks

## 1. Основа

- [x] 1.1 `picklist` в `theme.components`, `cargo xtask gen-theme`; `Listbox::header_content`
- [x] 1.2 Свериться с `moveRight/AllRight/Left/AllLeft`, `onSourceItemDblClick`, `*Disabled` исходника

## 2. Крейт `prime-picklist`

- [x] 2.1 Модуль `transfer.rs` с табличными тестами
- [x] 2.2 Компонент `PickList`, `PickListPt`: панели кнопок и два `Listbox`, SSR-тесты, rustdoc
- [x] 2.3 Перенос, перестановка, двойной клик, `keep_selection`, поиск, `disabled`, события
- [x] 2.4 Браузерные `tests/web.rs`

## 3. Showcase и e2e

- [x] 3.1 Страница `/picklist`, меню, маршрут, `stylesheet(&[..])`, крейт в workspace и showcase
- [x] 3.2 `e2e/tests/pages.json` и `picklist.spec.ts`

## 4. Бенчмарки и бюджеты

- [x] 4.1 Сценарии `-1` и `-1000`, `case-picklist`, бюджеты

## 5. Интеграция

- [x] 5.1 `docs/picklist.md`; `prime-picklist` в браузерный job `ci.yml` (три места)
- [x] 5.2 fmt, clippy, тесты, deny, machete, `openspec validate port-picklist`

## Workflow follow-up

- Волна 6 — CI один раз в конце волны; коммит и архив локально.
