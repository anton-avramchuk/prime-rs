# Design

## Context

Требования — в `specs/menu`. Исходники PrimeNG 21.1.10: `menu/menu.ts`, `menu/style/menustyle.ts`; демонстрации `apps/showcase/doc/menu/*-doc.ts`. Образцы в проекте: `prime-popover` (`PopoverRef`, панель внутри корня, `align_overlay(Relative)`, слушатели окна в `set_timeout(0)`), `prime-breadcrumb` (модель пункта без маршрутизатора), `prime-select` (клавиши по списку, фокус).

Наблюдения по исходнику:
- Один шаблон: корень → `start` → `ul[role=menu]` → пункты (плоско или по группам: `submenu.items`) → `end`. Группа — `li.p-menu-submenu-label` + её пункты; разделитель — `li.p-menu-separator`.
- Идентификатор пункта: `item.id ?? "<menuId>_<i>[_<j>]"`; фокусный пункт хранится по этому id (`focusedOptionIndex`), `p-focus` и `aria-activedescendant` берутся из него.
- Клавиши работают по DOM: «доступные» — `li[data-pc-section=item][data-p-disabled=false]`; ↓/↑ — индекс текущего ±1 (первый/нулевой по умолчанию), Home/End — первый/последний; Enter/Space кликают `a` пункта.
- Фокус на список в статичном режиме ставит фокусным первый пункт; потеря — сбрасывает.
- Popup: `toggle(event)` запоминает `currentTarget`, показывает; после показа — `position: absolute; top: 0`, `z-index`, слушатели документа (клик), окна (resize, не на сенсорных), прокрутки предков цели, `absolutePosition`, фокус на список. `preventDocumentDefault` — клик внутри панели не закрывает ее как «внешний».
- Корневой инлайн-стиль: `position: absolute` (popup) или `relative`; в CSS темы его нет.
- Анимация `p-anchored-overlay` — через `@primeuix/motion`; не переносим (как в Popover).

## Goals / Non-Goals

**Goals:** тот же DOM, те же классы и роли, поведение клавиш и закрытия, `MenuRef` для всплывающего режима.

**Non-Goals:** `routerLink`, `appendTo`, `@primeuix/motion`, `tooltipOptions`, `badge`, HTML-подписи (`escape`), `Ripple`, `Bind`, `pTemplate`.

## Decisions

### D1. Крейт и модель
`prime-menu` с `Menu`, `MenuRef`, `MenuItem`, `MenuCommand { event, item }`, `MenuPt`. `MenuItem` — строитель (`MenuItem::new(label)`, `.icon(..)`, `.url(..)`, `.command(..)`, `.items(..)`, `MenuItem::separator()`). `model` — `Prop<Vec<MenuItem>>` (значение или сигнал).

### D2. Чистая логика (`state.rs`)
- `root_classes(popup, style_class)`, `item_classes(item, focused)`, `root_style(popup)`.
- `item_id(item, menu_id, i, j)`.
- `flatten(model) -> Vec<Row>`: плоский список строк (`Separator`, `Header(label)`, `Item{ id, item }`) с учётом `visible` пунктов и групп — единый источник для разметки и клавиш.
- `enabled_ids(rows) -> Vec<String>`; `next_id(ids, current)`, `prev_id(ids, current)` (как `findNext/PrevOptionIndex`), `first`, `last`.
- `should_prevent_default(item)` (нет `url`) и `can_run_command(item)`.
Табличные тесты на каждую. Клавиши работают по этим данным, а не по DOM.

### D3. Состояние и разметка
Внутри `Menu`: `focused_id: RwSignal<Option<String>>`, `list_focused: RwSignal<bool>`, `visible: RwSignal<bool>` (popup). Строки получаем из `flatten` через `Memo`; список рисуется `For` по `id` строк. В замыкании списка сигнал фокуса читать только `*_untracked` — иначе перестраиваются все пункты и теряется фокус (урок SelectButton); `p-focus`/`aria-*` — через реактивные атрибуты каждого пункта.

### D4. Popup
`MenuRef` (`Copy`) как `PopoverRef`: `visible`, `target` в `StoredValue<_, LocalStorage>`; `toggle/show/hide`. После вставки панели (эффект следит за `NodeRef`, урок Popover): `align_overlay(Relative)`, `z-index` (`zindex::generate(ZGroup::Menu, ..)`, `revert` при скрытии), фокус на список, `on_show`. Слушатели документа (клик) и окна (resize, scroll предков цели) вешаются в `set_timeout(0)` и снимаются в `on_cleanup` и при скрытии. Клик внутри панели не закрывает (проверка `contains`). Сенсорное устройство — resize не закрывает.

### D5. Клавиши
`keydown` списка: ArrowDown/Up (Alt+Up в popup — закрыть и фокус на цель), Home, End, Enter/NumpadEnter/Space (клик по `a` фокусного пункта; в popup сначала фокус на цель), Escape/Tab (в popup — фокус на цель и `hide`). `preventDefault` там же, где в PrimeNG.

### D6. Представления
`item: Option<Callback<MenuItem, AnyView>>`, `submenu_header: Option<Callback<MenuItem, AnyView>>`, `start`/`end: Option<ViewFn>`.

### D7. CSS
`menu` добавляется в `theme.components`. Блок `For PrimeNG` не нужен (позиция — инлайн-стиль D2). Проверить, что `p-menu-*` и `p-menu-overlay` есть.

### D8. Тесты
SSR: разметка, группы, разделители, невидимые, `data-p`, popup не выводится, `style_class`, `pt`. Браузерные `tests/web.rs` (один последовательный прогон — слушатели окна общие): клик и `command`, отключённый пункт, клавиши, фокус и `aria-activedescendant`, popup `toggle/show/hide`, клик вне, Escape, resize, слой, `on_show/on_hide`, шаблоны, отписка при размонтировании. e2e: настоящая мышь и клавиши. Крейт добавляется в три места `ci.yml`.

### D9. Бенчмарки
Сценарии `menu-1`/`-1000` (три пункта) и `case-menu`; сначала `cargo bench -p render-bench --bench alloc`, для wasm — временный большой бюджет (заменять по уникальной строке/номеру строки).

## Risks / Trade-offs

- [Фокус] Перестройка списка роняет фокус → только `*_untracked` в замыканиях списка.
- [Общие слушатели окна] Несколько меню в одном тесте → один последовательный прогон.
- [Выравнивание] Родитель — `position: relative` без `overflow: hidden` (как у Popover); фиксируется в docs.

## Open Questions

- Нужна ли анимация показа: проверить, есть ли в теме `@keyframes` для `p-anchored-overlay`; если нет — без анимации.

## Migration Plan

Миграции нет. Откат — `git revert`.
