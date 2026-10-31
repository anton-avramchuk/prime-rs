# Proposal

## Why

Dialog — шестой компонент волны 3 и главный модальный компонент PrimeNG: окно поверх страницы с заголовком, содержимым и подвалом, которое можно перетаскивать, масштабировать и разворачивать на весь экран. Он строится на тех же средствах, что `Drawer` (`Portal`, маска, `block_body_scroll`, `FocusTrap`, слои `ZGroup::Modal`), и добавляет к ним работу с указателем (перетаскивание и изменение размера) — то, что затем понадобится `Toast` и `Menu`.

## What Changes

- Новый крейт `prime-dialog` (порт `p-dialog` из PrimeNG 21.1.10): `Dialog` — маска `div.p-dialog-mask` (с `p-overlay-mask` при `modal` и `p-dialog-<position>` для положения) > `div.p-dialog.p-component[role=dialog][aria-modal=true]` с `div.p-resizable-handle` (при `resizable`), `div.p-dialog-header` (название `span.p-dialog-title`, `div.p-dialog-header-actions` с кнопками `p-dialog-maximize-button` и `p-dialog-close-button`), `div.p-dialog-content` и необязательным `div.p-dialog-footer`. Маска и окно выносятся в `body` через `Portal`.
- Свойства: `visible` (двусторонний `RwSignal<bool>`), `header`, `position` (`DialogPosition`: `Center` по умолчанию, `Top`, `Bottom`, `Left`, `Right`, `TopLeft`, `TopRight`, `BottomLeft`, `BottomRight`), `modal` (нет), `dismissable_mask` (нет), `close_on_escape` (да), `closable` (да), `draggable` (да), `resizable` (да), `maximizable` (нет), `keep_in_viewport` (да), `min_x`, `min_y`, `show_header` (да), `block_scroll` (нет), `focus_on_show` (да), `focus_trap` (да), `auto_z_index` (да), `base_z_index` (0), `breakpoints` (`Vec<(String, String)>`: ширина экрана → ширина окна), `role` (`dialog`), `close_aria_label`, `content_style`, `content_style_class`, `mask_style`, `mask_style_class`, `style`, `style_class`, `pt`; представления `header_content`, `footer`, `close_icon`, `maximize_icon`, `minimize_icon`, `headless` (`ViewFn`); обратные вызовы `on_show`, `on_hide`, `on_resize_init`, `on_resize_end`, `on_drag_end`, `on_maximize`.
- Поведение: окно появляется и убирается с анимацией (`p-dialog-enter-active` / `p-dialog-leave-active` у окна, `p-overlay-mask-*-active` у модальной маски) и остаётся в DOM до `animationend`; перетаскивание за заголовок (кроме кнопок) с ограничением областью экрана; изменение размера за правый нижний угол; кнопка максимизации переключает `p-dialog-maximized` и блокирует прокрутку страницы; клик по маске при `dismissable_mask` и `closable`, кнопка закрытия и `Escape` у верхнего по `z-index` окна закрывают диалог; фокус на первый фокусируемый элемент содержимого (затем подвала и заголовка); при `breakpoints` в `head` добавляются правила `@media`, задающие ширину окна.
- Страница showcase `/dialog` с демонстрациями `basic`, `position`, `modal`, `withoutmodal`, `maximizable`, `responsive`, `longcontent`, `template`, `headless` (упрощённая); e2e с настоящей мышью и клавиатурой; бенчмарки и бюджеты; `docs/dialog.md`.
- Стили: `dialog` добавляется в `theme.components`.
- **Не входит:** `appendTo` (окно всегда в `body`), `@primeuix/motion` и `motionOptions`/`maskMotionOptions`/`transitionOptions` (анимации — CSS-классы), `closeButtonProps` и `maximizeButtonProps` (кнопки — `Button` с теми же свойствами по умолчанию), `OverlayService.emitParentDrag` (нужен только вложенным оверлеям), `rtl`, шаблон `content` (содержимое — `children`), `pt` с контекстом, демонстрация `overlaysinside` (у `Select` нет `appendTo`: его панель обрежется границами окна), устаревшие `closeIcon`/`maximizeIcon`/`minimizeIcon` как имена CSS-классов.

## Capabilities

### New Capabilities
- `dialog`: окно поверх страницы с маской, перетаскиванием, изменением размера, максимизацией, анимацией и ловушкой фокуса.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-dialog` (`publish = false`); зависимости: `prime-core`, `prime-dom`, `prime-button`, `prime-icons` (`times`, `windowmaximize`, `windowminimize`), `web-sys`.
- Изменяются `Cargo.toml` workspace, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (браузерный job, три места).
