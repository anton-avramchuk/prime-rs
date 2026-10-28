## ADDED Requirements

### Requirement: Разметка и классы Message
Компонент `Message` крейта `prime-message` SHALL отрисовывать корень `<div role="alert" aria-live="polite">` с классами `p-message p-component p-message-<severity>` (по умолчанию `info`), `p-message-<variant>` (при `variant`), `p-message-sm` и `p-message-lg` (по `size`); внутри SHALL идти `div.p-message-content-wrapper` > `div.p-message-content`. Корень и все части SHALL иметь `data-p` (через пробел `outlined`, `simple`, severity, size).

#### Scenario: Серьёзность по умолчанию
- **WHEN** `severity` не задан
- **THEN** корень имеет класс `p-message-info`, а `data-p` равен `info`

#### Scenario: Вариант и размер
- **WHEN** заданы `severity = success`, `variant = outlined`, `size = small`
- **THEN** есть классы `p-message-success`, `p-message-outlined`, `p-message-sm`, а `data-p` равен `outlined success small`

### Requirement: Содержимое Message
При непустом `text` Message SHALL выводить `span.p-message-text` с текстом, затем всегда второй `span.p-message-text` с дочерним содержимым. `icon` (CSS-класс) SHALL давать `<i class="p-message-icon …">`, `icon_template` — свой элемент без обёртки; `container_template` SHALL заменять текст и содержимое и получать колбэк закрытия.

#### Scenario: Текст и дети
- **WHEN** заданы `text = "Info"` и дочерний элемент
- **THEN** внутри `p-message-content` два `p-message-text`: первый с `Info`, второй с дочерним элементом

#### Scenario: Контейнер
- **WHEN** задан `container_template`
- **THEN** он выводится вместо текста и содержимого и закрывает сообщение через переданный колбэк

### Requirement: Закрытие и время жизни Message
При `closable` Message SHALL выводить `button.p-message-close-button` (`type="button"`, `aria-label` из локали) с иконкой `p-message-close-icon` (или `close_icon_template`/`close_icon`); клик SHALL скрывать сообщение (класс `p-message-leave-active`, `display: none`) и вызывать `on_close` с исходным событием. При `life` (мс) сообщение SHALL скрываться по таймеру в браузере.

#### Scenario: Закрытие
- **WHEN** пользователь нажимает кнопку закрытия
- **THEN** сообщение скрывается и вызван `on_close`

#### Scenario: Время жизни
- **WHEN** задан `life = 3000`
- **THEN** через 3000 мс сообщение скрыто

### Requirement: Pass-through Message
`pt` SHALL принимать `root`, `content_wrapper`, `content`, `icon`, `text`, `close_button`, `close_icon`, добавляя классы и атрибуты без замены собственных.

#### Scenario: Класс через pt
- **WHEN** в `pt.content` передан класс `custom`
- **THEN** у `p-message-content` есть и `custom`, и собственный класс

### Requirement: Стоимость отрисовки Message
Message со статическими свойствами, без `closable` и `life` SHALL использовать статическую ветвь; обе ветви SHALL давать одинаковую разметку. Число аллокаций на экземпляр при SSR SHALL укладываться в бюджет `message` в `benches/alloc-budgets.toml`, вклад в wasm — в бюджет `message` в `benches/budgets.toml`.

#### Scenario: Паритет ветвей
- **WHEN** один набор значений передан статически и сигналами
- **THEN** HTML совпадает

#### Scenario: Бюджеты
- **WHEN** выполняются `cargo xtask alloc-budget` и `cargo xtask wasm-size`
- **THEN** значения Message не превышают бюджеты
