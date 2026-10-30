# Proposal

## Why

Inplace — первый компонент волны 3 и самый простой в ней: область, которая по клику заменяет «вид» на «содержимое» (редактирование на месте, ленивая подгрузка). Не требует оверлеев и опирается на уже портированные `Button` и `TimesIcon`.

## What Changes

- Новый крейт `prime-inplace` с компонентом `Inplace` (порт `p-inplace` из PrimeNG 21.1.10): корень `div.p-inplace.p-component` (`aria-live="polite"`) с одной из двух частей: `div.p-inplace-display` (`tabindex="0"`, `role="button"`, `p-disabled` при `disabled`) или `div.p-inplace-content` с содержимым и, при `closable`, кнопкой закрытия `Button` со значком `TimesIcon`.
- Свойства: `active` (двусторонний `RwSignal<bool>`), `closable`, `disabled`, `prevent_click`, `close_icon` (`ViewFn`), `close_aria_label`, `style`, `style_class`, `pt`; части — `display: ViewFn`, `content: Callback<Callback<()>, AnyView>` (получает функцию закрытия, как `closeCallback`); обратные вызовы `on_activate`, `on_deactivate` (`Callback<()>`, без события).
- Поведение: клик по виду и `Enter` активируют (клик не срабатывает при `prevent_click`, `Enter` срабатывает всегда, как в PrimeNG); закрытие по кнопке и по `close`-колбэку; при `disabled` активация и закрытие не выполняются.
- Страница showcase `/inplace` с демонстрациями `basic`, `input`, `image`, `lazy`; e2e; бенчмарки и бюджеты; `docs/inplace.md`.
- Стили: `inplace` добавляется в `theme.components`.
- **Не входит:** `Ripple`, `closeIcon` как имя класса шрифта (вместо него `ViewFn`), селекторы `pInplaceDisplay`/`pInplaceContent` (вместо них свойства `display` и `content`), `Table` в демонстрации `lazy` (используется обычная `<table>`), `pAutoFocus` в демонстрации `input` (поле получает фокус вручную).

## Capabilities

### New Capabilities
- `inplace`: область, переключаемая между видом и содержимым по клику.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-inplace` (`publish = false`); зависимости: `prime-core`, `prime-button`, `prime-icons` (`times`), `web-sys`.
- Изменяются `Cargo.toml` workspace, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (браузерный job, три места, если появится `tests/web.rs`).
