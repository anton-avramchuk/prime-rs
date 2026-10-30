# Design

## Context

Требования — в `specs/popover`. Исходники PrimeNG 21.1.10: `popover/popover.ts`, `popover/style/*`; демонстрации `apps/showcase/doc/popover/*-doc.ts`. Образец оверлея в проекте — `prime-select` (панель внутри корня, `prime_dom::align_overlay(Relative)`, `ZGroup::Overlay`, клик снаружи и `resize` на `window`, проверки `is_disposed()` в таймерах, `on_cleanup`). Образец общего состояния с `Copy`-описателем — `prime-inplace`.

Наблюдения по исходнику:
- Popover императивный: `toggle(event, target?)`, `show`, `hide`. Цель — `target ?? event.currentTarget ?? event.target`; `show` при заданных `event` и `target` вызывает `stopPropagation`, чтобы тот же клик не закрыл панель через документ.
- Позиционирование — `absolutePosition(container, target)`; стрелка: если левый край панели левее левого края цели, `--p-popover-arrow-left = targetLeft − containerLeft − 2·borderRadius`, иначе 0; если верх панели выше верха цели — `data-p-popover-flipped` и `p-popover-flipped`.
- Закрытие: клик по документу (на iOS — `touchstart`), `document:keydown.escape`, `resize` окна (не на сенсорных), `ConnectedOverlayScrollHandler` по предкам цели. Флаг `selfClick` и `OverlayService.clickObservable` нужны, чтобы клик по вложенному оверлею не закрывал внешний панель.
- Анимация `p-anchored-overlay` через `@primeuix/motion`; панель создаётся при показе (`render`) и убирается после ухода.

## Goals / Non-Goals

**Goals:** тот же DOM, классы и поведение закрытия; императивное управление без внешнего состояния; повторное использование решений `Select` для оверлея.

**Non-Goals:** `appendTo`, переходы `@primeuix/motion`, шина `OverlayService`, `datatable`-демонстрация (см. proposal).

## Decisions

### D1. Крейт
`prime-popover` с `Popover`, `PopoverRef`, `PopoverPt`. Зависит от `prime-core`, `prime-dom`, `web-sys`.

### D2. Управляющий объект `PopoverRef`
`#[derive(Clone, Copy)] struct PopoverRef { visible: RwSignal<bool>, target: StoredValue<Option<SendWrapper<HtmlElement>>, LocalStorage> }`. `new()` создаёт сигналы; методы `show(&MouseEvent)`, `show_at(&MouseEvent, &HtmlElement)`, `toggle`, `toggle_at`, `hide()`, `visible() -> bool`. Для вызова из обработчика Leptos (`on:click=move |e| op.toggle(&e)`) цель берётся из `e.current_target()`. `Popover` читает `PopoverRef` из свойства `popover_ref`; без него создаётся собственный и недоступен (компонент бесполезен — `popover_ref` обязателен, тип `PopoverRef` без `Option`).

Альтернатива — `NodeRef` триггера и сигнал `open`: проще, но не повторяет API PrimeNG (`op.toggle($event)`, `op.show($event, target)`), поэтому не выбрана.

### D3. Чистая логика (`state.rs`)
- `root_classes(flipped)` → `p-popover p-component` (+ `p-popover-flipped`).
- `arrow_left(container_left, target_left, border_radius) -> f64` (`0`, если левый край панели не левее цели).
- `is_flipped(container_top, target_top) -> bool`.
- `outside_click(inside_panel, inside_target) -> bool`.
- `retarget(current: Option<&Id>, next: &Id) -> Retarget { Show, Toggle, Switch }` — решение `toggle` при видимой/скрытой панели и смене цели.
Табличные тесты на каждую.

### D4. Показ
`Popover` выводит `Show when=visible` панель. После вставки в DOM (`request_animation_frame`, только `wasm32`): `align_overlay(panel, target, Relative, false)` → `arrow_left`/`flipped` по `getBoundingClientRect` панели и цели, `z-index` из `prime_core::zindex::generate(ZGroup::Overlay, base)` (если `auto_z_index`), `autofocus`, затем `on_show`. Слушатели `window`: `click` (с проверками `outside_click` через `contains`), `resize`, `keydown` (Escape), `scroll` в фазе захвата по предкам цели — снимаются в `on_cleanup` и при скрытии (как в `Select`).

### D5. Тот же клик
`show`/`toggle` из обработчика клика цели вызывают `event.stop_propagation()` только когда события окна/документа подписываются уже после клика: слушатель снаружи вешается после `request_animation_frame`, поэтому клик, открывший панель, до него не доходит. Клик по цели при видимой панели обрабатывается `outside_click` (цель считается «внутри») — закрытие происходит через `toggle` обработчика цели, а не окна.

### D6. Смена цели
`toggle` с другой целью при видимой панели: `hide()`, затем в следующем тике `show` у новой цели (аналог `destroyCallback`). Состояние «ждём показ» — в `StoredValue`; проверка `is_disposed()` перед показом.

### D7. CSS
В `@primeuix/styles` есть модуль `popover`; добавляется в `theme.components`. `cargo xtask gen-theme`, проверка, что CSS остальных не изменился. Если в модуле есть блок `/* For PrimeNG */`, добавить `popover` в `primeng_extra`.

### D8. Тесты
SSR: скрыта по умолчанию, `id`/`style`/`aria-*`/`pt`. Браузерные `tests/web.rs`: `show`/`hide`/`toggle`, явная цель, смена цели, клик снаружи/внутри/по цели, `dismissable=false`, Escape, `resize`, `autofocus`, порядок `on_show`/`on_hide`, снятие слушателей. e2e: настоящая мышь и клавиатура, размещение над целью у нижнего края, прокрутка предка. Крейт добавляется в три места `ci.yml`.

### D9. Бенчмарки
Сценарии `popover-1`/`-1000` (скрытая панель) и `case-popover`; сначала `cargo bench -p render-bench --bench alloc`, для wasm — временный большой бюджет (заменять по номеру строки, не глобальным `sed`).

## Risks / Trade-offs

- [Панель внутри корня, а не в `body`] → родитель с `overflow: hidden` обрежет панель; фиксируется в docs, решение для `Select` то же. `Relative` требует предка с `position: relative`: в демонстрациях триггер и `Popover` лежат в нём.
- [Нет `OverlayService`] → клик во вложенный оверлей (например, `Select` внутри Popover) может закрыть внешнюю панель, если выпадающая панель вложенного оверлея оказывается вне DOM-поддерева. Внутри корня `Select` панель остаётся внутри DOM Popover, поэтому `contains` срабатывает; фиксируется в docs.
- [Нет анимации] → панель появляется мгновенно; отличие в docs.

## Open Questions

Нет: `appendTo` и анимации вынесены из объёма, остальное повторяет решения `Select`.

## Migration Plan

Миграции нет. Откат — `git revert`.
