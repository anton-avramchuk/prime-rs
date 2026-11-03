# confirmation Specification

## ADDED Requirements

### Requirement: Тип ответа
`prime-confirmation` SHALL предоставлять `ConfirmEventType` с вариантами `Accept` (согласие кнопкой), `Reject` (отказ кнопкой) и `Cancel` (закрытие окна без выбора: крестик, `Escape`, клик по маске), а `as_str` SHALL возвращать `"accept"`, `"reject"` и `"cancel"`.

#### Scenario: Значения
- **WHEN** запрошены строки типов `Accept`, `Reject` и `Cancel`
- **THEN** получены `"accept"`, `"reject"` и `"cancel"`
