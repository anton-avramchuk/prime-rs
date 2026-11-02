# toast Specification

## Purpose
Toast показывает кратковременные уведомления в углу экрана; сообщения поступают от общего сервиса из любого места приложения, исчезают сами или закрываются пользователем.

## Requirements

### Requirement: Разметка контейнера
`Toast` крейта `prime-toast` SHALL выводить `div.p-toast.p-component` с классом `p-toast-<position>`, инлайн-стилем `position: fixed` и сдвигами по положению (`top`/`right`/`bottom`/`left` 20px у соответствующих краёв, 50% у центра) и `data-p` с положением. Без сообщений контейнер SHALL оставаться пустым. `style_class` и `pt` SHALL применяться к контейнеру.

#### Scenario: Пустой контейнер
- **WHEN** отрисован `Toast` без сообщений
- **THEN** выведен `.p-toast.p-toast-top-right` без `.p-toast-message`

#### Scenario: Положение
- **WHEN** `position=BottomCenter`
- **THEN** контейнер имеет `p-toast-bottom-center`, `bottom: 20px` и `left: 50%`

### Requirement: Разметка сообщения
Каждое сообщение SHALL быть `div.p-toast-message` с классом `p-toast-message-<severity>` (`info` без явной серьёзности), `role="alert"`, `aria-live="assertive"`, `aria-atomic="true"` и `data-p` с серьёзностью, содержащим `div.p-toast-message-content` со значком (`Check`, `InfoCircle`, `TimesCircle`, `ExclamationTriangle` по серьёзности либо `span.p-toast-message-icon` с классом из `icon`), `div.p-toast-message-text` (`div.p-toast-summary`, `div.p-toast-detail`) и, если `closable` не ложно, `button.p-toast-close-button` с `aria-label` из локали. `style_class` и `content_style_class` сообщения SHALL добавляться к соответствующим элементам.

#### Scenario: Серьёзность и тексты
- **WHEN** добавлено сообщение `Success` с `summary="Done"` и `detail="Saved"`
- **THEN** выведено `.p-toast-message-success` с «Done» в `.p-toast-summary` и «Saved» в `.p-toast-detail`

#### Scenario: Без кнопки закрытия
- **WHEN** сообщение добавлено с `closable=false`
- **THEN** кнопки закрытия нет

### Requirement: Сервис сообщений
`MessageService` SHALL предоставляться через контекст и принимать `add`, `add_all`, `clear` и `clear_key`. Каждый `Toast` SHALL принимать только сообщения со своим `key` (оба без ключа — совпадение); `clear()` SHALL убирать сообщения всех `Toast`, `clear_key(k)` — только `Toast` с ключом `k`. После размонтирования `Toast` сервис SHALL не вызывать его.

#### Scenario: Ключи
- **WHEN** есть два `Toast` с ключами «a» и «b» и добавлено сообщение с ключом «a»
- **THEN** оно показано только в первом

#### Scenario: Очистка по ключу
- **WHEN** в обоих `Toast` есть сообщения и вызван `clear_key("a")`
- **THEN** сообщения первого закрываются, второго — остаются

### Requirement: Время жизни
Сообщение SHALL закрываться само через `life` сообщения, а при его отсутствии — через `life` `Toast` (по умолчанию 3000 мс); `sticky` SHALL отключать закрытие по времени. Наведение мыши на сообщение SHALL останавливать таймер, уход курсора — запускать заново (если сообщение не закрывается).

#### Scenario: Закрытие по времени
- **WHEN** сообщение добавлено с `life=200`
- **THEN** через 200 мс начинается анимация выхода, затем сообщение убирается

#### Scenario: Наведение
- **WHEN** курсор находится над сообщением после истечения времени жизни
- **THEN** сообщение остаётся, пока курсор не уйдёт

### Requirement: Закрытие и анимация
Кнопка закрытия и `clear` SHALL запускать анимацию выхода сообщения (`p-toast-message-leave-active`); вход SHALL сопровождаться `p-toast-message-enter-active`. После окончания анимации (`animationend` либо запасное время, если анимации нет) сообщение SHALL убираться из DOM, а `on_close` — вызываться с этим сообщением.

#### Scenario: Закрытие кнопкой
- **WHEN** пользователь нажимает кнопку закрытия
- **THEN** сообщение получает `p-toast-message-leave-active`, затем убирается, `on_close` вызван один раз

### Requirement: Дубликаты
При `prevent_open_duplicates` сообщение с теми же `summary`, `detail` и серьёзностью, что у уже показанного, SHALL не добавляться. При `prevent_duplicates` сообщение, совпадающее с любым ранее добавленным в этот `Toast`, SHALL не добавляться.

#### Scenario: Дубликат показанного
- **WHEN** `prevent_open_duplicates` включён и то же сообщение добавлено дважды подряд
- **THEN** показано одно

### Requirement: Слой
Пока есть сообщения, при `auto_z_index` контейнер SHALL иметь `z-index` группы модальных окон со смещением `base_z_index`; после ухода последнего сообщения индекс SHALL освобождаться.

#### Scenario: Слой
- **WHEN** сообщение показано, затем закрыто
- **THEN** `z-index` контейнера был задан и после закрытия освобождён

### Requirement: Свои представления
`message` SHALL заменять значок и тексты внутри содержимого сообщения (кнопка закрытия остаётся); `headless` SHALL заменять всё внутри `div.p-toast-message` и получать функцию закрытия сообщения.

#### Scenario: Headless
- **WHEN** задан `headless` и добавлено сообщение
- **THEN** внутри `.p-toast-message` только переданное представление, а его функция закрытия закрывает сообщение

### Requirement: Адаптивность
При `breakpoints` в `head` SHALL добавляться `style` с правилами `@media screen and (max-width: <ширина>)` для контейнера; при размонтировании он SHALL удаляться.

#### Scenario: Правило
- **WHEN** `breakpoints` содержит `920px` со стилем `width: 50%`
- **THEN** в `head` есть правило с `max-width: 920px` и `width: 50% !important`

### Requirement: Рендер без браузера
Вне браузера (SSR) `Toast` SHALL отрисовываться пустым контейнером и не обращаться к `window` и `document`; сервис SHALL работать без браузера (сообщения на сервере не показываются).

#### Scenario: SSR
- **WHEN** страница с `Toast` отрисована на сервере
- **THEN** в разметке есть пустой `.p-toast`
