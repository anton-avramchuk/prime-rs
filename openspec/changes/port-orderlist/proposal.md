# Proposal

## Why

OrderList — список с изменяемым порядком: пользователь выбирает элементы и двигает их кнопками вверх, в начало, вниз, в конец. Четвёртый компонент волны 6. Сам список, выбор и поиск уже есть в `Listbox`; здесь — чистая логика перестановки и панель кнопок.

## What Changes

- Новый крейт `prime-orderlist` (порт `p-orderlist` из PrimeNG 21.1.10).
- Чистый модуль `reorder.rs`: `move_up`, `move_top`, `move_down`, `move_bottom` над списком ключей и набором выбранных; табличные тесты.
- `OrderList`: `div.p-orderlist.p-component` (`p-orderlist-controls-left|right`) > `div.p-orderlist-controls` (четыре кнопки `Button` с `severity=secondary`: вверх, в начало, вниз, в конец; `aria-label` из перевода; недоступны без выбора или при `disabled`) и `Listbox` (множественный выбор, `striped`, `meta_key_selection`, поиск `filter`, `scroll_height`, `aria_label`, `tabindex`).
- Значение — `RwSignal<Vec<ListboxOption>>` (порядок элементов), выбор — `RwSignal<Vec<String>>` (значения выбранных). События `on_reorder` (перемещённые элементы), `on_selection_change`.
- Страница `/orderlist` (`basic`, `filter`, `controls`), e2e, бенчмарки и бюджеты, `docs/orderlist.md`, `orderlist` в `theme.components` и `primeng_extra`.
- **Не входит:** перетаскивание (`dragdrop`, CDK), `header`, шаблоны Angular, адаптивный `<style>` `responsive`/`breakpoint`, `filterBy`/`filterMatchMode` (поиск `Listbox` по метке), `buttonProps`.
- **Отличие в поведении:** перемещение нескольких выбранных сохраняет их взаимный порядок (у PrimeNG при `moveUp` смежные элементы у края меняются местами, а `moveTop`/`moveBottom` зависят от порядка кликов).

## Capabilities

### New Capabilities
- `orderlist`: список с изменяемым порядком элементов.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-orderlist` (`publish = false`); зависимости: `prime-core`, `prime-listbox`, `prime-button`, `prime-icons`.
- Изменяются `Cargo.toml` workspace, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml`.
