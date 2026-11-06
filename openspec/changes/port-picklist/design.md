# Design

## Context

Требования — в `specs/picklist`. Исходники PrimeNG 21.1.10: `picklist/picklist.ts` (`moveRight`, `moveAllRight`, `moveLeft`, `moveAllLeft`, `moveUp/Top/Down/Bottom`, `onDrop`), `style/pickliststyle.ts`, `@primeuix/styles/picklist`. Образец: `prime-orderlist` (кнопки + `Listbox`, `reorder`).

## Goals / Non-Goals

**Goals:** тот же DOM и поведение кнопок и переноса.

**Non-Goals:** перетаскивание, вертикальный адаптивный вид, шаблоны Angular.

## Decisions

### D1. Перенос (`transfer.rs`)
Функции над `Vec<ListboxOption>`: `move_selected(from, to, selection) -> Vec<ListboxOption>` и `move_all(from, to, visible) -> Vec<ListboxOption>`; дубликат определяется по значению. Видимость при поиске — `prime_listbox::matches_filter(label, текст)`, текст поиска приходит из `on_filter` каждого списка.

### D2. Перестановка
`prime_orderlist::reorder` над ключами выбранных, затем сортировка списка по полученному порядку.

### D3. Компонент
Два `Listbox` (`multiple`, `values` = выбор списка, `header_content` для `source_header`/`target_header`, `on_dbl_click` переносит выбранные). Кнопки — `Button` `severity=secondary`. `Listbox::header_content` добавлен как единственное изменение `prime-listbox`.

### D4. Тесты
Перенос — таблицы. SSR: пять блоков, состояние кнопок, заголовки, `show_*_controls`. Браузерные `tests/web.rs`: выбор и перенос в обе стороны, «всё», перестановка, двойной клик, `keep_selection`, поиск и «всё», `disabled`. e2e: стили и клики.

## Risks / Trade-offs

- [Порядок выбора] перенос следует порядку выбора, как в PrimeNG.

## Open Questions

- Нет.

## Migration Plan

Миграции нет. Откат — `git revert`.
