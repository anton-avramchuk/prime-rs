# Paginator

Port of PrimeNG 21.1.10 `p-paginator` (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-paginator::Paginator` | `packages/primeng/src/paginator/paginator.ts`, `paginator/style/paginatorstyle.ts` |
| CSS | `@primeuix/styles/paginator` and the Aura preset — `cargo xtask gen-theme`; the lists and the jump field take the `select` and `inputnumber` styles |
| Demos | `apps/showcase/doc/paginator/*-doc.ts` (`basic`, `currentpagereport`, `template`, `images`) |

Spec: `openspec/specs/paginator`.

## What matches PrimeNG

- Structure and order: `p-paginator-content-start`, `p-paginator-current`, `first`, `prev`, `p-paginator-pages` with `p-paginator-page` links (`p-paginator-page-selected`, `aria-current="page"`), the jump list `p-paginator-jtp-dropdown`, `next`, `last`, the `p-paginator-jtp-input` field, the rows list `p-paginator-rpp-dropdown`, `p-paginator-content-end`.
- Arithmetic: page count `ceil(total / rows)`, link window per `calculatePageLinkBoundaries` (the current page in the middle if possible, the window shifts at the edges), report with placeholders `{currentPage}`, `{totalPages}`, `{first}`, `{last}`, `{rows}`, `{totalRecords}` (the first occurrence of each is replaced).
- Buttons: "first" without the `disabled` attribute (only the `p-disabled` class and a guard in the handler), "previous", "next", "last" — with `disabled`; `aria-label` texts come from the translation; a click on the current link also fires the `on_page_change` event with `{ page, first, rows, page_count }`.
- The jump list shows the report instead of a label (like the `selectedItem` template), the rows list — a number; "show all" is an option with the value `total_records`, always last.
- The jump field is an `InputNumber` with the value "current page"; entering a number jumps to the page right away if it exists.
- `always_show = false` hides the root (`display: none`) when there is at most one link.

## Differences

| Difference | Reason |
|---|---|
| The root is a `<div>`, not the `<p-paginator>` host | The host in Angular is a framework detail. |
| `first` and `rows` are two-way signals (`RwSignal<usize>`), without them internal state (`default_first`, `default_rows`) | In PrimeNG `first` is an input, and `rows` changes inside via the list's `ngModel`; the signals replace `[first]`, `[rows]` and `(onPageChange)`. |
| The step back when `total_records` shrinks is clamped to the last existing page | PrimeNG goes to `page − 1` and does nothing if that page does not exist either (when the record count shrinks by several pages at once the state stays out of range). |
| No `dropdownAppendTo`/`appendTo`: the list panels are inside the component | As with Select: there is no shared Overlay. |
| Digits are always Latin, no `locale` | The project has no `Intl.NumberFormat`; numbers in list labels and links are formatted without a locale. |
| No `jumpToPageItemTemplate` and `dropdownItemTemplate` templates | Out of the change's scope. |
| Buttons without `Ripple` | The ripple effect is not ported in the project. |
| The `images` demo shows coloured blocks instead of photos from a CDN | Independence from the CDN. |
| The `template` demo: the slider and the list pick the row count via events, not `ngModel` | No Angular forms. |

The internal list and field signals are synced with `first`/`rows` by one-way effects (browser only); in SSR the values are taken at creation. Only the page links, the report and the button attributes are re-rendered — the navigation buttons themselves are not recreated, so focus is preserved after a click.

## Not verified

- Right-to-left layout (`:dir(rtl)` in the theme styles) and touch input.
- Screen reading: `aria-*` were checked against PrimeNG, screen reader behaviour was not.

## Performance

See `benches/` (`paginator-1`, `paginator-1000`, `case-paginator`) and `benches/*budgets.toml`. The lists and the field are rendered only when enabled by properties, so without `rows_per_page_options` and jumps the component is noticeably lighter.
