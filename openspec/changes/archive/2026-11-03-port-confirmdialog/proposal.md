# Proposal

## Why

ConfirmDialog — седьмой компонент волны 4: модальное окно подтверждения («Удалить запись?») поверх уже готовых `Dialog` и `ConfirmationService`. Это самое прямое применение сервиса, вынесенного в `prime-confirmation` при ConfirmPopup, и закрывает пару «всплывающее / модальное подтверждение».

## What Changes

- Новый крейт `prime-confirmdialog` (порт `p-confirmdialog` из PrimeNG 21.1.10) поверх `prime-dialog::Dialog`.
- `prime-confirmation`: добавляется `ConfirmEventType` (`Accept`, `Reject`, `Cancel`, как `ConfirmEventType` PrimeNG), другие типы не меняются.
- `ConfirmDialog` слушает `ConfirmationService`; подтверждение с совпадающим `key` показывает `Dialog` с ролью `alertdialog`, классом корня `p-confirmdialog` (добавляется к `p-dialog`) и классом маски `mask_style_class`. В теле — значок `p-confirmdialog-icon` и текст `p-confirmdialog-message`; в подвале — кнопки отказа `p-confirmdialog-reject-button` и согласия `p-confirmdialog-accept-button` (подписи по умолчанию — переводы `reject`/`accept`; `*_button_props` накладываются на кнопки).
- Поведение: `accept`/`reject` вызывают колбэк и скрывают окно; закрытие крестиком, `Escape` или кликом по маске (`dismissable_mask`) считается отказом (`reject` с типом `Cancel`); `Close` сервиса скрывает окно без колбэков; событие `on_hide` получает `ConfirmEventType`; фокус при показе — по `default_focus` (`accept` по умолчанию, `reject`, `close`, `none`).
- Свойства (значения по умолчанию, переопределяются `Confirmation`): `key`, `header`, `message`, `icon`, `accept_label`, `reject_label`, `accept_icon`, `reject_icon`, `accept_visible`, `reject_visible`, `accept_button_style_class`, `reject_button_style_class`, `close_on_escape`, `dismissable_mask`, `block_scroll`, `closable`, `modal`, `position` (`DialogPosition`), `default_focus`, `draggable`, `focus_trap`, `auto_z_index`, `base_z_index`, `breakpoints`, `close_aria_label`, `accept_aria_label`, `reject_aria_label`, `style`, `style_class`, `mask_style_class`, `pt`; представления `header_content`, `footer`, `message_content`, `icon_content`, `headless`, `accept_icon_content`, `reject_icon_content`; событие `on_hide`.
- Страница showcase `/confirmdialog` с демонстрациями `basic`, `position`, `template` (сообщение), `headless`; e2e; бенчмарки и бюджеты; `docs/confirmdialog.md`.
- Стили: `confirmdialog` добавляется в `theme.components` (стили `dialog` и `button` уже есть).
- **Не входит:** `appendTo` (окно всегда в `body`, как у `Dialog`), `transitionOptions` (анимация — классы темы), `rtl`, `Bind`, `pTemplate`, HTML в `message` (PrimeNG вставляет его через `innerHTML`; здесь только текст), `ConfirmationService.onAccept`, `closeButtonProps`.

## Capabilities

### New Capabilities
- `confirmdialog`: модальное окно подтверждения поверх Dialog и ConfirmationService.

### Modified Capabilities
- `confirmation`: добавляется `ConfirmEventType`.

## Impact

- Новый крейт `crates/prime-confirmdialog` (`publish = false`); зависимости: `prime-core`, `prime-dialog`, `prime-button`, `prime-confirmation`, `web-sys`.
- Изменяются `prime-confirmation` (тип события), `Cargo.toml` workspace, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (браузерный job, три места: `prime-confirmdialog`).
