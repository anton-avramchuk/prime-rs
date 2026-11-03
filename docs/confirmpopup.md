# ConfirmPopup

Порт `p-confirmpopup` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`). Общий сервис и описание подтверждения — в `docs/confirmation.md`.

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-confirmpopup::ConfirmPopup`, `ConfirmPopupPt`, `ConfirmPopupHeadless` | `packages/primeng/src/confirmpopup/confirmpopup.ts`, `confirmpopup/style/confirmpopupstyle.ts` |
| CSS | `@primeuix/styles/confirmpopup` и пресет Aura — `cargo xtask gen-theme` (блока `/* For PrimeNG */` нет); кнопки берут стили `button` |
| Демонстрации | `apps/showcase/doc/confirmpopup/*-doc.ts` (`basic`, `template`, `headless`) |

Спека: `openspec/specs/confirmpopup`.

```rust
let confirm = provide_confirmation_service();
view! {
    <ConfirmPopup />
    <Button label="Delete" on_click=move |e| confirm.confirm(
        Confirmation::new().target_event(&e).message("Delete this record?").accept(Callback::new(|()| { /* ... */ })),
    ) />
}
```

## Что совпадает с PrimeNG

- Разметка и классы: `div.p-confirmpopup.p-component[role=alertdialog]` > `div.p-confirmpopup-content` (значок `p-confirmpopup-icon`, текст `p-confirmpopup-message`) и `div.p-confirmpopup-footer` с кнопками `p-confirmpopup-reject-button` и `p-confirmpopup-accept-button`, оба размера `small`; подписи по умолчанию — переводы `reject` и `accept`.
- Показ только для подтверждений со своим `key`; панель выравнивается по `target` в `body` (`AlignMode::Absolute`), слой группы оверлеев, стрелка `--p-confirmpopup-arrow-left`, переворот над целью (`data-p-confirmpopup-flipped`, `p-confirm-popup-flipped`).
- Фокус при показе: `default_focus` подтверждения, иначе свойства компонента (`Accept`); `accept` и `reject` вызывают колбэк, скрывают панель и возвращают фокус на цель. `Escape` = отказ (если `close_on_escape` не `false`); клик вне панели и цели, `resize` окна (не на сенсорных) и прокрутка предка цели скрывают панель (клик — если `dismissable_mask` не `false`). Слушатели вешаются только на время показа, клик — в следующем такте.
- `content` заменяет блок содержимого, `headless` — всю панель, `accept_icon`/`reject_icon` — значки кнопок; `visible` управляет показом извне.

## Отличия

| Отличие | Причина |
|---|---|
| К классу переворота добавлен ещё `p-confirmpopup-flipped` | В PrimeNG класс называется `p-confirm-popup-flipped`, а правила темы — `.p-confirmpopup-flipped`, поэтому в PrimeNG перевёрнутая стрелка и отступ не оформляются. Мы оставили оба имени: исходное и то, на которое есть правила. |
| Нет `appendTo`: панель всегда в `body` (через `Portal`) | Так выводятся остальные оверлеи проекта. Контейнер `Portal` — служебный `div` в `body`. |
| Нет `showTransitionOptions`, `hideTransitionOptions`, `motionOptions` и выходной анимации: панель убирается сразу при скрытии | Тема не задаёт отдельных классов выхода; вход — без анимации (правило `p-anchored-overlay-*` не подключено). |
| `Confirmation` — строитель с `Callback` вместо эмиттеров `acceptEvent`/`rejectEvent`; значки — `ViewFn`, а не классы `pi pi-…` | Нет `EventEmitter` и PrimeIcons. |
| Повторное `confirm`, пока панель видна, скрывает её и показывает заново у новой цели, поэтому вызываются `on_hide` и `on_show` | В Angular панель перевыравнивается на месте; в Leptos пересоздать содержимое проще и надёжнее. |
| Нет `pt` кнопок (`pcAcceptButton`, `pcRejectButton`) и `Bind`; стилевые классы кнопок — через `accept_button_style_class`/`reject_button_style_class` | `pt` — простые добавки классов, стиля и атрибутов. |
| `#cp` с вызовами `cp.onAccept()`/`cp.onReject()` в `headless` заменён контекстом с обратными вызовами `accept` и `reject` | В Leptos нет шаблонных ссылок на компонент. |
| Значки PrimeIcons (`pi-exclamation-triangle`, `pi-info-circle`) в демонстрациях заменены значками `prime-icons` | Нет PrimeIcons. |

## Что не проверялось

- Раскладка справа налево и сенсорный ввод.
- Чтение с экрана: `aria-*` сверены с PrimeNG (`role=alertdialog`), поведение скринридеров — нет.
- Положение при нескольких панелях с разными ключами одновременно — проверено только, что чужой ключ игнорируется.

## Производительность

См. `benches/` (`confirmpopup-1`, `confirmpopup-1000`, `case-confirmpopup`) и `benches/*budgets.toml`. На сервере панели нет, замер — стоимость подписки на сервис.
