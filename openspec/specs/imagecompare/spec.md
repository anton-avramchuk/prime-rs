# imagecompare Specification

## Purpose
ImageCompare показывает два изображения друг над другом и позволяет ползунком открывать верхнее на нужную долю ширины.

## Requirements

### Requirement: Разметка
`ImageCompare` крейта `prime-imagecompare` SHALL выводить `div.p-imagecompare` с `tabindex`, `aria-label`, `aria-labelledby` (если заданы), `style`, `style_class` и `pt`, внутри — представление `left`, представление `right` и `input[type=range].p-imagecompare-slider` с `min="0"`, `max="100"` и `value="50"`, именно в этом порядке. Представления `left` и `right` SHALL выводиться как переданы, без обёрток.

#### Scenario: Состав
- **WHEN** отрисован `ImageCompare` с двумя `<img>`
- **THEN** корень содержит `img`, `img` и `input[type=range]` по порядку, у ползунка `min=0`, `max=100`, `value=50`

#### Scenario: Доступность
- **WHEN** заданы `tabindex=0` и `aria_label="Compare"`
- **THEN** корень имеет `tabindex="0"` и `aria-label="Compare"`

### Requirement: Ползунок
Ввод в ползунок SHALL задавать у элемента, стоящего непосредственно перед ним, `clip-path: polygon(0 0, V% 0, V% 100%, 0 100%)`, где `V` — значение ползунка; если у ползунка или его предка есть `dir="rtl"`, SHALL задаваться `polygon((100−V)% 0, 100% 0, 100% 100%, (100−V)% 100%)`. Ввод без предыдущего соседа SHALL не вызывать ошибок.

#### Scenario: Ввод значения
- **WHEN** значение ползунка меняется на 30
- **THEN** у предшествующего ему элемента `clip-path` равен `polygon(0 0, 30% 0, 30% 100%, 0 100%)`

#### Scenario: Справа налево
- **WHEN** корень внутри `dir="rtl"` и значение 30
- **THEN** `clip-path` равен `polygon(70% 0, 100% 0, 100% 100%, 70% 100%)`

### Requirement: Рендер без браузера
Вне браузера (SSR) `ImageCompare` SHALL отрисовываться полностью; `window` и `document` SHALL не использоваться.

#### Scenario: SSR
- **WHEN** страница с `ImageCompare` отрисована на сервере
- **THEN** в разметке есть оба изображения и ползунок
