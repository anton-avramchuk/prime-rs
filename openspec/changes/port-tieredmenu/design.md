# Design

## Context

Требования — в `specs/tieredmenu`. Исходники PrimeNG 21.1.10: `tieredmenu/tieredmenu.ts` (около 1400 строк: компонент `TieredMenuSub` для одного уровня и `TieredMenu` с состоянием и клавиатурой), `tieredmenu/style/tieredmenustyle.ts`; стили `@primeuix/styles/tieredmenu`; демонстрации `apps/showcase/doc/tieredmenu/{basic,popup,command,template}-doc.ts` (`router-doc` — маршрутизатор, не переносится). Образцы в проекте: `prime-menu` (модель `MenuItem`, `MenuRef`, popup: выравнивание, слушатели окна, фокус), `prime-dock` (`matchMedia`), `prime-dialog`/`prime-speeddial` (клавиатура и чистая логика в `state.rs`).

Наблюдения по исходнику:
- Дерево «обработанных пунктов» (`createProcessedItems`): у каждого `{ item, index, level, key, parent, parentKey, items }`, ключ — `"0"`, `"0_1"`, `"0_1_2"`.
- Состояние: `activeItemPath` (цепочка обработанных пунктов от корня до самой глубокой раскрытой группы), `focusedItemInfo { index, level, parentKey, item }`, флаги `dirty` (хоть раз раскрывали кликом), `focused`, `searchValue`, `visible` (popup).
- Видимый список (`visibleItems`) — дочерние пункты активной группы, чей ключ равен `parentKey` фокуса, иначе корневые. Пункт допустим (`isValidItem`), если он не отключён, не разделитель и виден.
- Клик (`onItemClick`): если пункт уже в пути — путь обрезается до него (сворачивает ветку), фокус остаётся на пункте, `dirty = true`; иначе группа раскрывается (`onItemChange`), а лист закрывает всё (`hide`) и переводит фокус на корневой пункт ветки.
- `onItemChange`: из пути убираются пункты с `parentKey` пункта и ключом пункта, группа добавляется в путь, фокус переходит на пункт; при наведении на узком окне (`queryMatches`) путь не меняется.
- Наведение (`onItemMouseEnter`): без сенсорного ввода срабатывает только если `dirty`; подуровень передаёт событие наверх только при `autoDisplay`.
- Клавиши: ↓ — следующий допустимый (или первый/выбранный при пустом фокусе), ↑ — предыдущий (или последний), `Alt+↑` в popup закрывает; → раскрывает группу, уходит внутрь и берёт первый пункт; ← поднимается на родителя и сворачивает подменю; `Home`/`End`; `Enter`/`Space` кликают по ссылке пункта; `Escape` закрывает и возвращает фокус; `Tab` активирует лист под фокусом и закрывает; печатаемый символ — поиск по началу подписи среди видимых (поиск циклический от текущего), сброс через 500 мс.
- Закрытие: `hide` сбрасывает путь и фокус, в popup ещё скрывает панель; слушатель клика на документе висит, пока путь не пуст.
- Положение подменю (`nestedPosition`): справа от пункта (`left: 100%`); если правый край выйдет за окно — слева (`left: -100%`).
- `TieredMenuSub` показывает подменю инлайн-стилем `display: flex|none` по активности пункта.

## Goals / Non-Goals

**Goals:** тот же DOM, классы, `aria-*`, клавиши, поведение путей и popup.

**Non-Goals:** `appendTo`, `routerLink`, подсказки пунктов, HTML-метки, `Ripple`, `Bind`, `pTemplate`, изменение `SplitButton`.

## Decisions

### D1. Модель и крейт
Пункты — `prime_menu::MenuItem` (в нём уже есть `items`, `separator`, `visible`, `disabled`, `command`, `url`, `target`, `title`, `icon`, `style_class`, `link_class`, `icon_class`). `TieredMenu` не клонирует дерево на каждый рендер: строится плоское описание по ключам.

