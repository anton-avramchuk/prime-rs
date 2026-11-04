# autocomplete Specification

## Purpose
AutoComplete — поле ввода, предлагающее варианты по набранному тексту.

## ADDED Requirements

### Requirement: Разметка
`AutoComplete` крейта `prime-autocomplete` SHALL выводить `div.p-autocomplete.p-component.p-inputwrapper` (классы состояния `p-invalid`, `p-focus`, `p-inputwrapper-filled|focus`, `p-autocomplete-open|clearable|fluid`). В одиночном режиме SHALL быть `input.p-autocomplete-input.p-inputtext[role=combobox][aria-autocomplete=list]` (`aria-expanded`, `aria-controls`, `aria-activedescendant`, `placeholder`, `type`, `disabled`, `readonly`) со значением — подписью выбранного варианта либо набранным текстом; в множественном — `ul.p-autocomplete-input-multiple[role=listbox]` с чипами `li.p-autocomplete-chip-item` и `li.p-autocomplete-input-chip` > `input.p-autocomplete-input`. Кнопка `button.p-autocomplete-dropdown` SHALL выводиться при `dropdown`, значок очистки `p-autocomplete-clear-icon` — при `show_clear`, непустом значении и доступном поле, спиннер `p-autocomplete-loader` — пока идёт запрос. Панель `div.p-autocomplete-overlay` SHALL выводиться только в открытом состоянии.

#### Scenario: Одиночный режим
- **WHEN** значение «Apple»
- **THEN** в поле «Apple», панели нет

#### Scenario: Множественный режим
- **WHEN** выбраны «A» и «B»
- **THEN** выведены два чипа и поле ввода

### Requirement: Запрос вариантов
Ввод текста длиной не менее `min_length` SHALL после `delay` вызывать `on_complete` с запросом (по умолчанию 1 символ и 300 мс); более короткий текст и очистка поля SHALL закрывать панель. До обновления `suggestions` SHALL показываться спиннер. Обновлённые варианты SHALL открывать панель (при пустом списке — сообщение `empty_message`, если `show_empty_message`). В одиночном режиме без `force_selection` набранный текст SHALL становиться значением. Кнопка-стрелка SHALL запрашивать варианты (`dropdown_mode`: пустой запрос либо текущий текст) или закрывать открытую панель; `complete_on_focus` SHALL запрашивать варианты при первом фокусе.

#### Scenario: Запрос
- **WHEN** пользователь вводит «ap» и ждёт `delay`
- **THEN** вызван `on_complete` с запросом «ap»

### Requirement: Выбор
Клик по варианту либо `Enter` на сфокусированном SHALL записывать значение варианта (в `multiple` — добавлять чип, если `unique` и его ещё нет), вызывать `on_select` и закрывать панель. Недоступные варианты и заголовки групп SHALL не выбираться и пропускаться клавишами. Чип SHALL удаляться кнопкой, `Backspace` в пустом поле удаляет последний чип. `force_selection` SHALL очищать ввод, не совпавший ни с одним вариантом. Очистка SHALL сбрасывать значение и вызывать `on_clear`.

#### Scenario: Выбор
- **WHEN** панель открыта и пользователь кликает по варианту «Apple»
- **THEN** значение «Apple», панель закрыта, `on_select` вызван

### Requirement: Клавиатура
↓ ↑ SHALL ходить по допустимым вариантам открытой панели, `Alt+↑` — выбирать сфокусированный и закрывать; `Escape` — закрывать с возвратом фокуса; `Tab` — выбирать сфокусированный вариант и закрывать. В `multiple` ← на пустом поле SHALL переводить на чипы, ← → ходят по чипам, `Backspace` удаляет сфокусированный. При `typeahead = false` в `multiple` набранное SHALL добавляться по `Enter`, `separator`, `Tab` (`add_on_tab`) и потере фокуса (`add_on_blur`); вставка со `separator` разбивается на чипы.

#### Scenario: Стрелки
- **WHEN** панель открыта и нажаты ↓ ↓
- **THEN** сфокусирован второй допустимый вариант

### Requirement: Шаблоны и состояния
`item` SHALL заменять содержимое варианта, `group_view` — заголовок группы, `selected_item` — содержимое чипа. При `disabled` поле SHALL не реагировать на ввод и клики; клик вне компонента SHALL закрывать панель.

#### Scenario: Свой вариант
- **WHEN** задан `item`
- **THEN** в `li.p-autocomplete-option` выведено его содержимое
