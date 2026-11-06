# PickList

Порт `p-picklist` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-picklist::PickList`, `PickListPt` | `packages/primeng/src/picklist/picklist.ts`, `picklist/style/pickliststyle.ts` |
| Перенос между списками | `prime-picklist::transfer` — чистая логика вместо `moveRight/AllRight/Left/AllLeft`, табличные тесты |
| Перестановка | `prime_orderlist::reorder` (общая с OrderList) |
| Списки, выбор, поиск | `prime-listbox::Listbox` (в PrimeNG — `p-listbox`); для заголовков в `Listbox` добавлено свойство `header_content` |
| CSS | `@primeuix/styles/picklist` (`picklist` в `theme.components`) |
| Демонстрации | `apps/showcase/doc/picklist/*-doc.ts` (`basic`, `filter`) |

Спека: `openspec/specs/picklist`. Источник и цель — `RwSignal<Vec<ListboxOption>>`, выбор — `RwSignal<Vec<String>>`.

```rust
let (source, target) = (RwSignal::new(items), RwSignal::new(Vec::new()));
view! { <PickList source=source target=target source_header="Available" target_header="Selected" filter=true /> }
```

## Что совпадает с PrimeNG

- Разметка: пять блоков (`p-picklist-source-controls`, `-source-list-container`, `-transfer-controls`, `-target-list-container`, `-target-controls`), по четыре кнопки в каждой панели, `aria-label` из перевода.
- Перенос выбранных в порядке выбора без дубликатов, «всё» — только видимые при поиске, очистка выбора источника, `keep_selection`, двойной клик по элементу.
- Правила недоступности кнопок и `disabled`.

## Отличия

| Отличие | Причина |
|---|---|
| Выбранные при перестановке сохраняют взаимный порядок (см. OrderList) | Исправление ошибки PrimeNG. |
| Нет перетаскивания (`dragdrop`) между списками | Построено на Angular CDK; общего решения для drag-and-drop в проекте пока нет. |
| Нет вертикального вида (`responsive`/`breakpoint`), значки не меняются на «вниз/вверх» | Вне объёма порта. |
| Нет шаблонов (`item`, `filter`, `empty`…), `filterBy`/`filterMatchMode`/`filterLocale`, `buttonProps`, `trackBy`, `dataKey` | У `Listbox` поиск идёт по метке; остальное вне объёма порта. |
| `show_source_controls`/`show_target_controls` читаются при создании, а не реактивно | Представление кнопок нельзя собрать повторно из сигнала. |
| При `keep_selection` и «всё в цель/источник» выбираются перенесённые элементы, а не прежний выбор источника | В PrimeNG `moveAllRight` дописывает `selectedItemsSource` (ошибка), `moveLeft` — вложенный массив. |

## Что не проверялось

- Перенос клавиатурой внутри списков (стандартное поведение `Listbox`).
- Чтение с экрана: `aria-*` сверены с PrimeNG, поведение скринридеров — нет.

## Производительность

См. `benches/` (`picklist-1`, `picklist-1000`, `case-picklist`) и `benches/*budgets.toml`.