### D2. Чистая логика (`model.rs`)
`Node { key, parent_key, level, index, item_path: Vec<usize> }` и функции над деревом: `build(items) -> Vec<Node>` (узлы в порядке обхода в глубину), `children(nodes, parent_key)`, `is_valid(item)`, `visible_items(nodes, active_path, focus_parent)`, `find_next/prev/first/last/selected`, `search(visible, focus_index, text)`, `on_click(path, node, is_group, is_root) -> ClickOutcome { Collapse(path), Expand(path), Leaf }`, `item_change(path, node, hover, mobile)`, `arrow_left/right(...)`, `aria_set_size/pos_inset`, `item_id(menu_id, key)`, `submenu_side(left, item_width, sub_width, viewport, scrollbar)`. Табличные тесты по каждой функции (включая вложенные деревья и отключённые/скрытые пункты).

### D3. Состояние компонента
`active_path: RwSignal<Vec<String>>` (ключи), `focus: RwSignal<Focus { key: Option<String>, parent_key: String }>`, `dirty: StoredValue<bool>`, `focused: RwSignal<bool>`, `search: StoredValue<String>` с таймером сброса, `mobile: RwSignal<bool>` (`matchMedia`, только браузер). Узлы дерева — `Memo` по `model`.

### D4. Разметка
Рекурсивный компонент-функция `level_view(parent_key)` выводит `ul` и дочерние `li` с вложенными `ul` для групп; `display` подменю — реактивный инлайн-стиль по `active_path`. Подпись и значок — `ViewFn` пункта; представление `item` заменяет содержимое `div.p-tieredmenu-item-content`.

### D5. Положение подменю
После показа подменю (эффект на браузере) считается сторона по `submenu_side` и записывается в `left` подменю (`100%` или `-100%`); нужная ширина берётся у скрытого элемента временным показом (как `element_size` в `prime-dom`).

### D6. Popup
`TieredMenuRef` (как `MenuRef` из `prime-menu`: `new`, `show`, `show_at`, `toggle`, `hide`, `visible`). Показ/скрытие, выравнивание `AlignMode::Relative`, слой `ZGroup::Menu`, слушатели клика/`resize`/прокрутки — копируем схему `prime-menu` (подписки в следующем тике, снятие при скрытии и размонтировании). В не-popup режиме слушатель клика висит, пока путь не пуст.

### D7. Клавиатура и фокус
Один обработчик `keydown` на корневом `ul` (как `menuKeydown`), `aria-activedescendant` — реактивный атрибут от `focus`. `Enter`/`Space` кликают по `a` пункта (`element.click()`), как в PrimeNG. Поиск — буфер строки, таймер 500 мс через `set_timeout` (только браузер).

### D8. Тесты
SSR: разметка корня, пункты, группы (`aria-haspopup`, `aria-expanded`, `aria-setsize`, `aria-posinset`), разделители, значки/метки, `popup` (панели нет, пока не показана), `pt`, `item`. Браузерные `tests/web.rs` одним последовательным тестом (общие слушатели окна): клик по группе раскрывает/сворачивает, повторный клик по ветке, лист вызывает `command` и закрывает всё, наведение после клика, `auto_display = false`, клавиши (↓/↑/→/←/`Home`/`End`/`Enter`/`Escape`/`Tab`), поиск по символам, отключённые/скрытые пункты, mobile (через `breakpoint` больше окна), popup (`toggle`, клик вне, `Escape`, `Alt+↑`, возврат фокуса), `item`/`submenu_icon`. e2e: стили темы, раскрытие вправо, отзеркаливание у правого края окна, клавиатура, popup, `command` с Toast.

### D9. Демонстрации и бенчмарки
`basic` (дерево File/Edit/Users/Events/Quit как в PrimeNG, значки заменены `prime-icons` и пропущены, где аналога нет), `popup`, `command` (Toast), `template`. Бенч `tieredmenu-1`/`-1000` (трёхуровневое дерево) и `case-tieredmenu`.

## Risks / Trade-offs

- [Объём состояния] Много связанных сигналов; снижаем риск тем, что вся логика переходов — чистые функции с таблицами.
- [Положение подменю] Зависит от размеров в браузере; сторона вычисляется чистой функцией и проверяется e2e у правого края.
- [Дублирование popup-схемы] Схема слушателей копируется из `prime-menu`; вынос общего в `prime-dom` — отдельное изменение, когда появятся ещё потребители (ContextMenu).

## Open Questions

- Подключать ли `TieredMenu` в `SplitButton`: оставлено на отдельное изменение.

## Migration Plan

Миграции нет. Откат — `git revert`.
