# Tasks

## 1. Основа

- [x] 1.1 `orderlist` в `theme.components` и `primeng_extra`, `cargo xtask gen-theme`; есть `p-orderlist-controls`
- [x] 1.2 Свериться с `moveUp/Top/Down/Bottom`, `moveDisabled` исходника

## 2. Крейт `prime-orderlist`

- [x] 2.1 Модуль `reorder.rs` с табличными тестами
- [x] 2.2 Компонент `OrderList`, `OrderListPt`: панель кнопок и `Listbox`, SSR-тесты, rustdoc
- [x] 2.3 Действия кнопок, `on_reorder`, `on_selection_change`, `disabled`
- [x] 2.4 Браузерные `tests/web.rs`

## 3. Showcase и e2e

- [x] 3.1 Страница `/orderlist`, меню, маршрут, `stylesheet(&[..])`, крейт в workspace и showcase
- [x] 3.2 `e2e/tests/pages.json` и `orderlist.spec.ts`

## 4. Бенчмарки и бюджеты

- [x] 4.1 Сценарии `-1` и `-1000`, `case-orderlist`, бюджеты

## 5. Интеграция

- [x] 5.1 `docs/orderlist.md`; `prime-orderlist` в браузерный job `ci.yml` (три места)
- [x] 5.2 fmt, clippy, тесты, deny, machete, `openspec validate port-orderlist`

## Workflow follow-up

- Волна 6 — CI один раз в конце волны; коммит и архив локально.
