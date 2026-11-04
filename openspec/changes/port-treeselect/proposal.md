# Proposal

## Why

TreeSelect — поле выбора узла (или узлов) дерева в выпадающей панели. Панель содержит готовый `Tree` (порт которого только что добавлен), а оболочка — как у `Select`/`MultiSelect`: метка с запятыми или чипами, значок очистки, кнопка раскрытия, панель внутри корня с выравниванием `align_overlay`.

## What Changes

- Новый крейт `prime-treeselect` (порт `p-treeselect` из PrimeNG 21.1.10) на `prime-tree`, `prime-chip`, `prime-dom`.
- `TreeSelect`: корень `div.p-treeselect.p-component.p-inputwrapper` (`p-treeselect-display-chip`, `p-disabled`, `p-invalid`, `p-focus`, `p-variant-filled`, `p-inputwrapper-filled|focus`, `p-treeselect-open`, `p-treeselect-clearable`, `p-treeselect-fluid`, `-sm`, `-lg`) > скрытое поле `div.p-hidden-accessible > input[role=combobox][readonly]` (`aria-haspopup="tree"`, `aria-expanded`, `aria-controls`) > `div.p-treeselect-label-container` > `div.p-treeselect-label` (запятая: подписи выбранных узлов через `, ` либо `placeholder`; чипы: `div.p-treeselect-chip-item` > `Chip.p-treeselect-chip`) > значок очистки `p-treeselect-clear-icon` > `div.p-treeselect-dropdown[role=button]` > панель `div.p-treeselect-overlay.p-component-overlay.p-component` > `div.p-treeselect-tree-container` (`max-height` = `scroll_height`, по умолчанию `400px`) > `Tree`.
- Значение — `RwSignal<Vec<String>>` ключей узлов (в `Single` — 0 или 1 ключ). Режимы `Single` (по умолчанию), `Multiple`, `Checkbox`; `meta_key_selection`, `propagate_selection_up/down`; фильтр, пустое сообщение, загрузка и раскрытие передаются в `Tree`.
- Поведение: клик по полю открывает/закрывает панель; `Single` закрывает панель после выбора и возвращает фокус; при открытии раскрываются предки выбранных узлов; клик вне, `resize` и `Escape` закрывают; очистка сбрасывает значение; положение панели пересчитывается после раскрытия узла.
- Клавиатура на скрытом поле: ↓ открывает и переводит фокус на первый узел дерева, `Space`/`Enter` открывают, `Escape` закрывает и возвращает фокус, `Tab` закрывает.
- Свойства: `options`, `value`, `selection_mode`, `display`, `placeholder`, `scroll_height`, `meta_key_selection`, `propagate_selection_up/down`, `show_clear`, `disabled`, `invalid`, `variant`, `size`, `fluid`, `filter`, `filter_by`, `filter_mode`, `filter_placeholder`, `empty_message`, `loading`, `input_id`, `tabindex`, `aria_label`, `aria_labelledby`, `id`, `style`, `style_class`, `panel_style_class`; события `on_node_select`, `on_node_unselect`, `on_node_expand`, `on_node_collapse`, `on_clear`, `on_show`, `on_hide`, `on_focus`, `on_blur`; представления `value` (метка), `node` (подпись узла).
- Страница `/treeselect` (`basic`, `multiple`, `checkbox`, `filter`, `clearicon`, `disabled`, `invalid`, `lazy`, `template`), e2e, бенчмарки и бюджеты, `docs/treeselect.md`, `treeselect` в `theme.components` и `primeng_extra`.
- **Не входит:** виртуальная прокрутка, `appendTo` (панель внутри корня), `Ripple`, `Bind`, `pTemplate`, шаблоны `header`/`footer`/`empty`/`triggericon`/`itemcheckboxicon`/`itemloadingicon`/`filtericon`/`closeicon`, формы Angular (`ngModel`, реактивные формы), скрытые фокус-ловушки панели, `resetFilterOnHide` (фильтр сбрасывается при каждом закрытии).

## Capabilities

### New Capabilities
- `treeselect`: выбор узлов дерева в выпадающей панели.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-treeselect` (`publish = false`); зависимости: `prime-core`, `prime-dom`, `prime-tree`, `prime-chip`, `prime-icons` (chevrondown, times).
- Изменяются `Cargo.toml` workspace, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml`. Поведение существующих крейтов не меняется.
