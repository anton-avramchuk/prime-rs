# Design

## Context

Требования — в `specs/confirmation` и `specs/confirmpopup`. Исходники PrimeNG 21.1.10: `confirmpopup/confirmpopup.ts` (около 530 строк), `confirmpopup/style/confirmpopupstyle.ts`, `api/confirmation.ts`, `api/confirmationservice.ts`; стили `@primeuix/styles/confirmpopup`; демонстрации `apps/showcase/doc/confirmpopup/{basic,template,headless}-doc.ts`. Образцы в проекте: `prime-popover` (панель у цели: `align_overlay`, стрелка, переворот, слушатели окна, перенацеливание), `prime-toast` (`MessageService` как `Copy`-хэндл в контексте с подписчиками), `prime-button`.

Наблюдения по исходнику:
- Компонент подписан на `requireConfirmation$`: `null` — скрыть; подтверждение с чужим `key` игнорируется (но если панель уже показана, она перевыравнивается в следующем кадре). Свои поля подтверждения копируются на компонент, `accept` и `reject` оборачиваются в эмиттеры.
- Панель в DOM, пока `render`; `onBeforeEnter` ставит автофокус, переносит в `body`, выравнивает (`absolutePosition`), стрелку, слой `overlay` и вешает слушатели (в `setTimeout`, чтобы не поймать клик, вызвавший `confirm`).
- Стрелка: `--p-confirmpopup-arrow-left` — разница левых краёв цели и панели, если панель левее цели; `p-confirm-popup-flipped` и `data-p-confirmpopup-flipped`, если панель выше цели.
- Закрытие: `Escape` (если `closeOnEscape !== false`) = «Нет»; клик вне панели и вне цели (`dismissableMask !== false`) = просто скрыть; `resize` окна (не на сенсорных); прокрутка предка цели. `accept`/`reject` скрывают и возвращают фокус на цель.
- Фокус при показе: `confirmation.defaultFocus ?? defaultFocus`: `accept` | `reject` | `none`.
- Кнопки: `size` по умолчанию `small`; `rejectButtonProps` по умолчанию без стиля (в демонстрациях `severity=secondary`, `outlined`); `acceptVisible`/`rejectVisible`.

## Goals / Non-Goals

**Goals:** тот же DOM, классы, роли и поведение панели; общий сервис и `Confirmation`, пригодные для ConfirmDialog без изменений.

**Non-Goals:** `appendTo`, `motionOptions`, `Bind`, `pTemplate`, `ConfirmationService.onAccept`.

## Decisions

### D1. Крейт `prime-confirmation`
`Confirmation` — строитель с публичными полями; `Callback` вместо эмиттеров; значки — `ViewFn`. Элемент-цель `Option<web_sys::HtmlElement>` не `Send`, поэтому `Confirmation` не `Send`: сервис хранит подписчиков в `StoredValue<_, LocalStorage>`, как `PopoverRef` хранит цель. `ConfirmationService` — `Copy`-хэндл: `confirm(Confirmation)`, `close()`, `subscribe(Callback<ConfirmEvent>) -> usize`, `unsubscribe(id)`; `ConfirmEvent { Confirm(Confirmation), Close }`. `provide_confirmation_service()` кладёт сервис в контекст, `use_confirmation_service()` достаёт его (паника с понятным текстом, если не положен — как у `MessageService`).

### D2. Крейт `prime-confirmpopup`: панель в `body`
Панель выводится в `Portal` (в `body`) и выравнивается режимом `Absolute`, как `appendTo = body` в PrimeNG. Логика слушателей — по образцу `Popover`, но сам он не оборачивается: другой корень (`role=alertdialog`, классы `p-confirmpopup`), другие CSS-переменные стрелки и подписка на сервис. Чистая логика стрелки и переворота (`arrow_left`, `is_flipped`, `outside_click`) уже есть в `prime-popover::state`; зависимость на `prime-popover` ради трёх функций не нужна — они копируются в `state.rs` вместе с табличными тестами (по 3–5 строк).

### D3. Чистая логика (`state.rs`)
`root_classes(flipped, extra)`, `arrow_left`, `is_flipped`, `outside_click`, `focus_target(per_confirmation, component) -> ConfirmFocus`, `accepts_key(component_key, confirmation_key)` (сравнение `Option<&str>`: `None == None`), `button_label(custom, default)`. Табличные тесты.

