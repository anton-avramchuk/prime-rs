# dock Specification

## Purpose
Dock показывает значки команд в панели у края контейнера и управляется мышью и клавиатурой.

## ADDED Requirements

### Requirement: Разметка
`Dock` крейта `prime-dock` SHALL выводить `div.p-dock.p-component.p-dock-<position>` (`style_class`, `style`, `pt`; `p-dock-mobile`, пока окно не шире `breakpoint`) > `div.p-dock-list-container` > `ul.p-dock-list[role=menu]` с `id`, `aria-orientation` (`horizontal` для `Bottom`/`Top`, `vertical` для `Left`/`Right`), `tabindex="0"`, `aria-label`, `aria-labelledby` и `aria-activedescendant` (пока есть фокус на пункте). Каждый пункт модели SHALL выводиться как `li.p-dock-item[role=menuitem]` с `id`, `aria-label` из `label`, `aria-disabled`, `data-p-disabled`, `data-p-focused`, классами `p-focus` (пункт под фокусом), `p-disabled` и классом пункта, а внутри — `div.p-dock-item-content` > `a.p-dock-item-link[aria-hidden=true]` с `href` (из `url`), `target`, `title` и классом ссылки.

#### Scenario: Состав
- **WHEN** отрисован `Dock` с тремя пунктами
- **THEN** выведены три `li.p-dock-item` с ссылками `a.p-dock-item-link`, у корня класс `p-dock-bottom`

#### Scenario: Положение
- **WHEN** задан `position = Left`
- **THEN** у корня класс `p-dock-left`, у списка `aria-orientation="vertical"`

### Requirement: Значок, бейдж и представление
Если у пункта есть `icon` и нет представления `item`, SHALL выводиться `span.p-dock-item-icon` со значком; представление `item` SHALL заменять значок и получать пункт. При `badge` SHALL выводиться `Badge` со значением. Пункт с `disabled` SHALL не вызывать `command` по клику.

#### Scenario: Значок
- **WHEN** у пункта задан `icon`
- **THEN** внутри ссылки есть `.p-dock-item-icon` со значком

#### Scenario: Шаблон
- **WHEN** задан `item`
- **THEN** внутри ссылки вместо значка переданное представление

### Requirement: Клик
Клик по пункту SHALL вызывать его `command` с исходным событием и пунктом, если пункт не отключён.

#### Scenario: Команда
- **WHEN** пользователь кликает по пункту с `command`
- **THEN** `command` вызвана с этим пунктом

#### Scenario: Отключённый пункт
- **WHEN** пользователь кликает по пункту с `disabled`
- **THEN** `command` не вызвана

### Requirement: Клавиатура и фокус
Получение фокуса списком SHALL вызывать `on_focus` и ставить фокус на первый доступный пункт; потеря фокуса — `on_blur` и сброс. Для `Bottom`/`Top` клавиша → SHALL переносить фокус на следующий доступный пункт, ← — на предыдущий; для `Left`/`Right` то же делают ↓ и ↑ (перпендикулярные стрелки отменяются без действия). `Home` SHALL переносить фокус на первый, `End` — на последний доступный пункт, `Enter` и `Space` — кликать по ссылке пункта под фокусом. Отключённые пункты SHALL пропускаться; на границах фокус SHALL оставаться на крайнем пункте.

#### Scenario: Навигация
- **WHEN** список в фокусе, положение `Bottom`, нажато →
- **THEN** фокус на втором доступном пункте, `aria-activedescendant` указывает на него

#### Scenario: Отключённый пункт
- **WHEN** второй пункт отключён и нажато → с первого
- **THEN** фокус на третьем

#### Scenario: Enter
- **WHEN** фокус на пункте с `command` и нажат Enter
- **THEN** `command` вызвана

### Requirement: Мобильный режим
Если окно не шире `breakpoint` (по умолчанию `960px`), корень SHALL получать класс `p-dock-mobile`; класс SHALL меняться при изменении ширины окна. Вне браузера (SSR) класса нет.

#### Scenario: Узкое окно
- **WHEN** ширина окна меньше `breakpoint`
- **THEN** у корня есть `p-dock-mobile`

### Requirement: Рендер без браузера
Вне браузера (SSR) `Dock` SHALL отрисовываться полностью; `window` и `document` SHALL не использоваться.

#### Scenario: SSR
- **WHEN** страница с `Dock` отрисована на сервере
- **THEN** в разметке есть список пунктов
