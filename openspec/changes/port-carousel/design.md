# Design

## Context

Требования — в `specs/carousel`. Исходники PrimeNG 21.1.10: `carousel/carousel.ts` (около 1000 строк, из них логики около 550), `carousel/style/carouselstyle.ts`; стили `@primeuix/styles/carousel`; демонстрации `apps/showcase/doc/carousel/{basic,responsive,circular,vertical}-doc.ts`. Образцы в проекте: `prime-timeline` (обобщённый `Vec<T>` и слот-шаблон), `prime-paginator` (страницы и границы), `prime-splitbutton`/`prime-speeddial` (кнопки), `prime-dock` (`matchMedia`/окно).

Наблюдения по исходнику (точная арифметика):
- Состояние: `_page`, `_numVisible`, `_numScroll`, `totalShiftedItems`, `remainingItems = (len − numVisible) % numScroll`, `isRemainingItemsAdded`. Точек `totalDots = ceil((len − numVisible)/numScroll) + 1` (0 при пустом списке).
- Пересчёт (`onAfterContentChecked` при смене `numScroll`/`numVisible`/длины значения): остаток; если страница вышла за последнюю — зажимается; `totalShifted = −page·numScroll` (и `−numVisible` в бесконечном режиме); на последней странице с остатком смещение корректируется на `−remaining + numScroll`.
- `step(dir, page?)`: при явной странице `totalShifted = −numScroll·page (− numVisible)`; иначе сдвиг на `numScroll·dir` (с учётом уже добавленного остатка), страница из смещения. Особые случаи: бесконечный режим на последней странице вперёд → смещение `−(len + numVisible)` и страница 0 (переход на клон); на нулевой назад → смещение 0 и последняя страница; последняя страница с остатком → доворот на остаток. Затем `transform`/`transition: transform 500ms ease 0s`, класс `p-items-hidden` снимается.
- `onTransitionEnd`: `transition` сбрасывается, добавляется `p-items-hidden`; в бесконечном режиме на крайних страницах `transform` пересчитывается по логическому смещению без анимации (скачок с клона на настоящий элемент). После каждого шага (`afterContentChecked`) логическое смещение приводится: страница 0 → `−numVisible`; смещение 0 → `−len`.
- Автопрокрутка: `setInterval(autoplayInterval)`: к следующей странице, с последней — на нулевую; любое ручное действие останавливает (`allowAutoplay = false`); `aria-live` = `polite`, пока автопрокрутка разрешена.
- Адаптивность: `calculatePosition` выбирает последний вариант с `breakpoint ≥ ширина окна`, пересчитывает страницу (`floor(page·oldScroll/newScroll)`), смещение и `numVisible`, перестраивает клоны; слушатель `resize`. Ширину элемента задаёт сгенерированный `<style>` (`flex: 1 0 100/visible %`) с медиа-запросами.
- Индикаторы: `tabindex` 0 у активной страницы, ←/→ перемещают фокус между точками.
- Свайп: `touchstart` запоминает точку, `touchend` при `|diff| > 20` листает; `touchmove` гасит прокрутку страницы.

## Goals / Non-Goals

**Goals:** одинаковые классы, роли, `aria-*`, арифметика страниц и клонов, автопрокрутка, адаптивность, свайп.

**Non-Goals:** фасеты `p-header`/`p-footer`, `Bind`, `pTemplate`, `unstyled`, `ButtonProps`, динамический `<style>`.

## Decisions

### D1. Чистая модель (`model.rs`)
`Track { page, num_visible, num_scroll, total_shifted, remaining, remaining_added }` с чистыми методами: `total_dots(len)`, `is_circular(circular, len)`, `recalc(len, circular) -> Track` (блок `afterContentChecked`), `step(len, circular, dir, page) -> Step { track, applied_shift }` (`applied_shift` — смещение для `transform` сразу после шага, до приведения), `settle(len, circular)` (приведение после шага), `after_transition_shift(circular)`, `first_index`, `last_index`, `nav_disabled(forward/backward)`, `responsive(options, window_width, defaults) -> Matched`, `swipe(diff)`, `autoplay_next(page, dots)`, `translate(shift, num_visible, vertical) -> String`. Тесты: таблицы для обычного, бесконечного, с остатком, адаптивного режимов (несколько длин и шагов), оба края, смена числа элементов.

