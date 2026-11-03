# Proposal

## Why

Carousel — девятый компонент волны 4: прокручиваемый ряд элементов с кнопками «назад/вперёд», точками-индикаторами, автопрокруткой, бесконечным режимом и адаптацией по ширине окна. Это самый «арифметический» компонент волны: основная ценность порта — точная копия логики страниц, смещений и клонов, поэтому она выносится в чистую модель с табличными тестами.

## What Changes

- Новый крейт `prime-carousel` (порт `p-carousel` из PrimeNG 21.1.10).
- `Carousel<T>`: корень `div.p-carousel.p-component.p-carousel-horizontal|vertical[role=region]` (`id`) > необязательный `div.p-carousel-header` > `div.p-carousel-content-container` (+ `content_class`) > `div.p-carousel-content[aria-live]` с кнопками `p-carousel-prev-button` / `p-carousel-next-button` (текстовые, круглые, `p-disabled`, `aria-label` из переводов) и окном `div.p-carousel-viewport` с `div.p-carousel-item-list` (`transform: translate3d(...)`, `transition`, класс `p-items-hidden` после перехода) > элементы `div.p-carousel-item[role=group]` (`p-carousel-item-active|start|end`, `aria-hidden`, `aria-label`, `aria-roledescription`, `data-p-carousel-item-*`) и клоны `p-carousel-item-clone` (при `circular`) > `ul.p-carousel-indicator-list` с `li.p-carousel-indicator` (`-active`) и кнопками `p-carousel-indicator-button` (`aria-label`, `aria-current`, `tabindex`) > необязательный `div.p-carousel-footer`.
- Модель страниц (чистая логика, как в PrimeNG): `num_visible`, `num_scroll`, число точек `⌈(n − visible)/scroll⌉ + 1`, остаток `remaining_items`, смещение `total_shifted_items`, переходы вперёд/назад/на страницу, бесконечный режим с клонами (`circular` действует, пока элементов не меньше `num_visible`), выход за границы при уменьшении числа страниц.
- Адаптивность: `responsive_options` (`breakpoint`, `num_visible`, `num_scroll`) пересчитываются при `resize` окна; ширина элемента задаётся инлайн-стилем `flex: 1 0 <100/visible>%`.
- Поведение: кнопки и точки переключают страницы (`on_page`), `page` — двусторонний сигнал; автопрокрутка по `autoplay_interval` до первого ручного действия; свайп (порог 20 px) по touch-событиям; клавиши ←/→ на индикаторах перемещают фокус между точками (roving `tabindex`).
- Свойства: `value`, `page`, `num_visible`, `num_scroll`, `responsive_options`, `orientation`, `vertical_view_port_height` (`300px`), `content_class`, `indicators_content_class`, `indicators_content_style`, `indicator_style_class`, `indicator_style`, `circular`, `show_indicators`, `show_navigators`, `autoplay_interval`, `id`, `style`, `style_class`, `prev_button_severity`/`next_button_severity`, `pt`; представления `item` (обязательное), `header`, `footer`, `previous_icon`, `next_icon`; событие `on_page`.
- Страница showcase `/carousel` с демонстрациями `basic`, `responsive`, `circular` (автопрокрутка), `vertical` и товарами-карточками (локальные данные, без CDN); e2e; бенчмарки и бюджеты; `docs/carousel.md`.
- Стили: `carousel` добавляется в `theme.components`.
- **Не входит:** `Header`/`Footer` как фасеты (`p-header`/`p-footer`) — только представления `header`/`footer`, `Bind`, `pTemplate`, `unstyled`, `ButtonProps` целиком (оставлена серьёзность), динамический `<style>` с медиа-запросами (адаптивность пересчитывается на `resize`).

## Capabilities

### New Capabilities
- `carousel`: прокручиваемый ряд элементов со страницами, индикаторами, автопрокруткой и бесконечным режимом.

### Modified Capabilities
<!-- Нет. -->

## Impact

- Новый крейт `crates/prime-carousel` (`publish = false`); зависимости: `prime-core`, `prime-button`, `prime-icons` (chevronleft/right/up/down), `web-sys`.
- Изменяются `Cargo.toml` workspace, `upstream.toml`, `showcase`, `benches/*`, `e2e/tests`, `docs/`, `ci.yml` (браузерный job, три места: `prime-carousel`).
