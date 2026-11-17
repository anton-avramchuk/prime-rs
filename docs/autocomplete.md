# AutoComplete

Port of PrimeNG 21.1.10 `p-autocomplete` (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-autocomplete::AutoComplete`, `AutoCompletePt`, `AutoCompleteOption`, `AutoCompleteCompleteEvent`, `AutoCompleteDropdownMode` | `packages/primeng/src/autocomplete/autocomplete.ts`, `autocomplete/style/autocompletestyle.ts` |
| Model (flat list, navigation, value label, match, separator) | `prime-autocomplete::state`: pure functions with table-driven tests |
| CSS | `@primeuix/styles/autocomplete`, the Aura preset and the `/* For PrimeNG */` block: `cargo xtask gen-theme` (`autocomplete` in `primeng_extra`) |
| Demos | `apps/showcase/doc/autocomplete/*-doc.ts` (`basic`, `dropdown`, `multiple`, `forceselection`, `group`, `clearicon`, `disabled`, `invalid`, `template`) |

Spec: `openspec/specs/autocomplete`. Options are `AutoCompleteOption` (`label`, `value` defaulting to the label, `disabled`, `items` for the nested options of a group). Single mode: `value: RwSignal<String>` (typed text or the value of the selected option); `multiple`: `values: RwSignal<Vec<String>>`.

```rust
let (value, suggestions) = (RwSignal::new(String::new()), RwSignal::new(Vec::new()));
view! {
    <AutoComplete
        suggestions=suggestions
        value=value
        on_complete=Callback::new(move |e: AutoCompleteCompleteEvent| suggestions.set(search(&e.query)))
    />
}
```

## What matches PrimeNG

- Markup and classes: `div.p-autocomplete.p-component.p-inputwrapper` > `input.p-inputtext.p-autocomplete-input[role=combobox]` or `ul.p-autocomplete-input-multiple` with chips `p-autocomplete-chip-item` > `Chip.p-autocomplete-chip` and `li.p-autocomplete-input-chip`; clear icon, `p-autocomplete-loader` spinner, `p-autocomplete-dropdown` button; panel `p-autocomplete-overlay` > `p-autocomplete-list-container` > `ul.p-autocomplete-list[role=listbox]` > `li.p-autocomplete-option` (`-selected`, `p-focus`, `p-disabled`, `aria-*`, `data-p-*`), `li.p-autocomplete-option-group`, `li.p-autocomplete-empty-message`.
- Query: typing from `min_length` characters calls `on_complete` after `delay`; the spinner is visible until `suggestions` are updated; new options open the panel (including the same set again); an empty result shows a message; the arrow button sends an empty query or the current text; `complete_on_focus`.
- Selection and keys per `onKeyDown`: down/up over eligible options, `Alt+Up` selects and closes, `Enter` selects, `Escape` closes with focus, `Tab` selects the focused one, `Backspace` in an empty `multiple` removes the last chip, left/right over chips, `force_selection` (clearing a non-matching input on `change` and close), `unique`, `typeahead = false` (adding the typed text on `Enter`, the separator, `Tab`, blur and paste).

## Differences

| Difference | Reason |
|---|---|
| Options are `AutoCompleteOption` (strings), no objects, `optionLabel`/`optionValue`/`dataKey` | No arbitrary objects; the value is a string. |
| No virtual scrolling and `lazy`, `appendTo`, Angular forms, `onKeyUp`/`onInputKeydown`, screen reader messages (`aria-live`), `selectOnFocus`, `autoHighlight` | Outside the port's scope. |
| The panel is rendered only when open and lives inside the root (`position: absolute` from the theme CSS, root `position: relative`) | In PrimeNG it is a separate `p-overlay`. |
| `Home`/`End` in the field do not touch the caret (the browser does that itself) | PrimeNG calls `setSelectionRange` manually, the result is the same. |
| Templates are the properties `item`, `group_view`, `selected_item`; no `header`/`footer`/`loader`/icons | There is no `pTemplate`. |
| The chip container handles only events on itself (field arrows do not reach it) | In PrimeNG `stopPropagation` is called in every handler for this. |

## What was not verified

- Right-to-left layout and touch input.
- Screen reading: `aria-*` were checked against PrimeNG, screen reader behaviour was not.
- A long option list (without virtual scrolling the panel scrolls normally).

## Performance

See `benches/` (`autocomplete-1`, `autocomplete-1000`, `case-autocomplete`) and `benches/*budgets.toml`.
