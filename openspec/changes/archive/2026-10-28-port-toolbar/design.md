# Design

## Context

Требования — в `specs/toolbar`. Исходники PrimeNG 21.1.10: `packages/primeng/src/toolbar/toolbar.ts`, `toolbar/style/toolbarstyle.ts`, демонстрации `apps/showcase/doc/toolbar`. Образец — `crates/prime-card` (`docs/card.md`): слоты `ViewFn`, статическая и реактивная ветви, `macro_rules!` разметки, `pt` по частям. Ни общей логики, ни клиентского кода у Toolbar нет.

## Decisions

### D1. Слоты
`start_template`, `center_template`, `end_template` — `Option<ViewFn>`; область рисуется только при `Some`. `children` (необязательные) стоят перед областями, как `<ng-content>` в шаблоне PrimeNG.

### D2. Ветви
Статическая — когда `aria_labelledby`, `style`, `style_class` статические. Реактивная — замыкания над `Arc<Props>` и `Arc<ToolbarPt>`, только для изменяемых атрибутов корня; области и слоты строятся один раз (слоты не зависят от сигналов свойств). Разметка — общий `macro_rules!`.

### D3. Классы и атрибуты
Корень: `p-toolbar p-component` + `style_class` + класс из `pt.root`, `role="toolbar"`, `aria-labelledby`. Порядок областей фиксирован. `data-p` у Toolbar в PrimeNG нет.

### D4. Демонстрации
`basic` собирается из имеющихся компонентов: кнопки с иконками (`Button`, `text`, `secondary`), `InputText` с иконкой поиска вместо IconField, `Button` «Save» вместо SplitButton. `custom` — логотип (SVG), кнопки текстом и аватар. Отличия фиксируются в `docs/toolbar.md`.

### D5. Тесты
SSR: структура и порядок, роль, пустые области, `children` перед областями, `aria-labelledby`, `style`, `pt`, паритет ветвей. Браузерных тестов нет: клиентского поведения нет (в проекте так же у Card).

## Risks / Trade-offs

- [Демонстрация `basic` отличается от PrimeNG компонентами] → отличие записано в docs, сравнение скриншотов остаётся отчётом, а не блокировщиком.

## Migration Plan

Миграции нет: новый крейт. Откат — `git revert`.

## Open Questions

Нет.
