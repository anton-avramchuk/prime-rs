# Proposal

## Why

Message — компонент вывода сообщений (информация, успех, предупреждение, ошибка) с закрытием и автоскрытием. После полей ввода это первый компонент волны 1 с таймером жизни и слотами содержимого.

## What Changes

- Новый крейт `prime-message` с компонентом `Message`: `severity` (`success`, `info` по умолчанию, `warn`, `error`, `secondary`, `contrast`), `variant` (`outlined`, `text`, `simple`), `size` (`small`, `large`), `text`, дочернее содержимое, `icon` (CSS-класс), слоты `icon_template`, `close_icon_template`, `container_template` (получает колбэк закрытия), `closable`, `close_icon` (CSS-класс), `life` (мс), `style`, `style_class`, `pt` (`root`, `content_wrapper`, `content`, `icon`, `text`, `close_button`, `close_icon`), событие `on_close`.
- Разметка и классы как в PrimeNG 21.1.10: корень `role="alert" aria-live="polite"` с классами `p-message p-component p-message-<severity>`, `p-message-<variant>`, `p-message-sm`/`-lg`; внутри `p-message-content-wrapper` > `p-message-content` с иконкой, текстом (`p-message-text`), кнопкой закрытия (`p-message-close-button` с `p-message-close-icon`, `aria-label` из локали); `data-p` на корне и всех частях.
- Закрытый Message скрывается (класс `p-message-leave-active` и `display: none`).
- Страница `/message` (`basic`, `closable`, `severity`, `outlined`, `simple`, `sizes`, `icon`, `life`), e2e, бенчмарки, бюджеты.
- **Не входит:** `escape=false` (HTML в `text`), анимации `@primeuix/motion` (`showTransitionOptions`, `hideTransitionOptions`, `motionOptions`), ripple на кнопке закрытия, интеграция с `MessageService`/`Toast`.

## Capabilities

### New Capabilities
- `message`: компонент Message — разметка, классы, закрытие, `life`, слоты, `pt`, производительность.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-message` (`publish = false`); зависимости `prime-core`, `prime-icons` (`times`), `web-sys`.
- Изменяются `showcase`, `benches/render`, `benches/wasm-size`, бюджеты, `e2e/tests`, браузерный job CI не меняется (браузерные тесты — для `life` и закрытия добавляются в него).
