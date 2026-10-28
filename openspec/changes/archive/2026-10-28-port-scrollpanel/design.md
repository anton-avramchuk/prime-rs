# Design

## Context

Требования — в `specs/scrollpanel`. Исходники PrimeNG 21.1.10: `packages/primeng/src/scrollpanel/scrollpanel.ts`, `scrollpanel/style/scrollpanelstyle.ts`, демонстрации `apps/showcase/doc/scrollpanel` (`basic`, `custom`). CSS `scrollpanel` добавится через `cargo xtask gen-theme` (`upstream.toml`); блок `/* For PrimeNG */` не нужен, но `.p-scrollpanel { display: block }` из `scrollpanelstyle.ts` стоит проверить: если отсутствует в сгенерированном CSS, добавить компонент в `primeng_extra`. Шаблон крейта — `prime-toolbar` (две ветви, `macro_rules!`, `Props`), клиентская часть — по образцу `prime-core::collapsible` (код под `cfg(target_arch = "wasm32")`, SSR-путь пустой).

## Decisions

### D1. Разметка и ветви
Статическая ветвь — когда `style` и `style_class` статические; иначе реактивная. Разметка — `macro_rules!`, выражения атрибутов корня меняются. `id` содержимого — `pn_id_{n}_content` из `CollapseIds`-подобного счётчика `prime-core` (тот же `next_unique`, детерминирован в SSR/гидрации); при необходимости открыть в `collapsible` небольшую публичную функцию `unique_id(suffix)`, а не дублировать счётчик.

### D2. Состояние — в обработчиках, не в сигналах
`lastScrollLeft/Top`, `orientation`, `scrollXRatio/YRatio`, `lastPageX/Y`, флаги перетаскивания — изменяемое состояние экземпляра. Хранится в `Rc<RefCell<State>>`/`StoredValue`, не в сигналах: пересчёт не должен вызывать перерисовку Leptos. Исключение — `aria-valuenow`: PrimeNG обновляет его через change detection; мы пишем атрибут напрямую в DOM (`set_attribute`) из обработчика `scroll`, поэтому реактивность не нужна.

### D3. Стили полос — прямая запись в DOM
`moveBar` измеряет контент и пишет `style.cssText` и `data-p-scrollpanel-hidden`/класс `p-scrollpanel-hidden` на элементы полос через `web-sys`, внутри `requestAnimationFrame` (как в оригинале). Атрибут `style` полос не отдаётся Leptos (иначе реактивная перезапись затрёт расчёт). Для `pt.bar_*.style` Leptos не пишет `style` на эти элементы; такой `pt`-стиль применяется к полосе один раз при монтировании через `cssText` до первого `moveBar` (и затирается им — отличие, фиксируется в docs).

### D4. Слушатели и очистка
Клиентский `Effect`/`on_mount` после монтирования: `scroll` и `mouseenter` на содержимом, `resize` на окне, `mousedown` на обеих полосах (через `on:` Leptos для полос и содержимого; `resize` и документные — вручную через `Closure`). Документные `mousemove`/`mouseup` вешаются на `mousedown` и снимаются на `mouseup` и в `on_cleanup`. Замыкания хранятся в состоянии для снятия. Используется `window.request_animation_frame`; если недоступен — `setTimeout(0)` (запасной вариант как `timeoutFrame`).

### D5. Клавиатура
`keydown`/`keyup`/`focus`/`blur` на полосах — `on:` Leptos. Таймер 40 мс — `set_timeout`, предыдущий очищается. `event.code` читается как в оригинале. Различие: `orientation` хранится в `State`.

### D6. `calculateContainerHeight`
Выполняется один раз после монтирования, только в браузере; `getHeight` PrimeUI заменяется `offsetHeight` минус паддинги и рамки (`getComputedStyle`). Результат записывается в `style.height` корня; поскольку Leptos может управлять `style` корня, запись идёт только если `style`/`pt.root` не задали `height`, иначе Leptos перезапишет при реактивном обновлении — случай реактивного `style` вместе с `max-height` документируется как ограничение.

### D7. Публичные методы
`ScrollPanelRef` — клонируемая ручка (`StoredValue<Option<…>>` с `NodeRef` содержимого и замыканием `refresh`). `refresh()` вызывает `moveBar`; `scroll_top` клампит значение. Вне wasm — no-op.

### D8. Слот содержимого
`content_template: Option<ViewFn>`; при наличии выводится вместо `children`. Для `ng-content` + `ngTemplateOutlet` в оригинале выводятся оба; у нас — взаимоисключающе (как `@if` в шаблоне оригинала).

### D9. Тесты
SSR-тесты — структура, ARIA, `pt`, паритет ветвей. Браузерные (`tests/web.rs`, `wasm-bindgen-test`) — скрытие полосы, формулы размера/позиции, перетаскивание (синтетические `MouseEvent`), стрелки, `scroll_top` с клампом, снятие слушателей. Чистая арифметика (`bar_size`, `bar_offset`) выносится в обычные функции и проверяется нативными юнит-тестами.

## Risks / Trade-offs

- [Запись в `style` полос вне Leptos] → полосы без реактивного `style`; сценарий с `pt.style` описан в docs.
- [Тесты зависят от геометрии вёрстки] → браузерный тест задаёт фиксированные размеры инлайном и проверяет относительные величины.
- [`mousemove` документа и утечки замыканий] → снятие в `mouseup` и `on_cleanup`; тест на размонтирование.
- [Возможный `display: block` из `scrollpanelstyle.ts`] → проверить после `gen-theme`.

## Migration Plan

Миграции нет: новый крейт. Откат — `git revert`.

## Open Questions

Нет.
