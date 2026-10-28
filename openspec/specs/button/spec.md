# button Specification

## Purpose
Компонент Button (порт p-button из PrimeNG): вид, состояния loading и disabled, иконка, метка, бейдж, события, доступность, ripple, pass-through.

## Requirements

### Requirement: Свойства Button
Компонент `Button` крейта `prime-button` SHALL принимать те же свойства, что и `p-button` в PrimeNG 21.1.10: `label`, `icon`, `icon_pos` (`left`, `right`, `top`, `bottom`, по умолчанию `left`), `loading`, `loading_icon`, `severity` (`secondary`, `success`, `info`, `warn`, `help`, `danger`, `contrast`), `variant` (`outlined`, `text`), `outlined`, `text`, `plain`, `link`, `raised`, `rounded`, `size` (`small`, `large`), `fluid`, `badge`, `badge_severity` (по умолчанию `secondary`), `disabled`, `button_type` (атрибут `type`, по умолчанию `button`), `aria_label`, `tabindex`, `autofocus`, `style`, `style_class`. Содержимое передаётся через `children`. Все свойства необязательны, изменяемые во время работы SHALL быть реактивными.

#### Scenario: Кнопка по умолчанию
- **WHEN** `Button` отрисован без свойств
- **THEN** создан элемент `<button type="button">` с классами `p-button p-component p-ripple`

#### Scenario: Метка
- **WHEN** задан `label = "Сохранить"`
- **THEN** внутри кнопки есть `<span class="p-button-label">Сохранить</span>`

### Requirement: Классы и атрибуты корня Button
Корневой `<button>` SHALL иметь класс `p-button p-component` и модификаторы, как в PrimeNG: `p-button-icon-only` (есть иконка, нет метки и бейджа), `p-button-vertical` (`icon_pos` равен `top` или `bottom` и есть метка), `p-button-loading`, `p-button-link`, `p-button-<severity>`, `p-button-raised`, `p-button-rounded`, `p-button-text` (`text` или `variant = text`), `p-button-outlined` (`outlined` или `variant = outlined`), `p-button-sm`, `p-button-lg`, `p-button-plain`, `p-button-fluid`. Атрибуты `data-p` (признаки через пробел), `data-p-disabled` и `data-p-severity` SHALL вычисляться так же, как в PrimeNG. Дополнительные `style_class` и `style` SHALL добавляться без потери собственных классов.

#### Scenario: Серьёзность и вид
- **WHEN** заданы `severity = danger`, `variant = outlined`, `size = small`
- **THEN** у корня классы `p-button-danger`, `p-button-outlined`, `p-button-sm`, а `data-p-severity` равен `danger`

#### Scenario: Только иконка
- **WHEN** задана иконка, но нет метки и бейджа
- **THEN** у корня есть `p-button-icon-only`, а в `data-p` есть `icon-only`

#### Scenario: Вертикальная раскладка
- **WHEN** `icon_pos = top`, заданы иконка и метка
- **THEN** у корня есть `p-button-vertical`

#### Scenario: Пользовательский класс и стиль
- **WHEN** заданы `style_class = "custom"` и `style = "margin: 4px"`
- **THEN** у корня есть класс `custom`, собственные классы остались, и атрибут `style` равен `margin: 4px`

### Requirement: Состояния disabled и loading
Кнопка SHALL быть недоступной (`disabled`), если `disabled` или `loading`, и тогда `data-p-disabled` равен `true`. При `loading` вместо иконки SHALL отображаться индикатор загрузки: значение `loading_icon`, если оно задано, иначе `SpinnerIcon` с вращением и классами `p-button-loading-icon`; индикатор имеет `aria-hidden="true"`.

#### Scenario: Загрузка
- **WHEN** `loading = true`
- **THEN** кнопка недоступна, есть `p-button-loading`, виден индикатор `p-button-loading-icon`, обычной иконки нет

#### Scenario: Свой индикатор загрузки
- **WHEN** `loading = true` и задан `loading_icon`
- **THEN** вместо `SpinnerIcon` отрисовано переданное содержимое в оболочке с классом `p-button-loading-icon`

#### Scenario: Недоступная кнопка
- **WHEN** `disabled = true`
- **THEN** у кнопки атрибут `disabled`, `data-p-disabled="true"` и она не вызывает `on_click`

### Requirement: Иконка, метка, бейдж и содержимое
Иконка (`icon`: содержимое Leptos) SHALL отображаться в `<span>` с классом `p-button-icon` и модификатором позиции (`p-button-icon-left`, `-right`, `-top`, `-bottom`; для кнопки без метки модификатора позиции нет). Если заданы `badge` и нет `children`, SHALL отрисовываться `Badge` с `value = badge` и `severity = badge_severity`. `children`, если заданы, SHALL выводиться первыми, как проецируемое содержимое в PrimeNG; в этом случае метка и бейдж не отображаются.

#### Scenario: Позиция иконки
- **WHEN** `icon_pos = right`, заданы иконка и метка
- **THEN** у иконки есть `p-button-icon-right`

