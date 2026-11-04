# InputMask

Порт `p-inputmask` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-inputmask::InputMask`, `InputMaskPt`, движок `Mask` | `packages/primeng/src/inputmask/inputmask.ts`, `inputmask/style/inputmaskstyle.ts` |
| Движок маски (буфер, `checkVal`, `shiftL/R`, ввод, удаление, фокус, потеря фокуса, `unmasked`) | `prime-inputmask::mask` — чистый порт без DOM, табличные тесты |
| CSS | в `@primeuix/styles` модуля `inputmask` нет — только блок `/* For PrimeNG */` (`inputmask` в `primeng_extra`); `xtask/theme/extract.mjs` пропускает отсутствующий модуль |
| Демонстрации | `apps/showcase/doc/inputmask/*-doc.ts` (`basic`, `mask`, `optional`, `slotchar`, `unmask`, `clearicon`, `disabled`, `invalid`) |

Спека: `openspec/specs/inputmask`. Значение — `RwSignal<String>`: маскированный текст либо (с `unmask`) только введённые символы.

```rust
let value = RwSignal::new(String::new());
view! { <InputMask mask="(999) 999-9999" value=value slot_char="_" on_complete=Callback::new(|_| log("полный номер")) /> }
```

## Что совпадает с PrimeNG

- Маска: `9` — цифра, `a` — буква (`character_pattern`, по умолчанию ASCII), `*` — буква или цифра, `?` — начало необязательной части, остальное — постоянные символы; заполнитель `slot_char` берётся по индексу символа маски (как `getPlaceholder`).
- Ввод: символ проверяется по шаблону позиции, вставляется со сдвигом вправо (`shiftR`), каретка переходит на следующую позицию; выделение удаляется со сдвигом влево (`shiftL`); `Backspace`/`Delete`; `Enter` завершает ввод; `Escape` возвращает текст на момент фокуса; вставка и ввод через `input` раскладываются по маске (`checkVal(true)`).
- Фокус показывает буфер маски (через 10 мс), каретка — на первой свободной позиции, у полной маски выделяется всё; потеря фокуса очищает неполное значение при `auto_clear`; `keep_buffer` сохраняет введённое; `on_complete` при заполнении обязательной части; `unmask`; `show_clear`.
- Разметка: корень `p-inputmask p-component p-inputwrapper` (`p-variant-filled`), поле на классах `InputText`, значок `p-inputmask-clear-icon`.

## Отличия

| Отличие | Причина |
|---|---|
| Корень — `span`, а не пользовательский элемент `p-inputmask`; `position: relative` и (при `fluid`) `width: 100%` заданы встроенным стилем | CSS `For PrimeNG` целится в тег `p-inputmask`, которого в Leptos нет. |
| Обычные символы обрабатываются в `keydown` (по `KeyboardEvent::key`), а не в устаревшем `keypress` | `keypress` объявлен устаревшим; поведение то же. |
| Нет ветки Android Chrome (`handleAndroidInput`) и iPhone-Backspace; ввод вне клавиш (IME, вставка, перетаскивание) обрабатывается общим разбором `input` | Не проверялось на устройствах. |
| Нет форм Angular, `Bind`, `pTemplate` и шаблона значка очистки; `on_input` без события | Вне объёма порта. |
| При фокусе с неполным значением и `auto_clear` буфер очищается уже на фокусе, а каретка остаётся на прежней позиции | Так же в PrimeNG (`checkVal()` без `allow`). |

## Что не проверялось

- Ввод на мобильных устройствах и через IME.
- Чтение с экрана: `aria-*` сверены с PrimeNG, поведение скринридеров — нет.
- Раскладка справа налево.

## Производительность

См. `benches/` (`inputmask-1`, `inputmask-1000`, `case-inputmask`) и `benches/*budgets.toml`.
