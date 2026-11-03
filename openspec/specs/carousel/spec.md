# carousel Specification

## Purpose
Carousel показывает ряд элементов страницами по `num_visible` штук и листает их кнопками, точками, автопрокруткой и свайпом.

## Requirements

### Requirement: Разметка
`Carousel` крейта `prime-carousel` SHALL выводить `div.p-carousel.p-component.p-carousel-horizontal|vertical[role=region]` (`id`, `style`, `style_class`, `pt`) с содержимым: `div.p-carousel-header` (если задан `header`), `div.p-carousel-content-container` (+ `content_class`) > `div.p-carousel-content` (`aria-live="polite"`, пока автопрокрутка разрешена, иначе `off`) с кнопкой `p-carousel-prev-button`, окном `div.p-carousel-viewport` (при вертикальной ориентации высота `vertical_view_port_height`, иначе `auto`) и кнопкой `p-carousel-next-button` (обе — при `show_navigators`; текстовые круглые, `p-disabled`, когда листать нельзя, `aria-label` из переводов `aria.prevPageLabel`/`aria.nextPageLabel`; значки — шевроны по ориентации или `previous_icon`/`next_icon`). В окне SHALL лежать `div.p-carousel-item-list` с элементами `div.p-carousel-item[role=group]` (`p-carousel-item-active|start|end`, `data-p-carousel-item-active|start|end`, `aria-hidden`, `aria-label` из `aria.slideNumber`, `aria-roledescription` из `aria.slide`), а в бесконечном режиме — ещё клоны `p-carousel-item-clone` до и после. При `show_indicators` SHALL выводиться `ul.p-carousel-indicator-list` (+ `indicators_content_class`, `indicators_content_style`) с `li.p-carousel-indicator` (`-active`) и `button.p-carousel-indicator-button` (`aria-label` из `aria.pageLabel`, `aria-current="page"` у активной, `tabindex`), затем `div.p-carousel-footer` (если задан `footer`).

#### Scenario: Состав
- **WHEN** отрисован `Carousel` из пяти элементов, `num_visible = 2`, `num_scroll = 1`
- **THEN** выведены пять элементов, четыре точки, обе кнопки, у корня `p-carousel-horizontal`

#### Scenario: Без навигаторов и точек
- **WHEN** `show_navigators = false`, `show_indicators = false`
- **THEN** кнопок и списка точек нет

### Requirement: Страницы и смещение
Число страниц SHALL быть `⌈(n − num_visible)/num_scroll⌉ + 1` (0 при пустом значении). Кнопка «вперёд» SHALL быть отключена на последней странице (если не бесконечный режим) и при пустом значении, «назад» — на нулевой. Листание на страницу SHALL сдвигать список на `−num_scroll·страница` элементов (`translate3d` по оси ориентации в процентах `100/num_visible` на элемент) с переходом `transform 500ms ease`. Если элементы делятся на страницы с остатком, последняя страница SHALL показывать последние `num_visible` элементов. Активными SHALL быть элементы от `first_index` до `first_index + num_visible − 1`. При уменьшении числа страниц страница SHALL зажиматься на последней с событием `on_page`.

#### Scenario: Остаток
- **WHEN** девять элементов, `num_visible = 3`, `num_scroll = 2`, пользователь листает вперёд до конца
- **THEN** на последней (четвёртой) странице видны элементы 7–9

#### Scenario: Границы
- **WHEN** открыта последняя страница без бесконечного режима
- **THEN** кнопка «вперёд» имеет `p-disabled`

### Requirement: Бесконечный режим
При `circular` и `n ≥ num_visible` SHALL выводиться по `num_visible` клонов начала и конца. Листание вперёд с последней страницы SHALL уходить на клоны и после окончания перехода (`transitionend`) возвращаться без анимации на начало списка, на нулевой странице «назад» — наоборот; кнопки при этом не отключаются. При `n < num_visible` бесконечный режим SHALL не включаться.

#### Scenario: Переход через границу
- **WHEN** открыта последняя страница бесконечного режима и нажата «вперёд»
- **THEN** страница становится нулевой, а после окончания перехода список без анимации встаёт на начало

### Requirement: Автопрокрутка
При `autoplay_interval > 0` SHALL запускаться таймер: каждые `autoplay_interval` мс SHALL открываться следующая страница, а с последней — нулевая. Любое ручное листание (кнопка, точка, внешнее изменение `page`) SHALL останавливать автопрокрутку навсегда для этого экземпляра. Таймер SHALL сниматься при размонтировании.

#### Scenario: Остановка
- **WHEN** идёт автопрокрутка и пользователь нажимает «вперёд»
- **THEN** автопрокрутка останавливается, `aria-live` становится `off`

### Requirement: Адаптивность
При `responsive_options` SHALL при монтировании и при `resize` окна выбираться последний вариант с `breakpoint ≥ ширина окна` (иначе значения по умолчанию), страница пересчитываться по формуле `⌊страница·старый_шаг/новый_шаг⌋`, клоны перестраиваться, `on_page` вызываться при смене страницы. Ширина каждого элемента SHALL задаваться инлайн-стилем `flex: 1 0 <100/num_visible>%`.

#### Scenario: Узкое окно
- **WHEN** окно уже `breakpoint` варианта с `num_visible = 1`
- **THEN** элементы занимают всю ширину окна, `num_visible` равен 1

### Requirement: Точки, клавиатура и свайп
Клик по точке SHALL открывать её страницу. У точки с фокусом `tabindex` SHALL быть `0`, у остальных `-1`; ←/→ на списке точек SHALL переносить фокус на соседнюю точку, не выходя за границы. Свайп по окну (touch-события) на расстояние больше 20 px SHALL листать вперёд (влево/вверх) или назад (вправо/вниз); `touchmove` SHALL не прокручивать страницу.

#### Scenario: Точка
- **WHEN** нажата третья точка
- **THEN** открыта третья страница, `on_page(2)`

#### Scenario: Свайп
- **WHEN** пользователь свайпает влево на 50 px
- **THEN** открывается следующая страница
