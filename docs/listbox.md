# Listbox

Port of `p-listbox` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-listbox::Listbox` | `packages/primeng/src/listbox/listbox.ts`, `listbox/style/listboxstyle.ts` |
| CSS | `@primeuix/styles/listbox`, the Aura preset and the `/* For PrimeNG */` block (header, search field, container height) — `cargo xtask gen-theme`, `primeng_extra` |
| Demos | `apps/showcase/doc/listbox/*-doc.ts` (`basic`, `multiple`, `group`, `filter`, `checkbox`, `checkmark`, `disabled`, `invalid`, `template`) |

Spec: `openspec/specs/listbox`.

## What matches PrimeNG

- Structure: root `div.p-listbox`, header `p-listbox-header` ("select all" checkbox, search field in `p-iconfield` with a `p-inputicon` icon), container `p-listbox-list-container` with `max-height` (default `14rem`), list `ul.p-listbox-list[role=listbox]`, options `li.p-listbox-option[role=option]`, group headers `li.p-listbox-option-group`, empty list message `div.p-listbox-empty-message`.
- Root classes and their order (`striped`, `p-disabled`, `p-invalid`, `p-listbox-fluid`), root `data-p`, `p-listbox-option-selected` (only with `highlight_on_select`), `p-focus`, `data-p-selected`/`-focused`/`-disabled` on options.
- `aria-*`: `aria-multiselectable`, `aria-disabled`, `aria-activedescendant` (on the list while it is focused, on the search field always when there is a highlighted option), `aria-selected`, `aria-setsize`/`aria-posinset` ignoring group headers, option identifiers `{id}_{index}` over the list including headers.
- Selection: `onOptionSelectSingle`/`Multiple` with `meta_key_selection` (in single mode without `Ctrl` a repeated click keeps the selection; in multiple mode without `Ctrl` a click replaces the selection), meta mode is disabled after a touch (`touchend`), `Shift`+arrows and `Ctrl+Shift+Home/End` select a range, `Ctrl+A` selects all eligible visible options.
- List keys: ↓ ↑ Home End Enter NumpadEnter Space Shift, type-ahead search (500 ms accumulation), `Tab` is not intercepted. Search field keys: ↓ ↑ ← → Home End Enter Shift.
- Focus: `auto_option_focus`, `focus_on_hover` (only while the list is focused), `select_on_focus`; blur clears the focused option, the range anchor and the accumulated search string.
- The "select all" checkbox works on visible (filtered) eligible options; `Enter` on it toggles, `Tab` moves focus into the list.
- Option and "select all" checkboxes use the same markup and classes as `Checkbox` (`p-checkbox`, `p-checkbox-input`, `p-checkbox-box`, `p-checkbox-icon`) with the extra class `p-listbox-option-check-icon`; the checkbox itself holds no state, a click on it is cancelled and handled by the list.

## Differences

| Difference | Reason |
|---|---|
| The root is a `<div>`, not a `<p-listbox>` host | The host in Angular is a framework detail. |
| The value is `value: RwSignal<Option<String>>` (single mode) and `values: RwSignal<Vec<String>>` (`multiple`) instead of `ngModel`; without them internal state (`default_value`, `default_values`) | Two-way binding without forms infrastructure; two signals instead of one value of arbitrary type. |
| Options are `ListboxOption { label, value, disabled }` and `ListboxGroup { label, items }`, values are strings; no `optionLabel`, `optionValue`, `optionDisabled`, `optionGroupLabel`, `optionGroupChildren`, `dataKey` | Fields of an arbitrary object are replaced by typed options, as in Select. |
| Search is a case-insensitive label substring (leading and trailing whitespace dropped); no `filterBy`, `filterFields`, `filterMatchMode`, `filterLocale`, `searchLocale` | PrimeNG does the same by default. |
| No virtual scroll and lazy loading, `dragdrop` and `dropListData`, hidden focus boundaries `p-hidden-focusable`, `aria-live` messages (`searchMessage`, `selectionMessage`, `emptyMessage` for screen readers), `header`, `footer`, `filter`, `empty`, `loader` templates, external `selectAll` | Out of scope for the change. |
| `PageUp`/`PageDown` scroll the container by the height of the visible area | In PrimeNG they call `scrollInView(0)`, scrolling to the first option, which in practice behaves like `Home` without changing focus; ours is ordinary page-wise scrolling. |
| The focused option on `mousedown` and hover moves without scrolling and without `select_on_focus` | Selection happens at that moment anyway by the click; otherwise with `select_on_focus` the click would select the option twice. |
| The "select all" checkbox label for screen readers is `aria.selectAll`/`aria.unselectAll` of the translation by state | In PrimeNG `toggleAllAriaLabel` is taken the same way, but from `config.translation`. |
| The `fluid` property is `OptProp<bool>`: without a value it is taken from the `Fluid` context | Like `hasFluid = fluid() ?? !!pcFluid` in PrimeNG; see `docs/field-wrappers.md`. |
| The identifier `pn_id_<n>` comes from the shared helper `prime_rs_core::ids::next_unique`, or the `id` property | The same on the server and in the browser. |
| No `reactiveforms`, `templatedrivenforms`, `virtualscroll`, `dragdrop` demos; in `template` country codes instead of CDN flags | Scope of the change and independence from the CDN. |

The list (`ul` and options) is rebuilt only when options, groups or the search string change; selection, focus and highlight are separate reactive option attributes, so elements are not recreated on navigation and click.

## What was not verified

- Touch: meta mode after `touchend` is verified only by logic, not on real touch input.
- Screen readers: `aria-*` attributes are checked against PrimeNG, screen reader behavior was not verified.

## Performance

See `benches/` (`listbox-1`, `listbox-1000`, `case-listbox`) and `benches/*budgets.toml`.
