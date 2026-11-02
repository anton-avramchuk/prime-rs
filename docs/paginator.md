# Paginator

Порт `p-paginator` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-paginator::Paginator` | `packages/primeng/src/paginator/paginator.ts`, `paginator/style/paginatorstyle.ts` |
| CSS | `@primeuix/styles/paginator` и пресет Aura — `cargo xtask gen-theme`; списки и поле перехода берут стили `select` и `inputnumber` |
| Демонстрации | `apps/showcase/doc/paginator/*-doc.ts` (`basic`, `currentpagereport`, `template`, `images`) |

Спека: `openspec/specs/paginator`.

## Что совпадает с PrimeNG

- Структура и порядок: `p-paginator-content-start`, `p-paginator-current`, `first`, `prev`, `p-paginator-pages` со ссылками `p-paginator-page` (`p-paginator-page-selected`, `aria-current="page"`), список перехода `p-paginator-jtp-dropdown`, `next`, `last`, поле `p-paginator-jtp-input`, список строк `p-paginator-rpp-dropdown`, `p-paginator-content-end`.
- Арифметика: число страниц `ceil(total / rows)`, окно ссылок по `calculatePageLinkBoundaries` (текущая по возможности посередине, у краёв окно сдвигается), отчёт с заполнителями `{currentPage}`, `{totalPages}`, `{first}`, `{last}`, `{rows}`, `{totalRecords}` (заменяется первое вхождение каждого).
- Кнопки: «первая» без атрибута `disabled` (только класс `p-disabled` и защита в обработчике), «предыдущая», «следующая», «последняя» — с `disabled`; подписи `aria-label` из перевода; клик по текущей ссылке тоже вызывает событие `on_page_change` с `{ page, first, rows, page_count }`.
- Список перехода показывает отчёт вместо метки (как шаблон `selectedItem`), список строк — число; «показать все» — вариант со значением `total_records`, всегда последний.
- Поле перехода — `InputNumber` со значением «текущая страница»; ввод числа сразу переходит на страницу, если она существует.
- `always_show = false` скрывает корень (`display: none`), когда ссылок не больше одной.

## Отличия

| Отличие | Причина |
|---|---|
| Корень — `<div>`, а не хост `<p-paginator>` | Хост в Angular — деталь фреймворка. |
| `first` и `rows` — двусторонние сигналы (`RwSignal<usize>`), без них внутреннее состояние (`default_first`, `default_rows`) | В PrimeNG `first` — вход, а `rows` меняется внутри через `ngModel` списка; сигналы заменяют `[first]`, `[rows]` и `(onPageChange)`. |
| Отступ назад при уменьшении `total_records` ограничен последней существующей страницей | PrimeNG переходит на `страница − 1` и ничего не делает, если такой страницы тоже нет (при уменьшении числа записей сразу на несколько страниц состояние остаётся вне пределов). |
| Нет `dropdownAppendTo`/`appendTo`: панели списков внутри компонента | Как у Select: нет общего Overlay. |
| Цифры всегда латинские, нет `locale` | В проекте нет `Intl.NumberFormat`; числа в подписях списков и ссылках форматируются без локали. |
| Нет шаблонов `jumpToPageItemTemplate` и `dropdownItemTemplate` | Вне объёма change. |
| Кнопки без `Ripple` | Эффект ряби в проекте не переносится. |
| Демонстрация `images` показывает цветные блоки вместо фотографий с CDN | Независимость от CDN. |
| Демонстрация `template`: ползунок и список выбирают число строк через события, а не `ngModel` | Нет форм Angular. |

Внутренние сигналы списков и поля синхронизируются с `first`/`rows` односторонними эффектами (только в браузере); в SSR значения берутся при создании. Перерисовываются только ссылки на страницы, отчёт и атрибуты кнопок — сами кнопки навигации не пересоздаются, поэтому фокус после клика сохраняется.

## Что не проверялось

- Правая-налево раскладка (`:dir(rtl)` в стилях темы) и сенсорный ввод.
- Чтение с экрана: `aria-*` сверены с PrimeNG, поведение скринридеров — нет.

## Производительность

См. `benches/` (`paginator-1`, `paginator-1000`, `case-paginator`) и `benches/*budgets.toml`. Списки и поле выводятся только когда включены свойствами, поэтому без `rows_per_page_options` и переходов компонент заметно легче.
