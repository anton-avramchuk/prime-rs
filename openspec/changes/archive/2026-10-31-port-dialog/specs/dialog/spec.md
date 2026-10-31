# dialog Specification

## Purpose
Dialog показывает окно поверх страницы с заголовком, содержимым и подвалом; окно можно перетаскивать, масштабировать и разворачивать на весь экран, оно закрывается кнопкой, клавишей Escape и (при настройке) кликом по маске.

## ADDED Requirements

### Requirement: Разметка
Пока `visible` ложно и анимация выхода не доиграла, `Dialog` крейта `prime-dialog` SHALL не выводить ничего. Показанный диалог SHALL состоять из маски `div.p-dialog-mask` и вложенного в неё окна `div.p-dialog.p-component` с `role` (по умолчанию `dialog`), `aria-modal="true"` и `aria-labelledby`, указывающим на название. Окно SHALL содержать `div.p-dialog-header` (при `show_header`), `div.p-dialog-content` с `children` и `div.p-dialog-footer` только при заданном `footer`. Маска и окно SHALL находиться в `body`, а не внутри места объявления компонента. `style`, `style_class`, `content_style`, `content_style_class`, `mask_style`, `mask_style_class` и `pt` SHALL применяться к соответствующим частям.

#### Scenario: Скрыт по умолчанию
- **WHEN** `visible` ложно и диалог ни разу не показывали
- **THEN** в документе нет `.p-dialog` и `.p-dialog-mask`

#### Scenario: Показан
- **WHEN** `visible` становится истинным при `header="Header"`
- **THEN** в `body` есть `.p-dialog-mask > .p-dialog[role=dialog][aria-modal=true]` с названием «Header»

### Requirement: Заголовок и кнопки
Заголовок SHALL содержать название `span.p-dialog-title` (при `header` и без `header_content`) либо `header_content`, и блок `div.p-dialog-header-actions` с кнопкой максимизации `p-dialog-maximize-button` при `maximizable` и кнопкой закрытия `p-dialog-close-button` при `closable`. Кнопки SHALL иметь `aria-label` из локали (`aria.close`, `aria.maximizeLabel` / `aria.minimizeLabel`). При `headless` SHALL выводиться только переданное представление, без заголовка, содержимого и подвала.

#### Scenario: Кнопки по настройкам
- **WHEN** `maximizable` включён, `closable` выключен
- **THEN** в заголовке есть кнопка максимизации и нет кнопки закрытия

### Requirement: Значение видимости
`visible` SHALL быть двусторонним сигналом `bool`: истина показывает диалог, ложь закрывает. Закрытие пользователем (кнопка, Escape, маска) SHALL записывать ложь в `visible`.

#### Scenario: Закрытие кнопкой
- **WHEN** диалог показан и пользователь нажимает кнопку закрытия
- **THEN** `visible` ложно, а окно убирается после анимации выхода

### Requirement: Позиция
`position` SHALL выбирать положение окна на экране: центр, край или угол. Маска SHALL получать класс `p-dialog-<position>` для положений, отличных от центра, и выравнивать окно соответствующим образом.

#### Scenario: Угол
- **WHEN** `position=BottomRight`
- **THEN** окно прижато к правому нижнему углу окна браузера, у маски `p-dialog-bottomright`

### Requirement: Модальность
При `modal` маска SHALL получать `p-overlay-mask`, перехватывать клики и блокировать прокрутку страницы на время показа. Без `modal` маска SHALL оставаться в разметке, но не перехватывать клики страницы и не блокировать прокрутку. При `dismissable_mask` и `closable` нажатие кнопки мыши на самой маске (не на окне) SHALL закрывать диалог.

#### Scenario: Немодальный диалог
- **WHEN** `modal` выключен и пользователь кликает по странице за окном
- **THEN** клик достигает страницы, диалог остаётся видимым

#### Scenario: Закрытие по маске
- **WHEN** `modal` и `dismissable_mask` включены и пользователь нажимает на маску вне окна
- **THEN** диалог закрывается

#### Scenario: Нажатие внутри окна
- **WHEN** `dismissable_mask` включён и пользователь нажимает внутри окна
- **THEN** диалог остаётся видимым

### Requirement: Escape
При `close_on_escape` и `closable` клавиша `Escape` SHALL закрывать диалог; при нескольких открытых — только верхний по `z-index`. После закрытия слушатель SHALL быть снят.

