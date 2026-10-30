# Design

## Context

Требования — в `specs/inputotp`. Исходники PrimeNG 21.1.10: `inputotp/inputotp.ts`, `inputotp/style/inputotpstyle.ts`; демонстрации `apps/showcase/doc/inputotp/{basic,mask,integeronly,sizes,template}-doc.ts`. Образцы в проекте: `crates/prime-inputtext` (модель значения `RwSignal<String>`, `write_classes`/`InputTextState`), `crates/prime-rating` и `prime-slider` (фокус и клавиши, `Effect` только под `cfg(wasm32)`), `prime_core::Pt`.

Наблюдения по исходнику:
- Состояние — массив токенов; `updateModel` склеивает его в строку. Отдельная ячейка читает токен по индексу.
- `onInput(event, index)`: если в первую ячейку попала строка длиннее одного символа — это вставка/автозаполнение (`handleOnPaste`); иначе записать токен и по `inputType` перейти назад (`deleteContentBackward`) или вперёд (`insertText`, `deleteContentForward`).
- `onKeyDown`: комбинации с Alt/Ctrl/Meta пропускаются; `ArrowLeft/Right` — соседняя ячейка, `ArrowUp/Down` — отмена; `Backspace` в пустой ячейке — назад; остальное: при `integerOnly` не цифра — отмена, при заполненных `length` символах без выделения — отмена (кроме `Delete`).
- `handleOnPaste`: берёт `length + 1` символов (в PrimeNG это не влияет — токены потом обрезаются значением), при `integerOnly` требует числа. Мы обрезаем строго до `length`.
- Стиль только два класса: `p-inputotp p-component` и `p-inputotp-input`; блока `/* For PrimeNG */` нет.

## Decisions

### D1. Крейт и зависимости
`prime-inputotp` с `InputOtp`, `InputOtpPt { root, input }`, `InputOtpSlot { token, index, events }`, `InputOtpEvents`. Зависит от `prime-core`, `prime-inputtext` (для `write_classes`/`InputTextState`) и `web-sys`. Сами ячейки рисуются как `<input>` с классами InputText плюс `p-inputotp-input`, а не вложенным `InputText`: у ячейки общее значение, а нужные ей обработчики другие.

### D2. Чистая логика (`state.rs`)
- `tokens_of(value: &str, length) -> Vec<String>`: символы значения по порядку, обрезанные до `length`.
- `join(tokens) -> String`.
- `key_decision(key, mods, integer_only, has_selection, filled, length, cell_empty) -> KeyAction` (`Allow`, `Prevent`, `Prev`, `Next`) — по `onKeyDown`.
- `paste_value(text, length, integer_only) -> Option<String>`: обрезка до `length`; `None`, если `integer_only` и в тексте не только цифры.
- `input_move(input_type) -> Move { Prev, Next, None }`.
Все — с табличными тестами.

### D3. Значение
`value: Option<RwSignal<String>>`. Токены вычисляются из сигнала (при `Some`) или лежат в локальном `RwSignal<Vec<String>>` (без `value`, неконтролируемый режим). Запись в `value` — склеенная строка токенов; пустые ячейки посреди кода сохраняются как пустые токены при расчёте, но склейка их пропускает (как `tokens.join('')` в PrimeNG).

### D4. Ячейки и события
Ячейка — `<input>` с атрибутами по спецификации и `prop:value` из токена. Обработчики (`on:input`, `on:keydown`, `on:focus`, `on:blur`, `on:paste`) создаются один раз на ячейку; фокус на соседнюю ячейку — через `Element::next_element_sibling`/`previous_element_sibling` и `HtmlInputElement::focus`/`select`. Всё, что трогает DOM, — под `cfg(target_arch = "wasm32")` или в обработчиках (которые на SSR не вызываются). `autofocus` — атрибут первой ячейки. Для шаблона `input` обработчики оборачиваются в `InputOtpEvents` (`Callback<web_sys::Event>` и т. п.).

### D5. CSS
`inputotp` добавляется в `theme.components`; `cargo xtask gen-theme`, проверка, что CSS остальных не изменился.

### D6. Тесты
SSR на каждый сценарий разметки и значения; табличные тесты `state.rs`; браузерные тесты `tests/web.rs` (wasm-bindgen-test) на ввод, автопереход, `Backspace`, стрелки, вставку, `integer_only`; e2e с настоящими клавишами и `clipboard`-вставкой. Если браузерные тесты потребуют нового job, добавить крейт в три места `ci.yml`.

### D7. Демонстрации
`/inputotp`: `basic`, `mask`, `integeronly`, `sizes`, `template` (кастомная ячейка). Демонстрации форм (`reactiveforms`, `templatedrivenforms`) и `sample` не переносятся.

### D8. Бенчмарки
Сценарии `inputotp-1` и `-1000`, `case-inputotp`; сначала `cargo bench -p render-bench --bench alloc`, затем бюджеты.

## Risks / Trade-offs

- [Автозаполнение из SMS и экранная клавиатура] → ведут себя по-разному в браузерах; проверяется только вставка и клавиши, остальное честно записано в `docs/inputotp.md`.
- [Фокус через соседей по DOM] → как в PrimeNG; шаблон `input` обязан выводить `<input>` соседними элементами.
- [Неконтролируемый режим] → токены в локальном сигнале; без `value` результат читается только через `on_change`.

## Migration Plan

Миграции нет. Откат — `git revert`.

## Open Questions

Нет.
