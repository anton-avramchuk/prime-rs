# OrderList

Порт `p-orderlist` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-orderlist::OrderList`, `OrderListPt`, `ControlsPosition` | `packages/primeng/src/orderlist/orderlist.ts`, `orderlist/style/orderliststyle.ts` |
| Перестановка выбранных элементов | `prime-orderlist::reorder` — чистая логика вместо `moveUp/moveTop/moveDown/moveBottom`, табличные тесты |
| Список, выбор, поиск | `prime-listbox::Listbox` (в PrimeNG — `p-listbox`) |
| CSS | `@primeuix/styles/orderlist` и блок `/* For PrimeNG */` (`orderlist` в `theme.components` и `primeng_extra`) |
| Демонстрации | `apps/showcase/doc/orderlist/*-doc.ts` (`basic`, `filter`; `controls` — наша) |

Спека: `openspec/specs/orderlist`. Порядок — `RwSignal<Vec<ListboxOption>>`, выбор — `RwSignal<Vec<String>>` (значения выбранных).

```rust
let value = RwSignal::new(vec![ListboxOption::new("Apple", "a"), ListboxOption::new("Pear", "p")]);
view! { <OrderList value=value filter=true on_reorder=Callback::new(|sel: Vec<String>| log(sel)) /> }
```

## Что совпадает с PrimeNG

- Разметка: `p-orderlist p-component` с `p-orderlist-controls-left|right`, панель `p-orderlist-controls` из четырёх кнопок (вверх, в начало, вниз, в конец), недоступных без выбора или при `disabled`, затем список.
- Список — множественный выбор, `striped_rows`, `meta_key_selection`, `scroll_height` (по умолчанию `14rem`), поиск.
- Перестановка по полному списку, даже если поиск скрывает часть элементов; выбор после перемещения сохраняется.

## Отличия

| Отличие | Причина |
|---|---|
| Выбранные элементы сохраняют взаимный порядок при любом перемещении | В PrimeNG при `moveUp` смежные элементы у края меняются местами, а `moveTop`/`moveBottom` зависят от порядка кликов — это ошибки; поведение исправлено сознательно. |
| Нет перетаскивания (`dragdrop`) | В PrimeNG оно построено на Angular CDK; общего решения для drag-and-drop в проекте пока нет. |
| Нет `header`, шаблонов, `filterBy`/`filterMatchMode`/`filterLocale`, `responsive`/`breakpoint`, `buttonProps` | У `Listbox` нет заголовочного слота; поиск идёт по метке; остальное вне объёма порта. |
| Значение — список `ListboxOption` (метка и строковое значение), а не произвольные объекты | Типизированная модель вместо `any`; свой вид элемента — `option`. |

## Что не проверялось

- Перемещение с клавиатуры внутри списка (стандартное поведение `Listbox`).
- Чтение с экрана: `aria-*` сверены с PrimeNG, поведение скринридеров — нет.

## Производительность

См. `benches/` (`orderlist-1`, `orderlist-1000`, `case-orderlist`) и `benches/*budgets.toml`.
