# Design

## Context

Требования — в `specs/menubar`. Исходники PrimeNG 21.1.10: `menubar/menubar.ts` (`MenubarSub` — уровень, `Menubar` — состояние, мобильная кнопка, автоскрытие), `menubar/style/menubarstyle.ts`; демонстрации `apps/showcase/doc/menubar/{basic,command,template}-doc.ts` (`router-doc` не переносится). Образцы — `prime-tieredmenu` и `prime-contextmenu` (модель, путь, фокус, поиск, сторона подменю, слушатели).

Отличия от `TieredMenu`/`ContextMenu` по исходнику:
- Корень горизонтальный (`role=menubar`): ← → ходят по корню, ↓ раскрывает группу (в `TieredMenu` ↓ — следующий пункт).
- `mobileActive`, кнопка с `aria-expanded`, слой на корневом списке при открытии, возврат фокуса на кнопку; `queryMatches` по `breakpoint` даёт `p-menubar-mobile`.
- `autoHide`/`autoHideDelay` — закрытие по `mouseleave` с задержкой (`Subject` с `debounce`).
- Слушатели клика вне и `resize` висят, пока путь не пуст; клик вне при открытой мобильной панели закрывает её.
- Слоты `start`/`end`.

## Goals / Non-Goals

**Goals:** тот же DOM, классы, `aria-*`, клавиши и поведение.

**Non-Goals:** `routerLink`, `Tooltip`/`Badge`, HTML-метки, `Ripple`, `Bind`, `pTemplate`, `appendTo`, анимации.

## Decisions

### D1. Модель
Переиспользуем `Tree`/`State` из `prime-tieredmenu`. Различия корня (← → и ↓) — отдельные методы модели (например `root_arrow_left/right/down`), выбираемые после чтения `onArrowDownKey/onArrowRightKey/onArrowLeftKey` исходника, чтобы не менять поведение `TieredMenu`. Новые функции — с табличными тестами в `prime-tieredmenu`.

### D2. Разметка и состояние
По образцу `ContextMenu`: `Core` с `state`, `focus`, `sides`, `mobile`, `mobile_active`, поиском; уровни рекурсивной функцией; классы `p-menubar-*`. Корневой список на узком окне скрыт стилями темы, показывается классом `p-menubar-mobile-active`.

### D3. Подменю
Сторона вложенных подменю — `submenu_side`, как у `TieredMenu`; подменю корня открываются вниз по CSS темы.

### D4. Автоскрытие
`mouseleave` корня запускает таймер `auto_hide_delay`, `mouseenter` отменяет; таймер в `StoredValue`, сбрасывается при размонтировании (только wasm).

### D5. Мобильная кнопка
`mobile_active` — сигнал; открытие выдаёт слой `Menu` (`zindex::generate`) корневому списку и в следующем тике делает первый показ; закрытие — `hide` и фокус на кнопку (`set_timeout(0)`). Слушатель клика вне различает корневой список и кнопку.

### D6. Тесты
SSR: разметка, слоты, кнопка, группы, разделители, `pt`, шаблоны. Браузерные `tests/web.rs` одним последовательным тестом: клик/повторный клик, лист и команда, наведение, `auto_hide`, клик вне, клавиши корня и подменю, поиск, мобильная кнопка (`breakpoint` больше окна), шаблоны, размонтирование. e2e: горизонтальная раскладка темы, подменю под пунктом, клавиатура, мобильная кнопка при узком окне.

### D7. Демонстрации и бенчмарки
`basic`, `command` (Toast), `template` (`start`/`end`, свой пункт). Бенч `menubar-1`/`-1000`, `case-menubar`.

## Risks / Trade-offs

- [Горизонтальная навигация] Расхождение с `TieredMenu` в клавишах корня → отдельные функции модели с тестами, `TieredMenu` не трогаем.
- [Мобильный режим] Зависит от стилей темы и `matchMedia`; проверяется e2e при узком окне.
- [Дублирование разметки] Осознанно (как у ContextMenu); вынос — после MegaMenu/PanelMenu.

## Open Questions

- Применяется ли слой `Menu` к подменю на десктопе (`autoZIndex` в `MenubarSub`) — уточнить по исходнику при apply.

## Migration Plan

Миграции нет. Откат — `git revert`.
