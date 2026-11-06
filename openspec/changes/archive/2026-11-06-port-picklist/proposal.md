# Proposal

## Why

PickList — два списка (источник и цель) и кнопки переноса элементов между ними, плюс изменение порядка в каждом списке. Пятый компонент волны 6. Списки, выбор и поиск уже есть в `Listbox`, перестановка — в `prime-orderlist::reorder`; здесь — перенос и панели кнопок.

## What Changes

- Новый крейт `prime-picklist` (порт `p-picklist` из PrimeNG 21.1.10).
- Чистый модуль `transfer.rs`: `move_selected` (выбранные в порядке выбора, без дубликатов по значению) и `move_all` (только видимые при поиске); табличные тесты.
- `PickList`: `div.p-picklist.p-component` > `.p-picklist-controls.p-picklist-source-controls` (вверх, в начало, вниз, в конец), `.p-picklist-list-container.p-picklist-source-list-container` (`Listbox`), `.p-picklist-controls.p-picklist-transfer-controls` (в цель, всё в цель, в источник, всё в источник), `.p-picklist-list-container.p-picklist-target-list-container`, `.p-picklist-controls.p-picklist-target-controls`.
- Данные — `source` и `target`: `RwSignal<Vec<ListboxOption>>`; выбор в каждом списке — внутренние сигналы; события `on_move_to_target`, `on_move_all_to_target`, `on_move_to_source`, `on_move_all_to_source`, `on_source_reorder`, `on_target_reorder`, `on_source_select`, `on_target_select`.
- Свойства: `source_header`, `target_header`, `filter` (с `show_source_filter`/`show_target_filter`), `striped_rows`, `meta_key_selection`, `disabled`, `scroll_height`, `show_source_controls`, `show_target_controls`, `keep_selection`, плейсхолдеры поиска, `source_style`/`target_style`, `style`, `style_class`, `option`.
- Двойной клик по элементу переносит выбранные (`onSourceItemDblClick`/`onTargetItemDblClick`).
- Страница `/picklist` (`basic`, `filter`), e2e, бенчмарки и бюджеты, `docs/picklist.md`, `picklist` в `theme.components`.
- В `prime-listbox` добавлено свойство `header_content` (слот заголовка).
- **Не входит:** перетаскивание, шаблоны Angular, `responsive`/`breakpoint` (вертикальный вид), `filterBy`/`filterMatchMode`/`filterLocale` (поиск `Listbox` по метке), `buttonProps`.

## Capabilities

### New Capabilities
- `picklist`: перенос элементов между двумя списками с изменением порядка.

### Modified Capabilities
<!-- Нет: свойство `header_content` у Listbox не меняет существующего поведения. -->

## Impact

- Новый крейт `crates/prime-picklist` (`publish = false`); зависимости: `prime-core`, `prime-listbox`, `prime-orderlist` (перестановка), `prime-button`, `prime-icons`.
- Изменяются `Cargo.toml` workspace, `upstream.toml`, `prime-listbox`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml`.