#### Scenario: Вложенные диалоги
- **WHEN** открыты два диалога и нажат Escape
- **THEN** закрывается только верхний

### Requirement: Перетаскивание
При `draggable` нажатие кнопки мыши на заголовке (но не на блоке кнопок) и движение указателя SHALL перемещать окно (`position: fixed`, `left`, `top`, `margin: 0`). При `keep_in_viewport` окно SHALL не выходить за пределы экрана и за `min_x` / `min_y`. После отпускания кнопки SHALL вызываться `on_drag_end`. Без `draggable` окно не перемещается.

#### Scenario: Перетаскивание заголовка
- **WHEN** пользователь нажимает на заголовок и сдвигает указатель на 50 px вправо
- **THEN** окно сдвинулось на 50 px вправо

#### Scenario: Граница экрана
- **WHEN** окно тянут за левую границу экрана при `keep_in_viewport`
- **THEN** левый край окна не заходит левее `min_x`

### Requirement: Изменение размера
При `resizable` окно SHALL содержать `div.p-resizable-handle`; нажатие на него и движение указателя SHALL менять ширину и высоту окна и высоту содержимого, пока окно помещается в экран и не меньше `min-width` / `min-height`. Начало SHALL вызывать `on_resize_init`, конец — `on_resize_end`.

#### Scenario: Расширение
- **WHEN** пользователь тянет ручку на 40 px вправо
- **THEN** окно стало шире

### Requirement: Максимизация
При `maximizable` кнопка максимизации SHALL переключать на окне класс `p-dialog-maximized` и значок кнопки (`windowmaximize` / `windowminimize`), вызывать `on_maximize` и блокировать прокрутку страницы, если диалог немодальный и без `block_scroll`. При закрытии максимизация SHALL сбрасываться.

#### Scenario: Разворачивание
- **WHEN** пользователь нажимает кнопку максимизации
- **THEN** у окна есть `p-dialog-maximized`, `on_maximize` вызван

### Requirement: Адаптивная ширина
При `breakpoints` в `head` SHALL добавляться элемент `style` с правилами `@media screen and (max-width: <ширина>)`, задающими ширину окна без `p-dialog-maximized`; после закрытия и размонтирования элемент SHALL удаляться.

#### Scenario: Правило для ширины
- **WHEN** `breakpoints` содержит `("575px", "90vw")`
- **THEN** в `head` есть правило с `max-width: 575px` и `width: 90vw`

### Requirement: Анимация
При показе окно SHALL получать класс `p-dialog-enter-active`, при закрытии — `p-dialog-leave-active`; модальная маска — `p-overlay-mask-enter-active` и `p-overlay-mask-leave-active`. Окно SHALL оставаться в DOM до `animationend` (или до истечения запасного времени, если анимации нет).

#### Scenario: Выход
- **WHEN** диалог закрывают
- **THEN** сначала у окна `p-dialog-leave-active`, после окончания анимации окна и маски нет в документе

### Requirement: Фокус
Пока окно показано, при `focus_trap` `Tab` и `Shift+Tab` SHALL циклически перемещать фокус внутри окна. При `focus_on_show` фокус SHALL получать первый фокусируемый элемент содержимого, а при его отсутствии — подвала, затем заголовка.

#### Scenario: Фокус при показе
- **WHEN** в содержимом есть поле ввода и диалог показан
- **THEN** фокус на этом поле

### Requirement: Слои
При `auto_z_index` окно SHALL получать `z-index` группы модальных окон со смещением `base_z_index`, а маска — на единицу ниже; индекс SHALL освобождаться после закрытия.

#### Scenario: Два диалога
- **WHEN** открыты два диалога
- **THEN** `z-index` позже открытого выше

### Requirement: События
`on_show` SHALL вызываться один раз после показа, `on_hide` — один раз при закрытии любым способом.

#### Scenario: Порядок событий
- **WHEN** диалог показан и закрыт
- **THEN** `on_show` вызван один раз до `on_hide`

### Requirement: Рендер без браузера
Вне браузера (SSR) `Dialog` SHALL отрисовываться скрытым и не обращаться к `window` и `document`; гидратация SHALL не расходиться с серверной разметкой.

#### Scenario: SSR
- **WHEN** страница с `Dialog` отрисована на сервере
- **THEN** маски и окна в разметке нет
