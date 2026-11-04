# Design

## Context

Требования — в `specs/tree`. Исходники PrimeNG 21.1.10: `tree/tree.ts` (`UITreeNode` — узел и клавиатура, `Tree` — выбор, фильтр, раскрытие), `tree/style/treestyle.ts`; демонстрации `apps/showcase/doc/tree/{basic,single,multiple,checkbox,controlled,event,filter,lazy,template}-doc.ts`. Образцы в проекте — `prime-panelmenu` (раскрытые ключи, фокус по ключу, `PanelMenuRef`), `prime-multiselect` (разметка флажка и фильтра), `prime-iconfield`/`prime-inputtext`.

В PrimeNG состояние (`expanded`, `partialSelected`) пишется в сами узлы, а фокус хранится в DOM (`tabindex` переставляется вручную). У нас модель неизменяема, а состояние лежит в сигналах.

## Goals / Non-Goals

**Goals:** тот же DOM, классы, `aria-*`, выбор, фильтр и клавиши.

**Non-Goals:** перетаскивание, виртуальная прокрутка, `contextMenu`, `trackBy`, шаблоны `header`/`footer`/`loader`, `filterOptions`.

## Decisions

### D1. Состояние
`expanded: RwSignal<HashSet<String>>` (ключи; начальные из `TreeNode::expanded`, внешний сигнал `expanded_keys` — двусторонний) и `selection: RwSignal<Vec<String>>` (ключи; внешний сигнал — двусторонний). Частичный выбор вычисляется из выбора (`partial(node) = не выбран и выбран хотя бы один потомок`), а не хранится.

### D2. Чистая модель (`state.rs`)
Плоский список узлов с ключом, родителем, уровнем и признаками (`leaf`, `selectable`), построенный из `value`. Функции без `web_sys` и с табличными тестами: `visible(nodes, expanded)`, `toggle_select(...)` для каждого режима (включая `meta`), `propagate_up/down`, `is_partial`, `filter_tree(nodes, text, by, strict)` (порт `_filter`/`isFilterMatched`/`findFilteredNodes`; возвращает отфильтрованное дерево и ключи для раскрытия), `next/prev/first/last`, `arrow_left/right`, `tab_stop`.

### D3. Фильтр
Поле — `InputText` в `IconField` с `SearchIcon`; текст — сигнал; отфильтрованное дерево — `Memo` от `value` и текста. Ключи, раскрытые фильтром, добавляются в `expanded` при вводе (как `node.expanded = true` на копиях PrimeNG).

### D4. Разметка и фокус
Рекурсивная функция уровня; фокус — сигнал ключа, `tabindex` у `li` реактивный (`0` у узла-«опоры»: сфокусированный, иначе выбранный, иначе первый); перенос фокуса — `focus()` по `id` (`<id дерева>_<ключ>`) под `cfg(wasm32)`. Чтение сигналов при построении вида — только `*_untracked` (иначе перестраивается всё дерево и теряется фокус).

### D5. Флажок
Разметка `p-checkbox` внутри узла своя (по `prime_checkbox::{CheckboxState, write_classes, data_p}`), как в MultiSelect; частичное состояние — `MinusIcon`, `data-p-partialchecked`.

### D6. Ленивая загрузка
Нет встроенного загрузчика: `on_node_expand` получает узел, а пользователь обновляет `value`-сигнал (`loading`, `children`). Спиннер вместо шеврона — при `loading_mode = Icon`, маска — при `loading` самого дерева.

### D7. Тесты
SSR: разметка, ARIA, режимы выбора, раскрытие из модели, пусто, шаблоны, `pt`. Браузерные `tests/web.rs` одним последовательным тестом: раскрытие, выбор во всех режимах, распространение флажков, фильтр, клавиатура, события; после `dispatch` клавиши ждать `sleep(10)`. e2e: раскладка темой, отступы по уровням, клавиатура, фильтр, флажки.

## Risks / Trade-offs

- [Своя модель вместо мутабельных узлов] Расхождение в частичном выборе → он выводится из выбора, как итог `propagateUp`, табличные тесты.
- [Фильтр копирует узлы] В PrimeNG выбор по отфильтрованным копиям; у нас ключи, поэтому выбор общий для полного и отфильтрованного дерева.
- [Большие деревья] Вид перестраивается при смене `value` и фильтра; раскрытие и выбор реактивны на уровне атрибутов. Виртуальной прокрутки нет.

## Open Questions

- Как PrimeNG ведёт себя при `Enter` на узле при `selection_mode = None` (ничего) — сверить.

## Migration Plan

Миграции нет. Откат — `git revert`.
