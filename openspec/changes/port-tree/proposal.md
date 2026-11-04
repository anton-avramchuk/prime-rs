# Proposal

## Why

Tree — следующий компонент волны 5: иерархические данные с раскрытием, выбором (одиночным, множественным, флажками) и фильтрацией. Он нужен и сам по себе, и как основа для TreeSelect. Раскрытие и клавиатура похожи на `PanelMenu`, выбор — на `Listbox`/`MultiSelect`, а флажок и поле фильтра строятся на готовых крейтах.

## What Changes

- Новый крейт `prime-tree` (порт `p-tree` из PrimeNG 21.1.10).
- Модель: `TreeNode` (`key` — обязательный уникальный ключ, `label`, `icon`, `expanded_icon`, `collapsed_icon`, `children`, `leaf`, `expanded`, `selectable`, `loading`, `type_`, `style_class`, `style`); состояние (раскрытые ключи, выбранные ключи) хранится в сигналах, а не в узлах.
- `Tree`: корень `div.p-tree.p-component` (`p-tree-selectable`, `p-tree-loading`, `p-tree-flex-scrollable`; `id`, `style`, `style_class`) > маска загрузки `div.p-tree-mask.p-overlay-mask` со спиннером (`loading_mode = Mask`) > поле фильтра (`filter`) в `IconField` > `div.p-tree-root` (`max-height` = `scroll_height`) > `ul.p-tree-root-children[role=tree]` > `li.p-tree-node[role=treeitem]` (`aria-label`, `aria-checked`, `aria-selected`, `aria-expanded`, `aria-setsize`, `aria-posinset`, `aria-level`, `tabindex`, `data-id`) > `div.p-tree-node-content` (отступ `level * indentation rem`) > кнопка `button.p-tree-node-toggle-button` с шевроном, флажок `p-tree-node-checkbox`, значок `p-tree-node-icon`, подпись `p-tree-node-label` + `ul.p-tree-node-children[role=group]`; `div.p-tree-empty-message`.
- Выбор: `Single` (повторный клик снимает), `Multiple`, `Checkbox` (с распространением вверх и вниз, частичное состояние `partialSelected`), `meta_key_selection`, `highlight_on_select`, недоступные для выбора узлы (`selectable = false`). Выбор — `RwSignal<Vec<String>>` ключей (в `Single` — 0 или 1 ключ).
- Раскрытие: кнопка-шеврон и стрелки клавиатуры; события `on_node_expand`/`on_node_collapse` (для ленивой загрузки: узел с `leaf = false` и без детей, `loading` показывает спиннер-значок при `loading_mode = Icon`).
- Фильтр: `filter`, `filter_by` (поля `label`/`key` через запятую), `filter_mode` (`Lenient`/`Strict`), `filter_placeholder`; совпавшие родители раскрываются; `on_filter`.
- Клавиатура (roving tabindex): ↓ ↑ по видимым узлам, → раскрывает или идёт к первому потомку, ← сворачивает или идёт к родителю, `Enter`/`Space` — выбор, `Home`/`End`, `Tab`.
- Свойства: `value`, `selection_mode`, `selection`, `expanded_keys`, `meta_key_selection`, `propagate_selection_up`/`down`, `highlight_on_select`, `loading`, `loading_mode`, `empty_message`, `aria_label`, `aria_labelledby`, `scroll_height`, `indentation` (1.5), `id`, `style`, `style_class`, `filter*`; события `on_node_select`, `on_node_unselect`, `on_node_expand`, `on_node_collapse`, `on_node_double_click`, `on_filter`; представления `node` (содержимое подписи; по `type_`), `togglericon`.
- Страница `/tree` (`basic`, `single`, `multiple`, `checkbox`, `controlled`, `event`, `filter`, `lazy`, `template`), e2e, бенчмарки и бюджеты, `docs/tree.md`, `tree` в `theme.components` (блока `For PrimeNG` в исходнике стилей нет).
- **Не входит:** перетаскивание узлов (`draggableNodes`, `droppableNodes`, `validateDrop`), виртуальная прокрутка и `lazy` ленты, `contextMenu` (контекстное меню подключается своим `ContextMenu` через `on:contextmenu` пользователя), `trackBy`, `filterOptions`/шаблон фильтра, `Ripple`, `Bind`, `pTemplate`, шаблоны `header`/`footer`/`loader`/`checkboxicon`/`loadingicon`/`filtericon`.

## Capabilities

### New Capabilities
- `tree`: дерево с раскрытием, выбором, флажками, фильтром и клавиатурой.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-tree` (`publish = false`); зависимости: `prime-core`, `prime-icons` (chevronright, chevrondown, spinner, search, check, minus), `prime-iconfield`, `prime-inputtext`, при необходимости `prime-checkbox` (классы).
- Изменяются `Cargo.toml` workspace, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml`. Поведение существующих крейтов не меняется.
