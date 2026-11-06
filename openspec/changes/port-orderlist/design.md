# Design

## Context

Требования — в `specs/orderlist`. Исходники PrimeNG 21.1.10: `orderlist/orderlist.ts`, `style/orderliststyle.ts` (+ блок `For PrimeNG`), `@primeuix/styles/orderlist`. OrderList в PrimeNG — оболочка над `p-listbox`; у нас то же поверх `prime-listbox`.

## Goals / Non-Goals

**Goals:** тот же DOM и поведение кнопок, переставляются элементы значения.

**Non-Goals:** перетаскивание, `header`, шаблоны, `responsive`.

## Decisions

### D1. Перестановка (`reorder.rs`)
Функции над `Vec<String>` ключей и `&[String]` выбранных, возвращают новый порядок. `move_up`/`move_down` — один проход с проверкой «сосед не выбран» (сохраняет порядок блока); `move_top`/`move_bottom` — стабильное разбиение на выбранные и остальные.

### D2. Компонент
`value: RwSignal<Vec<ListboxOption>>`, `selection: RwSignal<Vec<String>>` (внутренний при отсутствии). Список — `Listbox` с `multiple`, `options` из `value`, `values` = `selection`; `Listbox` меняет `selection` сам. Кнопки — `Button` с иконками `AngleUp`, `AngleDoubleUp`, `AngleDown`, `AngleDoubleDown`.

### D3. Тема
`orderlist` в `theme.components` и `primeng_extra` (блок `.p-orderlist-controls-right .p-orderlist-controls {order: 2}`).

### D4. Тесты
Логика — таблицы. SSR: разметка, позиции кнопок, состояние кнопок. Браузерные `tests/web.rs`: выбор, четыре кнопки, `on_reorder`, `disabled`, фильтр. e2e: стили и реальные клики.

## Risks / Trade-offs

- [Поведение] Намеренно исправлено перемещение нескольких элементов; записано в docs.

## Open Questions

- Нет.

## Migration Plan

Миграции нет. Откат — `git revert`.
