# confirmdialog Specification

## Purpose
ConfirmDialog показывает модальное окно с вопросом и кнопками «Да» и «Нет» по подтверждению из общего сервиса.

## ADDED Requirements

### Requirement: Показ по подтверждению
`ConfirmDialog` крейта `prime-confirmdialog` SHALL подписываться на `ConfirmationService` и показывать окно, когда приходит `Confirm` с `key`, равным его `key` (оба не заданы — тоже равны); подтверждение с другим `key` SHALL игнорироваться. Подписка SHALL сниматься при размонтировании. Окно SHALL быть `Dialog` с ролью `alertdialog`, классом `p-confirmdialog` на корне окна и классом `mask_style_class` на маске. Пока подтверждения нет, в разметке окна нет; на сервере его нет никогда. Подтверждение, пришедшее при открытом окне, SHALL заменять показанное на месте.

#### Scenario: Свой ключ
- **WHEN** сервису передано подтверждение с `key = "a"`, а `ConfirmDialog` имеет `key = "a"`
- **THEN** окно показано с текстом подтверждения

#### Scenario: Чужой ключ
- **WHEN** подтверждение приходит с другим `key`
- **THEN** окно этого `ConfirmDialog` не показывается

### Requirement: Содержимое и кнопки
Тело окна SHALL содержать значок `p-confirmdialog-icon` (если задан `icon`) и текст `p-confirmdialog-message`. Заголовок SHALL быть `header`, подвал — кнопкой отказа `p-confirmdialog-reject-button` и кнопкой согласия `p-confirmdialog-accept-button` (в таком порядке). Значения SHALL браться из подтверждения, а если оно их не задаёт, — из одноимённых свойств компонента. Подписи кнопок SHALL выбираться в порядке: `*_label`, `*_button_props.label`, перевод `reject`/`accept`; `reject_visible = false` и `accept_visible = false` SHALL убирать кнопки; `*_button_props` и `*_button_style_class` SHALL накладываться на кнопки. Представления `header_content`, `footer`, `message_content`, `icon_content`, `headless` SHALL заменять заголовок, подвал, значок с текстом, значок и всё содержимое окна соответственно; `accept_icon_content` и `reject_icon_content` — значки кнопок.

#### Scenario: Состав
- **WHEN** показано подтверждение с `header` и `message`
- **THEN** есть заголовок, текст и кнопки «No» и «Yes»

#### Scenario: Подпись из свойств кнопки
- **WHEN** у подтверждения заданы `accept_button_props.label = "Delete"` и не задан `accept_label`
- **THEN** кнопка согласия подписана «Delete»

### Requirement: Ответ и закрытие
Кнопка согласия SHALL вызывать `accept`, кнопка отказа — `reject` подтверждения и скрывать окно. Закрытие крестиком, клавишей `Escape` (если `close_on_escape` не `false`) или кликом по маске (если `dismissable_mask` истинно) SHALL вызывать `reject` и скрывать окно. `Close` сервиса SHALL скрывать окно без вызова `accept` и `reject`. При каждом скрытии SHALL вызываться `on_hide` с типом `ConfirmEventType` (`Accept`, `Reject`, `Cancel` соответственно, а при `Close` — без типа). После окончания выхода подтверждение SHALL забываться; повторный ответ на уже скрытое окно SHALL не вызывать колбэков.

#### Scenario: Согласие
- **WHEN** нажата кнопка согласия
- **THEN** вызван `accept`, окно скрыто, `on_hide` получил `Accept`

#### Scenario: Крестик
- **WHEN** окно закрыто крестиком
- **THEN** вызван `reject`, `on_hide` получил `Cancel`

#### Scenario: Close сервиса
- **WHEN** вызван `ConfirmationService::close`
- **THEN** окно скрыто, `accept` и `reject` не вызваны

### Requirement: Параметры окна и фокус
Свойства `modal`, `position`, `closable`, `block_scroll`, `draggable`, `focus_trap`, `auto_z_index`, `base_z_index`, `breakpoints`, `style`, `style_class`, `close_aria_label` SHALL передаваться `Dialog`, а значения `modal`, `position`, `closable`, `block_scroll`, `close_on_escape`, `dismissable_mask` подтверждения SHALL иметь приоритет над свойствами компонента. Положение SHALL разбираться из строки (`center`, `top`, `bottom`, `left`, `right`, `topleft`, `topright`, `bottomleft`, `bottomright`); неизвестная строка SHALL давать `center`. Окно SHALL быть неизменяемого размера и не максимизируемым. При показе фокус SHALL переходить на кнопку согласно `default_focus` (`Accept` по умолчанию, `Reject`, `Close` — кнопка закрытия окна, `None` — фокус не меняется).

#### Scenario: Положение
- **WHEN** у подтверждения `position = "bottomright"`
- **THEN** окно показано в правом нижнем углу

#### Scenario: Фокус по умолчанию
- **WHEN** окно показано без `default_focus`
- **THEN** фокус на кнопке согласия
