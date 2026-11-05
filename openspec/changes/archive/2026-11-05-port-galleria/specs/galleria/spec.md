# galleria Specification

## Purpose
Galleria — галерея изображений с миниатюрами, индикаторами, автосменой и полноэкранным режимом.

## ADDED Requirements

### Requirement: Окно миниатюр
Модель SHALL вычислять смещение ленты миниатюр по активному индексу по формулам PrimeNG: медианный индекс `floor(n/2)` (для чётного `n` — на единицу меньше), смещение `0`, пока `active <= median`; `n_visible − len` у конца; иначе `−active + median` (`+1` для чётного `n` у конца списка). Кнопки страниц SHALL быть недоступны, если миниатюр не больше видимых либо (без `circular`) активна крайняя.

#### Scenario: Начало
- **WHEN** `len = 6`, `n_visible = 3`, `active = 1`
- **THEN** смещение `0`

#### Scenario: Середина
- **WHEN** `len = 6`, `n_visible = 3`, `active = 3`
- **THEN** смещение `−2`

#### Scenario: Конец
- **WHEN** `len = 6`, `n_visible = 3`, `active = 5`
- **THEN** смещение `−3`

### Requirement: Разметка
`Galleria` SHALL выводить `div.p-galleria.p-component` (`role=region`, классы позиций `p-galleria-thumbnails-<pos>` и `p-galleria-indicators-<pos>` при показе, `p-galleria-inset-indicators`, `p-galleria-hover-navigators`, `p-galleria-fullscreen`) > `.p-galleria-content` > `.p-galleria-items-container` (`.p-galleria-items` > кнопки навигации при `show_item_navigators`, `.p-galleria-item` с `role=group`, `aria-label` «N», `.p-galleria-caption` при шаблоне подписи; `ul.p-galleria-indicator-list` при `show_indicators`) и `.p-galleria-thumbnails` при `show_thumbnails`. При пустом `value` содержимое SHALL отсутствовать.

#### Scenario: Основа
- **WHEN** три изображения, значения по умолчанию
- **THEN** есть `.p-galleria-item` с первым кадром, три `.p-galleria-thumbnail-item`, нет индикаторов и кнопок кадра

#### Scenario: Индикаторы
- **WHEN** `show_indicators = true`
- **THEN** три `li.p-galleria-indicator`, первый с `p-galleria-indicator-active`

### Requirement: Навигация по кадрам
Кнопки кадра, индикаторы и миниатюры SHALL менять активный индекс и вызывать обновление `active_index`. Следующий кадр: `circular` и последний → `0`, иначе `+1`; предыдущий: `circular` и первый → последний, иначе `−1` (не ниже 0). Кнопка недоступна (`p-disabled`) на краю без `circular`. Индикатор: клик, `Enter`/`Space`; при `change_item_on_indicator_hover` — наведение. Миниатюра: клик, `Enter`/`Space`; `ArrowLeft`/`ArrowRight`/`Home`/`End` двигают фокус по миниатюрам.

#### Scenario: Кнопка вперёд
- **WHEN** `show_item_navigators`, активен 0, клик по следующей
- **THEN** активный индекс `1`, показан второй кадр

#### Scenario: Круг
- **WHEN** `circular`, активен последний, клик по следующей
- **THEN** активный индекс `0`

### Requirement: Автосмена
При `auto_play` кадр SHALL меняться каждые `transition_interval` мс (после последнего — на первый при `circular`, иначе дальше не двигаться); действие пользователя (кнопки, индикаторы, миниатюры) SHALL останавливать смену, если `should_stop_autoplay_by_click`.

#### Scenario: Остановка
- **WHEN** идёт автосмена, пользователь кликает по миниатюре
- **THEN** автосмена остановлена

### Requirement: Адаптивность
`responsive_options` (`breakpoint` в пикселях, `num_visible`) SHALL выбирать число видимых миниатюр по ширине окна: наименьший `breakpoint`, не меньший ширины окна; иначе `num_visible` компонента. Число видимых SHALL не превышать число изображений.

#### Scenario: Узкое окно
- **WHEN** `[{960, 4}, {640, 2}]`, ширина 600
- **THEN** видимых `2`

### Requirement: Полноэкранный режим
При `full_screen` галерея SHALL выводиться в `Portal` внутри маски `p-galleria-mask p-overlay-mask` при `visible`, с кнопкой закрытия `p-galleria-close-button`, блокировкой прокрутки страницы и ловушкой фокуса; клик по самой маске и кнопка закрытия SHALL сбрасывать `visible`; выход SHALL играть анимацию `p-galleria-leave-active` до удаления из DOM.

#### Scenario: Открытие и закрытие
- **WHEN** `visible = true`, затем клик по кнопке закрытия
- **THEN** сначала маска и содержимое в DOM, затем `visible = false` и удаление после анимации

#### Scenario: Клик внутри
- **WHEN** клик по кадру внутри маски
- **THEN** галерея остаётся открытой
