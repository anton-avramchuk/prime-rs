# blockui Specification

## Purpose
BlockUI блокирует документ или выбранный элемент полупрозрачной маской и показывает по центру произвольное содержимое, пока `blocked` включено.

## ADDED Requirements

### Requirement: Показ и скрытие маски
`BlockUI` крейта `prime-blockui` SHALL при `blocked = false` не выводить маску. При `blocked = true` SHALL выводить `div.p-blockui.p-blockui-mask.p-overlay-mask.p-overlay-mask-enter-active` с `display: flex`, `aria-busy="true"`, `style_class`, `style` и `pt` и содержимым `children`. При снятии блокировки маска SHALL получать `p-overlay-mask-leave-active` и `aria-busy="false"`, а по окончании анимации (`animationend` либо запасной таймер) SHALL быть убрана из DOM. Вне браузера (SSR) маска SHALL не выводиться ни при каком `blocked`.

#### Scenario: Включение
- **WHEN** `blocked` становится `true`
- **THEN** в DOM появляется маска с `aria-busy="true"` и классом `p-overlay-mask-enter-active`

#### Scenario: Выключение
- **WHEN** `blocked` становится `false`
- **THEN** маска получает `p-overlay-mask-leave-active`, затем исчезает из DOM

#### Scenario: SSR
- **WHEN** страница с `blocked = true` отрисована на сервере
- **THEN** в разметке маски нет

### Requirement: Блокировка элемента
Если задан `target`, маска SHALL выводиться внутрь целевого элемента (как последний ребёнок) и растягиваться на него классом темы `p-blockui-mask.p-overlay-mask` (`position: absolute`); целевой элемент на время показа SHALL получать `position: relative`, а прежнее значение инлайн-свойства SHALL возвращаться после выхода маски. Маска SHALL не иметь класса `p-blockui-mask-document`, прокрутка страницы SHALL не блокироваться.

#### Scenario: Маска в цели
- **WHEN** `blocked` включается при заданном `target`
- **THEN** маска лежит внутри целевого элемента, у него `position: relative`, `body` без блокировки прокрутки

#### Scenario: Возврат позиции
- **WHEN** блокировка снята и маска убрана
- **THEN** инлайн-значение `position` целевого элемента равно прежнему

### Requirement: Блокировка документа
Без `target` маска SHALL выводиться в `body` с классом `p-blockui-mask-document` (`position: fixed`) и блокировать прокрутку страницы (`prime_dom::block_body_scroll` на время показа, общий счётчик с другими модальными компонентами), а после выхода маски SHALL снимать блокировку один раз.

#### Scenario: Документ
- **WHEN** `blocked` включается без `target`
- **THEN** маска лежит в `body`, у неё класс `p-blockui-mask-document`, страница не прокручивается

#### Scenario: Снятие
- **WHEN** блокировка снята и маска убрана
- **THEN** прокрутка страницы разрешена (если нет других модальных компонентов)

### Requirement: Слой
При `auto_z_index` (по умолчанию да) маска SHALL получать `z-index` группы `Modal` (`modal + base_z_index`, через общий генератор слоёв) и освобождать его после выхода; при выключенном `auto_z_index` `z-index` не задаётся.

#### Scenario: Слой
- **WHEN** блокировка включена и `auto_z_index` по умолчанию
- **THEN** `z-index` маски не меньше базового значения группы `Modal`

### Requirement: Повторное включение и размонтирование
Включение блокировки во время анимации выхода SHALL возвращать маску в состояние «заблокировано» (без повторной вставки и без двойной блокировки прокрутки). Размонтирование компонента SHALL освобождать слой и блокировку прокрутки и убирать маску.

#### Scenario: Быстрое переключение
- **WHEN** `blocked` включают снова, пока маска играет выход
- **THEN** маска остаётся одна, `aria-busy="true"`, классы входа

#### Scenario: Размонтирование
- **WHEN** компонент размонтируется при включённой блокировке документа
- **THEN** маски нет, прокрутка страницы разрешена
