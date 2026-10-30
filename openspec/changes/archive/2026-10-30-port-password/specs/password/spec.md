## Purpose
Password — поле ввода пароля с индикатором надёжности, переключателем показа и очисткой.

## ADDED Requirements

### Requirement: Разметка Password
Компонент `Password` крейта `prime-password` SHALL выводить корень `span.p-password.p-component.p-inputwrapper` с полем `InputText` (класс `p-password-input`, `type=password`). Корень SHALL получать `p-inputwrapper-filled` при непустом значении, `p-inputwrapper-focus` при фокусе, `p-variant-filled` при `variant = Filled`, `p-password-fluid` при `fluid` (или внутри `Fluid`) и `data-p`. Свойства `placeholder`, `autocomplete`, `input_id`, `max_length`, `disabled`, `readonly`, `invalid`, `required`, `size`, `aria_label`, `input_style`, `input_style_class` SHALL передаваться полю; `style`, `style_class` и `pt` — корню.

#### Scenario: Значение по умолчанию
- **WHEN** компонент отрисован без свойств
- **THEN** внутри корня есть `input.p-password-input.p-inputtext[type=password]`, панели нет

#### Scenario: Заполнено
- **WHEN** `value` непусто
- **THEN** корень имеет класс `p-inputwrapper-filled`

### Requirement: Двустороннее значение
`value: RwSignal<String>` SHALL отражаться в поле, а ввод пользователя SHALL записываться в сигнал. Без `value` значение SHALL храниться внутри компонента.

#### Scenario: Ввод
- **WHEN** пользователь вводит «abc»
- **THEN** сигнал `value` равен «abc»

### Requirement: Показ пароля
При `toggle_mask` компонент SHALL выводить значок `svg.p-password-toggle-mask-icon.p-password-unmask-icon` (`EyeIcon`), по клику на который тип поля SHALL становиться `text`, а значок — `svg.p-password-mask-icon` (`EyeSlashIcon`); повторный клик SHALL возвращать `password`. Шаблоны `show_icon` и `hide_icon` SHALL заменять значки. Без `toggle_mask` значков нет.

#### Scenario: Переключение
- **WHEN** `toggle_mask` включён и нажат значок «глаз»
- **THEN** `type` поля равен `text`, виден значок `p-password-mask-icon`

### Requirement: Очистка
При `show_clear` и непустом значении компонент SHALL выводить `svg.p-password-clear-icon`; клик SHALL очищать значение и вызывать `on_clear`. Шаблон `clear_icon` SHALL заменять значок. При пустом значении значка нет.

#### Scenario: Клик по значку
- **WHEN** `show_clear` включён, значение «abc», нажат значок очистки
- **THEN** значение пусто, значок исчез, `on_clear` вызван

### Requirement: Оценка надёжности
Оценка SHALL быть: пусто — подсказка; непусто — слабая; не менее 6 символов и два вида из {строчные латинские, прописные латинские, цифры} — средняя; не менее 8 символов и все три вида — сильная. `strength` SHALL заменять оценку (функция `String → PasswordStrength`). Логика SHALL быть чистой функцией с табличным тестом.

#### Scenario: Границы
- **WHEN** оцениваются «a», «abcDEF», «abcDEF1», «abcDEF12»
- **THEN** результат — слабая, средняя, средняя, сильная

#### Scenario: Пустое значение
- **WHEN** значение пусто
- **THEN** оценка — подсказка (нет уровня)

### Requirement: Панель надёжности
При `feedback` (по умолчанию `true`) панель `div.p-password-overlay.p-component` SHALL открываться по фокусу и по вводу, если она закрыта, и закрываться по потере фокуса и по `Escape`. Внутри — `div.p-password-content` > `div.p-password-meter` > `div.p-password-meter-label` и `div.p-password-meter-text`. Метка SHALL получать класс `p-password-meter-<weak|medium|strong>` и ширину 33.33 % / 66.66 % / 100 %; текст — подпись уровня (из `weak_label`, `medium_label`, `strong_label`, `prompt_label` или локали). Панель SHALL быть выровнена под полем, иметь z-index группы `Overlay`, а `header`, `footer`, `content` SHALL заменять соответствующие части. При `feedback = false` панели нет.

#### Scenario: Открытие
- **WHEN** поле получает фокус при `feedback`
- **THEN** панель показана с текстом «Enter a password»

#### Scenario: Уровень
- **WHEN** введено «abcDEF12»
- **THEN** метка имеет класс `p-password-meter-strong`, ширину 100 %, текст «Strong»

#### Scenario: Закрытие
- **WHEN** нажат `Escape` или поле теряет фокус
- **THEN** панель убрана

### Requirement: Обратные вызовы и локаль
`on_focus` и `on_blur` SHALL вызываться при фокусе и потере фокуса поля. Подписи SHALL по умолчанию браться из локали (`passwordPrompt`, `weak`, `medium`, `strong`) и реактивно меняться вместе с ней.

#### Scenario: Подпись из локали
- **WHEN** локаль переопределяет `weak`
- **THEN** панель показывает переопределённый текст для слабого пароля
