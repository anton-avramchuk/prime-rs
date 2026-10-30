# Design

## Context

Требования — в `specs/drawer`. Исходники PrimeNG 21.1.10: `drawer/drawer.ts`, `drawer/style/drawerstyle.ts`; демонстрации `apps/showcase/doc/drawer/*-doc.ts`. Образцы в проекте: `prime-popover` (`Core` с очищающими замыканиями, слушатели окна, `PopoverRef`), `prime-select` (`ZGroup`, `on_cleanup`), `prime-dom` (`Portal` и `FocusTrap` готовы, потребителей пока нет).

Наблюдения по исходнику:
- Состояние двухуровневое: `visible` (желаемое) и `modalVisible` (в DOM). Панель создаётся при `visible=true`, а убирается после `onAfterLeave` анимации выхода (`@primeuix/motion`).
- Маска создаётся императивно в `body` в `enableModality()` с `z-index = z панели − 1` (или `z верхней открытой панели − 1`); на закрытии получает `p-overlay-mask-leave-active` и убирается по `animationend`.
- Блок `For PrimeNG` в `drawerstyle.ts` помечен как `/** For PrimeNG **/` (две звёздочки), поэтому `xtask gen-theme` его не находит: нужна поддержка этого написания. В нём — позиции (`p-drawer-left` …), `position: fixed` и классы анимаций `p-drawer-enter-*` / `p-drawer-leave-*`, ссылающиеся на `@keyframes p-animate-drawer-*`; эти кадры в `base.css` проекта отсутствуют — проверить после генерации (см. Открытые вопросы).
- `Escape` закрывает панель через слушатель документа, только если её `z-index` равен верхнему (`ZIndexUtils.get`); обработчик `keydown` на самом корне вызывает `hide(false)`, который ничего не закрывает — этот остаток не переносится.
- `onHide` в PrimeNG вызывается только из `close()` (действия пользователя), при программном закрытии — нет.

## Goals / Non-Goals

**Goals:** тот же DOM, классы, закрытие и порядок слоёв; панель, маска, блокировка прокрутки и ловушка фокуса на готовых средствах `prime-dom`.

**Non-Goals:** `appendTo`, `@primeuix/motion`, `closeButtonProps` (см. proposal).

## Decisions

### D1. Крейт
`prime-drawer` с `Drawer`, `DrawerPosition`, `DrawerPt`. Зависит от `prime-core`, `prime-dom`, `prime-button`, `prime-icons` (`times`), `web-sys`.

### D2. Состояние: `visible` и `rendered`
`visible: RwSignal<bool>` — желаемое состояние (двусторонний сигнал или собственный). `rendered: RwSignal<bool>` — панель в DOM. `Effect`: `visible` стало истиной → `rendered=true`; стало ложным и `rendered` → запускается выход: корню меняется класс на `p-drawer-leave-*`, маске — на `p-overlay-mask-leave-active`, затем `rendered=false` после `animationend` корня либо через запасной таймер (`set_timeout` 700 мс — анимация длится 500 мс; нужен, если стили не подключены или включён `prefers-reduced-motion`), с проверкой `is_disposed()`.

### D3. Чистая логика (`state.rs`)
- `DrawerPosition` и `position_class(position, full_screen)` → `left|right|top|bottom|full`.
- `root_classes(position, full_screen, open) -> String`, `enter_class(..)`, `leave_class(..)`, `mask_classes(modal, leaving, full)`.
- `is_topmost(own_z, top_z) -> bool`.
- `mask_z(own_z) -> i32` (`own_z - 1`).
Табличные тесты на каждую.

### D4. Вывод в `body`
`Portal` из `prime-dom` выносит панель и маску в `body` одним блоком: сначала маска, затем панель (порядок для перекрытия по DOM; `z-index` задаётся явно). Маска — отдельный `div`, не потомок панели, чтобы клик по панели не доходил до неё. Вне браузера (SSR) `Portal` ничего не выводит.

