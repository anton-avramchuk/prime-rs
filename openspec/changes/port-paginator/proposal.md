# Proposal

## Why

Paginator — двенадцатый компонент волны 3: постраничная навигация (кнопки первой/предыдущей/следующей/последней страницы, ссылки на страницы, переход к странице списком или полем, выбор числа строк на странице, отчёт «страница N из M»). Он собирается из уже готовых Select и InputNumber и нужен как основа для будущих таблиц и списков с пагинацией.

## What Changes

- Новый крейт `prime-paginator` (порт `p-paginator` из PrimeNG 21.1.10).
- `Paginator`: корень `div.p-paginator.p-component` > необязательное `div.p-paginator-content-start` > отчёт `span.p-paginator-current` > кнопки `p-paginator-first`, `p-paginator-prev` > `span.p-paginator-pages` со ссылками `button.p-paginator-page` (`p-paginator-page-selected`, `aria-current="page"`) > выпадающий список перехода к странице `p-paginator-jtp-dropdown` (`Select`) > кнопки `p-paginator-next`, `p-paginator-last` > поле перехода `p-paginator-jtp-input` (`InputNumber`) > выпадающий список числа строк `p-paginator-rpp-dropdown` (`Select`) > необязательное `div.p-paginator-content-end`.
- Управление извне: двусторонние сигналы `first: RwSignal<usize>` (индекс первой записи) и `rows: RwSignal<usize>` (число строк на странице), `total_records`; событие `on_page_change` получает `PaginatorState { page, first, rows, page_count }` — как `onPageChange` PrimeNG.
- Свойства: `total_records`, `rows_per_page_options` (`Vec<RowsPerPage>`: число или «показать все»), `page_link_size` (5), `always_show` (да), `show_current_page_report`, `current_page_report_template` (`{currentPage} of {totalPages}`, поддержаны `{currentPage}`, `{totalPages}`, `{first}`, `{last}`, `{rows}`, `{totalRecords}`), `show_first_last_icon` (да), `show_page_links` (да), `show_jump_to_page_dropdown`, `show_jump_to_page_input`, `dropdown_scroll_height` (`200px`), `style`, `style_class`, `pt`; значки `first_page_link_icon`, `previous_page_link_icon`, `next_page_link_icon`, `last_page_link_icon`, `dropdown_icon`; представления `template_left`, `template_right` (получают состояние).
- Поведение: переходы кнопками и ссылками (границы и пустой список — кнопки недоступны), окно ссылок вокруг текущей страницы (`calculatePageLinkBoundaries`), переход списком и полем, смена числа строк пересчитывает страницу, при уменьшении `total_records` текущая страница отступает назад; при `always_show = false` и одной странице компонент скрыт.
- Страница showcase `/paginator` с демонстрациями `basic`, `currentpagereport`, `template`, `images`; e2e; бенчмарки и бюджеты; `docs/paginator.md`.
- Стили: `paginator` добавляется в `theme.components`.
- **Не входит:** `dropdownAppendTo`/`appendTo` (панели списков всегда внутри корня, как в Select), `locale` для цифр (цифры всегда латинские), шаблоны `jumpToPageItem` и `dropdownItem`, `Ripple`, `Bind`, `pTemplate`, `@primeuix/motion`.

## Capabilities

### New Capabilities
- `paginator`: постраничная навигация со ссылками на страницы, кнопками, переходом к странице, выбором числа строк и отчётом о текущей странице.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-paginator` (`publish = false`); зависимости: `prime-core`, `prime-icons`, `prime-select`, `prime-inputnumber`, `web-sys`.
- Изменяются `Cargo.toml` workspace, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (браузерный job, три места).
