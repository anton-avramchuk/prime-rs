# Tasks

## 1. Основа

- [x] 1.1 `dataview` в `theme.components`, `cargo xtask gen-theme`
- [x] 1.2 Свериться с `paginate`, `sort`, `filter`, `isEmpty`, шаблоном исходника

## 2. Крейт `prime-dataview`

- [x] 2.1 Модуль `data.rs` с табличными тестами
- [x] 2.2 Компонент `DataView<T>`, `DataViewPt`: разметка, пагинатор, пустое состояние, загрузка, SSR-тесты, rustdoc
- [x] 2.3 События `on_page`, `on_lazy_load`, возврат `first`, сортировка, `lazy`
- [x] 2.4 Браузерные `tests/web.rs`

## 3. Showcase и e2e

- [x] 3.1 Страница `/dataview`, меню, маршрут, крейт в workspace и showcase
- [x] 3.2 `e2e/tests/pages.json` и `dataview.spec.ts`

## 4. Бенчмарки и бюджеты

- [x] 4.1 Сценарии `-1` и `-1000`, `case-dataview`, бюджеты

## 5. Интеграция

- [x] 5.1 `docs/dataview.md`; `prime-dataview` в браузерный job `ci.yml` (три места)
- [x] 5.2 fmt, clippy, тесты, deny, machete, `openspec validate port-dataview`

## Workflow follow-up

- Волна 7: после каждого компонента — коммит и архив локально, CI один раз в конце волны.