### D5. Слои и сторона эффектов (только `wasm32`)
После вставки панели (`Effect`, следящий за `NodeRef` панели, как в Popover): `z-index` из `prime_core::zindex::generate(ZGroup::Modal, base)` (при `auto_z_index`), маске — `z-index − 1`; `FocusTrap::attach(&panel)`; `block_body_scroll()` при `block_scroll`; подписка `keydown` на `window` при `close_on_escape` (в следующем тике, как в Popover); `on_show`. При закрытии: слушатель, ловушка, `unblock_body_scroll()`, `revert(z)`, `on_hide`.

### D6. Закрытие
Единая `close()`: `visible.set(false)`. Источники: клик по маске при `dismissible` (клик по маске, а не по панели — маска отдельный элемент), кнопка закрытия, `Escape` у верхней панели (`is_topmost` по `zindex::current()`). `on_hide` вызывается при любом переходе `visible` в ложь (в PrimeNG — только при действиях пользователя; отличие записывается в docs).

### D7. Блокировка прокрутки
`prime_dom::dom::block_body_scroll()` / `unblock_body_scroll()` с общим счётчиком в потоке: класс `p-overflow-hidden` ставится при 0→1 и снимается при 1→0. Чистая часть — счётчик — тестируется отдельно; доступ к DOM — только в браузере. Ширина полосы прокрутки (`--p-scrollbar-width`) задаётся при блокировке, как в PrimeNG `blockBodyScroll`.

### D8. Кнопка закрытия
`Button` с `text`, `rounded`, `severity=Secondary` и `TimesIcon` (или `close_icon`); `aria_close_label` по умолчанию из локали (`aria.close`); `Enter` на кнопке закрывает (нативное поведение `button`).

### D9. CSS
`drawer` добавляется в `theme.components` и `primeng_extra`; `xtask gen-theme` расширяется на маркер `/** For PrimeNG **/` (тест рядом с существующими). Проверка: CSS остальных компонентов не изменился.

### D10. Тесты
SSR: скрыт по умолчанию, свойства не просачиваются. Браузерные `tests/web.rs` (один последовательный тест-прогон, как у Popover — слушатели окна общие): показ, закрытие кнопкой/маской/Escape, `dismissible=false`, `modal=false`, позиции, классы анимации, удаление из DOM после выхода, `block_scroll`, Tab-цикл, две панели (z-index, Escape закрывает верхнюю), `on_show`/`on_hide`, снятие слушателей. e2e: настоящая мышь и клавиатура, прокрутка заблокирована. Крейт добавляется в три места `ci.yml`.

### D11. Бенчмарки
Сценарии `drawer-1`/`-1000` (скрытая панель) и `case-drawer`; сначала `cargo bench -p render-bench --bench alloc`, для wasm — временный большой бюджет (заменять по уникальной строке, не глобальным `sed`).

## Risks / Trade-offs

- [Запасной таймер выхода] → если `animationend` не придёт и таймер короче анимации, панель исчезнет раньше времени; таймер 700 мс при анимации 500 мс.
- [Портал и гидратация] → панель в `body` не участвует в гидратации; скрытая по умолчанию панель SSR не отдаёт ничего, а открытая до гидратации — не поддерживается и фиксируется в docs.
- [Нет `@primeuix/motion`] → анимации только CSS-классами; `motionOptions` не переносятся.
- [`on_hide` при программном закрытии] → отличие от PrimeNG, записывается в docs.

## Open Questions

- Есть ли `@keyframes p-animate-drawer-*` в `@primeuix/styles/drawer` после `gen-theme`. Если нет — их нужно взять из исходников PrimeNG (поиск по `packages/primeng` и `@primeuix/*`); если их нет нигде, записать собственные кадры в `xtask` (сдвиг на ширину/высоту панели) и отметить в docs. Решается в первой задаче темы, до остальной работы со стилями.

## Migration Plan

Миграции нет. Откат — `git revert`.
