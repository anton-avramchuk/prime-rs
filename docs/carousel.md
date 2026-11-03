# Carousel

Порт `p-carousel` из PrimeNG 21.1.10 (коммит `fe82b291d6`, см. `upstream.toml`).

| Наш компонент | Исходники PrimeNG |
|---|---|
| `prime-carousel::Carousel<T>`, `CarouselPt`, `CarouselOrientation`, `Responsive`, чистая модель `Track` | `packages/primeng/src/carousel/carousel.ts`, `carousel/style/carouselstyle.ts` |
| CSS | `@primeuix/styles/carousel` и пресет Aura — `cargo xtask gen-theme` (блока `/* For PrimeNG */` нет); кнопки берут стили `button` |
| Демонстрации | `apps/showcase/doc/carousel/*-doc.ts` (`basic`, `responsive`, `circular`, `vertical`) |

Спека: `openspec/specs/carousel`.

```rust
view! {
    <Carousel value=products num_visible=3 num_scroll=1 circular=true autoplay_interval=3000
        item=Callback::new(|p: Product| view! { <ProductCard product=p /> }.into_any()) />
}
```

## Что совпадает с PrimeNG

- Разметка и классы: `div.p-carousel.p-component.p-carousel-horizontal|vertical[role=region]` > `p-carousel-header` (если есть), `p-carousel-content-container` > `p-carousel-content[aria-live]` с кнопками `p-carousel-prev-button`/`-next-button` (`p-disabled`) и `p-carousel-viewport` > `p-carousel-item-list` (`p-items-hidden`) > `p-carousel-item[role=group]` (`p-carousel-item-active|start|end`, `data-p-carousel-item-*`, `aria-hidden`, `aria-label`, `aria-roledescription`) и клоны `p-carousel-item-clone`; `ul.p-carousel-indicator-list` > `li.p-carousel-indicator` (`-active`) > `button.p-carousel-indicator-button` (`aria-current`, `tabindex`); `p-carousel-footer`.
- Арифметика страниц — точная копия исходника, вынесена в чистую модель `Track` и покрыта таблицами: число страниц `⌈(n − видимых)/шаг⌉ + 1`, остаток на последней странице (последняя страница показывает последние `num_visible` элементов), смещение `translate3d` в процентах на элемент, переходы вперёд/назад/на страницу, бесконечный режим (клоны, переход через границу по клонам и скачок без анимации после `transitionend`), зажатие страницы при смене значения.
- Автопрокрутка (`autoplay_interval`): следующая страница по таймеру, с последней — на нулевую; ручное листание останавливает её навсегда (`aria-live` меняется на `off`).
- Адаптивность: последний вариант с `breakpoint ≥ ширина окна`, пересчёт страницы по формуле PrimeNG, перестройка клонов, события `on_page`; свайп с порогом 20 px; клавиши ←/→ на индикаторах переносят фокус; вертикальная ориентация с `vertical_view_port_height`.

## Отличия

| Отличие | Причина |
|---|---|
| Ширина элемента задана инлайн-стилем `flex: 1 0 <100/visible>%` и пересчитывается на `resize`, а не динамическим `<style>` с медиа-запросами | Нет `Renderer2`; на сервере действуют значения по умолчанию, после гидратации вёрстка пересчитывается (в PrimeNG медиа-запросы работают уже до гидратации). |
| Нет фасетов `p-header`/`p-footer`: шапка и подвал — представления `header`/`footer` | Нет проекции содержимого Angular. |
| Вместо `prevButtonProps`/`nextButtonProps` — `prev_button_severity`/`next_button_severity` (кнопки всегда текстовые и круглые) | Единственное свойство, которое меняют демонстрации. |
| `page` — двусторонний `RwSignal<usize>` вместо пары `page`/`onPage`; внешнее значение за пределами страниц отбрасывается | Идиома Leptos; в PrimeNG такое значение оставляло компонент в несогласованном состоянии. |
| Нет `Bind`, `pTemplate`, `unstyled`; значки — `ViewFn` | Не переносятся в проекте. |
| `onTabKey`, `Home`/`End` для индикаторов не подключены | В PrimeNG они не вызываются из обработчика клавиш. |
| В демонстрациях фотографии товаров заменены цветными плитками, значки PrimeIcons — значками `prime-icons` | Независимость от сети, нет PrimeIcons. |
| `circular` действует, только пока элементов не меньше числа видимых | Как в PrimeNG (`isCircular`); для коротких списков бесконечный режим не включается. |

## Что не проверялось

- Раскладка справа налево.
- Реальные касания на устройстве: свайп проверен синтетическими `touch`-событиями.
- Чтение с экрана: `aria-*` сверены с PrimeNG, поведение скринридеров — нет.

## Производительность

См. `benches/` (`carousel-1`, `carousel-1000`, `case-carousel`) и `benches/*budgets.toml`.
