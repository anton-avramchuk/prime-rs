# Design

## Context

Требования — в `specs/contextmenu`. Исходники PrimeNG 21.1.10: `contextmenu/contextmenu.ts` (`ContextMenuSub` — уровень, `ContextMenu` — состояние, триггер, положение), `contextmenu/style/contextmenustyle.ts`; демонстрации `apps/showcase/doc/contextmenu/{basic,command,template}-doc.ts` (`router-doc`, `table-doc` не переносятся). Образец — `prime-tieredmenu` (модель, путь, фокус, клавиатура, поиск, положение подменю, схема слушателей).

Отличия от `TieredMenu` по исходнику:
- Нет `popup`/`show_at`; показ — только по событию (`show(event)` берёт `pageX/pageY`).
- Положение: `position()` — координаты курсора + переворот/прижатие по `getViewport()` и `scrollingElement`; подменю — `position(sublist)` в пикселях (`left = item_width` или `-sub_width`).
- Триггер: слушатель на `target` или `document` (`global`); на iOS/Android — `touchstart` с таймером `pressDelay` и `touchend`.
- Закрытие кликом игнорирует `button == 2` и `ctrlKey`; `resize` закрывает всегда.
- Фокус корневого списка — после показа (`onAfterEnter`).

## Goals / Non-Goals

**Goals:** тот же DOM, классы, `aria-*`, клавиши и поведение; точное положение у курсора.

**Non-Goals:** `Table`, `appendTo`, анимационные опции, `routerLink`, подсказки, HTML-метки, `Ripple`, `Bind`, `pTemplate`.

## Decisions

### D1. Переиспользование логики
`prime-contextmenu` зависит от `prime-tieredmenu` и берёт из него чистую модель (`build`, `is_valid`, `item_id`, поиск, переходы пути, `submenu_side`, `is_printable`), расширяя `pub use model::…` при необходимости. Разметка и состояние уровня пишутся в новом крейте по образцу `TieredMenu` (классы другие: `p-contextmenu-*`); общий компонент уровня не выносим — классы и `pt` различаются. Дублирование разметки осознанно: что выносить, решит третий потребитель (Menubar/MegaMenu).

### D2. Положение у курсора
Чистая функция `place(page_x, page_y, width, height, viewport, scroll) -> (left, top)`: `+1`, переворот, прижатие — с табличными тестами. Размер панели берётся после вставки в DOM (эффект под `cfg(wasm32)`); до расчёта панель скрыта (`visibility: hidden` инлайн-стилем), чтобы не мигала.

### D3. Триггер
`target: OptProp<String>` (CSS-селектор, разрешается при монтировании; для произвольного элемента — `ContextMenuRef::show` из своего обработчика) и `global: bool`. Слушатели вешаются эффектом под `cfg(wasm32)` и снимаются в `on_cleanup`. Сенсорное устройство определяется по `Navigator.userAgent` (iOS/Android), как в PrimeNG. `ContextMenuRef` (Copy) хранит сигналы `visible` и координаты.

### D4. Слушатели закрытия
Клик и `resize` на `window` — на время показа (клик через `set_timeout(0)`); клик закрывает, если он вне корня, не `button == 2` и без Ctrl. `watch_scroll` не нужен: PrimeNG ContextMenu прокрутку не слушает.

### D5. Состояние и клавиатура
Как в `TieredMenu`: `active_path`, `focus`, `dirty`, `search`, `mobile`; один `keydown` на корневом списке. `Escape`/`Tab` закрывают меню; возврата фокуса на цель нет (в PrimeNG его тоже нет).

### D6. Тесты
SSR: скрыто по умолчанию; разметка показанного меню, группы, разделители, `pt`, `item`. Чистые тесты `place`. Браузерные `tests/web.rs` одним последовательным тестом (общие слушатели окна): `contextmenu` на цели → показ у курсора и `preventDefault`, `global`, клик вне и правая кнопка, `resize`, лист, подменю и клавиши, поиск, долгое касание, `item`/`submenu_icon`. e2e: стили темы, положение у курсора и переворот у края, подменю вправо/влево, клавиатура, `command` с Toast.

### D7. Демонстрации и бенчмарки
`basic` (картинка и меню Copy/Rename), `command` (Toast), `template`; без `router`/`table`. Бенч `contextmenu-1`/`-1000` и `case-contextmenu` (с `dep:prime-menu`, `dep:prime-tieredmenu`).

## Risks / Trade-offs

- [Размеры до вставки] Положение зависит от размеров в браузере → скрытый первый кадр и e2e у края.
- [Дублирование разметки] Осознанно; снимается выносом после Menubar.
- [Сенсорный ввод] Долгое касание не проверяется на реальном устройстве — фиксируется в docs.

## Open Questions

- Нет.

## Migration Plan

Миграции нет. Откат — `git revert`.
