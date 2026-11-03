# ConfirmDialog

Порт `p-confirmdialog` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`). Общий сервис и описание подтверждения — в `docs/confirmation.md`, окно — `docs/dialog.md`.

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-confirmdialog::ConfirmDialog`, `ConfirmDialogPt`, `ConfirmDialogHeadless` | `packages/primeng/src/confirmdialog/confirmdialog.ts`, `confirmdialog/style/confirmdialogstyle.ts` |
| CSS | `@primeuix/styles/confirmdialog` и пресет Aura — `cargo xtask gen-theme` (блока `/* For PrimeNG */` нет); окно и кнопки берут стили `dialog` и `button` |
| Демонстрации | `apps/showcase/doc/confirmdialog/*-doc.ts` (`basic`, `position`, `template`, `headless`) |

Спека: `openspec/specs/confirmdialog`.

```rust
let confirm = provide_confirmation_service();
view! {
    <ConfirmDialog />
    <Button label="Delete" on_click=move |_| confirm.confirm(
        Confirmation::new().header("Confirmation").message("Delete this record?").accept(Callback::new(|()| { /* ... */ })),
    ) />
}
```

## Что совпадает с PrimeNG

- `ConfirmDialog` — обёртка над `Dialog` с ролью `alertdialog` и классом `p-confirmdialog` на окне. Тело: значок `p-confirmdialog-icon` и текст `p-confirmdialog-message`; подвал: кнопки `p-confirmdialog-reject-button` и `p-confirmdialog-accept-button` (сначала отказ). Подписи: `*_label`, затем `*_button_props.label`, затем переводы `reject` и `accept`.
- Каждое значение сначала берётся из `Confirmation`, затем из свойства компонента (`option()` в PrimeNG): заголовок, текст, значок, подписи и значки кнопок, видимость кнопок, классы кнопок, `modal`, `position`, `closable`, `block_scroll`, `close_on_escape`, `dismissable_mask`, `default_focus`. Подтверждение, пришедшее при открытом окне, заменяет показанное на месте.
- Закрытие: кнопки вызывают `accept` / `reject`; крестик, `Escape` и клик по маске — `reject` (в `on_hide` приходит `ConfirmEventType::Cancel`); `Close` сервиса скрывает окно без колбэков. `on_hide` получает тип закрытия.
- `header_content`, `footer`, `message_content`, `icon_content`, `headless`, `accept_icon_content`-значки через `accept_icon`/`reject_icon`; `breakpoints`, `draggable`, `focus_trap`, `auto_z_index`, `base_z_index`, `style`, `style_class`, `mask_style_class`.

## Отличия

| Отличие | Причина |
|---|---|
| Фокус при показе работает по `default_focus` (`Accept` по умолчанию), включая `Close`; кнопки ищутся по настоящим классам `p-confirmdialog-*-button` и `p-dialog-close-button` | В PrimeNG селекторы `.p-confirm-dialog-accept`/`-reject` не совпадают с классами кнопок, поэтому там фокус решает сам `Dialog`. |
| `message` выводится как текст, а не HTML | PrimeNG вставляет его через `innerHTML`; текст безопаснее и не требует `inner_html`. |
| `reject` не получает тип события; тип — только в `on_hide` | `Callback<()>` общий для ConfirmPopup и ConfirmDialog; `ConfirmEventType` нужен только окну. |
| `resizable` и `maximizable` выключены явно | В PrimeNG окно не изменяется по размеру; у нашего `Dialog` `resizable` по умолчанию включён. |
| Нет `appendTo`: окно всегда в `body` (как у `Dialog`) | Остальные оверлеи проекта тоже без него. |
| Нет `transitionOptions` и `rtl` | Анимация задана классами темы; раскладка справа налево не проверялась. |
| Положение подтверждения — строка (`"topleft"`, …), неизвестное значение даёт `center` | `Confirmation` не зависит от крейта `Dialog`. |
| Значки PrimeIcons в демонстрациях заменены значками `prime-icons`; `closeButtonProps` и `ConfirmationService.onAccept` не перенесены | Нет PrimeIcons; эти поля не используются демонстрациями. |
| Заголовок `Dialog` стал реактивным (`prime-dialog`) | Чтобы замена подтверждения на месте обновляла заголовок. |

## Что не проверялось

- Раскладка справа налево и сенсорный ввод.
- Чтение с экрана: `aria-*` и роль сверены с PrimeNG, поведение скринридеров — нет.
- Несколько `ConfirmDialog` с разными ключами одновременно открытыми — проверено только, что чужой ключ игнорируется.

## Производительность

См. `benches/` (`confirmdialog-1`, `confirmdialog-1000`, `case-confirmdialog`) и `benches/*budgets.toml`. На сервере окна нет, замер — стоимость подписки на сервис.
