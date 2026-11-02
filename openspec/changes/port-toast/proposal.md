# Proposal

## Why

Toast — восьмой компонент волны 3: всплывающие уведомления в углу экрана. В PrimeNG Toast не живёт сам по себе, а получает сообщения от глобального `MessageService` (`messageService.add({...})` из любого места приложения); поэтому, кроме компонента, нужен и сервис. Это первый компонент проекта с общей шиной событий между независимыми частями приложения (в будущем она понадобится `ConfirmDialog` и `ConfirmPopup`).

## What Changes

- Новый крейт `prime-toast` (порт `p-toast` и `MessageService` из PrimeNG 21.1.10).
- `MessageService` — `Copy`-хэндл, который кладётся в контекст (`provide_message_service()`) и берётся оттуда (`use_message_service()`): `add(ToastMessage)`, `add_all(Vec<ToastMessage>)`, `clear()` (все тосты), `clear_key(key)` (тосты с ключом). Подписчики (компоненты `Toast`) получают события синхронно; подписка снимается при размонтировании.
- `ToastMessage` (строитель): `severity` (`ToastSeverity`: `Success`, `Info`, `Warn`, `Error`, `Secondary`, `Contrast`, `Custom(String)`; по умолчанию `Info`), `summary`, `detail`, `key`, `id`, `life`, `sticky`, `closable` (да), `icon`, `close_icon`, `style_class`, `content_style_class`.
- `Toast` — корень `div.p-toast.p-component.p-toast-<position>` (положение: `TopRight` по умолчанию, `TopLeft`, `TopCenter`, `BottomLeft`, `BottomCenter`, `BottomRight`, `Center`; позиционирование `position: fixed` инлайн-стилями, как в PrimeNG) со списком сообщений `div.p-toast-message.p-toast-message-<severity>[role=alert]` > `div.p-toast-message-content` (значок по серьёзности или `icon`, `div.p-toast-message-text` с `div.p-toast-summary` и `div.p-toast-detail`, кнопка `button.p-toast-close-button`).
- Свойства `Toast`: `key`, `position`, `life` (3000 мс), `auto_z_index` (да), `base_z_index`, `prevent_open_duplicates`, `prevent_duplicates`, `breakpoints` (стили по ширине экрана), `style_class`, `pt`; `message` (представление сообщения вместо содержимого), `headless` (вместо всего сообщения, получает функцию закрытия); `on_close`.
- Поведение: сообщение попадает в `Toast` с тем же `key` (оба без ключа — тоже совпадение); выводится с анимацией входа; закрывается само через `life` (сообщения и `Toast`), кроме `sticky`; наведение мыши останавливает таймер, уход — запускает заново; кнопка закрытия и `clear()` запускают анимацию выхода, после которой сообщение убирается и вызывается `on_close`; `z-index` группы модальных окон назначается, пока есть сообщения.
- Страница showcase `/toast` с демонстрациями `basic`, `severity`, `multiple`, `position`, `sticky`, `life`, `clear`, `target`, `responsive`, `template`, `headless` (упрощённая); e2e; бенчмарки и бюджеты; `docs/toast.md`.
- Стили: `toast` добавляется в `theme.components`.
- **Не входит:** `@primeuix/motion`, `motionOptions` и `show/hideTransformOptions`/`TransitionOptions` (анимации — CSS-классы), `Ripple` в демонстрации `multiple`, `ToastMessageOptions.data` и `text`, `pTemplate`, `pt` с контекстом, `Bind`.

## Capabilities

### New Capabilities
- `toast`: уведомления в углу экрана, получаемые от общего сервиса сообщений.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-toast` (`publish = false`); зависимости: `prime-core`, `prime-icons` (`check`, `infocircle`, `timescircle`, `exclamationtriangle`, `times`), `prime-dom`, `web-sys`.
- Изменяются `Cargo.toml` workspace, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (браузерный job, три места).
