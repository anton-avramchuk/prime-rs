# CascadeSelect

Port of `p-cascadeselect` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-cascadeselect::CascadeSelect`, `CascadeSelectPt`, `CascadeOption`, `CascadeOptionContext` | `packages/primeng/src/cascadeselect/cascadeselect.ts` (`CascadeSelect` and `CascadeSelectSub`), `cascadeselect/style/cascadeselectstyle.ts` |
| Model (path, focus, keys, search) | `prime-cascadeselect::state` — pure functions with table-driven tests |
| CSS | `@primeuix/styles/cascadeselect`, the Aura preset and the `/* For PrimeNG */` block — `cargo xtask gen-theme` (`cascadeselect` in `primeng_extra`) |
| Demos | `apps/showcase/doc/cascadeselect/*-doc.ts` (`basic`, `clearicon`, `disabled`, `invalid`, `loading`, `template`) |

Spec: `openspec/specs/cascadeselect`. Options are `CascadeOption` (a leaf with `value` or a group with `children`), the value is `RwSignal<Option<String>>` holding the value of the selected leaf (if `value` is not set, the label is used). Node keys are ancestor indices joined by `_` (`"0_1_2"`); they are also in the `id` of the `li` elements (`<field id>_<key>`).

```rust
let value = RwSignal::new(None::<String>);
view! { <CascadeSelect options=countries value=value placeholder="Select a City" show_clear=true /> }
```

## What matches PrimeNG

- Markup and classes: `div.p-cascadeselect.p-component.p-inputwrapper` (states: `-clearable|mobile|open|fluid|sm|lg`, `p-disabled`, `p-invalid`, `p-focus`, `p-variant-filled`, `p-inputwrapper-filled|focus`) > hidden input `input[role=combobox][readonly]` > `p-cascadeselect-label` > clear icon > `p-cascadeselect-dropdown` > panel `p-cascadeselect-overlay` > `p-cascadeselect-list-container` > `ul.p-cascadeselect-list[role=tree]` > `li.p-cascadeselect-option[role=treeitem]` (`-option-group|active|selected`, `p-focus`, `p-disabled`, `aria-*`) > `div.p-cascadeselect-option-content` > text and group icon; nested lists `ul.p-cascadeselect-option-list[role=group]`.
- Path of active groups (`onOptionChange`/`onOptionClick`): a click on a group expands it and closes siblings, a repeated click collapses it; hover follows the `onOptionMouseEnter` rules (before the first expansion — root groups, then any; with a selected value — right away); in a narrow window hover does not change the path; on open the path to the selected value is expanded.
- Keyboard per `onArrow*Key`: ↓ ↑ over the level list, focus on a group expands it, → enters an expanded group (expands a closed one), ← closes the level group and returns focus to the parent, `Alt+↑` closes, `Home` `End` `Enter` `Space` `Escape` `Tab` `Backspace`, search by label prefix with a 500 ms buffer.

## Differences

| Difference | Reason |
|---|---|
| Options are `CascadeOption`, the value is a string | PrimeNG uses arbitrary objects with `optionLabel`/`optionValue`/`optionGroupLabel`/`optionGroupChildren`/`dataKey`. |
| Moving focus by an arrow onto a leaf closes the open groups of its level; an arrow does not collapse an already expanded group | In PrimeNG expanded groups stay open, and the focus move calls `onOptionClick`, which would collapse an expanded group. |
| Hover over a root leaf before the first click only moves focus | In PrimeNG such a hover would select the leaf. |
| No Angular forms, `appendTo`, `onBeforeShow`/`onBeforeHide`, `select_on_focus`, mirroring of the nested list at the window edge (`position()`), `header`/`footer`/icon templates and screen reader messages | Out of the port scope; the position of nested lists is set by the theme CSS. |
| The label of an empty field without a placeholder is the word `p-emptylabel` under `p-cascadeselect-label-empty` (hidden by the theme) | Same in PrimeNG. |
| `value_view` (label replacement) and `option` (option content) instead of `pTemplate` | There is no `pTemplate`. |

## Not verified

- Right-to-left layout and touch input.
- Screen reading: `aria-*` were checked against PrimeNG, screen reader behavior was not.
- Nested lists at the right window edge (they go beyond the window, as in PrimeNG without `position()`).

## Performance

See `benches/` (`cascadeselect-1`, `cascadeselect-1000`, `case-cascadeselect`) and `benches/*budgets.toml`.
