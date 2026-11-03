# Design

## Context

Требования — в `specs/speeddial`. Исходники PrimeNG 21.1.10: `speeddial/speeddial.ts` (около 580 строк), `speeddial/style/speeddialstyle.ts`; стили `@primeuix/styles/speeddial`; демонстрации `apps/showcase/doc/speeddial/{linear,circle,semicircle,quartercircle,mask,tooltip,template}-doc.ts`. Образцы в проекте: `prime-dock` (собственная модель пунктов, фокус по `aria-activedescendant`, чистая логика клавиш), `prime-button`, `prime-tooltip` (директива), `prime-popover` (слушатели окна).

Наблюдения по исходнику:
- Раскладка по `type`: `linear` — только `flex-direction`; `circle` — точки на окружности радиусом `radius || length*20` с шагом `2π/length`; `semi-circle` — шаг `π/(length-1)`, смещения по сторонам зависят от `direction`; `quarter-circle` — шаг `π/(2(length-1))` и диагональные направления. Каждая точка — `left/right/top/bottom: calc(<px> + var(--item-diff-x|y, 0px))`; `--item-diff-*` — полуразница размеров кнопки и первого пункта (считается после показа в браузере).
- `transition-delay`: при показе `index * transitionDelay`, при скрытии `(length - index - 1) * transitionDelay`.
- Направление клавиш: `ArrowUp`/`ArrowDown` зависят от `up`/`down`, `ArrowLeft`/`ArrowRight` — от `left`, `up-right`, `down-left` и `right`, `up-left`, `down-right`; для остальных значений есть запасной смысл.
- Фокус пункта хранится по идентификатору `<id>_<индекс>` среди пунктов без `p-disabled` ссылки; `Home`/`End` сбрасывают индекс и ведут к первому/последнему; `Enter`/`Space` кликают пункт, затем фокус возвращается на кнопку.
- Клик вне: слушатель `click` на документе только пока показано; игнорируется, если клик внутри контейнера или только что был клик по кнопке/пункту (`isItemClicked`).
- В PrimeNG есть опечатки логики: `findNextOptionIndex(-1)` при пустом списке падает; индекс по `id` ищется после фильтра отключённых. В порте индексы считаются по списку доступных пунктов, без исключений.

## Goals / Non-Goals

**Goals:** тот же DOM, классы, раскладка, клавиши и закрытие.

**Non-Goals:** `routerLink`, `buttonProps` целиком, `Ripple`, `Bind`, `pTemplate`, `unstyled`.

## Decisions

### D1. Крейт и модель
`prime-speeddial` с `SpeedDial`, `SpeedDialItem` (строитель, как `DockItem`), `SpeedDialCommand { event, item }`, `SpeedDialType`, `SpeedDialDirection`, `SpeedDialPt`, `SpeedDialItemContext { item, index, toggle }`. Своя модель, а не `MenuItem`: без зависимости от `prime-menu`.

### D2. Чистая логика (`state.rs`)
`root_classes(type, direction, open, disabled, extra)`, `root_flex_style(direction)`, `list_flex_style(direction)`, `item_classes(hidden, focused)`, `point_style(type, direction, index, length, radius) -> Vec<(&str, String)>`, `transition_delay(visible, index, length, step)`, `arrow_step(direction, key) -> Step {Next, Prev}`, `step_index(count, current, step)`, `button_icon_choice(visible, has_show, has_hide)`, `button_classes(rotate_with_hide_icon...)`. Табличные тесты, включая все типы и направления.

### D3. Состояние
`visible`: `RwSignal<bool>` (если задано свойство `visible: Option<RwSignal<bool>>` — оно же), `focused_index: RwSignal<Option<usize>>` (позиция среди доступных пунктов), `focused: RwSignal<bool>` (`aria-activedescendant` только пока список в фокусе), `item_clicked: StoredValue<bool>`. `show`/`hide` вызывают `on_visible_change`, `on_show`/`on_hide`.

### D4. Разметка
Один раз при монтировании строятся все пункты (скрытые остаются в DOM с классом `p-hidden` у `visible=false`; скрытие всего списка делает тема). Состояния пункта (`p-focus`, `data-p-active`, `tabindex` действия: нет при `disabled` или скрытом списке, иначе `tabindex` пункта или `0`) — реактивные атрибуты. `--item-diff-x/y` ставятся после монтирования в браузере для всех типов, кроме `linear`.

### D5. Кнопка
Кнопка переключателя — `Button` из `prime-button` с классами `p-speeddial-button p-button-rounded` (и `p-speeddial-rotate`, если нет `hide_icon`), `button_icon_only`; `aria-*` реактивные. Так как `Button` не принимает реактивные `aria-expanded`/`aria-controls`, при необходимости кнопка рисуется самим `SpeedDial` по образцу `SplitButton` (классы и `data-p` — из функций `prime-button`). Решение — по факту сборки в приложении задач.

### D6. Клик вне и `Escape`
Слушатель `click` на окне вешается на время показа в следующем тике (клик, открывший список, ещё всплывает); клик внутри контейнера, по кнопке или пункту (`item_clicked`) не скрывает. `Escape` на кнопке и в списке скрывает и возвращает фокус на кнопку.

### D7. Подсказки
`tooltip_options: Option<TooltipOptions>` — шаблон: для каждого пункта директива `tooltip` получает клон с текстом `label` (`disabled`, пока параметры не заданы); позиция берётся из шаблона.

### D8. Тесты
SSR: разметка корня и классы по типам/направлениям, пункты, `p-hidden`, кнопка (`aria-*`), `pt`, маска не выводится на сервере. Браузерные `tests/web.rs` одним последовательным тестом (общие слушатели окна): переключение кнопкой, события, команда пункта и скрытие, отключённый пункт, `hide_on_click_outside`, `Escape`, клавиши кнопки и списка по направлениям (все восемь), `Home`/`End`/`Enter`/`Space`, пропуск отключённых, `mask`, управляемый `visible`, `disabled`, раскладка (проверка `left/top/bottom/right` у пунктов круга), `transition-delay`. e2e: стили темы, раскрытие по направлениям, круг и полукруг, маска, подсказка, шаблоны.

### D9. Демонстрации и бенчмарки
Все семь демонстраций; значки — `prime-icons` (PlusIcon, PencilIcon?, RefreshIcon, TrashIcon?, UploadIcon?, ExternalLinkIcon? — выбрать существующие). Бенч `speeddial-1`/`-1000` и `case-speeddial`.

## Risks / Trade-offs

- [Геометрия раскладки] Без CSS темы положение пунктов не проверить в wasm-тестах; проверяем строки стилей и позиции в e2e.
- [Набор значков] Не все значки PrimeIcons есть в `prime-icons`; недостающие заменяются близкими.
- [Опечатки в исходнике] Индексы фокуса считаются по доступным пунктам; отличие документируется.

## Open Questions

- Нужна ли кнопка переключателя из `Button` или собственная — решается при реализации (D5).

## Migration Plan

Миграции нет. Откат — `git revert`.
