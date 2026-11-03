# Design

## Context

Требования — в `specs/dock`. Исходники PrimeNG 21.1.10: `dock/dock.ts` (около 350 строк), `dock/style/dockstyle.ts`; стили `@primeuix/styles/dock`; демонстрации `apps/showcase/doc/dock/{basic,advanced}-doc.ts`. Образцы в проекте: `prime-menu` (список пунктов, `MenuItem`, фокус и клавиши по чистой логике), `prime-breadcrumb` (собственный `BreadcrumbItem` без полей маршрутизатора), `prime-badge`.

Наблюдения по исходнику:
- В версии 21 «магнификации» пунктов (`currentIndex`, классы `second-prev` и т. п.) в стилях нет: `onItemMouseEnter` только запоминает индекс. Переносим как есть — без увеличения.
- Идентификатор пункта: `item.id` или номер `"0"`, `"1"`…; фокусный пункт хранится по идентификатору (`focusedOptionIndex`). Выбор следующего/предыдущего ищет в списке `li[data-pc-section=item][data-p-disabled=false]`, а `changeFocusedOptionIndex` зажимает индекс в пределах списка доступных.
- Список фокусируется целиком (`tabindex=0`); `onListFocus` ставит фокус на индекс 0 доступных, `onListBlur` сбрасывает.
- Клавиши: по ориентации; `Home`/`End`; `Enter`/`Space` — клик по `a, button` внутри пункта.
- `matchMedia("(max-width: <breakpoint>)")` → `queryMatches` → класс `p-dock-mobile`; слушатель `change`.
- Корень — `position: absolute` (тема), поэтому контейнер демонстрации должен быть `position: relative`.

## Goals / Non-Goals

**Goals:** тот же DOM, классы, роли, клавиши и мобильный режим.

**Non-Goals:** `routerLink`, `tooltipOptions`, `Ripple`, `Bind`, `pTemplate`, демонстрация `advanced`.

## Decisions

### D1. Крейт и модель
`prime-dock` с `Dock`, `DockItem`, `DockCommand { event, item }`, `DockPosition`, `DockPt`. `DockItem` — строитель (`DockItem::new().label(..).icon(..).command(..)`), по образцу `MenuItem`. Своя модель, а не `MenuItem`: dock не должен зависеть от крейта `prime-menu`.

### D2. Чистая логика (`state.rs`)
`root_classes(position, mobile, extra)`, `orientation(position)`, `item_classes(focused, disabled, extra)`, `item_id(list_id, item, index)`, `enabled_ids`, `key_action(position, code) -> Option<Action>` (`Next`, `Prev`, `First`, `Last`, `Activate`, `Ignore`), `step(ids, current, action) -> Option<usize>` с зажимом в границах. Табличные тесты.

### D3. Идентификаторы
Пункт без `id` получает `<id списка>_<индекс>`, а не голый номер: голые числа коллидируют с другими `id` на странице; в `aria-activedescendant` используется тот же идентификатор. Отличие фиксируется в docs.

### D4. Состояние и разметка
`focused_index: RwSignal<Option<usize>>` (индекс среди доступных). Список пунктов строится один раз от `model`; состояние пункта (`p-focus`, `data-p-focused`) — реактивные атрибуты. Клик по пункту — прямой обработчик; `Enter`/`Space` — `click()` у `a` пункта под фокусом (как в PrimeNG).

### D5. Мобильный режим
`cfg(wasm32)`: `window.matchMedia("(max-width: {breakpoint})")`, начальное значение `matches`, слушатель `change` (замыкание `wasm_bindgen`), снятие в `on_cleanup`. Нужны features `web-sys`: `MediaQueryList`, `MediaQueryListEvent`.

### D6. Бейдж
`Badge` из `prime-badge` с `value` пункта и `style_class` бейджа (`badge_style_class`); без значения — не выводится.

### D7. CSS
`dock` добавляется в `theme.components`. Проверить `p-dock-*` в пресете.

### D8. Тесты
SSR: разметка, положения, классы пункта, значок, бейдж, `item`, `href`/`target`/`title`, `aria-*`, `pt`, `style_class`. Браузерные `tests/web.rs`: клик и `command`, отключённый пункт, фокус и клавиши (все четыре положения), `Home`/`End`/`Enter`/`Space`, `on_focus`/`on_blur`, мобильный режим (через `matchMedia` с `breakpoint` от очень малого и очень большого значения). e2e: положение, клавиши, переключатель положений, скриншотная пара.

### D9. Демонстрации и бенчмарки
`basic`: радиокнопки положения и контейнер `position: relative` с фоном; значки — цветные плитки с эмодзи (картинки PrimeNG с CDN не используются), подсказка `use:tooltip`. Бенч `dock-1`/`-1000` и `case-dock`.

## Risks / Trade-offs

- [Отличие идентификаторов] Идентификатор пункта не совпадает с PrimeNG (там голый номер); поведение то же.
- [matchMedia] Слушатель снимается при размонтировании; в SSR значение всегда «нет».

## Open Questions

- Нужна ли магнификация пунктов: в PrimeNG 21 её нет.

## Migration Plan

Миграции нет. Откат — `git revert`.
