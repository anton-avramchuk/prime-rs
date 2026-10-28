## ADDED Requirements

### Requirement: Структура и классы ScrollPanel
Компонент `ScrollPanel` крейта `prime-scrollpanel` SHALL отрисовывать `<div class="p-scrollpanel p-component">`, внутри которого по порядку идут `div.p-scrollpanel-content-container` с вложенным `div.p-scrollpanel-content` (в нём содержимое), затем полоса `div.p-scrollpanel-bar.p-scrollpanel-bar-x` и полоса `div.p-scrollpanel-bar.p-scrollpanel-bar-y`. Содержимое SHALL задаваться `children` или слотом `content_template`; при заданном слоте `children` не выводятся. `style_class` и `style` SHALL добавляться без потери собственных классов.

#### Scenario: Порядок элементов
- **WHEN** отрисован ScrollPanel с `children`
- **THEN** HTML содержит контейнер содержимого с `p-scrollpanel-content` внутри, затем `p-scrollpanel-bar-x`, затем `p-scrollpanel-bar-y`

#### Scenario: Слот вместо children
- **WHEN** заданы `content_template` и `children`
- **THEN** внутри `p-scrollpanel-content` выведено только содержимое слота

### Requirement: Доступность полос ScrollPanel
Каждая полоса SHALL иметь `role="scrollbar"`, `tabindex="0"`, `aria-orientation` (`horizontal` для X, `vertical` для Y), `aria-valuenow` (последний известный `scrollLeft`/`scrollTop`, изначально 0), `aria-controls` со ссылкой на `id` содержимого и `data-pc-group-section="bar"`. Содержимое `p-scrollpanel-content` SHALL иметь этот `id`, уникальный на странице и одинаковый при SSR и гидрации.

#### Scenario: Атрибуты полос
- **WHEN** отрисован ScrollPanel
- **THEN** у `bar-x` `aria-orientation="horizontal"`, у `bar-y` `aria-orientation="vertical"`, у обеих `role="scrollbar"` и `aria-controls`, равный `id` содержимого

### Requirement: Свойства и pass-through ScrollPanel
Свойства `style`, `style_class` и `step` SHALL принимать значение или сигнал; `style` и `style_class` обновляют разметку при смене сигнала, `step` читается в момент нажатия клавиши. `pt` SHALL принимать части `root`, `content_container`, `content`, `bar_x`, `bar_y`, добавляя классы, стиль и атрибуты без замены собственных.

#### Scenario: Реактивный класс
- **WHEN** `style_class` передан сигналом и меняется
- **THEN** класс корня обновляется, собственные классы сохраняются

#### Scenario: Класс через pt
- **WHEN** в `pt.bar_y` передан класс `custom`
- **THEN** у вертикальной полосы есть `custom` и `p-scrollpanel-bar-y`

### Requirement: Расчёт и скрытие полос
В браузере ScrollPanel SHALL пересчитывать полосы при `scroll` и `mouseenter` на содержимом, при `resize` окна, после монтирования и по вызову `refresh()`. Для оси, где `clientSize / scrollSize >= 1`, полоса SHALL получать `data-p-scrollpanel-hidden="true"` и класс `p-scrollpanel-hidden`; иначе `data-p-scrollpanel-hidden="false"`, класс снят, а размер и положение полосы заданы в `style` по формулам PrimeNG (длина — `max(ratio × 100, 10)` процентов, смещение пропорционально прокрутке).

#### Scenario: Содержимое умещается
- **WHEN** содержимое не превышает видимую область по вертикали
- **THEN** вертикальная полоса скрыта (`p-scrollpanel-hidden`)

#### Scenario: Содержимое выше области
- **WHEN** содержимое выше области и прокручено на половину доступного хода
- **THEN** вертикальная полоса видима, её `height` — не менее 10% и `top` отражает положение прокрутки

#### Scenario: Максимальная высота
- **WHEN** у корня задана `max-height`, а собственная высота равна нулю
- **THEN** после монтирования высота корня равна `min(max-height, высота содержимого с отступами и рамкой)`

### Requirement: Перетаскивание полос
При `mousedown` на полосе ScrollPanel SHALL фокусировать её, ставить `data-p-scrollpanel-grabbed="true"` и класс `p-scrollpanel-grabbed` на полосе и на `body`, подписываться на `mousemove` и `mouseup` документа и отменять действие по умолчанию. На `mousemove` содержимое SHALL прокручиваться на `delta / ratio` соответствующей оси. На `mouseup` атрибуты и классы снимаются, слушатели документа отписываются. При размонтировании все слушатели SHALL быть сняты.

#### Scenario: Перетаскивание
- **WHEN** пользователь нажимает на вертикальную полосу и сдвигает мышь вниз
- **THEN** `scrollTop` содержимого растёт, а по `mouseup` класс `p-scrollpanel-grabbed` снят

### Requirement: Клавиатура ScrollPanel
При фокусе на полосе ScrollPanel SHALL определять активную ось (фокус на X — горизонтальная, на Y — вертикальная; прокрутка колесом/жестом переключает ось по изменившемуся `scrollLeft`/`scrollTop`). Для вертикальной оси `ArrowDown`/`ArrowUp` прокручивают на `±step` пикселей через 40 мс после нажатия (повтор до `keyup`), `ArrowLeft`/`ArrowRight` лишь отменяют действие по умолчанию; для горизонтальной — симметрично. `keyup` SHALL сбрасывать таймер. После `blur` с горизонтальной полосы ось SHALL вернуться к вертикальной.

#### Scenario: Стрелка вниз
- **WHEN** фокус на вертикальной полосе и нажата `ArrowDown`
- **THEN** через 40 мс `scrollTop` содержимого вырос на `step`, событие отменено

#### Scenario: Отпускание клавиши
- **WHEN** `keyup` пришёл раньше таймера
- **THEN** прокрутка не выполняется

### Requirement: Программное управление ScrollPanel
Свойство `panel_ref: Option<ScrollPanelRef>` SHALL давать ручку с методами `refresh()` (пересчёт полос) и `scroll_top(value)` (устанавливает `scrollTop` содержимого, значение ограничено диапазоном `0..=scrollHeight − clientHeight`). Вне браузера методы ничего не делают.

#### Scenario: Ограничение scroll_top
- **WHEN** вызван `scroll_top(100000)` для содержимого высотой 500 при области 150
- **THEN** `scrollTop` равен 350

### Requirement: Стоимость отрисовки ScrollPanel
ScrollPanel со статическими свойствами SHALL использовать статическую ветвь; обе ветви SHALL давать одинаковую разметку. SSR не обращается к DOM. Число аллокаций на экземпляр при SSR SHALL укладываться в бюджет `scrollpanel-1000` в `benches/alloc-budgets.toml`, вклад в wasm — в бюджет `scrollpanel` в `benches/budgets.toml`.

#### Scenario: Паритет ветвей
- **WHEN** один набор значений передан статически и сигналами
- **THEN** HTML совпадает

#### Scenario: Бюджеты
- **WHEN** выполняются `cargo xtask alloc-budget` и `cargo xtask wasm-size`
- **THEN** значения ScrollPanel не превышают бюджеты
