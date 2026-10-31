# Design

## Context

Требования — в `specs/dialog`. Исходники PrimeNG 21.1.10: `dialog/dialog.ts`, `dialog/style/dialogstyle.ts`; демонстрации `apps/showcase/doc/dialog/*-doc.ts`. Образцы в проекте: `prime-drawer` (состояние `visible`/`rendered`/`leaving`, `Portal` с построением разметки внутри замыкания, `Core` с очищающими замыканиями, `FocusTrap`, `block_body_scroll`, Escape у верхнего окна, выход по `animationend` с запасным таймером), `prime-popover` (слушатели окна), `prime-core::zindex`.

Наблюдения по исходнику:
- Маска `div.p-dialog-mask` есть **всегда** (и у немодального диалога): `position: fixed`, на весь экран, `display: flex`; выравнивание окна — через `justify-content` / `align-items` по `position`; `pointer-events: none` у немодального, у корня окна `pointer-events: auto`. Эти правила в PrimeNG — инлайн-стили из `dialogstyle.ts`, в CSS темы их нет: записываем их сами инлайн.
- Анимация окна: имя `p-dialog` у `@primeuix/motion` даёт классы `p-dialog-enter-active` / `p-dialog-leave-active`; маска модального диалога — `p-overlay-mask-enter-active` / `-leave-active`. Есть ли соответствующие `@keyframes` в `@primeuix/styles/dialog`, выясняется генерацией темы (см. Открытые вопросы).
- Перетаскивание: `mousedown` на заголовке (не внутри `[data-pc-section=headeractions]`) → `mousemove`/`mouseup` на `window`; расчёт `left`/`top` по `getBoundingClientRect` с поправкой на поля, `position: fixed`, `margin: 0`; при `keepInViewport` обновления, выходящие за `minX/minY` или за экран, пропускаются.
- Изменение размера: ручка `p-resizable-handle`, `mousemove` на `window`; в PrimeNG при «ещё не перетаскивали» (`top`/`left` не заданы) дельта удваивается — окно центрировано, поэтому растёт в обе стороны; повторяем.
- Максимизация: `maximized` переключается, `p-dialog-maximized`; для немодального без `blockScroll` блокирует прокрутку (`blockBodyScroll`); при закрытии сбрасывается.
- `breakpoints`: в `head` создаётся `<style>` с правилами `.p-dialog[<uuid>]:not(.p-dialog-maximized){width:… !important}`; на окне ставится атрибут `<uuid>`.
- `Escape` закрывает окно, только если `z-index` окна равен верхнему (`ZIndexUtils.getCurrent`).
- Закрытие по маске — `mousedown` на самой маске (цель события — маска).
- Фокус при показе: первый фокусируемый в содержимом → подвал → заголовок, с задержкой (`transitionOptions` или 5 мс).

## Goals / Non-Goals

**Goals:** тот же DOM, классы, поведение указателя и клавиатуры, порядок слоёв; средства `prime-dom` и решения `Drawer` без копирования ошибок.

**Non-Goals:** `appendTo`, `@primeuix/motion`, `OverlayService`, общий «модальный» помощник с `Drawer` (выделять, когда понадобится третьему потребителю).

## Decisions

### D1. Крейт
`prime-dialog` с `Dialog`, `DialogPosition`, `DialogPt`. Зависит от `prime-core`, `prime-dom`, `prime-button`, `prime-icons` (`times`, `windowmaximize`, `windowminimize`), `web-sys`.

### D2. Состояние
Как у `Drawer`: `visible` (желаемое), `rendered` (в DOM), `leaving` (играет выход); дополнительно `maximized: RwSignal<bool>`. Один `Effect` по `(visible, panel mounted)` запускает `opened()` / `begin_leave()`; `finish_leave()` — по `animationend` окна либо запасному таймеру (700 мс; анимация 150–300 мс, берём общий запас). Повторный показ во время выхода возвращает окно в состояние «открыто».

