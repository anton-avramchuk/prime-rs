# Proposal

## Why

`InputOtp` — следующий компонент волны 2: ввод одноразового кода по ячейке на символ, с автопереходом, вставкой всего кода и маской. Он опирается на готовый `InputText` (классы и вариант) и не требует оверлеев, поэтому подходит как первая интерактивная партия после компонентов отображения.

## What Changes

- Новый крейт `prime-inputotp` с компонентом `InputOtp`:
  - Корень `<div class="p-inputotp p-component">` (вместо хоста `<p-inputotp>`) и `length` ячеек `<input class="p-inputtext p-component p-inputotp-input …">` (классы `InputText`: `p-filled`, `p-inputtext-sm|lg`, `p-invalid`, `p-variant-filled`).
  - Значение — необязательный `RwSignal<String>`; символы ячеек — его символы по порядку.
  - Свойства: `value`, `length` (по умолчанию 4), `mask` (`type="password"`), `integer_only` (`inputmode="numeric"`, только цифры), `size`, `variant`, `invalid`, `disabled`, `readonly`, `autofocus` (первая ячейка), `tabindex`, `name`, `style_class`, `style`, `pt` (`root`, `input`); события `on_change` (новое значение), `on_focus`, `on_blur`; шаблон ячейки `input` — колбэк с токеном, индексом и обработчиками событий.
  - Поведение как в PrimeNG: ввод символа и переход в следующую ячейку, `Backspace` на пустой ячейке и `ArrowLeft` — в предыдущую, `ArrowRight` — в следующую, `ArrowUp`/`ArrowDown` отменяются, вставка раскладывается по ячейкам (обрезается до `length`), выделение содержимого при фокусе, ограничение набора цифрами при `integer_only`, поля не принимают ввод при `readonly` и `disabled`.
  - Чистая логика (токены, решения по клавишам и вставке) вынесена в `state.rs` и проверяется табличными тестами.
- Страница showcase `/inputotp` с демонстрациями `basic`, `mask`, `integeronly`, `sizes`, `template` из PrimeNG; e2e с настоящим вводом, вставкой и клавишами; бенчмарки и бюджеты; `docs/inputotp.md`.
- Стили: `inputotp` добавляется в `theme.components`; `inputtext` уже подключён.
- **Не входит:** интеграция с Angular Forms (`ngModel`, `ReactiveForms`); демонстрация `sample` (она опирается на кнопки и счётчик повторной отправки — составная страница); ввод с экранной клавиатуры и автозаполнение кода из SMS (`autocomplete="one-time-code"` проставляется, но поведение браузеров не проверялось).

## Capabilities

### New Capabilities
- `inputotp`: ввод одноразового кода по символам в отдельных ячейках.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-inputotp` (`publish = false`); зависимости: `prime-core`, `prime-inputtext` (классы ячейки), `web-sys` (`HtmlInputElement`, `KeyboardEvent`, `ClipboardEvent`, `DataTransfer`, `InputEvent`, `FocusEvent`, `Element`).
- Изменяются `Cargo.toml` workspace, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, а при браузерных тестах `tests/web.rs` — job в `ci.yml` (три места).
