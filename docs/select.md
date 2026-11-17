# Select

Port of PrimeNG 21.1.10 `p-select` (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-select::Select` | `packages/primeng/src/select/select.ts` (`Select` and `SelectItem`), `select/style/selectstyle.ts` |
| CSS | `@primeuix/styles/select` and `iconfield` (search field), the Aura preset and the `/* For PrimeNG */` block (placeholder color): `cargo xtask gen-theme`, `primeng_extra` |
| Demos | `apps/showcase/doc/select/*-doc.ts` (`basic`, `checkmark`, `clearicon`, `disabled`, `filled`, `filter`, `fluid`, `group`, `invalid`, `loadingstate`, `sizes`, `template`, `floatlabel`, `iftalabel`) |

Spec: `openspec/specs/select`. Scope of the first change: core, filter and groups.

## What matches PrimeNG

- Structure: root `div.p-select`, label `span.p-select-label[role=combobox]`, clear icon `p-select-clear-icon`, button `div.p-select-dropdown[role=button]` with an icon, panel `div.p-select-overlay` with hidden `p-hidden-focusable` boundaries, header `p-select-header` (search field `p-select-filter` in `p-iconfield` with a `p-inputicon` icon), list `ul.p-select-list[role=listbox]` with `li.p-select-option`, `li.p-select-option-group` (and `span.p-select-option-group-label`), `li.p-select-empty-message`.
- Root classes and their order, root and label `data-p`, `p-placeholder`, `p-select-label-empty`, `p-select-option-selected` (not with `checkmark`), `p-focus`, `data-p-focused`/`-highlight`/`-selected`/`-disabled` on options.
- `aria-*`: `aria-haspopup`, `aria-expanded`, `aria-controls`, `aria-activedescendant`, label `aria-label` (the given one or the option label), `aria-setsize`/`aria-posinset` ignoring group headers, option identifiers `{id}_{index}` over the list together with headers.
- Keyboard: `onKeyDown` and `onFilterKeyDown`: arrows, `Alt+ArrowUp`, `Home`/`End`, `PageUp`/`PageDown`, `Enter`/`Space`, `Escape`, `Tab`, `Delete`, type-ahead search accumulating characters for 500 ms; the first arrow down opens the panel (focus goes straight to the selected option, if any), the next ones go from it; without a selection and focus, arrow down goes to the first option, arrow up to the last.
- Focus on open is on the selected option (`show()`), `auto_option_focus` on the first; `focus_on_hover`, `select_on_focus`.
- Closing: repeated click, outside click, `Escape`, `Tab`, selection; `reset_filter_on_hide`; `on_show`/`on_hide`/`on_change`/`on_filter`/`on_clear`/`on_click`.
- Empty list and empty result messages come from `empty_message`/`empty_filter_message` and the translation (`emptyMessage`, `emptyFilterMessage`), the list label is `aria.listLabel`.

## Differences

| Difference | Reason |
|---|---|
| The root is a `<div>`, not a `<p-select>` host | The host in Angular is a framework detail. |
| `value: RwSignal<Option<String>>` instead of `ngModel`; without it internal state (`default_value`); panel openness is a two-way `open` | Two-way binding without forms infrastructure; `open` gives programmatic control and tests of the open panel. |
| Options are `SelectOption { label, value, disabled }` and `SelectGroup { label, items }`, values are strings; no `optionLabel`, `optionValue`, `optionDisabled`, `optionGroupLabel`, `optionGroupChildren`, `dataKey` | Arbitrary object fields are replaced by typed options, as in SelectButton. |
| The panel is always inside the component, no `appendTo` and `overlayOptions`; no show animation | The project has no shared Overlay component: the panel is positioned via `prime-dom` (`align_overlay`) and gets the overlay group z-index; an ancestor with `overflow` may clip it. |
| Search is a case-insensitive substring of the label (leading and trailing whitespace is dropped); no `filterBy`, `filterFields`, `filterMatchMode`, `filterLocale`, `filterTemplate` | Scope of the first change; PrimeNG does the same by default. |
| No virtual scrolling or lazy loading, `editable`, `header`, `footer`, `empty`, `loader` templates, `tooltip*`, `autofocus` | Outside the scope of the first change by the user's decision. |
| Closing on outside click and `resize` listens on `window` only while the panel is open | PrimeNG's `ConnectedOverlayScrollHandler` and `p-overlay` listeners are replaced by two subscriptions. |
| The last hidden boundary of the panel closes it and returns focus to the label | In PrimeNG it hands focus back through a complex `getFirstFocusableElement` chain; in practice the result is the same. |
| The label, placeholder and clear icon read the value directly from the option list; a value that is not among the options counts as unselected | In PrimeNG such a value stays in the model and is shown as an empty label. |
| The `fluid` property is `OptProp<bool>`: without a value it is taken from the `Fluid` context | Like `hasFluid = fluid() ?? !!pcFluid` in PrimeNG; see `docs/field-wrappers.md`. |
| The `pn_id_<n>` identifier comes from the shared helper `prime_rs_core::ids::next_unique` | Identical on the server and in the browser. |
| No `customfilter`, `editable`, `virtualscroll`, `lazyvirtualscroll`, `reactiveforms`, `templatedrivenforms` demos; in `template`, country codes instead of CDN flags | Change scope and independence from the CDN. |

The list is rebuilt only when options, groups or the search string change; option focus, selection and label state are separate closures, so the label and search field elements are not recreated during navigation.

## Performance

SSR: 230 allocations per instance with a closed panel and five options (budget 250); contribution to raw wasm +630 KB (budget 690 KB): keyboard, search, positioning and the list. This is the heaviest component in the project.
