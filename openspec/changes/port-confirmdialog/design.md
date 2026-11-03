# Design

## Context

Требования — в `specs/confirmdialog` и `specs/confirmation`. Исходники PrimeNG 21.1.10: `confirmdialog/confirmdialog.ts` (около 600 строк), `confirmdialog/style/confirmdialogstyle.ts`, `api/confirmationservice.ts`, `api/confirmeventtype.ts`; стили `@primeuix/styles/confirmdialog`; демонстрации `apps/showcase/doc/confirmdialog/{basic,position,template,headless}-doc.ts`. Образцы в проекте: `prime-dialog` (окно, маска, слой, прокрутка, фокус, `Escape`), `prime-confirmpopup` (подписка на сервис, `key`, перенацеливание, кнопки), `prime-confirmation`.

Наблюдения по исходнику:
- Компонент — тонкая обёртка над `p-dialog`: передаёт `role="alertdialog"`, `closable`, `modal`, `header`, `closeOnEscape`, `blockScroll`, `position`, `dismissableMask`, `draggable`, `baseZIndex`, `autoZIndex`, классы корня и маски; значения берутся функцией `option(name)` — сначала свойства подтверждения (его ключи копируются на компонент), затем свойства компонента.
- Тело: значок (`i.p-confirmdialog-icon` с классом из `icon`) и `span.p-confirmdialog-message` (содержимое `message` через `innerHTML`); шаблоны `message`, `icon`, `header`, `footer`, `headless`.
- Подвал: кнопка отказа (если `rejectVisible`), затем согласия (если `acceptVisible`); подписи `acceptLabel || acceptButtonProps.label || перевод`.
- Закрытие: `onVisibleChange(false)` (крестик, `Escape`, маска) → `close()` = `reject` с `ConfirmEventType.CANCEL` + скрытие; кнопки → `accept` / `reject` с типами `ACCEPT`/`REJECT`; `Close` сервиса → `hide()` без колбэков. `onHide` отдаёт тип.
- После скрытия (`onDialogHide`, т. е. после анимации выхода) подтверждение забывается. Фокус: `getElementToFocus` ищет `.p-confirm-dialog-accept`/`-reject` (в PrimeNG таких классов у кнопок нет — реальные классы `p-confirmdialog-*-button`), поэтому по умолчанию фокус там попадает на то, что решает `Dialog`.

## Goals / Non-Goals

**Goals:** тот же DOM, классы, роль и поведение закрытия; использование `Dialog` и общего сервиса без изменений их API.

**Non-Goals:** `appendTo`, `transitionOptions`, `rtl`, HTML в сообщении, `Bind`, `pTemplate`.

## Decisions

### D1. Обёртка над `Dialog`
`ConfirmDialog` создаёт `RwSignal<bool>` показа и передаёт его в `Dialog` (`visible`), а содержимое, подвал и заголовок собирает замыканиями. Остальные свойства `Dialog` (`modal`, `position`, `closable`, `close_on_escape`, `dismissable_mask`, `block_scroll`, `draggable`, `focus_trap`, `auto_z_index`, `base_z_index`, `breakpoints`, `style`, `style_class`, `mask_style_class`, `close_aria_label`) передаются `Signal::derive` по значению подтверждения или компонента, поэтому смена подтверждения при открытом окне применяется на месте. `role = "alertdialog"`, `resizable = false`, `maximizable = false` (в PrimeNG они не выставляются, значения по умолчанию `Dialog` PrimeNG: `resizable = false`; у нашего `Dialog` по умолчанию `resizable = true`, поэтому отключаем явно).

### D2. Приоритет значений
`option(name)`: поле `Confirmation`, если задано, иначе свойство компонента. Чистая функция `pick(from_confirmation, from_component)` и производные (`accept_label`, `position_of(&str) -> DialogPosition`, `default_focus`) — в `state.rs` с табличными тестами. Положение подтверждения — строка (`"topleft"`, …), разбирается в `DialogPosition`; неизвестное значение — `Center`.

