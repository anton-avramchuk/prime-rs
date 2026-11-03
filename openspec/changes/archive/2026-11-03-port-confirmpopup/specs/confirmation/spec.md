# confirmation Specification

## Purpose
Описание подтверждения и общий сервис, через который приложение просит компоненты ConfirmPopup и ConfirmDialog показать вопрос.

## ADDED Requirements

### Requirement: Описание подтверждения
`Confirmation` крейта `prime-confirmation` SHALL собираться строителем (`Confirmation::new()` и методы по полям) и нести поля `key`, `message`, `header`, `icon`, `target`, `accept`, `reject`, `accept_label`, `reject_label`, `accept_icon`, `reject_icon`, `accept_visible`, `reject_visible` (по умолчанию видимы), `accept_button_style_class`, `reject_button_style_class`, `accept_button_props`, `reject_button_props` (`ConfirmButtonProps`: `label`, `severity`, `outlined`, `text`, `size`), `default_focus` (`ConfirmFocus`: `Accept`, `Reject`, `Close`, `None`), `close_on_escape`, `dismissable_mask`, `block_scroll`, `closable`, `position`, `modal`; не заданные поля SHALL оставаться `None`.

#### Scenario: Строитель
- **WHEN** собрано `Confirmation::new().message("Sure?").key("a").accept(cb)`
- **THEN** поля `message`, `key`, `accept` заданы, остальные `None`, а `accept_visible()` и `reject_visible()` истинны

### Requirement: Сервис подтверждений
`ConfirmationService` SHALL быть `Copy`-хэндлом, лежащим в контексте Leptos (`provide_confirmation_service`, `use_confirmation_service`). `confirm(confirmation)` SHALL сообщать подписчикам `ConfirmEvent::Confirm`, `close()` — `ConfirmEvent::Close`; `subscribe(callback)` SHALL возвращать идентификатор, а `unsubscribe(id)` — снимать подписку. Подписчики вызываются синхронно и в порядке подписки; отписанный подписчик не вызывается. `use_confirmation_service` без `provide_confirmation_service` SHALL паниковать с понятным сообщением.

#### Scenario: Рассылка
- **WHEN** два подписчика подписаны и вызван `confirm`
- **THEN** оба получают `Confirm` с тем же подтверждением, в порядке подписки

#### Scenario: Отписка
- **WHEN** подписчик отписан и вызван `close`
- **THEN** он не вызывается
