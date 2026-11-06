# Design

## Context

Требования — в `specs/dataview`. Исходники PrimeNG 21.1.10: `dataview/dataview.ts` (`paginate`, `sort`, `filter`, `isEmpty`), `style/dataviewstyle.ts`, `@primeuix/styles/dataview`. Пагинатор — готовый `prime-paginator`.

## Decisions

### D1. Чистая логика (`data.rs`)
`prepare(items, filter, compare, order)` — фильтр и устойчивая сортировка; `page(...)` — срез страницы (в `lazy` — всё); `total(...)`; `clamp_first(...)`. Табличные тесты.

### D2. Обобщённый компонент
`DataView<T: Clone + Send + Sync + 'static>`; `list` и `grid` — `Callback<Vec<T>, AnyView>` (элементы страницы). Сортировка — `compare: Callback<(T, T), Ordering>` и `sort_order`; фильтр — `Callback<T, bool>` (читает свои сигналы, поэтому реактивен).

### D3. Состояние
`first` и `rows` — сигналы (внешние или внутренние). Под `cfg(wasm32)` эффекты: возврат `first` в допустимый диапазон, начальный `on_lazy_load`, сброс на первую страницу и `on_lazy_load` при смене порядка.

### D4. Тесты
Логика — таблицы. SSR: списки, сетка, страницы и позиции пагинатора, сортировка, фильтр, пусто, загрузка, `lazy`. Браузерные: переключение страниц, возврат `first`, сортировка и фильтр, `lazy`, пустое состояние. e2e: стили и клики.

## Risks / Trade-offs

- [Нормализация first] Пагинатор может повторно сообщить о странице при возврате `first`.

## Migration Plan

Миграции нет. Откат — `git revert`.
