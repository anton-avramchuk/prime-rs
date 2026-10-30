# Proposal

## Why

Drawer — пятый компонент волны 3: панель у края экрана с маской, блокировкой прокрутки и ловушкой фокуса. Это первый модальный оверлей проекта: на нём впервые используются `Portal` и `FocusTrap` из `prime-dom` (готовы, но без потребителей), маска и блокировка прокрутки `body`, которые затем понадобятся `Dialog`.

## What Changes

- Новый крейт `prime-drawer` (порт `p-drawer` из PrimeNG 21.1.10): `Drawer` — корень `div.p-drawer.p-component[role=complementary]` (классы `p-drawer-<position>`, `p-drawer-full`, `p-drawer-open`) с `div.p-drawer-header` (заголовок `div.p-drawer-title`, кнопка закрытия `p-drawer-close-button`), `div.p-drawer-content` и необязательным `div.p-drawer-footer`; при `modal` за панелью выводится маска `div.p-drawer-mask.p-overlay-mask`. Панель и маска выносятся в `body` через `Portal`.
- Свойства: `visible` (двусторонний `RwSignal<bool>`), `position` (`DrawerPosition`: `Left` по умолчанию, `Right`, `Top`, `Bottom`, `Full`), `full_screen`, `modal` (да), `dismissible` (да), `close_on_escape` (да), `closable` (да), `block_scroll` (нет), `auto_z_index` (да), `base_z_index` (0), `header`, `aria_close_label`, `mask_style`, `style`, `style_class`, `pt`; части-представления `header_content`, `footer`, `close_icon`, `headless` (`ViewFn`); обратные вызовы `on_show`, `on_hide`.
- Поведение: при `visible` панель и маска появляются с анимацией входа (`p-drawer-enter-<position>`, маска — `p-overlay-mask-enter-active`); при скрытии играют анимации выхода и только после `animationend` убираются из DOM; клик по маске при `dismissible`, кнопка закрытия и `Escape` у верхнего по `z-index` окна закрывают панель; `z-index` по `ZGroup::Modal`, маска на единицу ниже панели; при `block_scroll` на `body` ставится `p-overflow-hidden` на время показа; `Tab` не покидает панель (`FocusTrap`).
- Страница showcase `/drawer` с демонстрациями `basic`, `position`, `size`, `fullscreen`, `template`, `headless` (упрощённая); e2e с настоящей мышью и клавиатурой; бенчмарки и бюджеты; `docs/drawer.md`.
- Стили: `drawer` добавляется в `theme.components` и в `primeng_extra`; `xtask gen-theme` учится распознавать маркер `/** For PrimeNG **/`, которым размечен блок у Drawer.
- В `prime-dom` добавляются `block_body_scroll()` / `unblock_body_scroll()` (счётчик вложенных вызовов, порт `blockBodyScroll`).
- **Не входит:** `appendTo` (панель всегда в `body`), `@primeuix/motion` и `motionOptions`/`transitionOptions` (анимации — CSS-классы), устаревший `showCloseIcon`, `closeButtonProps` (кнопка закрытия — `Button` с теми же свойствами по умолчанию: вторичная, текстовая, круглая), шаблон `content` (содержимое — `children`), `pt` с контекстом, демонстрация `headless` в полном виде (меню профиля с `Ripple` и `PanelMenu`).

## Capabilities

### New Capabilities
- `drawer`: модальная или встроенная панель у края экрана с анимацией, маской, блокировкой прокрутки и ловушкой фокуса.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-drawer` (`publish = false`); зависимости: `prime-core`, `prime-dom`, `prime-button`, `prime-icons` (`times`), `web-sys`.
- Изменяются `prime-dom` (блокировка прокрутки), `xtask` (маркер), `Cargo.toml` workspace, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (браузерный job, три места).