#### Scenario: Бейдж
- **WHEN** `badge = "3"`, `badge_severity = danger`
- **THEN** внутри кнопки есть `Badge` со значением `3` и классом `p-badge-danger`

#### Scenario: Содержимое вместо метки
- **WHEN** заданы `children` и `label`
- **THEN** выведены `children`, а `p-button-label` и бейдж отсутствуют

### Requirement: События и фокус
Кнопка SHALL вызывать `on_click`, `on_focus`, `on_blur` с исходными событиями. При `autofocus = true` кнопка SHALL получать фокус после монтирования в браузере; на сервере фокус не выставляется.

#### Scenario: Клик
- **WHEN** пользователь нажимает на доступную кнопку
- **THEN** вызывается `on_click` ровно один раз с `MouseEvent`

#### Scenario: Автофокус
- **WHEN** кнопка с `autofocus = true` смонтирована в браузере
- **THEN** `document.activeElement` — эта кнопка

### Requirement: Доступность Button
Кнопка SHALL быть нативным `<button>` с доступной клавиатурой (Enter и Space) и передавать `aria_label` как `aria-label` и `tabindex` как `tabindex`. Индикаторы загрузки и иконка без метки не должны дублироваться в доступном имени.

#### Scenario: aria-label и tabindex
- **WHEN** заданы `aria_label = "Закрыть"` и `tabindex = 2`
- **THEN** у `<button>` атрибуты `aria-label="Закрыть"` и `tabindex="2"`

### Requirement: Ripple
Корень SHALL иметь класс `p-ripple`. Если в `PrimeConfig` включён `ripple`, после монтирования в браузере SHALL создаваться элемент `p-ink`, и при `mousedown` он активируется. Если `ripple` выключен, элемента `p-ink` нет.

#### Scenario: Ripple включён
- **WHEN** `PrimeConfig.ripple = true` и пользователь нажимает кнопку мышью
- **THEN** внутри кнопки есть `span.p-ink` с `data-p-ink-active="true"`

#### Scenario: Ripple выключен
- **WHEN** `PrimeConfig.ripple = false`
- **THEN** элемента `p-ink` нет

### Requirement: Pass-through Button
`Button` SHALL принимать `pt` для корня и частей `label`, `icon`, `loading_icon` и `badge`; классы и атрибуты из `pt` добавляются к собственным, не заменяя их.

#### Scenario: pt на части
- **WHEN** передан `pt` с классом `extra` для метки
- **THEN** у `p-button-label` есть и `p-button-label`, и `extra`

### Requirement: Соответствие PrimeNG
DOM и классы `Button` SHALL совпадать с PrimeNG: внутри Angular-хоста `<p-button>` находится `<button>`, в Leptos отрисовывается только `<button>`, потому что хост-элемент в PrimeNG не несёт классов и стилей. Страница Button в showcase SHALL повторять демонстрации PrimeNG, и еженедельное сравнение скриншотов SHALL включать её.

#### Scenario: Структура DOM
- **WHEN** отрисована кнопка с иконкой, меткой и бейджем
- **THEN** порядок дочерних элементов `<button>`: иконка, метка, бейдж

### Requirement: Производительность Button
Рендер `Button` SHALL иметь сценарии `button-1` и `button-1000` в `render-bench`, а вклад в размер wasm — бюджет `case-button` в `benches/budgets.toml`. Классы корня SHALL пересчитываться только при изменении входящих свойств.

#### Scenario: Бюджет
- **WHEN** выполняется `cargo xtask wasm-size`
- **THEN** вклад `case-button` не превышает бюджет

#### Scenario: Реактивность без лишних обновлений
- **WHEN** изменяется свойство, не влияющее на классы
- **THEN** атрибут `class` корня не перезаписывается

### Requirement: Стоимость рендера Button
Рендер `Button` со статическими свойствами SHALL NOT создавать сигналов и `Memo` (ни для свойств, ни для состояния), кроме эффектов автофокуса и ripple, которые существуют только при сборке для `wasm32`. Число аллокаций на экземпляр при SSR SHALL укладываться в бюджет, заданный в `benches` и проверяемый командой бенчмарков. Классы и атрибуты SHALL вычисляться в реактивных замыканиях атрибутов.

#### Scenario: Статический Button
- **WHEN** рендерится 1000 экземпляров `Button` с меткой и серьёзностью
- **THEN** число аллокаций на экземпляр не превышает бюджет `button`

#### Scenario: Полная кнопка
- **WHEN** рендерится 1000 кнопок с иконкой, меткой и бейджем
- **THEN** число аллокаций на экземпляр не превышает бюджет `button-full`

#### Scenario: Реактивная кнопка
- **WHEN** свойство `loading` передано сигналом и меняется
- **THEN** класс, атрибуты, блокировка и индикатор обновляются, а `class` не перезаписывается при изменении свойства, не влияющего на классы

#### Scenario: Поведение не изменилось
- **WHEN** выполняются тесты спеки `button` (SSR и браузерные)
- **THEN** все они проходят без изменений ожидаемых значений
