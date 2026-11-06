# DataView

Порт `p-dataview` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-dataview::DataView<T>`, `DataViewPt`, `DataViewLayout`, `PaginatorPosition`, `LazyLoadEvent`, `PageEvent` | `packages/primeng/src/dataview/dataview.ts`, `dataview/style/dataviewstyle.ts` |
| Фильтр, сортировка, страница, итог | `prime-dataview::data` — чистая логика `filter`, `sort`, `slice`, `updateTotalRecords`, табличные тесты |
| Пагинатор | `prime-paginator::Paginator` (в PrimeNG — `p-paginator`) |
| CSS | `@primeuix/styles/dataview` (`dataview` в `theme.components`) |
| Демонстрации | `apps/showcase/doc/dataview/*-doc.ts` (`basic`, `layout`, `pagination`, `sorting`, `loading`) |

Спека: `openspec/specs/dataview`. Компонент обобщён по типу элемента; `list` и `grid` — функции, которые получают элементы текущей страницы.

```rust
let list = Callback::new(|page: Vec<Product>| view! { /* строки */ }.into_any());
view! { <DataView value=products list=list paginator=true default_rows=10usize /> }
```

## Что совпадает с PrimeNG

- Разметка: `p-dataview p-component p-dataview-list|grid`, `p-dataview-loading` (маска и значок), `p-dataview-header`, пагинатор `p-dataview-paginator-top|bottom`, `p-dataview-content`, `p-dataview-empty-message`, `p-dataview-footer`.
- Режим `lazy` (значение уже страница, `total_records` для пагинатора, `on_lazy_load` при создании, смене страницы и сортировки), `paginator_position`, `always_show_paginator`, `rows_per_page_options` и остальные настройки пагинатора.
- Сортировка возвращает на первую страницу; `first` за пределами возвращается на последнюю страницу.

## Отличия

| Отличие | Причина |
|---|---|
| Сортировка — `compare: Callback<(T, T), Ordering>` и `sort_order` вместо `sortField`; фильтр — предикат `filter: Callback<T, bool>` вместо `filterBy`/`filterLocale` | Элементы типизированы, обращения к полю по имени (`resolveFieldData`) нет. |
| Данные не сортируются на месте: порядок вычисляется при показе | Сигналы приложения не мутируются. |
| `list` и `grid` — функции от страницы элементов, а не `ng-template` | В Leptos нет шаблонов Angular. |
| Нет `trackBy`, `gridStyleClass`, `loadingIcon` (иконка загрузки одна), `getBlockableElement`, событий `onChangeLayout`/`onSort` | Вне объёма порта: смену вида и порядка приложение знает само. |
| `on_page` может прийти повторно, если пагинатор нормализует вышедший за предел `first` | Пагинатор сообщает о нормализации. |

## Что не проверялось

- Чтение с экрана: `aria-*` сверены с PrimeNG, поведение скринридеров — нет.
- Работа в режиме `lazy` с реальным сервером (проверена событиями).

## Производительность

См. `benches/` (`dataview-1`, `dataview-1000`, `case-dataview`) и `benches/*budgets.toml`.
