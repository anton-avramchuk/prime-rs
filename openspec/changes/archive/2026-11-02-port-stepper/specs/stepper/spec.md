# stepper Specification

## Purpose
Stepper ведёт пользователя по нумерованным шагам: показывает список шагов с разделителями, активную панель содержимого, поддерживает свободный и линейный режимы и вертикальную раскладку.

## ADDED Requirements

### Requirement: Корень и значение
`Stepper` крейта `prime-stepper` SHALL выводить `div.p-stepper.p-component[role=tablist]` с `id` (свойство `id` или `pn_id_<N>`), `style_class`, `style` и `pt`; при `linear` корень SHALL получать класс `p-readonly`. Активный шаг SHALL определяться двусторонним сигналом `value` (`RwSignal<Option<u32>>`); без него компонент ведёт собственное состояние, начинающееся с `default_value`. Внешнее изменение `value` SHALL переключать активные шаг и панель.

#### Scenario: Начальное значение
- **WHEN** задан `default_value = 2`
- **THEN** шаг и панель со значением 2 активны

#### Scenario: Линейный режим
- **WHEN** задан `linear`
- **THEN** корень имеет класс `p-readonly`

### Requirement: Шаги
`StepList` SHALL выводить `div.p-steplist`. `Step` со значением `value` SHALL выводить `div.p-step[role=presentation]` с классами `p-step-active` (активный) и `p-disabled` (недоступный), `aria-current="step"` у активного, `data-p-active`, `data-p-disabled` и кнопкой `button.p-step-header[role=tab][type=button]` с `id` `<id>_step_<value>`, `aria-controls` `<id>_steppanel_<value>`, номером `span.p-step-number` (значение шага) и заголовком `span.p-step-title` (содержимое шага). Шаг SHALL считаться недоступным, если он не активен и (`Stepper.linear` или `Step.disabled`); у недоступного шага кнопка SHALL иметь атрибут `disabled` и `tabindex="-1"`.

#### Scenario: Разметка шага
- **WHEN** отрисован `Step value=1` с заголовком «Header I» в активном состоянии
- **THEN** есть `.p-step.p-step-active`, `button[role=tab]` с `.p-step-number` «1» и `.p-step-title` «Header I», у корня шага `aria-current="step"`

#### Scenario: Недоступный шаг
- **WHEN** `Step disabled=true` неактивен
- **THEN** кнопка имеет `disabled` и `tabindex="-1"`, корень шага — класс `p-disabled`

### Requirement: Разделители
После каждого шага списка, кроме последнего, SHALL выводиться `div.p-stepper-separator`. Для вертикальной раскладки разделитель SHALL выводиться в `StepPanel` (в `div.p-steppanel-content-wrapper` перед `div.p-steppanel-content`) для каждого `StepItem`, кроме последнего; `Step` внутри `StepItem` разделителя не выводит.

#### Scenario: Три шага
- **WHEN** в `StepList` три шага
- **THEN** выведено два разделителя

### Requirement: Переключение шага
Клик по доступному шагу SHALL записывать его значение в `value`. Клик по недоступному шагу (кнопка `disabled`) SHALL ничего не менять. `StepContext::activate(value)` SHALL записывать указанное значение в `value` независимо от `linear` (линейный режим блокирует только клики по шагам).

#### Scenario: Клик по шагу
- **WHEN** пользователь кликает по шагу 3 в свободном режиме
- **THEN** `value` равно 3, шаг 3 активен

#### Scenario: Линейный режим
- **WHEN** `linear`, активен шаг 1 и пользователь кликает по шагу 3
- **THEN** `value` остаётся равным 1

#### Scenario: Программный переход
- **WHEN** в линейном режиме вызван `activate(2)`
- **THEN** `value` равно 2

### Requirement: Панели
`StepPanels` SHALL выводить `div.p-steppanels`. `StepPanel` со значением `value` SHALL выводить `div.p-steppanel[role=tabpanel]` с `id` `<id>_steppanel_<value>`, `aria-controls` `<id>_step_<value>`, `data-p-active`, `data-pc-name="steppanel"`. В горизонтальной раскладке содержимое (`div.p-steppanel-content-wrapper` > `div.p-steppanel-content`) SHALL выводиться только у активной панели. Содержимое SHALL задаваться дочерними элементами либо свойством `content`.

#### Scenario: Одна панель
- **WHEN** активен шаг 2 в горизонтальной раскладке
- **THEN** содержимое есть только у панели 2

### Requirement: Вертикальная раскладка
`StepItem` со значением `value` SHALL выводить `div.p-stepitem` (`p-stepitem-active` у активного, `data-p-active`) и передавать своё значение вложенным `Step` и `StepPanel`, у которых собственное значение не задано. В вертикальной раскладке панель SHALL получать класс `p-steppanel-active` у активной; все панели SHALL оставаться в разметке, неактивные скрытыми, а раскрытие и сворачивание SHALL анимироваться классами темы `p-collapsible-enter-active` и `p-collapsible-leave-active`.

#### Scenario: Вертикальный список
- **WHEN** три `StepItem`, активен второй
- **THEN** `.p-stepitem-active` только у второго, у его панели `p-steppanel-active`

### Requirement: Свои представления
`content` у `Step` SHALL заменять кнопку-заголовок (и получать `StepContext { value, active, activate }`), а `content` у `StepPanel` — содержимое `div.p-steppanel-content`. `activate` SHALL вызываться без привязки к клику по шагу, например из кнопок «Далее»/«Назад».

#### Scenario: Свой шаг
- **WHEN** задан `content` у `Step`
- **THEN** внутри `.p-step` вместо `button.p-step-header` — переданное представление

### Requirement: Рендер без браузера
Вне браузера (SSR) `Stepper` SHALL отрисовываться полностью (шаги, разделители, активная панель); `window` и `document` SHALL не использоваться.

#### Scenario: SSR
- **WHEN** страница со `Stepper` отрисована на сервере
- **THEN** в разметке есть список шагов, разделители и содержимое активной панели