### D3. Чистая логика (`state.rs`)
- `DialogPosition`, `mask_classes(position, modal)`, `root_classes(maximized, leaving)`.
- `mask_inline_style(position, modal) -> String` (`position:fixed; … justify-content / align-items / pointer-events`), `root_inline_style()`.
- `drag_target(offset_left, offset_top, margin_left, margin_top, dx, dy) -> (left, top)` и `clamp_drag(left, top, width, height, viewport_w, viewport_h, min_x, min_y, keep) -> (Option<left>, Option<top>)` — раздельное принятие осей, как в PrimeNG.
- `resize_to(width, height, dx, dy, has_been_dragged, min_w, min_h, offset_left, offset_top, viewport_w, viewport_h) -> (Option<width>, Option<height>)`.
- `breakpoints_css(id, &[(bp, width)]) -> String`.
- `is_topmost(own_z, top_z)`, `mask_z(panel_z)`.
Табличные тесты на каждую.

### D4. Вывод в `body`
`Portal` выносит маску и вложенное в неё окно одним блоком (в PrimeNG окно — потомок маски, поэтому `Portal` содержит `div.p-dialog-mask > div.p-dialog`). Разметка строится внутри замыкания `Portal` (свойства — в `Arc<Parts>`), как у `Drawer`. Клик по маске определяется по цели события (`target == currentTarget`).

### D5. Указатель
Перетаскивание и изменение размера — слушатели `mousemove`/`mouseup` на `window`, подписанные при `mousedown` и снимаемые на `mouseup` и при закрытии; значения применяются прямо к стилям элемента (не через сигналы — частые события). Блокировка выделения текста — атрибут `data-p-unselectable-text` и `user-select: none` на `body` на время жеста. Позиция запоминается в стиле окна и сбрасывается при закрытии.

### D6. Максимизация и прокрутка
`maximize()` переключает сигнал и вызывает `on_maximize`; блокировка прокрутки — через `block_body_scroll` с привязкой к `Core.scroll_blocked` (отдельный флаг для максимизации и для модальности, оба снимаются в `release_layer`). Общий счётчик `prime-dom` делает вложенные диалоги безопасными.

### D7. Адаптивность
`breakpoints` → `<style>` в `head` создаётся при показе и удаляется при `release_layer`/размонтировании; окно получает атрибут `data-dlg-<id>` (вместо голого `uuid`), правило `.p-dialog[data-dlg-<id>]:not(.p-dialog-maximized)`.

### D8. Фокус
`focus()` по цепочке содержимое → подвал → заголовок через `prime_dom::first_focusable`, задержка 5 мс (`set_timeout`); `FocusTrap` при `focus_trap`.

### D9. CSS
`dialog` добавляется в `theme.components`; блока `For PrimeNG` у `dialogstyle.ts` нет (позиция и выравнивание — инлайн-стили, см. D3). Проверка: CSS остальных компонентов не изменился.

### D10. Тесты
SSR: скрыт по умолчанию, свойства не просачиваются. Браузерные `tests/web.rs` (один последовательный прогон, как у `Drawer`): показ в `body`, кнопки, закрытие кнопкой/Escape/маской (`dismissable_mask`), немодальный (клики проходят), позиции, перетаскивание и граница, изменение размера, максимизация, `breakpoints`, фокус, два диалога, повторный показ во время выхода, события, снятие слушателей. e2e: настоящая мышь (`page.mouse`) — перетаскивание, размер, максимизация; клавиатура. Крейт добавляется в три места `ci.yml`.

### D11. Бенчмарки
Сценарии `dialog-1`/`-1000` (скрытый диалог) и `case-dialog`; сначала `cargo bench -p render-bench --bench alloc`, для wasm — временный большой бюджет (заменять по уникальной строке).

## Risks / Trade-offs

- [Анимации] `p-dialog-enter-active` может не иметь `@keyframes` в теме → решается в первой задаче; запасной таймер выхода работает и без анимации.
- [Запасной таймер] если анимация длиннее таймера, окно исчезнет раньше; берём 700 мс (как у `Drawer`).
- [Указатель на сенсорных экранах] перетаскивание и размер — только мышь (как в PrimeNG); фиксируется в docs.
- [Дублирование с Drawer] общая часть (слои, прокрутка, Escape, выход) повторена; выделение — отдельным изменением, когда появится третий потребитель.

## Open Questions

- Есть ли `@keyframes` для `p-dialog-enter-active` / `p-dialog-leave-active` в `@primeuix/styles/dialog` после `gen-theme`. Если нет — искать в `packages/primeng` и `@primeuix/*`; если их нигде нет, записать собственные кадры в `xtask` (масштаб 0.7 и прозрачность, `transformOptions: scale(0.7)` в исходнике) и отметить в docs. Решается в первой задаче.

## Migration Plan

Миграции нет. Откат — `git revert`.
