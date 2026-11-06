# Tasks

## 1. Основа

- [x] 1.1 `treetable` в `theme.components` и `primeng_extra`, `cargo xtask gen-theme`
- [x] 1.2 Свериться с `serializeNodes`, `serializePageNodes`, `sortNodes`, `propagateSelectionUp/Down`, `TreeTableToggler` исходника

## 2. Крейт `prime-treetable`

- [x] 2.1 Модель `tree.rs` с табличными тестами
- [x] 2.2 Компонент `TreeTable<T>`, `TreeNode<T>`, `TreeTablePt`: разметка, кнопка раскрытия, заголовки, пагинатор, SSR-тесты, rustdoc
- [x] 2.3 Раскрытие, сортировка, выбор кликом и флажками, события узлов
- [x] 2.4 Браузерные `tests/web.rs`

## 3. Showcase и e2e

- [x] 3.1 Страница `/treetable`, меню, маршрут, крейт в workspace и showcase
- [x] 3.2 `e2e/tests/pages.json` и `treetable.spec.ts`

## 4. Бенчмарки и бюджеты

- [x] 4.1 Сценарии `-1` и `-1000`, `case-treetable`, бюджеты

## 5. Интеграция

- [x] 5.1 `docs/treetable.md`; `prime-treetable` в браузерный job `ci.yml` (три места)
- [x] 5.2 fmt, clippy, тесты, deny, machete, `openspec validate port-treetable`

## Workflow follow-up

- Волна 7 завершена: пуш и один прогон CI всей волны.
