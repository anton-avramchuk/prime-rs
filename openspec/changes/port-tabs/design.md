# Design

## Context

Требования — в `specs/tabs`. Исходники PrimeNG 21.1.10: `tabs/{tabs,tablist,tab,tabpanels,tabpanel}.ts` и `tabs/style/*`; демонстрации `apps/showcase/doc/tabs/*-doc.ts`. Образцы в проекте: `prime-accordion` (составной компонент, контекст, `Nav`), `prime-scrolltop` (подписка на элементы, `Closure`), `prime-select` (двусторонний `Option<String>`).

Наблюдения по исходнику:
- Фокус по `ArrowRight`/`ArrowLeft` ищет соседа через `nextElementSibling`/`previousElementSibling`, пропуская отключённых (`data-p-disabled`) и полосу (`data-pc-section="activebar"`); без соседа переходит к первой/последней вкладке. Мы повторяем поиск по DOM: список вкладок образуют соседние элементы, а не коллекция, известная корню.
- Полоса — `span`, `width`/`left` пишутся прямо в DOM после `setTimeout(0)` при смене значения и из `MutationObserver` вкладки; кнопки обновляются по `scroll` содержимого и `ResizeObserver` корня списка.
- Содержимое неактивной панели скрывается атрибутом `hidden`; `lazy` откладывает вывод до первой активации.
- Значок кнопок по умолчанию — `ChevronLeftIcon`/`ChevronRightIcon`, подписи — `aria.previous`/`aria.next` из локали.

## Decisions

### D1. Крейт и зависимости
`prime-tabs` с `Tabs`, `TabList`, `Tab`, `TabPanels`, `TabPanel`, `TabListPt`. Зависит от `prime-core`, `prime-icons` (`chevronleft`, `chevronright`), `web-sys`.

### D2. Контекст
`TabsCtx { id, value: RwSignal<Option<String>>, lazy, select_on_focus, show_navigators, tabindex, ink: RwSignal<Ink>, tabs_ref }` вводится в `Tabs` через `provide_context` (как в `Accordion`); части берут его `use_context` и паникуют с понятным сообщением вне `Tabs`. `TabList` кладёт ссылку на элемент списка вкладок в контекст через `RwSignal<Option<...>>`/`NodeRef`, чтобы `Tab` находил первую и последнюю вкладки.

### D3. Чистая логика (`state.rs`)
- `tab_id(tabs_id, value)`, `panel_id(tabs_id, value)`.
- `is_active(current, value)`; `tabindex_for(disabled, active, tabs_tabindex)`.
- `root_classes(scrollable)`, `tab_classes(active, disabled)`, `panel_classes(active)`.
- `Key` и `key_action(code) -> Option<KeyAction>` (`Next`, `Prev`, `First`, `Last`, `ScrollFirst`, `ScrollLast`, `Activate`).
- `wrap_target(index_of_current, enabled: &[bool], action) -> Option<usize>` — поиск цели по кругу среди доступных; DOM-обход использует её на массиве признаков «доступна».
- `should_render(lazy, has_rendered, active)`.
- `prev_enabled(scroll_left) / next_enabled(list_width, content_width, scroll_left, scroll_width, width)`, `prev_scroll(scroll_left_abs, width) -> f64`, `next_scroll(scroll_left, width, buttons, scroll_width) -> f64`.
Табличные тесты на каждую.

### D4. Навигация по DOM
В обработчике `keydown` вкладки по `NodeRef` вкладки берётся родитель (список вкладок), собираются его дочерние элементы с `role=tab`, формируется вектор «доступна» и применяется `wrap_target`; фокус — `HtmlElement::focus`, затем `scrollIntoView({block: nearest})`. Это избавляет от обхода `nextElementSibling` с рекурсией и даёт тестируемую чистую функцию.

### D5. Индикатор
`ink: RwSignal<(f64, f64)>` (ширина, смещение). Обновление: `Effect` (только `wasm32`) от `value` через `set_timeout(0)` и `ResizeObserver` на списке вкладок; измерение — `getBoundingClientRect` активной вкладки (`[role=tab][data-p-active=true]`) и списка. Стиль полосы — `width: Xpx; left: Ypx`. На SSR стиля нет.

### D6. Кнопки прокрутки
`prev_visible`/`next_visible: RwSignal<bool>`; `update_buttons` вызывается при монтировании (если `show_navigators`), по `scroll` содержимого и `ResizeObserver`. Клик меняет `scrollLeft` (с учётом `rtl`).

### D7. Ленивые панели
`Memo` от `should_render`; флаг «уже выводилась» хранится в `StoredValue<bool>` и устанавливается при первой активации. Содержимое панели — `ChildrenFn`, выводится замыканием по `Memo`.

### D8. CSS
В `@primeuix/styles` есть только модуль `tabs`, в нём стили всех пяти частей; он добавляется в `theme.components`. `cargo xtask gen-theme`, проверка, что CSS остальных не изменился.

### D9. Тесты
SSR: разметка частей, активность, `hidden`, связь `id`, `disabled`, `lazy`, `scrollable`. Браузерные `tests/web.rs`: клик, отключённая, `select_on_focus`, клавиши (по кругу, пропуск отключённой, `Home`/`End`, `Enter`/`Space`), roving `tabindex`, `lazy` с сохранением, индикатор, кнопки прокрутки. e2e: настоящая клавиатура, прокрутка кнопками, индикатор. Крейт добавляется в три места `ci.yml`.

### D10. Бенчмарки
Сценарии `tabs-1`/`-1000` (три вкладки и три панели), `case-tabs`; сначала `cargo bench -p render-bench --bench alloc`, для wasm — временный большой бюджет (заменять строкой, а не глобальным `sed`).

## Risks / Trade-offs

- [Нет `MutationObserver`] → индикатор не обновляется, если содержимое вкладки меняет её ширину без изменения размера списка; фиксируется в docs.
- [Контекст на `provide_context` без дочернего владельца] → как в `Accordion`; части создаются синхронно внутри `Tabs`.
- [Значения только строки] → отличие записывается в docs.

## Migration Plan

Миграции нет. Откат — `git revert`.

## Open Questions

Нет.
