# cascadeselect Specification

## Purpose
CascadeSelect позволяет выбрать значение из вложенных списков, раскрывающихся каскадом.

## Requirements

### Requirement: Разметка
`CascadeSelect` крейта `prime-cascadeselect` SHALL принимать `Vec<CascadeOption>` и выводить `div.p-cascadeselect.p-component.p-inputwrapper` с классами состояния, скрытое поле `input[role=combobox][readonly]` (`aria-haspopup="tree"`, `aria-expanded`, `aria-controls`, `aria-activedescendant`), метку `span.p-cascadeselect-label` (подпись выбранного листа либо `placeholder` с `p-placeholder`; без подсказки у пустого — `p-cascadeselect-label-empty`) и кнопку `div.p-cascadeselect-dropdown` со стрелкой (при `loading` — спиннер). Панель `div.p-cascadeselect-overlay` > `div.p-cascadeselect-list-container` > `ul.p-cascadeselect-list[role=tree]` SHALL выводиться только при открытом поле. Вариант SHALL быть `li.p-cascadeselect-option[role=treeitem]` с `aria-level`, `aria-setsize`, `aria-posinset`, `aria-label`, `aria-selected` (у листьев), `id` и содержимым `div.p-cascadeselect-option-content` > `span.p-cascadeselect-option-text`; у группы — `p-cascadeselect-option-group` и `span.p-cascadeselect-group-icon`.

#### Scenario: Закрыто
- **WHEN** выбран лист «Sydney»
- **THEN** метка содержит «Sydney», панели нет

#### Scenario: Открыто
- **WHEN** поле открыто
- **THEN** выведен корневой список с группами и листьями

### Requirement: Раскрытие и выбор
Клик по полю SHALL открывать или закрывать панель. Клик по группе SHALL раскрывать её вложенный список `ul.p-cascadeselect-option-list[role=group]` (`p-cascadeselect-option-active` у группы), повторный клик — сворачивать. Клик по листу SHALL записывать его значение, вызывать `on_change` и закрывать панель. Наведение SHALL раскрывать группы (до первого клика — только корневые; после клика либо при выбранном значении — любые); на узком окне путь SHALL не меняться. При открытии SHALL раскрываться путь к выбранному значению. Клик вне, `resize` SHALL закрывать панель; закрытие сбрасывает путь.

#### Scenario: Каскад
- **WHEN** пользователь кликает по группе корня, затем по группе в раскрывшемся списке
- **THEN** выведены два вложенных списка, у обеих групп `p-cascadeselect-option-active`

#### Scenario: Лист
- **WHEN** пользователь кликает по листу
- **THEN** значение записано, панель закрыта, метка обновлена

### Requirement: Очистка
При `show_clear`, непустом значении и доступном поле SHALL выводиться значок `p-cascadeselect-clear-icon`; клик по нему SHALL очищать значение, вызывать `on_clear` и не открывать панель.

#### Scenario: Очистка
- **WHEN** пользователь кликает по значку очистки
- **THEN** значение пусто, метка показывает подсказку

### Requirement: Клавиатура
На скрытом поле ↓ SHALL открывать панель, а в открытой — переходить к следующему допустимому варианту уровня; фокус на группу раскрывает её список. ↑ — к предыдущему (при закрытой панели открывает её), `Alt+↑` — закрывать. → на группе SHALL раскрывать её и переходить на первый вариант вложенного списка, ← — закрывать список и возвращать фокус на группу-родителя. `Home`/`End`, `Enter`/`Space` (клик по варианту; на листе — закрытие), `Escape` (закрыть и вернуть фокус), `Tab` (закрыть панель), `Backspace` (очистка при `show_clear`) и поиск по началу подписи SHALL работать как в PrimeNG. Недоступные варианты SHALL пропускаться.

#### Scenario: Вход в группу
- **WHEN** панель открыта, фокус на группе и нажата →
- **THEN** список группы раскрыт, фокус на его первом варианте

### Requirement: Мобильный режим
Пока окно не шире `breakpoint`, у корня SHALL быть `p-cascadeselect-mobile`, у панели — `p-cascadeselect-mobile-active`, а наведение SHALL не менять путь.

#### Scenario: Узкое окно
- **WHEN** окно уже `breakpoint`
- **THEN** у корня есть `p-cascadeselect-mobile`