### D3. Закрытие и поток событий
Подписка на сервис — как у ConfirmPopup. `answered: StoredValue<bool>` отмечает, что окно закрывают программно (кнопки, `Close`); `Effect` (только wasm) следит за `visible`: если оно стало `false`, а программного закрытия не было, это крестик, `Escape` или маска → `reject` + `on_hide(Some(Cancel))`. После программного закрытия флаг сбрасывается. `Dialog.on_hide` (после анимации) забывает подтверждение. Повторное `confirm` при открытом окне заменяет подтверждение и увеличивает `generation` (замыкания содержимого следят за ним).

### D4. Кнопки подвала
Те же `Button` и свойства, что у ConfirmPopup, но размер по умолчанию обычный (в PrimeNG `size` не задаётся) и классы `p-confirmdialog-reject-button`/`-accept-button`. Помощник подписи (`button_label`) повторяет порядок «подпись → `*_button_props.label` → перевод»; порядок в PrimeNG такой же. Выносить общую логику кнопок в `prime-confirmation` не будем: у обоих компонентов своё оформление, дублируется десяток строк.

### D5. Фокус
`Dialog.focus_on_show = false`; в `Dialog.on_show` ищем кнопку по классу (`accept`, `reject`) или `p-dialog-close-button` (`close`) и фокусируем (`set_timeout(5)`), `none` — ничего. Это отличие от PrimeNG, где селекторы не совпадают с классами (см. Context), документируется.

### D6. Шаблоны
`header_content` → `Dialog.header_content`; `footer` заменяет кнопки; `message_content: Callback<Confirmation, AnyView>` заменяет значок и текст; `icon_content: ViewFn` заменяет только значок; `headless: Callback<ConfirmDialogHeadless, AnyView>` (контекст — подтверждение, `accept`, `reject`) → `Dialog.headless`.

### D7. Сообщение
Текст `message` выводится как текст, не HTML: безопаснее и не требует `inner_html`. Значок подтверждения — `ViewFn` (`Confirmation.icon`) в `span.p-confirmdialog-icon`.

### D8. CSS
`confirmdialog` добавляется в `theme.components`.

### D9. Тесты
`prime-confirmation`: `ConfirmEventType` (`as_str`). `prime-confirmdialog`: SSR (окна нет до подтверждения), браузерный `tests/web.rs` одним последовательным тестом (общие слушатели и стек `z-index`, счётчик блокировки прокрутки): показ по `confirm`, разметка, роль, классы, подписи по умолчанию и свои, accept/reject и типы в `on_hide`, крестик и `Escape` = отказ `Cancel`, клик по маске при `dismissable_mask`, `close()` без колбэков, чужой `key`, замена подтверждения при открытом окне, `default_focus`, `accept_visible`/`reject_visible`, `position`, шаблоны, блокировка прокрутки снята после закрытия. e2e: стили темы (маска, окно по центру, кнопки подвала), позиции, accept/reject с Toast, шаблоны.

### D10. Демонстрации и бенчмарки
`basic` (Delete/Save с Toast), `position` (девять позиций), `template` (свое сообщение), `headless`. Бенч `confirmdialog-1`/`-1000` и `case-confirmdialog`.

## Risks / Trade-offs

- [Двойной источник закрытия] Различие «программно» и «пользователем» держится на флаге `answered`; проверяется браузерными тестами.
- [Эффект только в браузере] На сервере окна нет, поэтому SSR-проверок состава нет.
- [Отличия `Dialog`] Если у `Dialog` поменяются значения по умолчанию, ConfirmDialog наследует их; поэтому важные (`resizable`, `maximizable`) задаются явно.

## Open Questions

- Нужен ли общий помощник кнопок подтверждения: подождём SpeedDial/другие потребители; сейчас дублирование мало.

## Migration Plan

Миграции нет. Откат — `git revert`.
