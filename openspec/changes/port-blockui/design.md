# Design

## Context

Требования — в `specs/blockui`. Исходники PrimeNG 21.1.10: `blockui/blockui.ts` (около 200 строк), `blockui/style/blockuistyle.ts`; стили `@primeuix/styles/blockui`; демонстрации `apps/showcase/doc/blockui/{basic,document}-doc.ts`. Образцы в проекте: `prime-drawer` (машина состояний `visible → rendered → leaving`, `Portal`, слой `ZGroup::Modal`, `block_body_scroll`, запасной таймер выхода, `animationend`), `prime-dialog` (то же, общий счётчик прокрутки).

Наблюдения по исходнику:
- Хост — сама маска: `block()` ставит `display: flex`, переносит хост в `target.getBlockableElement()` (и задаёт ему `position: relative`) либо в `body` (с `blockBodyScroll()`), задаёт слой `modal + baseZIndex` и классы `p-overlay-mask p-overlay-mask-enter-active`; `unblock()` сменяет класс входа на `p-overlay-mask-leave-active` и по `animationend` вызывает `destroyModal()` (display none, снятие классов и слоя, возврат из `body`, `unblockBodyScroll()`).
- Корневые классы: `p-blockui p-blockui-mask`, у документа ещё `p-blockui-mask-document`; `aria-busy` = `blocked`.
- Тема: `.p-blockui-mask.p-overlay-mask { position: absolute }`, `.p-blockui-mask-document.p-overlay-mask { position: fixed }`.
- В PrimeNG хост без блокировки не скрыт (его содержимое видно); здесь без блокировки в DOM ничего нет.
- `position: relative` у цели PrimeNG не возвращает; здесь инлайн-значение восстанавливается.

## Goals / Non-Goals

**Goals:** те же классы и поведение слоя, прокрутки и анимации; цель — любой элемент через `NodeRef`; освобождение ресурсов при размонтировании.

**Non-Goals:** интерфейс `BlockableUI`, шаблон `content`, `Bind`, `pTemplate`.

## Decisions

### D1. Крейт и API
`prime-blockui` с `BlockUI`, `BlockTarget`. `BlockTarget` — `Clone`-обёртка над `Arc<dyn Fn() -> Option<web_sys::Element> + Send + Sync>`; `From<NodeRef<E>>` для элементов Leptos. Свойства: `blocked: Prop<bool>`, `target: Option<BlockTarget>`, `auto_z_index: OptProp<bool>`, `base_z_index: Prop<i32>`, `style`, `style_class`, `pt: Option<Pt>`, `children: Option<ChildrenFn>`.

### D2. Состояние
`rendered`, `leaving`, `mask: NodeRef<Div>`, `z_index`, `scroll_blocked`, `saved_position: StoredValue<Option<(Element, String)>>`, `timer`, `was_open` — как в Drawer. Эффект (только `cfg(wasm32)`): `blocked` вкл. → отмена таймера, `leaving = false`, `rendered = true`, а когда маска появилась в DOM — `opened()` (слой, прокрутка, `position: relative` цели); `blocked` выкл. → `begin_leave()` (класс выхода, таймер-запас 400 мс); `animationend` маски или таймер → `finish_leave()` (освобождение слоя, прокрутки и позиции, `rendered = false`).

### D3. Вывод
`rendered.get().then(|| <Portal mount=target><div class=… aria-busy=… style="display: flex …">…</div></Portal>)`. Цель вычисляется при построении `Portal` (`BlockTarget::resolve()`); если цель задана, но элемента ещё нет — маска не выводится и пытается снова при следующем изменении `blocked`. Содержимое строится заново при каждом показе (`ChildrenFn`).

### D4. Чистая логика (`state.rs`)
`mask_classes(document, leaving, extra)`, `aria_busy(leaving)`, `mask_style(auto_z, z, extra)` — классы и строки в таблицах тестов.

### D5. CSS
`blockui` добавляется в `theme.components`. Правила маски в `base.css` (`p-overlay-mask` и анимации) уже используются Drawer/Dialog; проверить `p-blockui-mask*`.

### D6. Тесты
SSR: при любом `blocked` маски нет (`rustdoc`-тест). Браузерные `tests/web.rs`: маска в цели (положение, `aria-busy`, содержимое, слой), восстановление `position`, документ (маска в `body`, класс, блокировка и снятие прокрутки), выход (классы, удаление), повторное включение во время выхода, размонтирование при включённой блокировке, `auto_z_index = false`. e2e: настоящая мышь по кнопкам, перекрытие панели маской (клик под маской не доходит), документ блокируется на три секунды и затем разблокируется.

### D7. Демонстрации и бенчмарки
`basic` — `Panel` с `NodeRef` на контейнер и кнопки Block/Unblock; `document` — кнопка «Block», блокировка снимается через три секунды (`set_timeout`). Бенч `blockui-1`/`-1000` (SSR, ничего не рисуется, измеряется обвязка), `case-blockui`.

## Risks / Trade-offs

- [Цель не готова при `blocked = true` с самого начала] Маска не выводится, пока элемента нет; после монтирования следующее изменение сигнала покажет её. Фиксируется в docs; для начальной блокировки цели нужен `blocked`, включаемый после монтирования.
- [Общий счётчик прокрутки] Снимаем блокировку ровно один раз за вызов (`scroll_blocked`), как Drawer.

## Open Questions

- Нужны ли события `on_block`/`on_unblock`: в PrimeNG нет, не добавляем.

## Migration Plan

Миграции нет. Откат — `git revert`.
