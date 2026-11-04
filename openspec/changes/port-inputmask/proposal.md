# Proposal

## Why

InputMask — поле ввода по маске (телефон, дата, серийный номер): пользователь печатает только допустимые символы, постоянные символы маски подставляются сами. Это последний компонент волны 5; движок маски — самостоятельная чистая логика (буфер, проверка символов, сдвиги при вводе и удалении), а оболочка — обычное поле `InputText` с необязательным значком очистки.

## What Changes

- Новый крейт `prime-inputmask` (порт `p-inputmask` из PrimeNG 21.1.10).
- Маска: `9` — цифра, `a` — буква (`character_pattern`, по умолчанию ASCII-буквы), `*` — буква или цифра, `?` — начало необязательной части; остальные символы — постоянные. Пример: `(999) 999-9999`, `99-aaa?9`.
- `InputMask`: `span.p-inputmask.p-component.p-inputwrapper` (`p-variant-filled`) > `input.p-inputtext.p-component` (`type`, `placeholder`, `readonly`, `disabled`, `aria-*`, `title`, `autocomplete`) и значок очистки `p-inputmask-clear-icon` (при `show_clear`, непустом значении и доступном поле). Значение — `RwSignal<String>`: маскированный текст либо только введённые символы при `unmask`.
- Поведение: при фокусе поле показывает буфер маски с символами-заполнителями `slot_char` (`_`), каретка встаёт на первую свободную позицию (если маска полностью заполнена — выделяется вся); печатные символы проверяются по маске, вставляются со сдвигом вправо и пропускают постоянные символы; `Backspace`/`Delete` удаляют символ или выделение со сдвигом влево; `Enter` завершает ввод; `Escape` возвращает текст на момент фокуса; вставка и ввод через `input` перераспределяют символы по маске; при потере фокуса неполное значение очищается (`auto_clear`) либо остаётся (`keep_buffer` — буфер сохраняется); при заполнении обязательной части вызывается `on_complete`.
- Свойства: `mask`, `value`, `slot_char`, `auto_clear`, `show_clear`, `unmask`, `character_pattern`, `keep_buffer`, `input_type`, `placeholder`, `readonly`, `disabled`, `invalid`, `variant`, `size`, `fluid`, `input_id`, `tabindex`, `title`, `aria_label`, `aria_labelledby`, `aria_required`, `autocomplete`, `id`, `style`, `style_class`; события `on_complete`, `on_focus`, `on_blur`, `on_input`, `on_clear`, `on_unmasked_change`.
- Страница `/inputmask` (`basic`, `mask`, `optional`, `slotchar`, `unmask`, `clearicon`, `disabled`, `invalid`), e2e, бенчмарки и бюджеты, `docs/inputmask.md`, `inputmask` в `theme.components` и `primeng_extra`; `xtask gen-theme` умеет компоненты без модуля в `@primeuix/styles` (только блок `For PrimeNG`).
- **Не входит:** ввод на Android Chrome через событие `input` (`handleAndroidInput`) — вместо него общий разбор `input`; `keypress` (используется `keydown`); формы Angular; `Bind`, `pTemplate`, шаблон значка очистки; `iPhone`-ветка Backspace.

## Capabilities

### New Capabilities
- `inputmask`: поле ввода по маске.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-inputmask` (`publish = false`); зависимости: `prime-core`, `prime-dom`, `prime-inputtext` (классы), `prime-icons` (times).
- Изменяются `Cargo.toml` workspace, `upstream.toml`, `xtask/theme/extract.mjs` (компонент без модуля стилей), `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml`.