### D4. Состояние и показ
`Core`: `confirmation: RwSignal<Option<Confirmation>>` в `LocalStorage`, `visible: RwSignal<bool>`, `target` берётся из подтверждения. Подписка на сервис — в теле компонента (`subscribe`, `on_cleanup` → `unsubscribe`). `confirm` с тем же `key`: записать подтверждение, `visible = true`; если панель уже показана — перевыровнять в следующем тике (`set_timeout(0)`) и пересоздать содержимое. `close` — скрыть. Панель строится при `visible`, убирается после выхода (классы `p-anchored-overlay-enter-active`/`-leave-active` на корне, запасной таймер, как в `Popover`).

### D5. Слушатели
Как в `Popover`: клик, `Escape`, `resize`, `scroll` с захватом; подписка в `set_timeout(0)`; снятие в `closed()` и `on_cleanup`. Различие: `Escape` вызывает `reject` (если `close_on_escape != false`), клик снаружи только скрывает (если `dismissable_mask != false`).

### D6. Кнопки
`Button` из `prime-button` с `size=Small`; свойства из `ConfirmButtonProps` накладываются на значения по умолчанию; `autofocus` по `default_focus`. Фокус при показе — как в `Popover.focus_on_show`: находим кнопку по классу `p-confirmpopup-accept-button`/`-reject-button` и фокусируем в `set_timeout(5)`. После `accept`/`reject` фокус возвращается на цель.

### D7. Управляемый показ
`visible` (необязательный `Prop<bool>`): если задан, он определяет показ (`computedVisible` в PrimeNG); внутренний сигнал нужен, когда `visible` не задан. Закрытие пользователем меняет только внутреннее состояние, а показ остаётся за владельцем `visible`.

### D8. Headless и content
`headless` (`Callback<HeadlessContext, AnyView>`): `HeadlessContext { confirmation, accept: Callback<()>, reject: Callback<()> }` — заменяет всё содержимое панели; `content` (`Callback<Confirmation, AnyView>`) — только блок `p-confirmpopup-content`. В PrimeNG `#cp` дает `cp.onAccept()`; у нас вместо этого контекст.

### D9. CSS
`confirmpopup` добавляется в `theme.components`. Если в стилях нет `p-confirmpopup-reject-button`/`accept-button` — только классы (как в PrimeNG).

### D10. Тесты
`prime-confirmation`: сервис (подписчики, `confirm`, `close`, `unsubscribe`, контекст) — нативные. `prime-confirmpopup`: SSR (панели нет, пока не показана; на сервере сервис без вызовов), браузерный `tests/web.rs` одним последовательным тестом (общие слушатели окна и стек `z-index`): показ по `confirm`, разметка, подписи по умолчанию и свои, `accept`/`reject` и колбэки, возврат фокуса на цель, `Escape`, клик снаружи, `dismissable_mask=false`, `close_on_escape=false`, чужой `key`, повторное `confirm` с другой целью, `close()`, `default_focus`, `accept_visible`/`reject_visible`, `content` и `headless`, `visible`. e2e: положение под целью, стрелка, переворот у нижнего края, три демонстрации.

### D11. Демонстрации и бенчмарки
`basic` (Save/Delete с Toast), `template` (содержимое), `headless`. Бенч `confirmpopup-1`/`-1000` (на сервере панели нет — измеряется только корневой маркер) и `case-confirmpopup`.

## Risks / Trade-offs

- [Общий сервис в отдельном крейте] Лишний крейт, но ConfirmDialog не зависит от ConfirmPopup и наоборот.
- [Копия функций стрелки] Три маленькие функции дублируются; вынос в `prime-dom` — отдельное улучшение, когда понадобится третьему потребителю.
- [`Confirmation` не `Send`] Из-за `HtmlElement`; сервис и компонент используют `LocalStorage`.

## Open Questions

- `ConfirmationService.onAccept` нужен только ConfirmDialog PrimeNG — вернёмся к нему в `port-confirmdialog`.

## Migration Plan

Миграции нет. Откат — `git revert`.
