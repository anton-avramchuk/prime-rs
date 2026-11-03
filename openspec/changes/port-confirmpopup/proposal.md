# Proposal

## Why

ConfirmPopup — шестой компонент волны 4: всплывающее подтверждение у элемента, вызвавшего действие («Удалить запись?»). Он строится на общем сервисе подтверждений `ConfirmationService`, которым затем воспользуется ConfirmDialog, поэтому сервис и описание подтверждения выносятся в отдельный крейт без интерфейса.

## What Changes

- Новый крейт `prime-confirmation` (порт `api/confirmation.ts` и `api/confirmationservice.ts` из PrimeNG 21.1.10), без компонентов и стилей:
  - `Confirmation` — описание подтверждения (строитель): `key`, `message`, `header`, `icon` (`ViewFn`), `target` (элемент-цель для ConfirmPopup), `accept`, `reject` (`Callback<()>`), подписи и значки кнопок, `accept_visible`, `reject_visible`, `accept_button_style_class`, `reject_button_style_class`, `accept_button_props`, `reject_button_props` (`ConfirmButtonProps`: `label`, `severity`, `outlined`, `text`, `size`), `default_focus` (`ConfirmFocus`), `close_on_escape`, `dismissable_mask`, `block_scroll`, `closable`, `position`, `modal` — все поля, что есть в `Confirmation` PrimeNG, чтобы ConfirmDialog не менял описание.
  - `ConfirmationService` (`Copy`-хэндл в контексте Leptos): `confirm`, `close`, `subscribe`/`unsubscribe`; `provide_confirmation_service` и `use_confirmation_service`.
- Новый крейт `prime-confirmpopup` (порт `p-confirmpopup`).
- `ConfirmPopup`: слушает сервис; подтверждение с ключом, совпадающим с `key` компонента, показывает панель `div.p-confirmpopup.p-component[role=alertdialog]` у `target` > `div.p-confirmpopup-content` (значок `p-confirmpopup-icon`, текст `p-confirmpopup-message`) и `div.p-confirmpopup-footer` с кнопками «Нет» (`p-confirmpopup-reject-button`) и «Да» (`p-confirmpopup-accept-button`), оба размера `small`; подписи по умолчанию — переводы `reject` и `accept`.
- Поведение: выравнивание по цели с переворотом над целью и сдвигом стрелки (`--p-confirmpopup-arrow-left`, `p-confirm-popup-flipped`, `data-p-confirmpopup-flipped`); слой группы оверлеев; фокус на кнопку по `default_focus` (по умолчанию «Да»); `accept` и `reject` вызывают колбэк, скрывают панель и возвращают фокус на цель; закрытие `Escape` (как «Нет»), кликом вне панели и цели (`dismissable_mask`), `resize` окна (не на сенсорных устройствах), прокруткой предка цели; повторное `confirm` перенацеливает панель.
- Свойства: `key`, `default_focus` (`accept`), `auto_z_index`, `base_z_index`, `visible` (управляемый показ), `style`, `style_class`, `pt`; представления `content`, `headless`, `accept_icon`, `reject_icon`; события `on_show`, `on_hide`.
- Страница showcase `/confirmpopup` с демонстрациями `basic`, `template` (содержимое), `headless`; e2e; бенчмарки и бюджеты; `docs/confirmpopup.md`, `docs/confirmation.md`.
- Стили: `confirmpopup` добавляется в `theme.components`.
- **Не входит:** `appendTo` (панель всегда в `body` через `Portal`), `showTransitionOptions`/`hideTransitionOptions`/`motionOptions` (анимация — классы темы), `Bind`, `pTemplate`, класс-значки `pi pi-…` (значки — `ViewFn`), `ConfirmationService.onAccept` (его слушает только ConfirmDialog PrimeNG; добавится при необходимости вместе с ним).

## Capabilities

### New Capabilities
- `confirmation`: описание подтверждения и общий сервис подтверждений.
- `confirmpopup`: всплывающее подтверждение у элемента-цели.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новые крейты `crates/prime-confirmation` (только нативные тесты) и `crates/prime-confirmpopup` (`publish = false`); зависимости: `prime-core`, `prime-dom`, `prime-button`, `prime-confirmation`, `web-sys`.
- Изменяются `Cargo.toml` workspace, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (браузерный job, три места: `prime-confirmpopup`).
