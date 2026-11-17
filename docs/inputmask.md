# InputMask

Port of `p-inputmask` from PrimeNG 21.1.10 (commit `fe82b291d6`, see `upstream.toml`).

| Our component | PrimeNG sources |
|---|---|
| `prime-inputmask::InputMask`, `InputMaskPt`, the `Mask` engine | `packages/primeng/src/inputmask/inputmask.ts`, `inputmask/style/inputmaskstyle.ts` |
| The mask engine (buffer, `checkVal`, `shiftL/R`, input, deletion, focus, blur, `unmasked`) | `prime-inputmask::mask` — a pure port without the DOM, table-driven tests |
| CSS | `@primeuix/styles` has no `inputmask` module — only a `/* For PrimeNG */` block (`inputmask` in `primeng_extra`); `xtask/theme/extract.mjs` skips the missing module |
| Demos | `apps/showcase/doc/inputmask/*-doc.ts` (`basic`, `mask`, `optional`, `slotchar`, `unmask`, `clearicon`, `disabled`, `invalid`) |

Spec: `openspec/specs/inputmask`. The value is a `RwSignal<String>`: the masked text or (with `unmask`) only the entered characters.

```rust
let value = RwSignal::new(String::new());
view! { <InputMask mask="(999) 999-9999" value=value slot_char="_" on_complete=Callback::new(|_| log("full number")) /> }
```

## What matches PrimeNG

- Mask: `9` — digit, `a` — letter (`character_pattern`, ASCII by default), `*` — letter or digit, `?` — start of an optional part, the rest are constant characters; the placeholder `slot_char` is taken by the index of the mask character (as `getPlaceholder`).
- Input: a character is checked against the position template, inserted with a shift to the right (`shiftR`), the caret moves to the next position; a selection is removed with a shift to the left (`shiftL`); `Backspace`/`Delete`; `Enter` finishes the input; `Escape` returns the text as it was at focus; paste and input via `input` are laid out by the mask (`checkVal(true)`).
- Focus shows the mask buffer (after 10 ms), the caret goes to the first free position, for a full mask everything is selected; blur clears an incomplete value with `auto_clear`; `keep_buffer` keeps what was entered; `on_complete` when the required part is filled; `unmask`; `show_clear`.
- Markup: root `p-inputmask p-component p-inputwrapper` (`p-variant-filled`), the field with `InputText` classes, the clear icon `p-inputmask-clear-icon`.

## Differences

| Difference | Reason |
|---|---|
| The root is a `span`, not a custom `p-inputmask` element; `position: relative` and (with `fluid`) `width: 100%` are set by an inline style | The `For PrimeNG` CSS targets the `p-inputmask` tag, which does not exist in Leptos. |
| Ordinary characters are handled in `keydown` (by `KeyboardEvent::key`), not in the obsolete `keypress` | `keypress` is deprecated; the behaviour is the same. |
| No Android Chrome branch (`handleAndroidInput`) and no iPhone Backspace handling; input not from keys (IME, paste, drag and drop) is handled by the common `input` parsing | Not verified on devices. |
| No Angular forms, `Bind`, `pTemplate` and no clear icon template; `on_input` has no event | Outside the scope of the port. |
| On focus with an incomplete value and `auto_clear` the buffer is cleared already on focus, and the caret stays at its previous position | The same in PrimeNG (`checkVal()` without `allow`). |

## Not verified

- Input on mobile devices and through IME.
- Screen readers: `aria-*` were checked against PrimeNG, screen reader behaviour was not.
- Right-to-left layout.

## Performance

See `benches/` (`inputmask-1`, `inputmask-1000`, `case-inputmask`) and `benches/*budgets.toml`.
