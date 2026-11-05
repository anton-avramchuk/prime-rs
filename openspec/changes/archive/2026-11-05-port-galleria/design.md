# Design

## Context

Требования — в `specs/galleria`. Исходники PrimeNG 21.1.10: `galleria/galleria.ts` (`Galleria`, `GalleriaContent`, `GalleriaItem`, `GalleriaThumbnails`), `style/galleriastyle.ts`, `@primeuix/styles/galleria`. Образцы в проекте: `prime-drawer` (`Portal`, маска, `rendered`/`leaving`, `FocusTrap`, `block_body_scroll`), `prime-carousel` (сдвиг ленты, стрелки), `prime-slider` (слушатели).

## Goals / Non-Goals

**Goals:** тот же DOM и классы, те же формулы окна миниатюр.

**Non-Goals:** системный полный экран, `Bind`, формы Angular, настройка анимации через опции.

## Decisions

### D1. Чистая модель (`model.rs`)
Функции без DOM: `median_index(n)`, `total_pages(len, n)`, `shifted_items(len, n, active) -> i32` (формулы `onAfterContentChecked`), `nav_backward_disabled`/`nav_forward_disabled`, `first_active`/`last_active`/`is_item_active`, `next_index`/`prev_index` (с `circular`), `responsive_num_visible(options, width, base)`, `position_class(prefix, pos)`, `thumbnail_item_classes`. В PrimeNG `step()` перед сменой активного индекса всегда перекрывается пересчётом из индекса, поэтому смещение у нас — функция `(active, num_visible)`; отдельного состояния нет.

### D2. Данные и шаблоны
`GalleriaImage { item_image_src, thumbnail_image_src, alt, title }`; свои шаблоны — `item: Callback<GalleriaImage, AnyView>`, `thumbnail`, `caption` (`Callback`), `indicator: Callback<usize, AnyView>`, `header`/`footer` — `ViewFn`. Все хранятся в `Arc<Parts>` (дети `Portal` — `Fn + Send + Sync`).

### D3. Состояние
`active: RwSignal<usize>` (внешний или внутренний), `slide_show_active: RwSignal<bool>`, `num_visible_now: RwSignal<usize>` (адаптивный), `items_hidden: RwSignal<bool>` (класс `p-items-hidden` на ленте: снимается при смене индекса, ставится по `transitionend`). Смещение — реактивный `translate3d(…)` и `transition: transform 500ms ease 0s` на ленте.

### D4. Автосмена
`set_interval` под `cfg(wasm32)` внутри `Effect`, привязанного к `auto_play` и `slide_show_active`; снятие в `on_cleanup`; остановка — `stop_slide_show`.

### D5. Полный экран
Как у Drawer: `visible` + `rendered` + `leaving`; маска и содержимое — в `Portal`; корень с `p-galleria-fullscreen`; `animationend` корня (`p-galleria-leave-active`) или запасной таймер 700 мс завершает выход. Клик по маске закрывает только при `target == currentTarget`. Слой — `ZGroup::Modal`.

### D6. Адаптивность
`responsive_num_visible` вызывается при создании и на `resize` окна (`window_event_listener`) только при заданных `responsive_options`; `flex: 1 0 (100/n)%` для миниатюр — встроенным стилем (вместо динамического `<style>` PrimeNG).

### D7. Тесты
Модель — таблицы. SSR: разметка основ, индикаторов, подписи, позиций, `full_screen` закрыт. Браузерные `tests/web.rs` одним последовательным тестом: навигация, `circular`, индикаторы, клавиши миниатюр, автосмена, полный экран (открытие/закрытие/клик по маске). e2e: реальные клики и стили.

## Risks / Trade-offs

- [Касание] миниатюры листаются касанием — только эмуляция.
- [Анимация выхода] `animationend` + запасной таймер, как у Drawer.

## Open Questions

- Нет.

## Migration Plan

Миграции нет. Откат — `git revert`.