### D2. Состояние компонента
`RwSignal<Track>`, `applied_shift: RwSignal<i32>`, `transitioning: RwSignal<bool>` (для `transition`), `items_hidden: RwSignal<bool>` (класс `p-items-hidden`; сначала `false`). Страница наружу — `page: Option<RwSignal<usize>>`: внешнее изменение (отличное от внутренней страницы) запускает `step` на страницу (как сеттер `page` в PrimeNG: остановка автопрокрутки; вперёд — `step(-1, val)`, назад — `step(1, val)`).

### D3. Обобщённый компонент
`Carousel<T>` с `T: Clone + Send + Sync + 'static`, `value: Prop<Vec<T>>`, `item: Callback<T, AnyView>` (обязательное). Элементы строятся один раз на смену `value`; позиции, классы и `aria-*` — реактивные атрибуты. Клоны (`circular`) — копии крайних `num_visible` элементов; рисуются тем же `item`.

### D4. Адаптивность
`responsive_options: Vec<CarouselResponsive>`; ширина окна читается при монтировании и на `resize` (только браузер); результат `Track.responsive` обновляет `num_visible`/`num_scroll` и перестраивает клоны. Ширина элемента — инлайн `flex: 1 0 <100/num_visible>%` у каждого `div.p-carousel-item`. Серверный вывод использует значения по умолчанию, после гидратации страница пересчитывается — известное отличие от PrimeNG (там медиа-запросы работают уже на сервере).

### D5. Автопрокрутка и свайп
`set_interval` (только браузер) через `StoredValue<Option<IntervalHandle>>`, остановка при ручном действии и при размонтировании. Свайп — обработчики `touchstart`/`touchend`/`touchmove` на окне; логика порога — чистая функция.

### D6. Индикаторы и фокус
`tabindex` — реактивно: `0` у точки с `focus_dot` (по умолчанию активная страница), остальные `-1`; ←/→ меняют `focus_dot` и фокусируют кнопку; смена страницы возвращает `focus_dot` на неё.

### D7. Кнопки навигации
`Button` из `prime-button` (`text`, `rounded`, `severity = Secondary` по умолчанию, `on_click`), `style_class` реактивный (`p-carousel-prev-button`, `p-disabled`); значок — `previous_icon`/`next_icon` или шевроны по ориентации (влево/вверх, вправо/вниз). `aria-label` — из переводов `aria.prevPageLabel`/`aria.nextPageLabel`.

### D8. Переводы
`aria.slide`, `aria.slideNumber` (с `{slideNumber}`), `aria.pageLabel` (с `{page}`) уже есть в `translation_defaults.rs`; подстановка — чистая функция.

### D9. Тесты
Модель: табличные тесты (`model.rs`). SSR: разметка, роли, `aria-*`, клоны, индикаторы, отсутствие навигаторов/индикаторов по флагам, `pt`, ориентация, шаблоны. Браузерные `tests/web.rs` (один последовательный тест из-за таймеров и `resize`): кнопки и точки, границы, `p-disabled`, `on_page`, внешний `page`, бесконечный режим (переход на клон и скачок после `transitionend` — принудительным событием), остаток, автопрокрутка (короткий интервал) и её остановка, клавиатура индикаторов, `responsive_options` (через изменение ширины контейнера нельзя — через `breakpoint` больше ширины окна), свайп (синтетические `touch`-события), вертикальная ориентация. e2e: стили темы, листание кнопками и точками, бесконечный режим, автопрокрутка, адаптивность (смена размера окна), вертикальный режим, свайп (touch-контекст).

### D10. Демонстрации и бенчмарки
Товары — локальный список из 9 элементов (без сетевых картинок, цветные плитки). Бенч `carousel-1`/`-1000` и `case-carousel`.

## Risks / Trade-offs

- [Сложность арифметики] Ошибка на краях страниц заметна только визуально; защищаемся табличными тестами модели и сверкой с исходником по каждому ветвлению.
- [Анимация в тестах] Без CSS темы `transitionend` не приходит; в wasm-тестах переход завершаем вручную, а переходы проверяем в e2e.
- [SSR и адаптивность] На сервере используются значения по умолчанию; после гидратации вёрстка пересчитывается.

## Open Questions

- Нужен ли режим `unstyled`: не переносим, как и у остальных компонентов.

## Migration Plan

Миграции нет. Откат — `git revert`.
