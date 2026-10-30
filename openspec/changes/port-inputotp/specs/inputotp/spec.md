## Purpose
InputOtp позволяет вводить одноразовый код по одному символу в отдельных ячейках с автопереходом, вставкой всего кода и маскированием.

## ADDED Requirements

### Requirement: Разметка InputOtp
Компонент `InputOtp` крейта `prime-inputotp` SHALL выводить `<div class="p-inputotp p-component">` с `length` (по умолчанию 4) полями `<input type="text">`, каждое с классами `p-inputtext p-component p-inputotp-input`. `style_class` SHALL добавляться к классам корня, `style` — задавать стиль корня, `pt.root` и `pt.input` — добавлять классы, стиль и атрибуты корню и каждой ячейке.

#### Scenario: Число ячеек
- **WHEN** `length = 6`
- **THEN** выводится шесть полей `input.p-inputotp-input`

#### Scenario: По умолчанию
- **WHEN** `length` не задан
- **THEN** выводятся четыре поля

#### Scenario: pt
- **WHEN** `pt.input` содержит класс `custom`
- **THEN** каждое поле имеет класс `custom`

### Requirement: Значение InputOtp
Значение SHALL быть строкой, символы которой по порядку заполняют ячейки; ячейки сверх длины значения пусты, лишние символы сверх `length` не отображаются. Ввод пользователя SHALL записываться в `value` как слитная строка токенов.

#### Scenario: Начальное значение
- **WHEN** `value = "12"` при `length = 4`
- **THEN** первые две ячейки содержат `1` и `2`, остальные пусты

#### Scenario: Запись
- **WHEN** пользователь вводит `1`, `2`, `3`, `4` в ячейки по порядку
- **THEN** `value` равно `1234`, и вызван `on_change` с этим значением

### Requirement: Атрибуты ячеек InputOtp
Ячейки SHALL получать `maxlength="1"` (первая — `maxlength` равный `length`, чтобы принимать вставку), `inputmode="numeric"` при `integer_only` и `text` иначе, `type="password"` при `mask`, а также `name`, `tabindex`, `required`, `readonly` и `disabled` из свойств. Классы ячейки SHALL следовать правилам `InputText`: `p-invalid`, `p-variant-filled`, `p-inputtext-sm|lg`.

#### Scenario: Маска
- **WHEN** `mask = true`
- **THEN** каждая ячейка имеет `type="password"`

#### Scenario: Только цифры
- **WHEN** `integer_only = true`
- **THEN** каждая ячейка имеет `inputmode="numeric"`

#### Scenario: Размер и недопустимое значение
- **WHEN** `size = Small`, `invalid = true`
- **THEN** ячейки имеют классы `p-inputtext-sm` и `p-invalid`

#### Scenario: Отключено
- **WHEN** `disabled = true`
- **THEN** каждая ячейка имеет атрибут `disabled`

### Requirement: Переходы между ячейками InputOtp
После ввода символа в ячейку фокус SHALL переходить на следующую ячейку, а при удалении назад — на предыдущую. `ArrowRight` и `ArrowLeft` SHALL переводить фокус на соседнюю ячейку, `ArrowUp` и `ArrowDown` — отменяться, `Backspace` в пустой ячейке — переводить на предыдущую. При получении фокуса содержимое ячейки SHALL выделяться.

#### Scenario: Автопереход
- **WHEN** пользователь вводит символ в первую ячейку
- **THEN** фокус переходит во вторую

#### Scenario: Backspace в пустой ячейке
- **WHEN** пользователь нажимает `Backspace` в пустой второй ячейке
- **THEN** фокус переходит в первую

#### Scenario: Стрелки
- **WHEN** пользователь нажимает `ArrowRight`, затем `ArrowLeft`
- **THEN** фокус смещается на следующую, затем на предыдущую ячейку

### Requirement: Ограничения набора InputOtp
При `integer_only` набор нецифровых символов SHALL отменяться. При заполненных `length` символах дальнейший набор без выделения SHALL отменяться (кроме `Delete`). Клавиши с `Alt`, `Ctrl` или `Meta` SHALL не обрабатываться.

#### Scenario: Только цифры
- **WHEN** `integer_only = true`, пользователь нажимает `a`
- **THEN** символ не вводится

#### Scenario: Переполнение
- **WHEN** все ячейки заполнены и пользователь нажимает цифру без выделения
- **THEN** символ не вводится

#### Scenario: Сочетание клавиш
- **WHEN** пользователь нажимает `Ctrl+A`
- **THEN** нажатие не отменяется компонентом

### Requirement: Вставка в InputOtp
Вставка из буфера обмена в любую ячейку SHALL раскладывать до `length` символов по ячейкам с начала и записывать значение; при `integer_only` вставка с нецифровым содержимым SHALL игнорироваться. При `readonly` и `disabled` вставка SHALL игнорироваться.

#### Scenario: Вставка кода
- **WHEN** пользователь вставляет `123456` при `length = 4`
- **THEN** ячейки содержат `1`, `2`, `3`, `4`, значение равно `1234`

#### Scenario: Нецифровая вставка
- **WHEN** `integer_only = true`, пользователь вставляет `12ab`
- **THEN** значение не меняется

#### Scenario: Только чтение
- **WHEN** `readonly = true`, пользователь вставляет код
- **THEN** значение не меняется

### Requirement: Шаблон ячейки InputOtp
Шаблон `input` SHALL заменять стандартное поле каждой ячейки, получая токен ячейки, её индекс и обработчики событий (`input`, `keydown`, `focus`, `blur`, `paste`), которые шаблон подключает к своему полю.

#### Scenario: Своё поле
- **WHEN** задан шаблон `input`, выводящий `input.custom-otp-input`
- **THEN** выводится `length` таких полей вместо стандартных

### Requirement: Стоимость отрисовки InputOtp
Число аллокаций на экземпляр при SSR SHALL укладываться в бюджет `inputotp-1000` в `benches/alloc-budgets.toml`, вклад в wasm — в бюджет `inputotp` в `benches/budgets.toml`.

#### Scenario: Бюджеты
- **WHEN** выполняются `cargo xtask alloc-budget` и `cargo xtask wasm-size`
- **THEN** значения InputOtp не превышают бюджеты
