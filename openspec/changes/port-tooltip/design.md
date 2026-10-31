# Design

## Context

Требования — в `specs/tooltip`. Исходники PrimeNG 21.1.10: `tooltip/tooltip.ts`, `tooltip/style/tooltipstyle.ts`; демонстрации `apps/showcase/doc/tooltip/*-doc.ts`. Образцы в проекте: `prime-popover` (позиционирование, слушатели окна, захват `scroll`, очищающие замыкания), `prime-core::zindex` (`ZGroup::Tooltip` уже есть), `prime_dom::dom::{viewport, scroll}`.

Наблюдения по исходнику:
- Это **директива** (`[pTooltip]`), а не компонент: никакой разметки хоста, всё на `ElementRef` цели; слушатели `mouseenter`/`mouseleave`/`click`/`touchstart`/`touchend` (hover) и `focus`/`blur` (focus) вешаются на цель; для `focus` ищется `.p-component` внутри, а для `p-inputwrapper` — вложенный `input`.
- Подсказка создаётся императивно при показе (`create()`): `div.p-tooltip.p-component[role=tooltip]` > `div.p-tooltip-arrow` + `div.p-tooltip-text`; кладётся в `body` (`appendTo` по умолчанию `body`); при скрытии удаляется.
- Позиция считается по `getBoundingClientRect` цели + прокрутка окна; основная сторона и запасные по приоритету, `isOutOfBounds` — по границам окна; стрелка ставится по краю и центру (`top`/`bottom` — в `left` стрелки записывается середина подсказки).
- Опции собираются в `_tooltipOptions` из отдельных входов и `tooltipOptions`; при смене текста во время показа вызывается `updateText()` + `align()` или `hide()`.
- Скрытие по Escape — слушатель `keydown.escape` документа на время показа; по `resize` окна и по прокрутке предков (`ConnectedOverlayScrollHandler`).
- Появление — JS-функция `fadeIn(container, 250)`.
- Leptos умеет `use:`-директивы (`tachys::html::directive`): функция `handler(el: web_sys::Element, param: P)` вызывается при построении и гидратации элемента (в SSR не вызывается) и при каждом `rebuild` атрибута; своего хука очистки у директивы нет, но обработчик выполняется в реактивном владельце элемента, поэтому `on_cleanup` внутри него срабатывает при размонтировании.

## Goals / Non-Goals

**Goals:** тот же внешний вид, позиционирование и события, что у `pTooltip`; подключение без обёрток и изменения разметки; безопасность для SSR.

**Non-Goals:** `appendTo`, `pt`, `Bind` (см. proposal).

## Decisions

### D1. Форма API: директива `use:tooltip`
`pub fn tooltip(el: web_sys::Element, options: TooltipOptions)`; использование `<input use:tooltip=TooltipOptions::text("…") />`. Альтернативы отвергнуты: компонент-обёртка (меняет DOM и ломает `:hover`-стили вложенных селекторов, лишний элемент вокруг любого поля), хук `use_tooltip(NodeRef)` (требует `NodeRef` на каждом элементе). Проверено спайком (wasm-тест): `use:` вызывается один раз на обычном элементе и на корне `Button` и `InputText` без каких-либо доработок этих компонентов; запасной вариант (`NodeRef` + `Effect`) не нужен.

### D2. Состояние на элемент
`rebuild` директивы вызывается повторно, поэтому состояние не должно дублироваться: таблица `thread_local` `id → Rc<State>`; `id` записывается в JS-свойство элемента (`__primeTooltip`, без DOM-атрибута). Повторный вызов обновляет опции существующего состояния (`RwSignal<TooltipOptions>`-подобное), первый — создаёт состояние и подписывает слушатели. `on_cleanup` (из владельца элемента) снимает слушатели, убирает подсказку и запись в таблице.

