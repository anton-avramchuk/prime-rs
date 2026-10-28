# Design

## Context

Требования — в `specs/accordion` и дельте `specs/collapsible`. Исходники PrimeNG 21.1.10: `packages/primeng/src/accordion/accordion.ts` (четыре компонента в одном файле), `accordion/style/accordionstyle.ts`, демонстрации `apps/showcase/doc/accordion`. Образцы: `crates/prime-panel`, `crates/prime-fieldset`, `prime-core::collapsible`. Анимация в PrimeNG идёт на элементе `p-motion` внутри `p-accordioncontent`; CSS `.p-accordioncontent .p-motion { display: grid; grid-template-rows: 1fr }` лежит в блоке `/* For PrimeNG */` файла `accordionstyle.ts`, а не в `@primeuix/styles`, поэтому генератор темы его пока не берёт.

## Decisions

### D1. Контекст между частями
`Accordion` кладёт в контекст Leptos `AccordionCtx { id, value: RwSignal<Vec<String>>, multiple: Prop<bool>, select_on_focus: Prop<bool>, on_open, on_close, pt: Arc<AccordionPt> }`, `AccordionPanel` — `PanelCtx { value: String, disabled: Prop<bool>, active: Signal<bool> }`. Заголовок и содержимое читают оба контекста и не принимают значений сами. Вне `Accordion` или `AccordionPanel` части паникуют с понятным сообщением (`expect`).

### D2. Значение
`value: Option<RwSignal<Vec<String>>>` и `default_value: Vec<String>`; список общий для обоих режимов (в PrimeNG — `string | number | array`). Значения — строки; числа приводятся `to_string()`. Переключение: одиночный режим — `[v]` либо пусто, если `v` уже единственное; `multiple` — добавить или убрать. Это чистая функция `toggle_value(&[String], &str, multiple) -> Vec<String>` с таблицей юнит-тестов. `active = value.contains(panel_value)`.

### D3. События
Клик по заголовку (и Enter/Space, `select_on_focus`) вычисляет `was_active` и `is_active` до и после `toggle`, как в PrimeNG: открытие даёт `on_open`, закрытие — `on_close`. Событие `AccordionToggle { original_event: web_sys::Event, index: String }`.

### D4. Анимация через общую логику
В `prime-core::collapsible` добавляется `bind_region_with(region, collapsed: impl Fn() -> bool + 'static, on_done)`; `bind_region` сводится к ней (`move || state.get()`), поведение прежнее. Содержимое: `div.p-accordioncontent[role=region] > div.p-motion (NodeRef, анимируется) > div.wrapper > div.content`. Начальный стиль `display: none` у `p-motion`, если панель неактивна (`hidden_style`). Скрытие — `display: none`, а не `visibility`, как у Panel и Fieldset (отличие от PrimeNG `hideStrategy="visibility"`; фиксируется в docs).

### D5. CSS темы
`xtask gen-theme` читает из `<name>/style/<name>style.ts` часть после маркера `/* For PrimeNG */` до закрывающего обратного апострофа, прогоняет `dt('a.b.c')` через тот же `resolved` и добавляет к CSS компонента. Для компонентов без маркера ничего не меняется (CSS Panel, Fieldset и остальных остаётся побайтово прежним, проверяется `git diff`). Нерешённый `dt()` по-прежнему ошибка.

### D6. Клавиатура
Обработчик `keydown` на заголовке (клиент): `ArrowDown/ArrowUp/Home/End` ищут соседние `.p-accordionpanel` в родителе (`next/previous_element_sibling`), пропуская `data-p-disabled="true"`, и фокусируют внутренний `.p-accordionheader`; конец списка зацикливается. `Enter/Space/NumpadEnter` вызывают переключение и `preventDefault`. `select_on_focus` — обработчик `focus`. Навигация по DOM вынесена в функции, чистая часть (выбор индекса следующего с зацикливанием и пропуском) тестируется юнит-тестами.

### D7. Ветви и перерисовка
Активность зависит от общего сигнала, поэтому статической ветви нет: классы, `aria-*` и иконка — вложенные замыкания над `active`; вся панель не перерисовывается. `NodeRef` только на `wasm32`. Разметка — `macro_rules!` на каждую часть.

### D8. Демонстрации
`basic`, `multiple`, `disabled`, `controlled` (внешний сигнал и кнопки), `dynamic` (`<For>`), `template` (слот иконки и аватар/бейдж). Иконки «плюс/минус» — SVG из `prime-icons`.

## Risks / Trade-offs

- [Расхождение `p-motion` с PrimeNG] → класс и позиция элемента те же, имя в docs; визуальное сравнение в `pages.json`.
- [Значения только строки] → компактный API; числа через `to_string`, фиксируется в docs.
- [Изменение xtask затрагивает все компоненты] → правило применяется только при наличии маркера; `git diff` по сгенерированному CSS остальных компонентов должен быть пустым.
- [Вложенные Accordion] → контекст берётся ближайший (`use_context`), вложенность работает.

## Migration Plan

Миграции нет: новый крейт, добавление функции в `prime-core`. Откат — `git revert`.

## Open Questions

Нет.
