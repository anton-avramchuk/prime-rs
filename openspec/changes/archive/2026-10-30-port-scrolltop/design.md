# Design

## Context

Требования — в `specs/scrolltop`. Исходники PrimeNG 21.1.10: `scrolltop/scrolltop.ts`, `scrolltop/style/scrolltopstyle.ts`; демонстрации `apps/showcase/doc/scrolltop/{basic,element}-doc.ts`. Образцы в проекте: `prime-button` (кнопка), `prime-slider` (подписка на `window` из обработчика, handle в `StoredValue<_, LocalStorage>`, снятие в `on_cleanup`), `prime-select` (z-index группы `Overlay`, проверка `is_disposed()` в замыканиях таймеров), `prime_core::zindex`.

Наблюдения по исходнику:
- Корень — сам `p-button` с `styleClass = cn(cx('root'), styleClass)`; `buttonProps` по умолчанию `{ rounded: true }`. Значок: свой класс (`icon`), иначе `ChevronUpIcon` с классом `p-scrolltop-icon`.
- `checkVisibility(scrollY)`: `scrollY > threshold` → `visible = true` и `render = true`; иначе `visible = false`. Элемент снимается (`render = false`) в `onAfterLeave` анимации `pMotion` (имя `p-scrolltop`).
- `onBeforeEnter`: `position = sticky` для `parent`, иначе `fixed`; `ZIndexUtils.set('overlay', …)`; `onBeforeLeave` освобождает z-index.
- `onClick`: цель — `defaultView` или родитель хоста; `scroll({ top: 0, behavior })`.
- Начальная видимость не вычисляется: пока не придёт первое событие `scroll`, кнопки нет, даже если страница уже прокручена. Мы начальную проверку делаем (в спеке).

## Decisions

### D1. Крейт и зависимости
`prime-scrolltop` с `ScrollTop`, `ScrollTopTarget { Window, Parent }`, `ScrollTopBehavior { Smooth, Auto }`, `ScrollTopPt`. Зависит от `prime-core`, `prime-button`, `prime-icons` (feature `chevronup`) и `web-sys`. Кнопка — `Button` с `rounded` (по умолчанию `true`), `severity` и `raised` из свойств; классы корня добавляются через `style_class`.

### D2. Чистая логика (`state.rs`)
- `is_visible(scroll: f64, threshold: f64) -> bool` — строгое `>`.
- `root_classes(target) -> &'static str` — `p-scrolltop` или `p-scrolltop p-scrolltop-sticky`.
- `enter_classes(started)` / `leave_classes(started)` — классы анимации.
Табличные тесты.

### D3. Видимость и жизненный цикл
Состояние `phase: RwSignal<Phase>`, `Phase { Hidden, Shown, Leaving }`. Подписка на `scroll` ставится только под `cfg(target_arch = "wasm32")`.

Родитель: у `Parent` нужен `parent_element()` самого компонента, а пока кнопка скрыта, элемента у компонента нет. Поэтому для `Parent` кнопка всегда лежит в DOM, а в фазе `Hidden` скрыта стилем `display: none` (у `Window` кнопка в фазе `Hidden` вообще не выводится). После монтирования родитель берётся через `NodeRef` кнопки; подписка ставится на него, и сразу выполняется начальная проверка. Для `Window` подписка и проверка делаются в `Effect` после монтирования, слушатель — `window_event_listener`, handle в `StoredValue<_, LocalStorage>`, снятие в `on_cleanup`. Это отличие фиксируется в `docs/scrolltop.md`.

### D4. Анимация
Классы `p-scrolltop-enter-from|active|to` и `leave-*` берутся из CSS темы: при входе ставятся `enter-from` + `enter-active`, на следующем кадре `enter-from` заменяется на `enter-to`; по окончании (таймер 300 мс по `transition: opacity 300ms` из темы, с проверкой `is_disposed()` и счётчиком поколений против устаревших таймеров) классы снимаются. Для выхода то же с `leave-*`, затем фаза `Hidden` и освобождение z-index. Таймеры и слушатели снимаются в `on_cleanup`.

### D5. Положение и z-index
Положение задаёт CSS темы (`position: fixed !important` для `.p-scrolltop`, `sticky !important` для `.p-scrolltop-sticky`), inline остаётся только `z-index: N`, где `N = zindex::generate(Overlay, config.z_index.base(Overlay))`; при выходе — `zindex::revert`. На SSR кнопка у `Window` не выводится, поэтому z-index не выдаётся.

### D6. Действие
Обработчик клика вызывает `Window::scroll_with_scroll_to_options` или `Element::scroll_with_scroll_to_options` родителя с `top = 0` и `behavior`.

### D7. CSS
`scrolltop` добавляется в `theme.components`; `cargo xtask gen-theme`, проверка, что CSS остальных не изменился. Если в CSS окажется блок `/* For PrimeNG */`, он подхватывается через `primeng_extra`.

### D8. Тесты
SSR: скрыт по умолчанию, классы по `target`. Браузерные `tests/web.rs`: порог (400/401), возврат, начальная проверка, `Parent`, клик, позиция, z-index и его возврат. e2e: настоящая прокрутка окна и вложенного контейнера, клик, клавиши `Enter`/`Space`. Крейт добавляется в три места `ci.yml`.

### D9. Бенчмарки
Сценарии `scrolltop-1` и `-1000` (в SSR `Window`-вариант скрыт, поэтому измеряется «пустой» экземпляр; для честности второй сценарий не нужен), `case-scrolltop` для wasm; сначала `cargo bench -p render-bench --bench alloc`.

## Risks / Trade-offs

- [`Parent` всегда в DOM, `Window` — только при показе] → единственный способ найти родителя до первого показа; фиксируется в docs.
- [Нет `@primeuix/motion`] → анимация классами и таймер по длительности перехода из темы (300 мс).
- [Касание и инерционная прокрутка на мобильных] → не проверялось, честно записано в docs.

## Migration Plan

Миграции нет. Откат — `git revert`.

## Open Questions

Нет.