### D3. Чистая логика (`state.rs`)
- `TooltipPosition`, `TooltipEvent`, `priority(position) -> [TooltipPosition; 4]`.
- `place(position, host: Rect, tip: Size) -> (left, top)` — смещения основной стороны относительно элемента без `position_top/left`.
- `arrow_style(position, tip_width) -> ArrowStyle` (`top`/`right`/`bottom`/`left`).
- `out_of_bounds(left, top, width, height, viewport_w, viewport_h) -> bool` по тем же условиям, что `isOutOfBounds`.
- `root_classes(position, style_class) -> String`.
- `activation_delay(show_delay, life) -> (Option<u32>, Option<u32>)` (когда показать и когда скрыть по `life`).
- `has_ellipsis(offset_w, scroll_w, offset_h, scroll_h) -> bool`.
- `leave_keeps_open(auto_hide, related_target_is_tooltip) -> bool`.
Табличные тесты на каждую.

### D4. Жизненный цикл подсказки
`activate()`: проверки (`show_on_ellipsis`, `disabled`, пустое содержимое) → таймер показа (`set_timeout`) → `show()` создаёт DOM и позиционирует (`display: none` на время сборки, затем `inline-block`; прозрачность 0 → 1 переходом 250 мс через `style.transition`); `deactivate()`: отмена показа, `hide_delay`, `hide()`. Слушатели окна (`keydown` Escape, `resize`, захват `scroll`) вешаются на время показа и снимаются в `hide()`. `scroll` скрывает подсказку, только если прокручен вложенный предок элемента: прокрутка самой страницы (документа) не скрывает, потому что подсказка лежит в координатах страницы и едет вместе с ней (иначе первый же показ после прокрутки к элементу немедленно скрывался бы). Контейнер, таймеры и z-index освобождаются в одном месте (`remove()`).

### D5. Содержимое
`Text(Prop<String>)` пишется в `text_content` (экранирование), `Html(String)` — в `inner_html`, `View(ViewFn)` монтируется внутрь `p-tooltip-text` через `leptos::mount::mount_to`, `UnmountHandle` хранится до скрытия. Реактивный `Text` и `disabled` отслеживаются `Effect`ом: смена во время показа → `update_text()` + `align()` либо `hide()`.

### D6. Касание
`touchstart` показывает, `touchend` скрывает при `auto_hide`; без `auto_hide` на документ вешается `touchstart`, скрывающий подсказку при касании вне цели и вне подсказки. Не проверялось на реальном вводе (как у `Slider`/`Knob`); записывается в docs.

### D7. CSS
`tooltip` добавляется в `theme.components`; блока `For PrimeNG` у `tooltipstyle.ts` нет. Проверить, что в CSS есть стрелка и позиционирование (`.p-tooltip-arrow` через `border`), а `fadeIn` заменяется переходом (в теме для этого ничего нет).

### D8. Тесты
SSR: директива ничего не выводит и не меняет разметку элемента. Браузерные `tests/web.rs` (один последовательный прогон — слушатели окна общие): показ/скрытие по hover, focus, both; click скрывает; задержки (`show_delay` отменяется уходом, `hide_delay`, `life`); `auto_hide=false` и наведение на подсказку; 4 стороны и перенос при нехватке места; смещения; Escape, resize, scroll; HTML/текст/View; реактивная смена текста и `disabled`; `show_on_ellipsis`; `z-index` над `Dialog`; очистка при размонтировании; повторный `use:` не дублирует слушатели. e2e: настоящая мышь и клавиатура на странице `/tooltip`. Крейт добавляется в три места `ci.yml`.

### D9. Бенчмарки
Сценарии `tooltip-1`/`-1000` (элемент с директивой в SSR: директива не вызывается, измеряется стоимость построения `TooltipOptions`) и `case-tooltip` (элемент + директива в wasm); сначала `cargo bench -p render-bench --bench alloc`, для wasm — временный большой бюджет (заменять по уникальной строке).

## Risks / Trade-offs

- [`use:` на корне компонента] проверено на `Button` и `InputText`; для других компонентов проекта проверяется в showcase.
- [`rebuild` директивы] повторный вызов не должен дублировать слушатели → D2 и отдельный тест.
- [Нет хука очистки у директивы] полагаемся на `on_cleanup` владельца; проверяется тестом размонтирования.
- [Касание не проверялось на реальном вводе] записывается в docs.
- [Переход прозрачности вместо `fadeIn`] визуальная разница пренебрежимо мала; записывается в docs.

## Open Questions

- Закрыт: `use:` принимается на корне `Button` и `InputText` (спайк в первой задаче).

## Migration Plan

Миграции нет. Откат — `git revert`.
