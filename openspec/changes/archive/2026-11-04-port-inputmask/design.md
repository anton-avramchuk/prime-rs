# Design

## Context

Требования — в `specs/inputmask`. Исходники PrimeNG 21.1.10: `inputmask/inputmask.ts` (`initMask`, `checkVal`, `shiftL`/`shiftR`, `onKeyPress`, `onInputKeydown`, `onInputFocus`/`Blur`), `style/inputmaskstyle.ts` (в `@primeuix/styles` модуля `inputmask` нет — только блок `For PrimeNG`). Образцы — `prime-inputnumber` (поле на классах `InputText`, ввод по `keydown`), `prime-password`.

## Goals / Non-Goals

**Goals:** то же поведение движка маски и тот же DOM.

**Non-Goals:** ветка Android Chrome (`handleAndroidInput`), iPhone-Backspace, `keypress`, формы Angular.

## Decisions

### D1. Чистый движок (`mask.rs`)
`Mask` хранит `tests: Vec<Option<Kind>>` (`Digit`, `Alpha`, `AlphaNum`), `len`, `partial_position`, `first_non_mask`, `last_required`, буфер `Vec<char>`, `default_buffer`, `slot`, `auto_clear`, `keep_buffer` и предикат буквы. Методы без DOM, каждый возвращает новый текст и каретку: `check_val(text, allow) -> (text, pos)`, `key_press(ch, caret) -> Option<Typed { text, caret, completed }>`, `delete(caret, forward) -> (text, caret)`, `on_focus(text) -> (text, caret_begin, caret_end)`, `on_blur(text) -> text`, `unmasked()`, `is_completed()`, `seek_next/prev`, `shift_l/shift_r` — табличные тесты на формулах PrimeNG (`(999) 999-9999`, `99-aaa?9`, `*`, вставка со сдвигом, удаление, необязательная часть, `keep_buffer`).

### D2. Оболочка
Поле неконтролируемое: текст пишет движок (`input.set_value`), `value: RwSignal<String>` — результат (`updateModel`). Каретка (`set_selection_range`) — только в браузере. Ввод — `keydown` для печатных символов (с `preventDefault`), `Backspace`/`Delete`/`Enter`/`Escape` как в исходнике, `input` и `paste` — через `check_val(true)` в `set_timeout(0)`. Внешнее изменение `value` (эффект под `cfg(wasm32)`) пересчитывает текст через `check_val`.

### D3. SSR
Начальный текст — `check_val(value)` без DOM; в `value`-атрибуте.

### D4. Тема
`xtask/theme/extract.mjs` пропускает отсутствующий модуль `@primeuix/styles/<имя>`, если для компонента есть блок `For PrimeNG`.

### D5. Тесты
Движок — таблицы. SSR: разметка, начальное значение, очистка, классы. Браузерные `tests/web.rs` одним последовательным тестом: ввод цифр и неподходящих символов, `Backspace`/`Delete` со сдвигом, вставка, `on_complete`, потеря фокуса (очистка/`keep_buffer`/`auto_clear = false`), `unmask`, `readonly`, внешнее значение, очистка.

## Risks / Trade-offs

- [Каретка в браузере] Положение каретки проверяется через `selectionStart`; фокус в headless Chrome нужен реальный.
- [Ввод через IME и Android] Покрывается общим разбором `input`, но не проверялся на устройствах.

## Open Questions

- Нужен ли `keypress` для совместимости со старыми клавиатурами — нет, `keydown` с `key` покрывает.

## Migration Plan

Миграции нет. Откат — `git revert`.
