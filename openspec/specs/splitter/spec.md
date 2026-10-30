# splitter Specification

## Purpose
Splitter делит область на панели, размеры которых меняются разделителями.

## Requirements

### Requirement: Разметка Splitter
Компонент `Splitter` крейта `prime-splitter` SHALL выводить корень `div.p-splitter.p-component.p-splitter-<layout>` со стилем `display: flex; flex-wrap: nowrap` (и `flex-direction: column` при `Vertical`) и `data-p` с раскладкой (и `nested` при `nested`). Для каждой панели из `panels` SHALL выводиться `div.p-splitterpanel` (с `p-splitterpanel-nested` при `nested`, `tabindex="-1"`, `data-pc-section="panel"`), затем (кроме последней панели) `div.p-splitter-gutter[role=separator][tabindex="-1"][data-p-gutter-resizing="false"]` с вложенной `div.p-splitter-gutter-handle[tabindex="0"]`, у которой `aria-orientation` равен раскладке, `aria-valuenow` — размеру предыдущей панели с четырьмя знаками, а размер по оси раскладки — `gutter_size` пикселей. `style`, `style_class`, `panel_style`, `panel_style_class`, `pt` SHALL применяться к корню и панелям.

#### Scenario: Две панели
- **WHEN** отрисован `Splitter` с двумя панелями
- **THEN** есть две `.p-splitterpanel` и один `.p-splitter-gutter` между ними, у ручки `tabindex="0"`

#### Scenario: Вертикальная раскладка
- **WHEN** `layout = Vertical`
- **THEN** корень имеет `p-splitter-vertical` и `flex-direction: column`, ручка — `aria-orientation="vertical"` и высоту `gutter_size`

### Requirement: Начальные размеры
Размер каждой панели SHALL быть `panel_sizes[i]`, а если значения нет или оно равно нулю — `100 / n` процентов. Панель SHALL получать `flex-basis: calc(<размер>% - <(n-1)*gutter_size>px)`.

#### Scenario: Поровну
- **WHEN** `panel_sizes` не задан и панелей две
- **THEN** у обеих `flex-basis: calc(50% - 4px)`

#### Scenario: Заданные
- **WHEN** `panel_sizes = [20, 80]`
- **THEN** первая панель `calc(20% - 4px)`, вторая `calc(80% - 4px)`

### Requirement: Перетаскивание
Нажатие мыши на разделитель SHALL начать изменение размеров, вызвать `on_resize_start` с текущими размерами и добавить корню `p-splitter-resizing`, разделителю — `p-splitter-gutter-resizing` и `data-p-gutter-resizing="true"`. Движение мыши по документу SHALL переносить долю процента между предыдущей и следующей панелями пропорционально смещению относительно размера корня по оси раскладки (в `rtl` горизонтальное смещение инвертируется). Отпускание SHALL завершить изменение, вызвать `on_resize_end` и снять классы и подписки. Касание SHALL работать так же.

#### Scenario: Смещение
- **WHEN** корень шириной 400 px, две панели по 50 %, мышь сдвинута на 40 px вправо
- **THEN** размеры равны 60 % и 40 %

#### Scenario: Завершение
- **WHEN** мышь отпущена
- **THEN** `on_resize_end` вызван с итоговыми размерами, классы изменения сняты

### Requirement: Минимальные размеры
Изменение SHALL отвергаться целиком, если новый размер предыдущей панели меньше `min_sizes[i]` или следующей меньше `min_sizes[i+1]` (нулевое или отсутствующее значение не ограничивает). `aria-valuenow` SHALL обновляться и при отвергнутом изменении.

#### Scenario: Ограничение
- **WHEN** `min_sizes = [10, 0]` и перетаскивание делает первую панель 5 %
- **THEN** размеры не меняются

### Requirement: Клавиатура
`keydown` на ручке SHALL смещать границу на `step` пикселей: для `Horizontal` — `ArrowRight` увеличивает предыдущую панель, `ArrowLeft` уменьшает; для `Vertical` — `ArrowUp` увеличивает, `ArrowDown` уменьшает; остальные направления и клавиши SHALL игнорироваться. Действие по умолчанию стрелок SHALL отменяться всегда. `keyup` SHALL завершать изменение: сохранить состояние и вызвать `on_resize_end`.

#### Scenario: Стрелка вправо
- **WHEN** на ручке нажат `ArrowRight`, корень шириной 400 px, `step = 5`
- **THEN** предыдущая панель вырастает на 1.25 процентного пункта, следующая уменьшается на столько же

### Requirement: Сохранение состояния
При заданном `state_key` размеры SHALL сохраняться при завершении изменения в `localStorage` (`state_storage = Local`) или `sessionStorage` (`Session`) как JSON-массив и восстанавливаться при монтировании, если там есть значение.

#### Scenario: Восстановление
- **WHEN** в хранилище под ключом лежит `[30,70]` и компонент смонтирован
- **THEN** первая панель `calc(30% - 4px)`
