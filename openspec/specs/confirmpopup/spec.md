# confirmpopup Specification

## Purpose
ConfirmPopup показывает вопрос с кнопками «Да» и «Нет» рядом с элементом, вызвавшим действие.

## Requirements

### Requirement: Показ по подтверждению
`ConfirmPopup` крейта `prime-confirmpopup` SHALL подписываться на `ConfirmationService` и показывать панель, когда приходит `Confirm` с `key`, равным его `key` (оба не заданы — тоже равны); подтверждение с другим `key` SHALL игнорироваться. `Close` SHALL скрывать панель. Пока панель не показана, в разметке её нет; на сервере панели нет никогда. Подписка SHALL сниматься при размонтировании. Если свойство `visible` задано, оно SHALL определять показ вместо внутреннего состояния.

#### Scenario: Свой ключ
- **WHEN** сервису передано подтверждение с `key = "a"`, а `ConfirmPopup` имеет `key = "a"`
- **THEN** панель показана у `target` подтверждения

#### Scenario: Чужой ключ
- **WHEN** подтверждение приходит с `key = "b"`
- **THEN** панель этого `ConfirmPopup` не показывается

### Requirement: Разметка панели
Панель SHALL выводиться как `div.p-confirmpopup.p-component[role=alertdialog]` (`style`, `style_class`, `pt`) с содержимым `div.p-confirmpopup-content`: значок `p-confirmpopup-icon` (если задан `icon`) и `span.p-confirmpopup-message` с текстом `message`; затем `div.p-confirmpopup-footer` с кнопками отказа `p-confirmpopup-reject-button` и согласия `p-confirmpopup-accept-button` размера `small`. Подписи кнопок SHALL быть `reject_label`/`accept_label`, а без них — переводами `reject` и `accept`; `reject_visible = false` и `accept_visible = false` SHALL убирать кнопки. Свойства `reject_button_props` и `accept_button_props` SHALL накладываться на кнопки (`label`, `severity`, `outlined`, `text`, `size`); `reject_button_style_class` и `accept_button_style_class` — добавлять класс. Представление `content` SHALL заменять содержимое блока `p-confirmpopup-content`, `headless` — всё содержимое панели, `accept_icon` и `reject_icon` — значки кнопок.

#### Scenario: Состав
- **WHEN** показано подтверждение с `message`
- **THEN** есть блок содержимого с текстом и подвал с двумя кнопками «No» и «Yes»

#### Scenario: Скрытая кнопка
- **WHEN** `reject_visible = false`
- **THEN** кнопки отказа нет

### Requirement: Положение и стрелка
Панель SHALL выравниваться по `target` подтверждения со слоем группы оверлеев (`auto_z_index`, `base_z_index`). Сдвиг стрелки SHALL записываться в `--p-confirmpopup-arrow-left` (разница левых краёв цели и панели, если панель левее цели, иначе `0`). Если панель оказалась выше цели, SHALL выставляться `data-p-confirmpopup-flipped="true"` и класс `p-confirm-popup-flipped`. Повторное подтверждение с другой целью при показанной панели SHALL переносить панель к новой цели.

#### Scenario: Под целью
- **WHEN** у цели достаточно места снизу
- **THEN** панель расположена под целью без класса `p-confirm-popup-flipped`

#### Scenario: Перенацеливание
- **WHEN** при показанной панели приходит подтверждение с другой целью
- **THEN** панель выравнивается по новой цели

### Requirement: Согласие и отказ
Кнопка согласия SHALL вызывать `accept`, кнопка отказа — `reject` подтверждения, затем скрывать панель и возвращать фокус на `target`. При показе фокус SHALL переходить на кнопку согласно `default_focus` подтверждения, а без него — свойства `default_focus` компонента (`Accept` по умолчанию; `Reject` — кнопка отказа; `None`/`Close` — фокус не меняется).

#### Scenario: Согласие
- **WHEN** нажата кнопка согласия
- **THEN** вызван `accept`, панель скрыта, фокус на цели

#### Scenario: Фокус по умолчанию
- **WHEN** панель показана без `default_focus`
- **THEN** фокус на кнопке согласия

### Requirement: Закрытие
Клавиша `Escape` SHALL вызывать `reject` и скрывать панель, если `close_on_escape` не `false`. Клик вне панели и вне цели SHALL скрывать панель (без `accept`/`reject`), если `dismissable_mask` не `false`. Изменение размера окна (кроме сенсорных устройств) и прокрутка предка цели SHALL скрывать панель. Слушатели SHALL вешаться на время показа (клик — в следующем тике, чтобы не поймать клик, вызвавший подтверждение) и сниматься при скрытии и размонтировании. При показе SHALL вызываться `on_show`, после скрытия — `on_hide`.

#### Scenario: Escape
- **WHEN** панель показана и нажат `Escape`
- **THEN** вызван `reject`, панель скрыта

#### Scenario: Клик снаружи
- **WHEN** пользователь кликает вне панели и цели
- **THEN** панель скрыта, `accept` и `reject` не вызваны

#### Scenario: Отключённое закрытие
- **WHEN** у подтверждения `dismissable_mask = false`
- **THEN** клик снаружи панель не скрывает
