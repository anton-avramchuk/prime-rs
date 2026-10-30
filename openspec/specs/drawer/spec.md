# drawer Specification

## Purpose
Drawer показывает панель с содержимым у края экрана поверх страницы: с маской, анимацией входа и выхода, закрытием по маске, кнопке и клавише Escape, блокировкой прокрутки страницы и ловушкой фокуса.

## Requirements

### Requirement: Разметка
Пока `visible` ложно и анимация выхода не доиграла, `Drawer` крейта `prime-drawer` SHALL не выводить ничего. Видимая панель SHALL быть `div.p-drawer.p-component[role=complementary]` с классом `p-drawer-<position>` (`left`, `right`, `top`, `bottom`, `full`), `p-drawer-open` при `visible` и `p-drawer-full` при `position=Full` или `full_screen`, и дочерними `div.p-drawer-header` (заголовок `div.p-drawer-title` при `header`, кнопка закрытия `button.p-drawer-close-button` при `closable`), `div.p-drawer-content` с `children` и `div.p-drawer-footer` только при заданном `footer`. При `headless` SHALL выводиться только переданное представление, без заголовка, содержимого и подвала. `style`, `style_class` и `pt` SHALL применяться к корню панели. Панель и маска SHALL находиться в `body`, а не внутри места объявления компонента.

#### Scenario: Скрыт по умолчанию
- **WHEN** `visible` ложно и панель ни разу не показывали
- **THEN** в документе нет `.p-drawer` и `.p-drawer-mask`

#### Scenario: Показан
- **WHEN** `visible` становится истинным при `header="Left Drawer"`
- **THEN** в `body` есть `.p-drawer.p-drawer-left.p-drawer-open[role=complementary]` с заголовком «Left Drawer» и кнопкой закрытия

#### Scenario: Headless
- **WHEN** задан `headless`
- **THEN** внутри `.p-drawer` только переданное представление

### Requirement: Позиция и размер
`position` SHALL выбирать край экрана: `Left`, `Right`, `Top`, `Bottom`; `Full` и `full_screen` SHALL растягивать панель на весь экран. Размеры по умолчанию (ширина 20rem у боковых, высота 10rem у верхней и нижней) SHALL задаваться стилями темы и переопределяться `style`.

#### Scenario: Боковая позиция
- **WHEN** `position=Right`
- **THEN** панель прижата к правому краю и занимает всю высоту окна

#### Scenario: Свой размер
- **WHEN** `style="height: auto"` у верхней панели
- **THEN** высота панели определяется содержимым

### Requirement: Значение видимости
`visible` SHALL быть двусторонним сигналом `bool`: установка в истину показывает панель, в ложь — закрывает. Закрытие пользователем (маска, кнопка, Escape) SHALL записывать ложь в `visible`.

#### Scenario: Закрытие кнопкой
- **WHEN** панель видима и пользователь нажимает кнопку закрытия
- **THEN** `visible` становится ложным и панель убирается после анимации выхода

### Requirement: Анимация
При показе корень SHALL получать класс анимации входа `p-drawer-enter-<position>` (`p-drawer-enter-full` для полноэкранной), при закрытии — `p-drawer-leave-<position>`; панель SHALL оставаться в DOM до `animationend` (или до истечения запасного времени, если анимации нет) и только потом убираться, а маска — получать `p-overlay-mask-enter-active` при показе и `p-overlay-mask-leave-active` при закрытии.

#### Scenario: Выход
- **WHEN** панель закрывают
- **THEN** сначала у корня `p-drawer-leave-left`, после окончания анимации корня и маски `.p-drawer` нет в документе

### Requirement: Маска и закрытие по ней
При `modal` за панелью SHALL выводиться маска `div.p-drawer-mask.p-overlay-mask` с `z-index` на единицу ниже панели и `mask_style`; при `dismissible` клик по маске SHALL закрывать панель, клик по самой панели — нет. Без `modal` маски нет.

#### Scenario: Клик по маске
- **WHEN** панель модальная, `dismissible` включён и пользователь кликает по маске
- **THEN** панель закрывается

#### Scenario: Не закрываемая маской
- **WHEN** `dismissible` выключен и пользователь кликает по маске
- **THEN** панель остаётся видимой

#### Scenario: Без маски
- **WHEN** `modal` выключен
- **THEN** `.p-drawer-mask` нет в документе

### Requirement: Escape
При `close_on_escape` клавиша `Escape` SHALL закрывать панель; при нескольких открытых панелях — только верхнюю по `z-index`. Слушатель SHALL быть снят после закрытия.

#### Scenario: Две панели
- **WHEN** открыты две панели и нажат Escape
- **THEN** закрывается только верхняя

#### Scenario: Escape выключен
- **WHEN** `close_on_escape` выключен и нажат Escape
- **THEN** панель остаётся видимой

### Requirement: Блокировка прокрутки
При `block_scroll` на время показа `body` SHALL получать класс `p-overflow-hidden`; после закрытия класс SHALL сниматься, а при нескольких панелях — когда закрыта последняя из блокирующих.

#### Scenario: Блокировка и снятие
- **WHEN** панель с `block_scroll` показана и затем закрыта
- **THEN** класс есть на `body` во время показа и снят после закрытия

### Requirement: Фокус
Пока панель показана, `Tab` и `Shift+Tab` SHALL циклически перемещать фокус внутри панели; после закрытия ловушка SHALL быть снята.

#### Scenario: Цикл по Tab
- **WHEN** фокус на последнем элементе панели и нажат Tab
- **THEN** фокус переходит на первый фокусируемый элемент панели

### Requirement: Слои
При `auto_z_index` панель SHALL получать `z-index` группы модальных окон со смещением `base_z_index`; индекс SHALL освобождаться после закрытия.

#### Scenario: Две панели
- **WHEN** открыты две панели
- **THEN** `z-index` позже открытой выше

### Requirement: События
`on_show` SHALL вызываться один раз при показе, `on_hide` — один раз при закрытии любым способом.

#### Scenario: Порядок событий
- **WHEN** панель показана и закрыта
- **THEN** `on_show` вызван один раз до `on_hide`

### Requirement: Рендер без браузера
Вне браузера (SSR) `Drawer` SHALL отрисовываться скрытым и не обращаться к `window` и `document`; гидратация SHALL не расходиться с серверной разметкой.

#### Scenario: SSR
- **WHEN** страница с `Drawer` отрисована на сервере
- **THEN** панели и маски в разметке нет
