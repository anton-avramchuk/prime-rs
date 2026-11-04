# treeselect Specification

## Purpose
TreeSelect позволяет выбрать узел или несколько узлов дерева в выпадающей панели.

## Requirements

### Requirement: Разметка
`TreeSelect` крейта `prime-treeselect` SHALL принимать `Vec<TreeNode>` и выводить `div.p-treeselect.p-component.p-inputwrapper` (классы состояния: `p-treeselect-display-chip`, `p-disabled`, `p-invalid`, `p-focus`, `p-variant-filled`, `p-inputwrapper-filled`, `p-inputwrapper-focus`, `p-treeselect-open`, `p-treeselect-clearable`, `p-treeselect-fluid`, `p-treeselect-sm`, `p-treeselect-lg`), скрытое поле `input[role=combobox][readonly]` (`aria-haspopup="tree"`, `aria-expanded`, `aria-controls`, `tabindex`, `disabled`), контейнер метки с `div.p-treeselect-label` и кнопку `div.p-treeselect-dropdown[role=button]` со стрелкой. Метка SHALL показывать подписи выбранных узлов через запятую либо `placeholder` (`p-placeholder`), а при `display = Chip` — чипы `p-treeselect-chip-item` > `p-treeselect-chip`. Панель SHALL выводиться только в открытом состоянии.

#### Scenario: Закрыто
- **WHEN** выбран узел «Work»
- **THEN** метка содержит «Work», панели нет, `aria-expanded="false"`

#### Scenario: Пусто
- **WHEN** ничего не выбрано
- **THEN** метка показывает `placeholder`

### Requirement: Открытие и выбор
Клик по полю SHALL открывать или закрывать панель `div.p-treeselect-overlay` с `div.p-treeselect-tree-container` (`max-height` = `scroll_height`, по умолчанию `400px`) и деревом; панель SHALL находиться внутри корня и выравниваться по полю. При открытии SHALL раскрываться предки выбранных узлов. В режиме `Single` выбор узла SHALL закрывать панель и возвращать фокус на поле; в `Multiple` и `Checkbox` панель остаётся открытой. Значение SHALL быть двусторонним сигналом ключей. Клик вне компонента, `resize` и `Escape` SHALL закрывать панель.

#### Scenario: Единичный выбор
- **WHEN** панель открыта и пользователь кликает по узлу
- **THEN** ключ узла в значении, панель закрыта, `on_hide` вызван

#### Scenario: Предки выбранного
- **WHEN** значение содержит вложенный узел и панель открывается
- **THEN** его предки раскрыты

### Requirement: Очистка
При `show_clear` и непустом значении SHALL выводиться значок `p-treeselect-clear-icon`; клик по нему SHALL очищать значение, вызывать `on_clear` и не открывать панель.

#### Scenario: Очистка
- **WHEN** пользователь кликает по значку очистки
- **THEN** значение пусто, панель не открыта

### Requirement: Клавиатура
На скрытом поле ↓ SHALL открывать панель и переводить фокус на первый узел дерева, `Space`/`Enter` — открывать, `Escape` — закрывать панель и оставлять фокус на поле, `Tab` — закрывать панель. Отключённое поле SHALL не реагировать на клики и клавиши.

#### Scenario: Стрелка вниз
- **WHEN** фокус на поле и нажата ↓
- **THEN** панель открыта, фокус на первом узле дерева

### Requirement: Параметры дерева
Режим выбора, `meta_key_selection`, распространение выбора, фильтр (`filter`, `filter_by`, `filter_mode`, `filter_placeholder`), `empty_message`, `loading` SHALL передаваться в `Tree`; фильтр SHALL сбрасываться при закрытии панели.

#### Scenario: Фильтр
- **WHEN** задан `filter` и панель открыта
- **THEN** в панели есть поле фильтра дерева
