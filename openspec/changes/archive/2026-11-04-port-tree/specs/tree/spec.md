# tree Specification

## Purpose
Tree показывает иерархические данные с раскрытием, выбором узлов (в том числе флажками) и фильтром.

## ADDED Requirements

### Requirement: Разметка
`Tree` крейта `prime-tree` SHALL принимать `Vec<TreeNode>` и выводить `div.p-tree.p-component` (`p-tree-selectable` при заданном режиме выбора, `p-tree-loading` при `loading`, `p-tree-flex-scrollable` при `scroll_height = "flex"`), `div.p-tree-root` и `ul.p-tree-root-children[role=tree]` (`aria-label`, `aria-labelledby`). Узел SHALL быть `li.p-tree-node[role=treeitem]` (`p-tree-node-leaf` у листа; `aria-label`, `aria-level`, `aria-setsize`, `aria-posinset`, `aria-expanded`, `aria-selected` в режимах `Single`/`Multiple`, `aria-checked` в `Checkbox`, `tabindex`, `data-id` = `key`) с `div.p-tree-node-content` (`p-tree-node-selectable`, `p-tree-node-selected`; левый отступ `уровень × indentation` rem), кнопкой `button.p-tree-node-toggle-button`, флажком `p-tree-node-checkbox` (в режиме `Checkbox`), значком `p-tree-node-icon`, подписью `p-tree-node-label` и, у раскрытого узла с детьми, `ul.p-tree-node-children[role=group]`. При пустом списке и без загрузки SHALL выводиться `div.p-tree-empty-message`.

#### Scenario: Состав
- **WHEN** отрисовано дерево с раскрытым узлом из двух детей
- **THEN** выведены `ul.p-tree-root-children`, узел, кнопка-шеврон и `ul.p-tree-node-children` с двумя `li`

#### Scenario: Пусто
- **WHEN** `value` пуст
- **THEN** выведено сообщение из `empty_message`

### Requirement: Раскрытие
Клик по кнопке узла SHALL переключать его раскрытие и вызывать `on_node_expand`/`on_node_collapse`; шеврон SHALL быть вправо у свёрнутого и вниз у раскрытого узла. Начальное раскрытие SHALL браться из `TreeNode::expanded` либо из переданного сигнала `expanded_keys`. У узла со `loading = true` при `loading_mode = Icon` вместо шеврона SHALL быть спиннер.

#### Scenario: Переключение
- **WHEN** пользователь кликает по кнопке свёрнутого узла
- **THEN** узел раскрыт, вызван `on_node_expand`

### Requirement: Выбор
В режиме `Single` клик по узлу SHALL выбирать его, повторный — снимать; в `Multiple` — добавлять и убирать из выбора (с `meta_key_selection` без Ctrl/Meta выбор заменяется, с ним — дополняется или снимается). В режиме `Checkbox` клик SHALL переключать узел и, при `propagate_selection_down`, всех потомков, а при `propagate_selection_up` — пересчитывать предков (предок выбран, когда выбраны все его дети; частично выбранный предок получает `data-p-partialchecked` и неполное состояние флажка). Узлы с `selectable = false` SHALL не выбираться. Клик по кнопке раскрытия SHALL не менять выбор. События `on_node_select`/`on_node_unselect` SHALL вызываться при выборе/снятии.

#### Scenario: Единичный выбор
- **WHEN** пользователь кликает по узлу в режиме `Single`, затем по нему же
- **THEN** сначала ключ в выборе и `aria-selected="true"`, затем выбор пуст

#### Scenario: Флажки
- **WHEN** в режиме `Checkbox` отмечены все дети узла
- **THEN** узел тоже отмечен; если отмечены не все — узел частично выбран

### Requirement: Фильтр
При `filter` SHALL выводиться поле поиска с значком. Ввод SHALL оставлять узлы, у которых `filter_by` (поля через запятую: `label`, `key`) содержит текст без учёта регистра; в режиме `Lenient` совпавший узел сохраняет всех детей, в `Strict` — только совпавших; предки совпавших узлов остаются и раскрываются. Пустое поле SHALL возвращать полное дерево; вызывается `on_filter`.

#### Scenario: Совпадение вложенного узла
- **WHEN** введён текст, совпадающий только с подписью вложенного узла
- **THEN** остаются он и его предки, предки раскрыты

### Requirement: Клавиатура
Один узел SHALL иметь `tabindex="0"` (выбранный, иначе первый), остальные `-1`. ↓/↑ SHALL переходить к следующему/предыдущему видимому узлу, → — раскрывать свёрнутый узел и переходить к его первому потомку, ← — сворачивать раскрытый либо идти к родителю, `Enter`/`Space` — выбирать узел, `Home`/`End` — к первому/последнему видимому, `Tab` — выходить из дерева.

#### Scenario: Стрелки
- **WHEN** фокус на свёрнутом узле с детьми, нажата →
- **THEN** узел раскрыт, фокус на первом ребёнке

### Requirement: Шаблоны
`node` SHALL заменять содержимое подписи узла (получает узел), `togglericon` — шеврон.

#### Scenario: Свой узел
- **WHEN** задан `node`
- **THEN** внутри `p-tree-node-label` выведено его содержимое
