# confirmation Specification

## Purpose
The description of a confirmation and the shared service through which the application asks the ConfirmPopup and ConfirmDialog components to show a question.

## ADDED Requirements

### Requirement: Confirmation description
`Confirmation` of the `prime-confirmation` crate SHALL be assembled with a builder (`Confirmation::new()` and methods per field) and carry the fields `key`, `message`, `header`, `icon`, `target`, `accept`, `reject`, `accept_label`, `reject_label`, `accept_icon`, `reject_icon`, `accept_visible`, `reject_visible` (visible by default), `accept_button_style_class`, `reject_button_style_class`, `accept_button_props`, `reject_button_props` (`ConfirmButtonProps`: `label`, `severity`, `outlined`, `text`, `size`), `default_focus` (`ConfirmFocus`: `Accept`, `Reject`, `Close`, `None`), `close_on_escape`, `dismissable_mask`, `block_scroll`, `closable`, `position`, `modal`; fields that are not set SHALL remain `None`.

#### Scenario: Builder
- **WHEN** `Confirmation::new().message("Sure?").key("a").accept(cb)` is assembled
- **THEN** the fields `message`, `key`, `accept` are set, the others are `None`, and `accept_visible()` and `reject_visible()` are true

### Requirement: Confirmation service
`ConfirmationService` SHALL be a `Copy` handle that lives in the Leptos context (`provide_confirmation_service`, `use_confirmation_service`). `confirm(confirmation)` SHALL report `ConfirmEvent::Confirm` to the subscribers, `close()` — `ConfirmEvent::Close`; `subscribe(callback)` SHALL return an identifier, and `unsubscribe(id)` — remove the subscription. The subscribers are called synchronously and in the order of subscription; an unsubscribed subscriber is not called. `use_confirmation_service` without `provide_confirmation_service` SHALL panic with a clear message.

#### Scenario: Broadcast
- **WHEN** two subscribers are subscribed and `confirm` is called
- **THEN** both receive `Confirm` with the same confirmation, in the order of subscription

#### Scenario: Unsubscribing
- **WHEN** a subscriber is unsubscribed and `close` is called
- **THEN** it is not called
