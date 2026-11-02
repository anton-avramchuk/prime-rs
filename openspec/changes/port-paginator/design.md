# Design

## Context

Требования — в `specs/paginator`. Исходники PrimeNG 21.1.10: `paginator/paginator.ts`, `paginator/style/paginatorstyle.ts`; демонстрации `apps/showcase/doc/paginator/*-doc.ts`. Образцы в проекте: `prime-select` (список с `value: RwSignal<Option<String>>`, `selected_item_template`, `style_class`), `prime-inputnumber` (`value: RwSignal<Option<f64>>`), `prime-rating`/`prime-slider` (двусторонние числовые сигналы и пара «значение + событие»).

Наблюдения по исходнику:
- Состояние компонента — `first` (вход) и `rows` (вход, меняется внутри через `ngModel` списка числа строк); всё остальное (`page`, `pageCount`, окно ссылок, отчёт) выводится из них и `totalRecords`.
- `changePage(p)`: если `0 <= p < pageCount` — `first = rows × p`, пересчёт, `onPageChange` с `{ page, first, rows, pageCount }`. Клик по текущей ссылке тоже вызывает событие.
- Окно: `visible = min(pageLinkSize, pageCount)`; `start = max(0, ceil(page − visible/2))`; `end = min(pageCount − 1, start + visible − 1)`; `delta = pageLinkSize − (end − start + 1)`; `start = max(0, start − delta)`. При `pageLinkSize > pageCount` `delta` положителен и `start` остаётся 0.
- Кнопка «первая» не имеет атрибута `disabled` (только класс `p-disabled` и защиту в обработчике); остальные — `[disabled]`.
- `updateFirst()` при смене `totalRecords`: если `page > 0` и `first >= totalRecords` — асинхронно на предыдущую страницу.
- Список перехода показывает отчёт вместо метки (шаблон `selectedItem`); значения вариантов — индексы страниц, подписи «1»…«N».
- Список числа строк: подписи — локализованные числа, значение «Показать все» — `totalRecords`.
- Отчёт: `replace` строкой — меняется только первое вхождение каждого заполнителя.
- Корень скрывается через `display: none`, если `!alwaysShow` и ссылок не больше одной.

## Goals / Non-Goals

**Goals:** тот же DOM, классы и `aria-*`; та же арифметика страниц и окна ссылок; `first`/`rows` как двусторонние сигналы; событие `on_page_change`.

**Non-Goals:** `appendTo`, цифры по `locale`, шаблоны элементов списков, `Ripple`, `Bind`, `pTemplate`.

## Decisions

### D1. Крейт и состояние
`prime-paginator` с `Paginator`, `PaginatorState`, `RowsPerPage`, `PaginatorPt`. `first: Option<RwSignal<usize>>` и `rows: Option<RwSignal<usize>>` — двусторонние; без них внутреннее состояние (`default_first`, `default_rows`, по умолчанию 0 и 10). `total_records: Prop<usize>`.

### D2. Чистая логика (`state.rs`)
- `page_count(total, rows)` (0 при `rows == 0`), `page(first, rows)`, `is_first/is_last/is_empty`.
- `link_bounds(page, page_count, link_size) -> (start, end)` — формула PrimeNG; `links(...) -> Vec<usize>` (номера с единицы).
- `report(template, page, page_count, first, rows, total)` — замена первого вхождения каждого заполнителя.
- `rows_items(options, total) -> Vec<(label, value)>` и `page_items(count)`.
- `clamp_first_after_total(first, rows, total) -> Option<usize>` — страница для отступа назад (`updateFirst`).
- `next_first_for_rows(first, new_rows, total)` — страница, в которой остаётся прежний `first` после смены числа строк: `page = first / new_rows`, ограниченная последней существующей.
- Классы корня/кнопок/ссылок.
Табличные тесты на каждую.

### D3. Разметка
Один компонент без разбиения на подкомпоненты; части — функции-представления. Реактивные атрибуты кнопок (`disabled`, `p-disabled`), список ссылок и отчёт читают только `first`, `rows`, `total_records`, поэтому клик по ссылке не пересоздаёт кнопки навигации (фокус не теряется): ссылки перестраиваются целиком, кнопки — только атрибуты.

### D4. Списки
Оба `Select` создаются с внутренними сигналами `Option<String>`. Список числа строк: `value` = `rows` как строка; `on_change` записывает `rows` и вызывает `change_page` с пересчитанной страницей (D2). Список страниц: `value` = номер текущей страницы, подпись — отчёт через `selected_item_template`; `on_change` вызывает `change_page`. Обратная синхронизация (внешнее изменение `first`/`rows` → значение списка) — `Effect` под `cfg(target_arch = "wasm32")`, как в Select/SelectButton (без executor в нативных тестах паникует); в SSR начальное значение берётся из сигналов.

### D5. Поле перехода
`InputNumber` со значением `current_page` как `f64` и `on_change`; пустое значение игнорируется, число приводится к целому и ограничивается проверкой `0 <= p < page_count` внутри `change_page`.

### D6. Внешние изменения
`Effect` под `cfg(wasm32)` следит за `total_records`, `first` и `rows` и выполняет `updateFirst`: пока `page > 0` и `first >= total`, вызывает `change_page(page − 1)` (синхронно в эффекте; событие по одному на шаг, как в PrimeNG).

### D7. Скрытие
Корневой `style` содержит `display: none`, если `!always_show` и ссылок не больше одной (число ссылок = `min(link_size, page_count)`).

### D8. CSS
`paginator` добавляется в `theme.components`; `select`, `inputnumber`, `inputtext` уже подключены. Проверить `p-paginator-*` в пресете. Блок `/* For PrimeNG */` не используется (если он есть — проверить).

### D9. Тесты
SSR: разметка по свойствам, окно ссылок, отчёт, классы и `disabled`, значки, шаблоны, скрытие, пусто, `pt`. Браузерные `tests/web.rs`: клики по кнопкам и ссылкам, события, переход списком и полем, смена числа строк, внешние сигналы и отступ назад. e2e: настоящая мышь и клавиши (Tab/Enter по кнопкам). Крейт добавляется в три места `ci.yml`.

### D10. Бенчмарки
Сценарии `paginator-1`/`-1000` (`total_records = 120`, `rows = 10`, `rows_per_page_options = [10, 20, 30]`, значения по умолчанию) и `case-paginator`; сначала `cargo bench -p render-bench --bench alloc`; для wasm — отдельный файл бюджетов `--budgets`.

## Risks / Trade-offs

- [Размер и аллокации] Два `Select` и `InputNumber` внутри — самый тяжёлый «составной» компонент; бюджеты подбираются по замеру, списки создаются только когда включены свойствами.
- [Синхронизация сигналов] Двусторонняя связь внутренних сигналов списков с `first`/`rows` через `Effect` → односторонние эффекты и проверка «значение уже совпадает», чтобы не зациклиться.
- [Цифры] Цифры всегда латинские (нет `Intl.NumberFormat`); фиксируется в docs.

## Open Questions

- Нужны ли `jump_to_page_item_template` и `dropdown_item_template`: нет, по умолчанию достаточно; вернёмся, если понадобятся таблицам.

## Migration Plan

Миграции нет. Откат — `git revert`.
