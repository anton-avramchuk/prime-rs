# Design

## Context

Мотивация — в `proposal.md`, требования — в `specs/badge` и `specs/button`. Исходники PrimeNG 21.1.10: `packages/primeng/src/button/button.ts` (компонент `p-button`, строки 570–920), `button/style/buttonstyle.ts` (`classes`), `badge/badge.ts`, `badge/style/badgestyle.ts`.

Что выяснено при изучении:
- `p-button` в Angular — хост-элемент без классов; внутри него `<button class="p-button p-component …">`. Все классы и `data-p*` живут на `<button>`.
- Порядок содержимого: проецируемое содержимое, затем загрузочный индикатор или иконка, затем метка (`span.p-button-label`), затем `p-badge`. Метка и бейдж не выводятся при пользовательском шаблоне содержимого.
- Иконка в PrimeNG — строка с именем класса (`pi pi-check`) или шаблон; у нас шрифта нет, иконки — компоненты `prime-icons`.
- `p-badge` — хост-элемент `<p-badge>` с классами на нём самом, внутри только текст.
- В `bootstrap-workspace` готовы: `Pt`, `ClassNames`/`class_memo`, `PrimeConfig` (в т. ч. `ripple`), `Ripple`, `FocusTrap`, `SpinnerIcon` (feature `spinner`), CSS `button.css` и `badge.css` в `prime-theme`, бенчмарк-сценарии, бюджеты, showcase, `gen-spec`.

## Goals / Non-Goals

**Goals:**
- Button и Badge, совпадающие с PrimeNG по классам, атрибутам `data-p*`, порядку и составу DOM (кроме хост-элементов).
- Решения, которые станут шаблоном для остальных компонентов: форма пропсов, `pt`, иконки, ripple, тесты, бенчмарки, страница showcase.

**Non-Goals:**
- Директива `pButton`, `ButtonLabel`, `ButtonIcon`, `buttonProps`, контекст `Fluid` (`fluid` — только свойство), `unstyled`-режим, `SplitButton`, `ButtonGroup`.
- Полный набор хуков `pt` PrimeNG (`pt:*` на любую часть, опции `ptOptions`): сейчас корень и перечисленные части.

## Decisions

### D1. Реактивность свойств
Свойства, которые на практике меняются во время работы (`label`, `loading`, `disabled`, `severity`, `badge`, `icon_pos`, `value` у Badge), принимаются как `Signal<T>` (`#[prop(into)]`, значения и замыкания приводятся автоматически). Остальные (`type`, `tabindex`, `aria_label`) — простые значения. Это позволяет передавать и константу, и сигнал без `.into()` у вызывающего, а статические кнопки не платят за подписки.
*Альтернатива*: все свойства — `Signal` (проще единообразие, но лишние подписки на тысячах статических кнопок); все простые (нельзя менять `loading` без пересоздания).

### D2. Иконки и содержимое
`icon` и `loading_icon` — `Option<ViewFn>`. Иконка оборачивается в `<span class="p-button-icon …" data-p="…">` (как строковая иконка в PrimeNG), загрузочный индикатор по умолчанию — `<SpinnerIcon spin=true class="p-button-loading-icon …">`. `children` — `Option<ChildrenFn>`: выводятся первыми.
*Отличие от PrimeNG*: при шаблонной иконке PrimeNG передаёт класс самому SVG; мы всегда оставляем оболочку `span`, чтобы CSS (`order`, отступы) работал для любой иконки без знания её разметки.

### D3. DOM без хост-элемента
Отрисовывается только `<button>`. Хост `<p-button>`/`<p-badge>` в PrimeNG — деталь Angular; CSS темы привязан к классам. У Badge корнем становится `<span class="p-badge …">`.

### D4. Классы и `data-p`
Одна чистая функция на компонент строит набор модификаторов из структуры «снимок свойств» (`ButtonState`), из неё выводятся и `class`, и `data-p*`. Классы собираются `ClassNames` внутри `class_memo`, поэтому `class` обновляется только при смене строки. Функции тестируются без рендера (таблица «свойства → классы»), и их легко сверять с `buttonstyle.ts`.
Общий помощник для `data-p` (признаки через пробел, как `cn({...})`) добавляется в `prime-core` (`DataP`).

### D5. Pass-through
`prime-core::Pt` получает `style`; компонент определяет структуру частей: `ButtonPt { root, label, icon, loading_icon, badge }` (каждая — `Pt`), `BadgePt { root }`. Классы из `Pt` добавляются после собственных (`Pt::merge_class`), атрибуты — через `Pt::attrs()` с `{..}`. Структура частей лежит в крейте компонента, общий механизм — в `prime-core`.

### D6. Ripple и автофокус
На корне всегда класс `p-ripple`. `Ripple::attach` вызывается из `Effect` (только в браузере, после монтирования) при `PrimeConfig.ripple`, `Ripple` хранится и освобождается через `on_cleanup`. Автофокус — тот же `Effect`, `NodeRef<Button>`; на сервере `Effect` не выполняется, поэтому SSR безопасен.

### D7. Зависимости крейтов
`prime-button` → `prime-core`, `prime-dom`, `prime-icons` (feature `spinner`), `prime-badge`. `prime-badge` → `prime-core`. Тема подключается приложением (`prime-theme`, features `button`, `badge`); крейты компонентов `prime-theme` не тянут, чтобы не навязывать CSS.

### D8. Тесты
1. Юнит: таблица «свойства → классы и `data-p`» для обоих компонентов (SSR-независимо).
2. SSR (`to_html`): структура DOM, порядок, атрибуты, `pt`, a11y.
3. `wasm-bindgen-test` (Chrome): клик и недоступность, фокус и автофокус, ripple.
4. Бенчмарки: `button-1`, `button-1000`, `badge-1`, `badge-1000`; браузерный замер — сценарий в showcase.
5. Сравнение с PrimeNG: страница `/button` в showcase и запись в `e2e/tests/pages.json`.

### D9. Showcase
Появляется маршрутизация по компонентам: `/`, `/button`, `/badge` через `leptos_router` (0.8). Страница компонента повторяет демонстрации PrimeNG (`apps/showcase/doc/button/*`), список которых берётся из исходников.
*Альтернатива*: одна длинная страница — не подходит для сравнения скриншотов по страницам.

## Risks / Trade-offs

- [Оболочка `span` вокруг иконки отличается от PrimeNG при шаблонной иконке] → зафиксировано в спеке; визуальное сравнение в еженедельном job покажет расхождения.
- [`Signal`-свойства увеличивают размер компонента] → бюджет `case-button`; если вклад окажется большим, часть свойств станет простыми значениями.
- [Роутер Leptos увеличит showcase и `target/`] → он нужен только showcase и не попадает в библиотечные крейты.
- [Список демонстраций PrimeNG велик] → переносим те, что покрывают все свойства спеки; остальные добавляются по мере необходимости.

## Migration Plan

Миграции нет: новые крейты. Откат — `git revert`. Бюджеты в `benches/budgets.toml` пересматриваются после первого измерения Button.

## Open Questions

- Как называть поле `type` в Rust (`r#type` или `button_type`)? Решается при реализации, на спеку не влияет (в DOM остаётся атрибут `type`).
